import { Address, address, TransactionSigner } from "@solana/kit";
import {
  CreateAirdropAsyncInput,
  CreateBitmapAsyncInput,
  DepositTokensAsyncInput,
  getCreateAirdropInstructionAsync,
  getCreateBitmapInstructionAsync,
  getDepositTokensInstructionAsync,
  getInitializeAirdropMasterInstructionAsync,
  InitializeAirdropMasterAsyncInput,
} from "../../clients/js/src/generated";
import { createAndMintToken, DropsyClient } from "../helpers/common";
import { DROPSY_PROTOCOL_TREASURY } from "../helpers/constants";
import {
  getAirdropDerivedAddress,
  getAirdropMasterDerivedAddress,
} from "../helpers/derive";
import { sendInstructions } from "../helpers/transaction";
import { createMerkleTree, getMerkleRootArray } from "../helpers/merkle";
import { findAssociatedTokenPda } from "@solana-program/token";
import { decodeAccountData, detectAccountType } from "../helpers/decode";
import {
  getDefaultAirdropData,
  getDefaultCreateClaimMapInput,
  getDefaultDepositTokensInput,
} from "../helpers/default";

export const createAirdrop = async (
  client: DropsyClient,
  airdropId: number = 0,
  delegate?: Address,
) => {
  const { mintAddress, mintAccount } = await createAndMintToken({ client });
  const amount = 1_000_000n;
  const input = await getDefaultAirdropData(
    client.payer,
    mintAddress,
    airdropId,
    delegate,
  );
  const createBitmapInput = await getDefaultCreateClaimMapInput(
    client.payer,
    mintAddress,
    airdropId,
  );
  const createSecondBitmapInput = await getDefaultCreateClaimMapInput(
    client.payer,
    mintAddress,
    airdropId,
    1,
  );
  const depositTokensInput = await getDefaultDepositTokensInput(
    client.payer,
    airdropId,
    mintAddress,
    amount,
    mintAccount.programAddress,
  );

  const createAirdropIx = await getCreateAirdropInstructionAsync(input);

  const createBitmapIx = await getCreateBitmapInstructionAsync(
    createBitmapInput,
  );

  const createSecondBitmapIx = await getCreateBitmapInstructionAsync(
    createSecondBitmapInput,
  );

  const createDepositIx = await getDepositTokensInstructionAsync(
    depositTokensInput,
  );

  const instructions = [
    createAirdropIx,
    createBitmapIx,
    createSecondBitmapIx,
    createDepositIx,
  ];

  await sendInstructions({
    signer: client.payer,
    instructions,
    client,
  });
  console.log("airdrop", input.airdrop);
  const airdropAccount = client.svm.getAccount(input.airdrop);
  if (!airdropAccount.exists) throw new Error("airdrop can't be fetched");

  const decodedData = decodeAccountData(
    "Airdrop",
    airdropAccount.data,
    input.airdrop,
  );

  console.log("Airdrop address:", input.airdrop);

  console.log("Account Airdrop :", airdropAccount);

  return decodedData;
};
