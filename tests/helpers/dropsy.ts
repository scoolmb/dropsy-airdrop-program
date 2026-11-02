import { address, TransactionSigner } from "@solana/kit";
import {
  CreateAirdropAsyncInput,
  InitializeAirdropConfigAsyncInput,
  InitializeAirdropMasterAsyncInput,
} from "../../clients/js/src/generated";
import { DROPSY_PROTOCOL_TREASURY } from "./constants";

export const getDefaultAirdropConfig = (
  authority: TransactionSigner,
): InitializeAirdropConfigAsyncInput => ({
  protocolTreasury: DROPSY_PROTOCOL_TREASURY,
  authority,

  args: {
    protocolTreasury: DROPSY_PROTOCOL_TREASURY,
    wlRoot: null,

    protocolFee: 10_000n,
    airdropMasterCreateFee: 1_000_000n,

    maxClaimFee: 500_000n,
    maxActionFee: 200_000n,

    minAirdropDuration: 3_600n,
    defaultAirdropDuration: 86_400n,
    maxAirdropDuration: 604_800n,

    updateGracePeriod: 3_600n,

    masterFeeBps: 500,
  },
});
