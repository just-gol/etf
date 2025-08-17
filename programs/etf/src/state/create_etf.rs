use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateEtfArgs {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub uri: String,
    pub assets: Vec<Asset>,
}

#[account]
#[derive(InitSpace)]
pub struct Etf {
    pub owner: Pubkey,
    mint_account: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(24)]
    pub symbol: String,
    #[max_len(256)]
    pub description: String,
    pub created_at: i64,
    #[max_len(10)]
    pub assets: Vec<Asset>,
}

impl Etf {
    pub fn new(
        owner: Pubkey,
        mint_account: Pubkey,
        name: String,
        symbol: String,
        description: String,
        assets: Vec<Asset>,
    ) -> Self {
        Etf {
            owner,
            mint_account,
            name,
            symbol,
            description,
            created_at: Clock::get().unwrap().unix_timestamp,
            assets,
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct Asset {
    pub token: Pubkey,
    pub weight: u8,
}
