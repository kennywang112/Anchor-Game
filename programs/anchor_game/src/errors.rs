use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("collection not same")]
    CollectionNotSame,
    #[msg("collection not verified")]
    CollectionNotVerified,
}