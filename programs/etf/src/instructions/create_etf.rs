use anchor_lang::prelude::*;
use anchor_spl::{associated_token::
  {create_idempotent, get_associated_token_address, AssociatedToken,Create}, 
  metadata::{ create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, Metadata}, token::Mint};

use crate::{error::EtfErrorCode, state::{CreateEtfArgs, Etf}};

pub fn create_etf<'info>(ctx: Context<'_, '_, '_, 'info,CreateETF<'info>>, args: CreateEtfArgs) -> Result<()> {
  let etf_mint_account = ctx.accounts.etf_mint_account.key();
  let signer_seeds: &[&[&[u8]]] = &[&[b"etf",etf_mint_account.as_ref(),&[ctx.bumps.etf]]];

  create_metadata_accounts_v3(
    CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(),
     CreateMetadataAccountsV3{
        metadata: ctx.accounts.metadata_account.to_account_info(),
        mint: ctx.accounts.etf_mint_account.to_account_info(),
        mint_authority: ctx.accounts.etf.to_account_info(),
        payer: ctx.accounts.authority.to_account_info(),
        update_authority: ctx.accounts.etf.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    }, signer_seeds), 
    DataV2{
        name: args.name.to_string(),
        symbol: args.symbol.to_string(),
        uri: args.uri.to_string(),
        seller_fee_basis_points: 0,
        creators:None,
        collection: None,
        uses: None,
    },
     false, 
     true, None)?;


     require!(
      ctx.remaining_accounts.len() == 2 * args.assets.len(),
      EtfErrorCode::InvalidAccountsLen
  );

     for (i,asset) in args.assets.iter().enumerate() {
      let min = &ctx.remaining_accounts[2 * i];
      let vault = &ctx.remaining_accounts[2 * i + 1];
      
      require_keys_eq!(min.key(), asset.token, EtfErrorCode::InvalidMint);
      
      let expected = get_associated_token_address(&ctx.accounts.etf.key(), &asset.token);
      require_keys_eq!(vault.key(), expected, EtfErrorCode::InvalidAccount);

      // 创建ATA（带签名验证）
      create_idempotent(
          CpiContext::new(
              ctx.accounts.associated_token_program.to_account_info(),
              Create {
                  payer: ctx.accounts.authority.to_account_info(),
                  associated_token: vault.to_account_info(),
                  authority: ctx.accounts.etf.to_account_info(),
                  mint: min.to_account_info(),
                  system_program: ctx.accounts.system_program.to_account_info(),
                  token_program: ctx.accounts.token_program.to_account_info(),
              },
          )
      )?;

     }  

   let etf =  Etf::new( ctx.accounts.authority.key(),ctx.accounts.etf_mint_account.key(), args.name.to_string(),  args.symbol.to_string(), args.description.to_string(), args.assets); ctx.accounts.etf.set_inner(etf);

       Ok(())
}

#[derive(Accounts)]
#[instruction(args: CreateEtfArgs)]
pub struct CreateETF<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      init,
      payer = authority,
      space = 8 + Etf::INIT_SPACE,
      seeds = [
        b"etf",
        etf_mint_account.key().as_ref(),
      ],
      bump
    )]
    pub etf: Account<'info, Etf>,

    #[account(
      init,
      seeds = [
        b"mint",args.name.as_bytes(),
      ],
      bump,
      payer = authority,
      mint::decimals = 9,
      mint::authority = etf,
    )]
    pub etf_mint_account: Account<'info, Mint>,

    #[account(
      mut,
      seeds=[
        b"metadata",
        token_metadata_program.key().as_ref(), 
        etf_mint_account.key().as_ref()],
        bump
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, anchor_spl::token::Token>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub rent: Sysvar<'info, Rent>,

    pub associated_token_program: Program<'info, AssociatedToken>, 
}
