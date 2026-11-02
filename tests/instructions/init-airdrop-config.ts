import { getInitializeAirdropConfigInstructionAsync } from "../../clients/js/src/generated";
import { DropsyClient, getClient } from "../helpers/common";
import {
  getAirdropConfigDerivedAddress,
  getAirdropMasterDerivedAddress,
} from "../helpers/derive";
import { getDefaultAirdropConfig } from "../helpers/dropsy";
import { sendInstructions } from "../helpers/transaction";

export const initAirdropConfig = async (client: DropsyClient) => {
  const input = getDefaultAirdropConfig(client.payer);

  const [airdropConfigAddress] = await getAirdropConfigDerivedAddress();

  console.log("Derived config:", airdropConfigAddress);

  const instruction = await getInitializeAirdropConfigInstructionAsync(input);

  console.log("Instruction accounts:", instruction.accounts);

  await sendInstructions({
    signer: client.payer,
    instructions: [instruction],
    client,
  });
  const airdropConfigAccount = client.svm.getAccount(airdropConfigAddress);

  console.log("Fetched account:", airdropConfigAccount);
};
