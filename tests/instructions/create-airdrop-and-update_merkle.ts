import {
  Address,
  address,
  generateKeyPairSigner,
  lamports,
  TransactionSigner,
} from "@solana/kit";
import {
  getRedeemAirdropTokensInstructionAsync,
  getUpdateAirdropInstructionAsync,
  UpdateAirdropAsyncInput,
} from "../../clients/js/src/generated";
import { DropsyClient, toMerkleRootBytes } from "../helpers/common";
import { sendInstructions } from "../helpers/transaction";
import { decodeAccountData } from "../helpers/decode";
import { getDefaultRedeemTokensInput } from "../helpers/default";
import { createAirdrop } from "./create-airdrop";
import { DROPSY_PROTOCOL_TREASURY } from "../helpers/constants";
import { createMerkleTree, getMerkleRootArray } from "../helpers/merkle";

export const createAirdropAndUpdateMerkle = async (
  client: DropsyClient,
  airdropId: number = 0,
  delegate: TransactionSigner,
) => {
  createAirdropAndUpdateMerkle;
  const data = await createAirdrop(client, 0, delegate?.address);
  console.log("Decoded Airdrop before update :", data);
  const fakeDelegate = await generateKeyPairSigner();
  client.svm.airdrop(fakeDelegate.address, lamports(500_000_000n));

  const protocolTreasury = DROPSY_PROTOCOL_TREASURY;
  const claimList = [
    {
      index: 0,
      address: "6TMmpVof9fGXak2VUgP4X9sXNfPHA7XWyi9ZMUCrHP6A",
      amount: 500,
    },
    {
      index: 1,
      address: "CeN4TqZaxMc9adCeCtE6VqfpwRbmt6DRYMnH41vv9V5F",
      amount: 500,
    },
  ];

  const merkleTree = createMerkleTree(claimList);
  const merkleRootHex = merkleTree.getRoot().toString("hex");
  const merkleRoot = toMerkleRootBytes(merkleRootHex);

  const UpdateAirdropInput: UpdateAirdropAsyncInput = {
    mint: address(data.mint),
    authority: client.payer.address,
    signer: delegate,
    airdrop: data.address,
    protocolTreasury,
    masterCreator: address("5gB7WLoBWELAvRDYPUvGj7e7BHEv65j6YDhissdMTQ4F"),
    treasury: address("5gB7WLoBWELAvRDYPUvGj7e7BHEv65j6YDhissdMTQ4F"),
    id: data.id,
    startsAt: null,
    endsAt: null,
    merkleRoot,
  };

  const UpdateAirdropIx = await getUpdateAirdropInstructionAsync(
    UpdateAirdropInput,
  );

  const instructions = [UpdateAirdropIx];

  await sendInstructions({
    signer: delegate,
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
  console.log("Decoded Account Airdrop after update :", decodedData);
};
