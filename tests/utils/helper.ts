import { Address } from "@solana/kit";
import { IAirdrop } from "../types/airdrop.js";
import { IAirdropMaster } from "../types/airdrop-master.js";
import { IBitmapAccount } from "../types/claim-map.js";
import {
  Airdrop,
  AirdropMaster,
  BitmapAccount,
} from "../../clients/js/src/generated/index.js";

export function toUiAirdrop(airdrop: Airdrop, address: Address): IAirdrop {
  return {
    address,
    authority: airdrop.authority,
    delegateAuthority: airdrop.delegateAuthority,
    mint: airdrop.mint,
    master: airdrop.master,
    supply: airdrop.supply.toString(),
    merkleRoot: Buffer.from(airdrop.merkleRoot).toString("hex"),
    bitmapCount: airdrop.bitmapCount.toString(),
    startsAt: airdrop.startsAt.toString(),
    endsAt: airdrop.endsAt.toString(),
    id: airdrop.id.toString(),
    //status: AirdropStatusCalculator.getStatus(airdrop)
  };
}

export function toUiAirdropMaster(
  master: AirdropMaster,
  address: Address,
): IAirdropMaster {
  return {
    address,

    creator: master.creator,
    authority: master.authority,
    treasury: master.treasury,

    createdAirdrops: master.createdAirdrops.toString(),
    points: master.points.toString(),
    totalClaimCount: master.totalClaimCount.toString(),

    airdropUpdateFee: master.airdropUpdateFee.toString(),
    airdropCreationFee: master.airdropCreationFee.toString(),
    airdropClaimFee: master.airdropClaimFee.toString(),
    bitmapCreationFee: master.bitmapCreationFee.toString(),

    bump: master.bump,
  };
}

export function toUiBitmapAccount(
  bitmap: BitmapAccount,
  address: Address,
): IBitmapAccount {
  return {
    address,

    authority: bitmap.authority,
    airdrop: bitmap.airdrop,

    total: bitmap.total,

    claimedBitmap: claimedIndices(
      Buffer.from(bitmap.claimedBitmap).toString("hex"),
    ),

    id: bitmap.id,
    version: bitmap.version,
    bump: bitmap.bump,
  };
}

function claimedIndices(bitmapHex: string): number[] {
  const bytes = Buffer.from(bitmapHex, "hex");
  const claimed: number[] = [];

  for (let byte = 0; byte < bytes.length; byte++) {
    for (let bit = 0; bit < 8; bit++) {
      if (bytes[byte] & (1 << bit)) {
        claimed.push(byte * 8 + bit);
      }
    }
  }

  return claimed;
}

export function isClaimed(bitmapHex: string, index: number): boolean {
  const byteIndex = Math.floor(index / 8);
  const bitIndex = index % 8;

  const byte = parseInt(bitmapHex.slice(byteIndex * 2, byteIndex * 2 + 2), 16);

  return (byte & (1 << bitIndex)) !== 0;
}

export function fromUiAmount(uiAmount: string | number, decimals = 9): bigint {
  // Convert string to number
  const amountNum =
    typeof uiAmount === "string" ? parseFloat(uiAmount) : uiAmount;

  if (isNaN(amountNum) || amountNum < 0) {
    throw new Error("Invalid amount");
  }

  // Multiply by 10^decimals as string to avoid floating point precision issues
  const multiplier = 10 ** decimals;
  const lamportsStr = (amountNum * multiplier).toFixed(0); // round to nearest integer as string

  return BigInt(lamportsStr);
}
