import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Etf } from "../../target/types/etf";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.etf as Program<Etf>;

export { program }

export function getWallet() {
  return anchor.Wallet.local();
}

export type Asset = {
  token: string;   // Pubkey 转成 base58 string
  weight: number; // or BN，如果你需要大数
};