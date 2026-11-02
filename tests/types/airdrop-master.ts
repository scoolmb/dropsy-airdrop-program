import { Address } from "@solana/kit";

export interface IAirdropMaster {
    address: Address;

    creator: Address;
    authority: Address;
    treasury: Address;

    createdAirdrops: string;
    points: string;
    totalClaimCount: string;

    airdropUpdateFee: string;
    airdropCreationFee: string;
    airdropClaimFee: string;
    bitmapCreationFee: string;

    bump: number;
}