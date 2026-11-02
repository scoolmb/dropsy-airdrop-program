import { address, TransactionSigner } from "@solana/kit";
import {
  getInitializeAirdropMasterInstructionAsync,
  InitializeAirdropMasterAsyncInput,
} from "../../clients/js/src/generated";
import { DropsyClient } from "../helpers/common";
import { DROPSY_PROTOCOL_TREASURY } from "../helpers/constants";
import { getAirdropMasterDerivedAddress } from "../helpers/derive";
import { sendInstructions } from "../helpers/transaction";

export const initAirdropMaster = async (client: DropsyClient) => {
  console.log("initialize Master");
  const input = getDefaultAirdropMasterData(client.payer);
  const [airdropMasterAddress] = await getAirdropMasterDerivedAddress(
    client.payer.address,
  );

  const instruction = await getInitializeAirdropMasterInstructionAsync(input);

  console.log("Instruction accounts:", instruction.accounts);

  await sendInstructions({
    signer: client.payer,
    instructions: [instruction],
    client,
  });
  const airdropMasterAccount = client.svm.getAccount(airdropMasterAddress);

  console.log("Fetched account:", airdropMasterAccount);
  console.log("Airdrop master:", airdropMasterAddress);

  console.log("Account Airdrop Master:", airdropMasterAccount);
};

export const getDefaultAirdropMasterData = (
  authority: TransactionSigner,
): InitializeAirdropMasterAsyncInput => ({
  protocolTreasury: DROPSY_PROTOCOL_TREASURY,
  masterCreator: authority,
  bitmapCreationFee: 100_000n,
  airdropClaimFee: 400_000n,
  airdropCreationFee: 100_000n,
  airdropDelegateFee: 100_000n,
  airdropUpdateFee: 100_000n,
  treasury: address("5gB7WLoBWELAvRDYPUvGj7e7BHEv65j6YDhissdMTQ4F"),
  wlProof: null,
});
