use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};
use anchor_spl::metadata::MetadataAccount;
use std::str::FromStr;
use crate::ROOM_DEFAULT_SIZE;
use crate::RoomState;
use crate::ROOM_PREFIX;
use crate::errors::Errors;

pub use anchor_spl::metadata::mpl_token_metadata;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitRoomIx {
    initializer_amount: u64,
    taker_amount: u64,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: InitRoomIx)]
pub struct InitializeCtx<'info> {
    #[account(
        seeds = [
            b"metadata", 
            mpl_token_metadata::ID.as_ref(), 
            mint.key().as_ref()
        ],
        seeds::program = mpl_token_metadata::ID,
        bump,
        constraint = metadata_account.collection.as_ref().unwrap().verified @ Errors::CollectionNotVerified,
        constraint = metadata_account.collection.as_ref().unwrap().key == Pubkey::from_str("8E8BHMvZiKq7q9xn1dw8rbZr7Vf2uPUdshaNU5mmFeZ8").unwrap() @ Errors::CollectionNotSame
    )]
    pub metadata_account: Account<'info,MetadataAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        seeds = [b"authority".as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = vault_authority
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount >= ix.initializer_amount,
        constraint = ix.initializer_amount == 50
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds =  [ROOM_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump,
        payer = initializer,
        space = ROOM_DEFAULT_SIZE
    )]
    pub room_state: Box<Account<'info, RoomState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    //pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub associated_token_program: Program<'info, AssociatedToken>,

}

impl<'info> InitializeCtx<'info> {
    fn into_transfer_to_pda_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.initializer_deposit_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.initializer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(
    ctx: Context<InitializeCtx>,
    ix: InitRoomIx
) -> Result<()> {
    ctx.accounts.room_state.initializer_key = *ctx.accounts.initializer.key;
    ctx.accounts.room_state.initializer_deposit_token_account = *ctx
        .accounts
        .initializer_deposit_token_account
        .to_account_info()
        .key;
    ctx.accounts.room_state.initializer_receive_token_account = *ctx
        .accounts
        .initializer_receive_token_account
        .to_account_info()
        .key;
    // ctx.accounts.room_state.initializer_amount = ix.initializer_amount;
    ctx.accounts.room_state.initializer_amount = ix.initializer_amount;
    ctx.accounts.room_state.taker_amount = ix.taker_amount;
    ctx.accounts.room_state.identifier = ix.identifier;

    let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&[b"authority"], ctx.program_id);
    ctx.accounts.room_state.vault_authority_bump = vault_authority_bump;

    token::transfer_checked(
        ctx.accounts.into_transfer_to_pda_context(),
        ctx.accounts.room_state.initializer_amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}