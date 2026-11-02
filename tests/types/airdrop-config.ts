
import { AirdropConfig } from "@dropsy/airdrop";
import {Address} from "@solana/kit"

export interface IAirdropConfig {
  address: Address;

  authority: string;
  protocolTreasury: string;

  wlRoot: string;

  protocolFee: string;
  airdropMasterCreateFee: string;
  maxClaimFee: string;
  maxActionFee: string;

  minAirdropDuration: string;
  defaultAirdropDuration: string;
  maxAirdropDuration: string;
  updateGracePeriod: string;

  masterFeeBps: number;
  version: number;
  bump: number;
}

export function toUiAirdropConfig(
  config: AirdropConfig,
  address: Address,
): IAirdropConfig {
  return {
    address,

    authority: config.authority,
    protocolTreasury: config.protocolTreasury,

    wlRoot: Buffer.from(config.wlRoot).toString("hex"),

    protocolFee: config.protocolFee.toString(),
    airdropMasterCreateFee: config.airdropMasterCreateFee.toString(),
    maxClaimFee: config.maxClaimFee.toString(),
    maxActionFee: config.maxActionFee.toString(),

    minAirdropDuration: config.minAirdropDuration.toString(),
    defaultAirdropDuration: config.defaultAirdropDuration.toString(),
    maxAirdropDuration: config.maxAirdropDuration.toString(),
    updateGracePeriod: config.updateGracePeriod.toString(),

    masterFeeBps: config.masterFeeBps,
    version: config.version,
    bump: config.bump,
  };
}