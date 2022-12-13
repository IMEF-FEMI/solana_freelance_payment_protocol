use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

pub fn assert_unique_owners(owners: &[Pubkey]) -> Result<()> {
    for i in 0..owners.len() {
        for j in (i + 1)..owners.len() {
            require!(owners[i] != owners[j], ErrorCode::UniqueOwners)
        }
    }
    Ok(())
}
