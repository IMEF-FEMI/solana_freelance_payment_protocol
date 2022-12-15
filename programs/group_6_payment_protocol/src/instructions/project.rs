use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::system_program::Transfer;

use crate::state::project_info::ProjectInfo;
use crate::state::project_info::ProjectStatus;
use crate::state::Multisig;

pub fn initialize_project(
    ctx: Context<InitializeProjectContext>,
    total_project_funds: u64,
    milestones: u8,
) -> Result<()> {
    //initialize multisig
    let multisig = &mut ctx.accounts.multisig;
    let owners = vec![
        ctx.accounts.client.key(),
        ctx.accounts.freelancer.key(),
        ctx.accounts.observer.key(),
    ];
    Multisig::init(multisig, owners, 2)?;

    //initialize data acct
    let project_info_account = &mut ctx.accounts.project_info_account;
    project_info_account.client = ctx.accounts.client.key();
    project_info_account.freelancer = ctx.accounts.freelancer.key();
    project_info_account.total_project_funds = total_project_funds;
    project_info_account.milestones = milestones;
    project_info_account.multisig = multisig.key();
    project_info_account.milestones_reached = 0;
    project_info_account.milestone_funds_withdrawn = 0;

    //transfer funds and lock funds from client
    system_program::transfer(
        ctx.accounts.transfer_funds_from_client(),
        total_project_funds,
    )?;

    Ok(())
}

// update the state of the project
//The only way this can be invoked
// is via a recursive call from execute_transaction -> start_project.
pub fn start_project(ctx: Context<MultisigAuth>) -> Result<()> {
    let project_info = &mut ctx.accounts.project_info_account;
    project_info.status = ProjectStatus::Running.to_code();
    Ok(())
}
// mark current milestone as completed
// so freelancer can withdraw funds for the milestone
//The only way this can be invoked
// is via a recursive call from execute_transaction -> start_project.
pub fn mark_current_milestone_completed(ctx: Context<MultisigAuth>) -> Result<()> {
    let project_info = &mut ctx.accounts.project_info_account;
    project_info.milestones_reached += 1;
    Ok(())
}
// withdraw funds for completed milestones
// so freelancer can withdraw funds for the milestone
//The only way this can be invoked
// is via a recursive call from execute_transaction -> start_project.
pub fn withdraw_milestone_funds(ctx: Context<WithdrawMilestoneFundsContext>) -> Result<()> {
    let project_info = &mut ctx.accounts.project_info_account;

    let amount_per_milestone = project_info
        .total_project_funds
        .checked_div(project_info.milestones.into())
        .unwrap();

    let milestone_funds_withdrawn_so_far = project_info.milestone_funds_withdrawn;
    let milestones_not_withdrawn = project_info
        .milestones_reached
        .checked_sub(milestone_funds_withdrawn_so_far)
        .unwrap();

    let mut amount_to_withdraw = amount_per_milestone
        .checked_mul(milestones_not_withdrawn.into())
        .unwrap();

    let escrow_balance = ctx.accounts.token_escrow.lamports();
    if project_info.milestones_reached == project_info.milestones {
        amount_to_withdraw = escrow_balance;
        project_info.status = ProjectStatus::Completed.to_code();
    }

    let bump = *ctx.bumps.get("token_escrow").unwrap();
    let project_info_key = ctx.accounts.project_info_account.clone().key();

    let signer_seed: &[&[&[u8]]] = &[&[b"token_escrow", project_info_key.as_ref(), &[bump]]];

    system_program::transfer(
        ctx.accounts
            .transfer_funds_to_freelancer()
            .with_signer(signer_seed),
        amount_to_withdraw,
    )?;

    ctx.accounts.project_info_account.milestone_funds_withdrawn += 1;
    Ok(())
}

