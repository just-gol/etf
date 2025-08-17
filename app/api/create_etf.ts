import { getWallet, Asset, program } from "./wallet";
import { PublicKey } from "@solana/web3.js";
import { getEtfMintAccountPda, getEtfPda } from "./address";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
export async function createEtf(
  name: string,
  symbol: string,
  description: string,
  uri: string,
  assets: { token: PublicKey, weight: number }[],
) {

  const pda = getEtfPda(name);
  const rem: Array<{ pubkey: PublicKey; isWritable: boolean; isSigner: boolean }> = [];
  for (const { token, } of assets) {
    const address = getAssociatedTokenAddressSync(token, pda, true);
    rem.push({
      pubkey: token,
      isWritable: false,
      isSigner: false,
    });
    rem.push({ pubkey: address, isWritable: true, isSigner: false })

    console.log("token:", token.toBase58())
    console.log("vaulet:", address.toBase58())
  }

  return await program.methods
    .createEtf({ name, symbol, description, uri, assets })
    .accounts({
    })
    .remainingAccounts(rem)
    .rpc();

}