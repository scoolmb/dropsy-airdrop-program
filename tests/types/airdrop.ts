import { ReadonlyUint8Array } from "@solana/kit";

export interface IAirdrop {
  address: string;
  authority: string;
  delegateAuthority: string;
  mint: string;
  master: string;
  supply: string;
  merkleRoot: string;
  bitmapCount: string;
  startsAt: string;
  endsAt: string;
  id: string;
}
