use anchor_lang::prelude::*;

declare_id!("DWc5YECbiHnRR1TvVqW1EBbE5xGz9kn6qxfFWrXtBkkJ");

pub mod error;
pub mod instructions;
use instructions::*;
pub mod state;
pub use state::*;

#[program]
pub mod etf {
    use crate::state::CreateEtfArgs;

    use super::*;

    pub fn create_etf<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateETF<'info>>,
        args: CreateEtfArgs,
    ) -> Result<()> {
        instructions::create_etf(ctx, args)
    }

    pub fn prepare_funds<'info>(
        ctx: Context<'_, '_, '_, 'info, PrepareFunds<'info>>,
    ) -> Result<()> {
        instructions::prepare_funds(ctx)
    }

    pub fn purchase_etf<'info>(
        ctx: Context<'_, '_, '_, 'info, PurchaseEtf<'info>>,
        lamports: u64,
    ) -> Result<()> {
        instructions::purchase_etf(ctx, lamports)
    }
}
