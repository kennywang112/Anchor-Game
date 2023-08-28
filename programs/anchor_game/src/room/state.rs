use anchor_lang::prelude::*;

pub const ROOM_DEFAULT_SIZE: usize = 24 + 32 + 32 + 32 + 8 + 8 + 1 + 32 ;
pub const ROOM_PREFIX: &str = "state";

#[account]
pub struct RoomState {
    pub identifier: String,
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub vault_authority_bump: u8,
    //pub nft_token_account: Pubkey
}