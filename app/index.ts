import { createEtf } from "./api/create_etf";
import * as anchor from "@coral-xyz/anchor";

(async () => {
  const r1 = await createEtf(
    "My ETF",
    "ETF",
    "This is a test ETF",
    "https://example.com/metadata.json",
    [
      { token: new anchor.web3.PublicKey("DjNQjdCotvY6Fs6mJFvuByiDTK36RqiT33YAdd24idfG"), weight: 20 },
      { token: new anchor.web3.PublicKey("D5dSKipZq9sGMmSKAkbQ3c6frHSuRb5BxbjB2KXjkeU4"), weight: 80 },
    ]
  );

  console.log("create etf", r1);
})()