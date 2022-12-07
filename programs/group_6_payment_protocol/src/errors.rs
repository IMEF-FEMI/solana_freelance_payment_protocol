use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

    #[msg("Only Client can can this function")]
    ClientOnly,
}