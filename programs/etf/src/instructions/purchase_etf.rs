
use std::collections::HashMap;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::{create_idempotent, get_associated_token_address, AssociatedToken, Create}, token::{mint_to, Mint, MintTo, TokenAccount}};

use crate::{error::EtfErrorCode, Etf};

pub fn prepare_funds<'info>(ctx: Context<'_, '_, '_, 'info,PrepareFunds<'info>>)->Result<()>{
  let assets  = &mut  ctx.accounts.etf.assets;
  for (i,asset) in assets.iter().enumerate(){
    let min = &ctx.remaining_accounts[2*i];
    let vault =  &ctx.remaining_accounts[2*i+1];
    require_keys_eq!(min.key(), asset.token, EtfErrorCode::InvalidMint);
      
    let expected = get_associated_token_address(&ctx.accounts.authority.key(), &asset.token);
    require_keys_eq!(vault.key(), expected, EtfErrorCode::InvalidAccount);

    create_idempotent(CpiContext::new(ctx.accounts.associated_token_program.to_account_info(),
     Create { 
      payer: ctx.accounts.authority.to_account_info(),
      associated_token:vault.to_account_info(),
      authority:ctx.accounts.authority.to_account_info(),
      mint: min.to_account_info(),
      system_program:ctx.accounts.system_program.to_account_info(),
      token_program: ctx.accounts.token_program.to_account_info(), 
    }))?;
  }
  Ok(())
}

#[derive(Accounts)]
pub struct PrepareFunds<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub etf: Account<'info, Etf>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, anchor_spl::token::Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn purchase_etf<'info>(ctx: Context<'_, '_, '_, 'info,PurchaseEtf<'info>>,lamports:u64) -> Result<()> {
  let accounts: HashMap<Pubkey, AccountInfo<'info>> = ctx
  .remaining_accounts
  .iter()
  .map(|x| (x.key(), x.to_owned()))
  .collect();
    for asset in  &mut ctx.accounts.etf.assets.iter(){
       // 获取 ata,判断是否存在ata
       let from  = accounts.get(&get_associated_token_address(&ctx.accounts.authority.key(),&asset.token)).unwrap();

       let to  = accounts.get(&get_associated_token_address(&ctx.accounts.etf.key(),&asset.token)).unwrap();

       let amount = lamports * (asset.weight as u64) / 100;
       anchor_spl::token::transfer(CpiContext::new(
         ctx.accounts.token_program.to_account_info(),
         anchor_spl::token::Transfer {
           from: from.to_account_info(),
           to: to.to_account_info(),
           authority: ctx.accounts.authority.to_account_info(),
         },
       ), amount)?;

    }

    let etf = ctx.accounts.etf_mint_account.key();
    let signer_seeds: &[&[&[u8]]]  = &[&[b"etf",etf.as_ref(),&[ctx.bumps.etf]]];
    mint_to(CpiContext::new_with_signer(
      ctx.accounts.token_program.to_account_info(),
       MintTo{
        mint: ctx.accounts.etf_mint_account.to_account_info(),
        to: ctx.accounts.authority_ata_etf.to_account_info(),
        authority: ctx.accounts.etf.to_account_info(),
    }, signer_seeds), lamports)?;

    Ok(())
}

#[derive(Accounts)]
pub struct PurchaseEtf<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [
        b"etf",
        etf_mint_account.key().as_ref(),
      ],
      bump
    )]
    pub etf: Account<'info, Etf>,

    #[account(mut)]
    pub etf_mint_account: Account<'info, Mint>,

    #[account(
      init_if_needed,
      payer = authority,
      associated_token::mint = etf_mint_account, 
      associated_token::authority = authority)]
    pub authority_ata_etf: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, anchor_spl::token::Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