///collective effort to stop the project regardless of the current state
pub fn stop_project(ctx: Context<MultisigAuth>) -> Result<()> {
    ctx.accounts.project_info_account.status = ProjectStatus::Cancelled.to_code();
    Ok(())
}
///stop the project before it gets started(project status changes to running)
pub fn cancel_project(ctx: Context<StopProjectContext>) -> Result<()> {
    //transfer funds back to client
    require!(
        ctx.accounts.client.key() == ctx.accounts.project_info_account.client,
        ErrorCode::ClientOnly
    );
    require!(
        ProjectStatus::from(ctx.accounts.project_info_account.status).unwrap()
            == ProjectStatus::Pending
            || ProjectStatus::from(ctx.accounts.project_info_account.status).unwrap()
                == ProjectStatus::Cancelled,
        ErrorCode::InvalidStatus
    );

    let bump = *ctx.bumps.get("token_escrow").unwrap();
    let project_info_account = ctx.accounts.project_info_account.key().clone();

    let signer_seed: &[&[&[u8]]] = &[&[b"token_escrow", project_info_account.as_ref(), &[bump]]];
    let transfer_accounts = system_program::Transfer {
        from: ctx.accounts.token_escrow.to_account_info().clone(),
        to: ctx.accounts.client.to_account_info().clone(),
    };
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_accounts,
    );
    system_program::transfer(
        transfer_ctx.with_signer(signer_seed),
        ctx.accounts.token_escrow.lamports(),
    )?;

    Ok(())
}
#[derive(Accounts)]
pub struct InitializeProjectContext<'info> {
    #[account(
        init,
        payer = client,
        space = 8 + ProjectInfo::MAX_SIZE,
        seeds = [
            b"project_info_account",
            client.key().as_ref(),
            freelancer.key().as_ref(),
        ],
        bump,
    )]
    project_info_account: Box<Account<'info, ProjectInfo>>,
    #[account(
        init,
        payer = client,
        space = 8 + Multisig::MAX_SIZE,
        seeds = [b"multisig", project_info_account.key().as_ref()],
        bump,
    )]
    multisig: Box<Account<'info, Multisig>>,
    /// CHECK:
    #[account(
        mut,
        seeds = [
            b"token_escrow",
            project_info_account.key().as_ref()
        ],
        bump
    )]
    token_escrow: AccountInfo<'info>,
    /// CHECK:
    freelancer: AccountInfo<'info>,
    /// CHECK:
    observer: AccountInfo<'info>,
    #[account(mut)]
    client: Signer<'info>,
    system_program: Program<'info, System>,
}

impl<'info> InitializeProjectContext<'info> {
    pub fn transfer_funds_from_client(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let transfer_acct = Transfer {
            from: self.client.to_account_info().clone(),
            to: self.token_escrow.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), transfer_acct)
    }
}

#[derive(Accounts)]
pub struct StopProjectContext<'info> {
    #[account(
        mut,
        close = client,
        seeds = [
            b"project_info_account",
            client.key().as_ref(),
            freelancer.key().as_ref(),
        ],
        bump,
    )]
    project_info_account: Account<'info, ProjectInfo>,
    /// CHECK:
    #[account(
        mut,
        seeds = [
            b"token_escrow",
            project_info_account.key().as_ref()
        ],
        bump
    )]
    token_escrow: AccountInfo<'info>,
    /// CHECK:
    freelancer: AccountInfo<'info>,
    #[account(mut)]
    client: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MultisigAuth<'info> {
    #[account(mut)]
    project_info_account: Box<Account<'info, ProjectInfo>>,
    #[account(
        seeds = [b"multisig", project_info_account.key().as_ref()],
        bump,
    )]
    multisig_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawMilestoneFundsContext<'info> {
    #[account(mut)]
    project_info_account: Box<Account<'info, ProjectInfo>>,
    /// CHECK:
    #[account(
        mut,
        seeds = [
            b"token_escrow",
            project_info_account.key().as_ref()
        ],
        bump
    )]
    token_escrow: AccountInfo<'info>,

    #[account(
        mut,
        constraint = project_info_account.freelancer == freelancer.key()
    )]
    freelancer: Signer<'info>,
    system_program: Program<'info, System>,
}
impl<'info> WithdrawMilestoneFundsContext<'info> {
    pub fn transfer_funds_to_freelancer(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let transfer_acct = Transfer {
            from: self.token_escrow.to_account_info().clone(),
            to: self.freelancer.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), transfer_acct)
    }
}
