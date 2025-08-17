import { getWallet, Asset, program } from "./wallet";
import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { getEtfMintAccountPda } from "./address";
export async function prepareFunds(
  etfPda: PublicKey,
  wallet: anchor.Wallet,
  assets: { token: PublicKey, weight: number }[]) {
  const rem: Array<{ pubkey: PublicKey, isSigner: boolean, isWritable: boolean }> = [];
  for (const { token } of assets) {
    rem.push({ pubkey: token, isSigner: false, isWritable: false });

    const ata = getAssociatedTokenAddressSync(token, wallet.payer.publicKey, false);
    rem.push({ pubkey: ata, isSigner: false, isWritable: true })
    console.log("用户ata: ", ata.toBase58())
  }

  return await program.methods
    .prepareFunds()
    .accounts({
      etf: etfPda
    }).remainingAccounts(rem)
    .rpc();

}

export async function purchaseEtf(
  etfPda: PublicKey,
  etfMintPda: PublicKey,
  wallet: anchor.Wallet,
  lamports: number,
) {
  const etf = await program.account.etf.fetch(etfPda);
  const accounts = etf.assets.flatMap(asset => {
    const user_ata = getAssociatedTokenAddressSync(asset.token, wallet.payer.publicKey, false);
    const contract_ata = getAssociatedTokenAddressSync(asset.token, etfPda, true);
    return [user_ata, contract_ata];
  })

  return await program.methods.purchaseEtf(new anchor.BN(lamports))
    .accounts({
      etfMintAccount: etfMintPda
    })
    .remainingAccounts(accounts.map(x => ({ pubkey: x, isSigner: false, isWritable: true })))
    .rpc();
}

