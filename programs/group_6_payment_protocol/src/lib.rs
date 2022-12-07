use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
pub use crate::errors::*;

mod errors;

#[program]
pub mod group_6_payment_protocol {
    use anchor_lang::{solana_program::native_token::LAMPORTS_PER_SOL, system_program};

    use super::*;

    pub fn start_project(
        ctx: Context<StartProjectContext>,
        total_funds_for_project: u64,
        milestones: u8,
        bump: u8,
    ) -> Result<()> {
        //initialize data acct
        let mut project_info_account = ctx.accounts.project_info_account.clone();
        project_info_account.client = ctx.accounts.client.key();
        project_info_account.freelancer = ctx.accounts.freelancer.key();
        project_info_account.total_funds_for_project = total_funds_for_project;
        project_info_account.funds_locked = total_funds_for_project;
        project_info_account.milestones = milestones;
        project_info_account.milestones_reached = 0;
        project_info_account.signer_bump = bump;

        //transfer funds and lock funds from client
        let transfer_accounts = system_program::Transfer {
            from: ctx.accounts.client.to_account_info().clone(),
            to: ctx.accounts.token_escrow.to_account_info().clone(),
        };
        let ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        system_program::transfer(ctx, LAMPORTS_PER_SOL * total_funds_for_project)?;

        Ok(())
    }

    pub fn cancel
}

#[derive(Accounts)]
pub struct StartProjectContext<'info> {
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
    project_info_account: Account<'info, ProjectInfo>,
    #[account(
        seeds = [
            b"token_escrow",
            project_info_account.key().as_ref()
        ],
        bump
    )]
    token_escrow: AccountInfo<'info>,
    freelancer: AccountInfo<'info>,
    #[account(mut)]
    client: Signer<'info>,
    system_program: Program<'info, System>,
}



/// state
#[account]
pub struct ProjectInfo {
    total_funds_for_project: u64,
    funds_locked: u64,
    milestones: u8,
    milestones_reached: u8,
    client: Pubkey,
    freelancer: Pubkey,
    signer_bump: u8,
}

impl ProjectInfo {
    pub const MAX_SIZE: usize = 8 //    total_funds_for_project
    + 8 //funds_locked
    + 1 //milestones
    + 1 //milestones_reached
    + 32 //client
    + 32 //freelancer
    + 1; //signer_bump
         //see more at: https://book.anchor-lang.com/anchor_references/space.html
}

// #[derive(Clone, AnchorSerialize, AnchorDeserialize)]
// pub struct InitEscrowParams {
//
// }
