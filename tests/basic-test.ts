import { generateKeyPairSigner, lamports } from "@solana/kit";
import { RedeemAirdropTokensAsyncInput } from "../clients/js/src/generated";
import { createAndMintToken, getClient } from "./helpers/common";
import { createAirdrop } from "./instructions/create-airdrop";
import { createAirdropAndRedeemAndCloseBitmap } from "./instructions/create-airdrop-and-close-bitmap";
import { createAirdropAndRedeem } from "./instructions/create-airdrop-and-redeem";
import { createAirdropAndUpdateMerkle } from "./instructions/create-airdrop-and-update_merkle";
import { initAirdropConfig } from "./instructions/init-airdrop-config";
import { initAirdropMaster } from "./instructions/init-airdrop-master";
import { generateKeyPairSignerWithSol } from "./helpers/client";

// Payer first, then the LiteSVM transport.
const test = async () => {
  const client = await getClient();
  const delegate = await generateKeyPairSigner();

  client.svm.airdrop(delegate.address, lamports(5_0000_000_000n));
  await initAirdropConfig(client);
  await initAirdropMaster(client);
  //await createAirdrop(client);
  /*console.log("Decoded Account Airdrop :", data);*/
  await createAirdropAndRedeemAndCloseBitmap(client);
  //await createAirdropAndUpdateMerkle(client, 0, delegate);
};
test().catch((err) => {
  console.error(err);
  process.exit(1);
});
