use anchor_lang::prelude::*;

pub const ESCROW_DEFAULT_SIZE: usize = 8 + 121;
pub const ESCROW_PREFIX: &str = "state";

#[account]
pub struct EscrowState {
    pub identifier: String,
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub vault_authority_bump: u8,
}