use anchor_lang::prelude::*;

#[error_code]
pub enum EtfErrorCode {
    InvalidInstruction,
    InvalidAccount,
    InvalidMint,
    InvalidVaultAta,
    InvalidAccountsLen,
    InvalidAsset,
    InvalidAssetWeight,
    InvalidAssetToken,
    InvalidAssetTokenAccount,
    InvalidAssetTokenAccountMint,
    InvalidAssetTokenAccountOwner,
    InvalidAssetTokenAccountDecimals,
    InvalidAssetTokenAccountAmount,
    InvalidAssetTokenAccountAmountLessThanZero,
    InvalidAssetTokenAccountAmountMoreThanMax,
}
