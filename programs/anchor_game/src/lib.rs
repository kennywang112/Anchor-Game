pub mod room;
pub use room::*;

use anchor_lang::prelude::*;

pub mod errors;

declare_id!("34kGtvyQYq4qsv7XtBejcZo5ZGD652hFM81x7Gc4TToq");

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