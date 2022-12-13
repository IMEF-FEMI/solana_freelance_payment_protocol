use crate::errors::ErrorCode;
use crate::state::{Multisig, Transaction, TransactionAccount};
use anchor_lang::{
    prelude::*,
    solana_program::{self},
};
use solana_program::instruction::Instruction;
use std::ops::Deref;

pub fn create_transaction(
    ctx: Context<CreateTransaction>,
    program_id: Pubkey,
    transaction_accounts: Vec<TransactionAccount>,
    data: Vec<u8>,
) -> Result<()> {
    let owner_index = ctx
        .accounts
        .multisig
        .owners
        .iter()
        .position(|a| a == ctx.accounts.proposer.key)
        .ok_or(ErrorCode::InvalidOwner)?;

    let mut signers: Vec<bool> = Vec::new();
    signers.resize(ctx.accounts.multisig.owners.len(), false);
    signers[owner_index] = true;

    let tx = &mut ctx.accounts.transaction;
    tx.program_id = program_id;
    tx.accounts = transaction_accounts;
    tx.data = data;
    tx.signers = signers;
    tx.multisig = ctx.accounts.multisig.key();
    tx.did_execute = false;
    tx.seqno = ctx.accounts.multisig.seqno;
    tx.proposer = ctx.accounts.proposer.key();

    Ok(())
}
// Approve and Executes the given transaction if threshold owners have signed it.
pub fn approve(ctx: Context<Approve>) -> Result<()> {
    let owner_index = ctx
        .accounts
        .multisig
        .owners
        .iter()
        .position(|a| a == ctx.accounts.owner.key)
        .ok_or(ErrorCode::InvalidOwner)?;

    ctx.accounts.transaction.signers[owner_index] = true;

    // Do we have enough signers.
    let sig_count = ctx
        .accounts
        .transaction
        .signers
        .iter()
        .filter(|&did_sign| *did_sign)
        .count() as u64;

    if sig_count < ctx.accounts.multisig.threshold {
        return Ok(());
    }

    // Has this been executed already?
    if ctx.accounts.transaction.did_execute {
        return Err(ErrorCode::AlreadyExecuted.into());
    }

    //execute
    let mut ix: Instruction = (*ctx.accounts.transaction).deref().into();
    ix.accounts = ix
        .accounts
        .iter()
        .map(|acc| {
            let mut acc = acc.clone();
            if &acc.pubkey == ctx.accounts.multisig_signer.key {
                acc.is_signer = true;
            }
            acc
        })
        .collect();

    let bump = *ctx.bumps.get("multisig_signer").unwrap();
    let project_info_key = ctx.accounts.project_info_account.key();

    let seeds = &[b"multisig", project_info_key.as_ref(), &[bump]];
    let signer = &[&seeds[..]];
    let accounts = ctx.remaining_accounts;
    solana_program::program::invoke_signed(&ix, accounts, signer)?;

    ctx.accounts.transaction.did_execute = true;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    /// CHECK: just pubkey needed
    project_info_account: AccountInfo<'info>,
    #[account(
        seeds = [b"multisig", project_info_account.key().as_ref()],
        bump,
    )]
    multisig: Box<Account<'info, Multisig>>,
    #[account(
        init_if_needed,
        payer = proposer,
        space = 8 + Transaction::MAX_SIZE,
        seeds = [
            b"multisig",
            multisig.seqno.to_le_bytes().as_ref(),
        ],
        bump
    )]
    transaction: Box<Account<'info, Transaction>>,
    #[account(mut)]
    proposer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    /// CHECK: just pubkey needed
    project_info_account: AccountInfo<'info>,
    #[account(
            seeds = [b"multisig", project_info_account.key().as_ref()],
            bump,
        constraint = multisig.seqno == transaction.seqno
    )]
    multisig: Box<Account<'info, Multisig>>,
    /// CHECK
    #[account(
        seeds = [b"multisig", project_info_account.key().as_ref()],
        bump,
    constraint = multisig.seqno == transaction.seqno
)]
    multisig_signer: AccountInfo<'info>,
    #[account(mut, has_one = multisig)]
    transaction: Box<Account<'info, Transaction>>,
    // One of the multisig owners. Checked in the handler.
    owner: Signer<'info>,
}
