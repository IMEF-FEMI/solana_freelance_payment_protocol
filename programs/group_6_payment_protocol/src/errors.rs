use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Owners must be unique")]
    UniqueOwners,   
    #[msg("The given transaction has already been executed.")]
    AlreadyExecuted,
    #[msg("Threshold must be less than or equal to the number of owners.")]
    InvalidThreshold,
    #[msg("Multisig has already been created.")]
    MultisigAlreadyInitialized,
    #[msg("The given owner is not part of this multisig.")]
    InvalidOwner,
    #[msg("Owners length must be non zero.")]
    InvalidOwnersLen,
    #[msg("Only Client can can this function")]
    ClientOnly,
    #[msg("The given owners is empty.")]
    InvalidOwnerLength,
    #[msg("Not enough owners signed this transaction.")]
    NotEnoughSigners,
    #[msg("Cannot delete a transaction that has been signed by an owner.")]
    TransactionAlreadySigned,
    #[msg("Overflow when adding.")]
    Overflow,
    #[msg("Cannot delete a transaction the owner did not create.")]
    UnableToDelete,
    #[msg("The given transaction has already been executed.")]
    ParamLength,
    #[msg("Wrong Project Status")]
    InvalidStatus,
}