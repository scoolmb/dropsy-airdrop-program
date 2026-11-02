import { address } from "@solana/kit";
import { getRedeemAirdropTokensInstructionAsync } from "../../clients/js/src/generated";
import { DropsyClient } from "../helpers/common";
import { sendInstructions } from "../helpers/transaction";
import { decodeAccountData } from "../helpers/decode";
import { getDefaultRedeemTokensInput } from "../helpers/default";
import { createAirdrop } from "./create-airdrop";

export const createAirdropAndRedeem = async (
  client: DropsyClient,
  airdropId: number = 0,
) => {
  const data = await createAirdrop(client);
  console.log("Decoded Account Airdrop :", data);
  const mintAccount = await client.token.accounts.mint.fetch(
    address(data.mint),
  );

  const RedeemTokensInput = await getDefaultRedeemTokensInput(
    client.payer,
    0,
    data.mint,
    mintAccount.programAddress,
  );

  const RedeemTokenTx = await getRedeemAirdropTokensInstructionAsync(
    RedeemTokensInput,
  );

  const instructions = [RedeemTokenTx];

  await sendInstructions({
    payer: client.payer,
    instructions,
    client,
  });
  const airdropAccount = client.svm.getAccount(address(data.address));
  if (!airdropAccount.exists) throw new Error("airdrop can't be fetched");

  const decodedData = decodeAccountData(
    "Airdrop",
    airdropAccount.data,
    address(data.address),
  );

  console.log("Airdrop address:", data.address);

  console.log("Account Airdrop :", airdropAccount);
  console.log("Decoded Account Airdrop :", decodedData);
};
