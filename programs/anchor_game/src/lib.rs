pub mod room;
pub use room::*;

use anchor_lang::prelude::*;


declare_id!("C6eqnSPN75gz2if2e8M3H7xdz4WUX7kPfnDbmHxG9Ri5");

#[program]
pub mod anchor_game {

    use super::*;

    /// room
    pub fn init_room(ctx: Context<InitializeCtx>, ix: InitRoomIx) -> Result<()> {
        room::init_room::handler(ctx, ix)
    }

    /// exchange
    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        room::exchange::handler(ctx)
    }

    /// cancel
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        room::cancel::handler(ctx)
    }

}