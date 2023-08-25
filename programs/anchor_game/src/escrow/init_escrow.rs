use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};
use crate::ESCROW_DEFAULT_SIZE;
use crate::EscrowState;
use crate::ESCROW_PREFIX;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitHouseIx {
    initializer_amount: u64,
    taker_amount: u64,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: InitHouseIx)]
pub struct InitializeCtx<'info> {
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
        constraint = initializer_deposit_token_account.amount >= ix.initializer_amount
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds =  [ESCROW_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump,
        payer = initializer,
        space = ESCROW_DEFAULT_SIZE
    )]
    pub escrow_state: Box<Account<'info, EscrowState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
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
    ix: InitHouseIx
) -> Result<()> {
    ctx.accounts.escrow_state.initializer_key = *ctx.accounts.initializer.key;
    ctx.accounts.escrow_state.initializer_deposit_token_account = *ctx
        .accounts
        .initializer_deposit_token_account
        .to_account_info()
        .key;
    ctx.accounts.escrow_state.initializer_receive_token_account = *ctx
        .accounts
        .initializer_receive_token_account
        .to_account_info()
        .key;
    ctx.accounts.escrow_state.initializer_amount = ix.initializer_amount;
    ctx.accounts.escrow_state.taker_amount = ix.taker_amount;
    ctx.accounts.escrow_state.identifier = ix.identifier;

    let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&[b"authority"], ctx.program_id);
    ctx.accounts.escrow_state.vault_authority_bump = vault_authority_bump;

    token::transfer_checked(
        ctx.accounts.into_transfer_to_pda_context(),
        ctx.accounts.escrow_state.initializer_amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}