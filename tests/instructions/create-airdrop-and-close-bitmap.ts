import { address } from "@solana/kit";
import {
  CloseBitmapInput,
  getCloseAirdropInstructionAsync,
  getCloseBitmapInstruction,
  getCloseBitmapInstructionAsync,
  getRedeemAirdropTokensInstructionAsync,
} from "../../clients/js/src/generated";
import { DropsyClient } from "../helpers/common";
import { sendInstructions } from "../helpers/transaction";
import { decodeAccountData } from "../helpers/decode";
import {
  getDefaultCloseBitmapInput,
  getDefaultRedeemTokensInput,
} from "../helpers/default";
import { createAirdrop } from "./create-airdrop";
import {
  getAirdropConfigDerivedAddress,
  getAirdropDerivedAddress,
  getClaimMapDerivedAddress,
} from "../helpers/derive";
import { DROPSY_PROTOCOL_TREASURY } from "../helpers/constants";

export const createAirdropAndRedeemAndCloseBitmap = async (
  client: DropsyClient,
  airdropId: number = 0,
) => {
  const data = await createAirdrop(client);
  console.log("Decoded Account Airdrop :", data);
  const mintAccount = await client.token.accounts.mint.fetch(
    address(data.mint),
  );
  const [airdrop, airdropBump] = await getAirdropDerivedAddress(
    client.payer.address,
    data.mint,
    airdropId,
  );
  const [bitmap] = await getClaimMapDerivedAddress(airdrop, 0);
  const [SecondBitmap] = await getClaimMapDerivedAddress(airdrop, 1);

  const [airdropConfig, _bump] = await getAirdropConfigDerivedAddress();
  console.log("airdrop id : ", airdrop);

  const RedeemTokensInput = await getDefaultRedeemTokensInput(
    client.payer,
    airdropId,
    data.mint,
    mintAccount.programAddress,
  );
  const RedeemTokenTx = await getRedeemAirdropTokensInstructionAsync(
    RedeemTokensInput,
  );

  const closeBitmapInput: CloseBitmapInput = {
    airdrop,
    airdropConfig,
    mint: data.mint,
    bitmap,
    airdropId,
    id: 0,
    protocolTreasury: DROPSY_PROTOCOL_TREASURY,
    authority: client.payer,
  };

  const close2ndBitmapInput: CloseBitmapInput = {
    airdrop,
    airdropConfig,
    mint: data.mint,
    bitmap: SecondBitmap,
    airdropId,
    id: 1,
    protocolTreasury: DROPSY_PROTOCOL_TREASURY,
    authority: client.payer,
  };

  console.log("closeBitmapInput", closeBitmapInput);

  const CloseBitmapTx = getCloseBitmapInstruction(closeBitmapInput);
  const CloseSecondBitmapTx = getCloseBitmapInstruction(close2ndBitmapInput);

  const CloseAirdropTx = await getCloseAirdropInstructionAsync({
    airdropId,
    mint: data.mint,
    authority: client.payer,
    protocolTreasury: DROPSY_PROTOCOL_TREASURY,
  });

  const instructions = [
    RedeemTokenTx,
    CloseSecondBitmapTx,
    CloseBitmapTx,
    CloseAirdropTx,
  ];

  await sendInstructions({
    signer: client.payer,
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
