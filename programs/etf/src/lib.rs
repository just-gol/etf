use anchor_lang::prelude::*;

declare_id!("DWc5YECbiHnRR1TvVqW1EBbE5xGz9kn6qxfFWrXtBkkJ");

pub mod error;
pub mod instructions;
use instructions::*;
pub mod state;

#[program]
pub mod etf {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
