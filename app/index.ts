import { createEtf } from "./api/create_etf";
import * as anchor from "@coral-xyz/anchor";
import { getEtfPda, getEtfMintAccountPda } from "./api/address";
import { getWallet } from "./api/wallet";
import { prepareFunds, purchaseEtf } from "./api/purchase_etf";
(async () => {
  const wallet = getWallet();
  const name = "My ETF 00001";
  // const r1 = await createEtf(
  //   name,
  //   "ETF",
  //   "This is a test ETF",
  //   "https://example.com/metadata.json",
  //   [
  //     { token: new anchor.web3.PublicKey("FMD6pHMT392X1tbqLkG5ag4B2vHSWv1EGfubUZVoQBR4"), weight: 20 },
  //     { token: new anchor.web3.PublicKey("6UaWuXh9eV1CjrfC4zTU87NcP3JPAD8ea4F87xn6bhxF"), weight: 80 },
  //   ]
  // );

  // console.log("create etf", r1);

  const pda = getEtfPda(name);
  // console.log("etf pda", pda.toBase58());
  // const r2 = await prepareFunds(pda, wallet, [
  //   { token: new anchor.web3.PublicKey("FMD6pHMT392X1tbqLkG5ag4B2vHSWv1EGfubUZVoQBR4"), weight: 20 },
  //   { token: new anchor.web3.PublicKey("6UaWuXh9eV1CjrfC4zTU87NcP3JPAD8ea4F87xn6bhxF"), weight: 80 },
  // ]);

  // console.log("prepareFunds", r2);


  const etfMintPda = getEtfMintAccountPda(name);

  const r3 = await purchaseEtf(pda, etfMintPda, wallet, 100 * anchor.web3.LAMPORTS_PER_SOL);
  console.log("purchaseEtf", r3);
})() 