---
title: Airdrop Master
description: Overview of the Airdrop Master account
---

# Airdrop Master

## Account data

```ts
type AirdropMaster = {
    discriminator: bytes;
    creator: Address;
    authority: Address;
    treasury: Address;
    createdAirdrops: number /* u64 */;
    points: number /* u64 */;
    totalClaimCount: number /* u64 */;
    airdropUpdateFee: number /* u64 */;
    airdropCreationFee: number /* u64 */;
    airdropClaimFee: number /* u64 */;
    bitmapCreationFee: number /* u64 */;
    bump: number /* u8 */;
    padding: bytes;
    reserved: bytes;
}
```

This account has a fixed size of 296 bytes.