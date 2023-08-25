pub mod escrow;
pub use escrow::*;
use anchor_lang::prelude::*;

declare_id!("C6eqnSPN75gz2if2e8M3H7xdz4WUX7kPfnDbmHxG9Ri5");

#[program]
pub mod anchor_game {

    use super::*;

    /// escrow
    pub fn init_escrow(ctx: Context<InitializeCtx>, ix: InitHouseIx) -> Result<()> {
        escrow::init_escrow::handler(ctx, ix)
    }

    /// exchange
    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        escrow::exchange::handler(ctx)
    }

    /// cancel
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        escrow::cancel::handler(ctx)
    }
}