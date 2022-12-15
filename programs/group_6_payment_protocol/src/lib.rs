use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

#[program]
pub mod group_6_payment_protocol {

    use super::*;

    pub fn initialize_project(
        ctx: Context<InitializeProjectContext>,
        total_funds_for_project: u64,
        milestones: u8,
    ) -> Result<()> {
        instructions::project::initialize_project(ctx, total_funds_for_project, milestones)
    }

    // update the state of the project
    //The only way this can be invoked
    // is via a recursive call from execute_transaction -> start_project.
    pub fn start_project(ctx: Context<MultisigAuth>) -> Result<()> {
        instructions::project::start_project(ctx)
    }
    ///stop the project before it gets started(project status changes to running)
    pub fn cancel_project(ctx: Context<StopProjectContext>) -> Result<()> {
        instructions::project::cancel_project(ctx)
    }
    // withdraw funds for completed milestones
    // so freelancer can withdraw funds for the milestone
    //The only way this can be invoked
    // is via a recursive call from execute_transaction -> start_project.
    pub fn withdraw_milestone_funds(ctx: Context<WithdrawMilestoneFundsContext>) -> Result<()> {
        instructions::project::withdraw_milestone_funds(ctx)
    }

    // mark current milestone as completed
    // so freelancer can withdraw funds for the milestone
    //The only way this can be invoked
    // is via a recursive call from execute_transaction -> start_project.
    pub fn mark_current_milestone_completed(ctx: Context<MultisigAuth>) -> Result<()> {
        instructions::project::mark_current_milestone_completed(ctx)
    }

    ///collective effort to stop the project regardless of the current state
    pub fn stop_project(ctx: Context<MultisigAuth>) -> Result<()> {
        instructions::project::stop_project(ctx)
    }

    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        transaction_accounts: Vec<TransactionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::multisig::create_transaction(ctx, pid, transaction_accounts, data)
    }

    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        instructions::multisig::approve(ctx)
    }
}
