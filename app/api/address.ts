import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";
// etf_mint_account  pda
export function getEtfMintAccountPda(name: string) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), Buffer.from(name)],
    program.programId
  )[0];
}

export function getEtfPda(name: string) {
  const etf_mint_account_pda = getEtfMintAccountPda(name);
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("etf"), etf_mint_account_pda.toBuffer()],
    program.programId
  )[0];
}