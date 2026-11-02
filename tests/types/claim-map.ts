import type { Address } from "@solana/kit";

export interface IBitmapAccount {
    address: Address;

    authority: Address;
    airdrop: Address;

    total: number;

    claimedBitmap: number[];

    id: number;
    version: number;
    bump: number;
}