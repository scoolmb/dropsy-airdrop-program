---
title: Airdrop Config
description: Overview of the Airdrop Config account
---

# Airdrop Config

## Account data

```ts
type AirdropConfig = {
    discriminator: bytes;
    authority: Address;
    protocolTreasury: Address;
    wlRoot: bytes;
    protocolFee: number /* u64 */;
    airdropMasterCreateFee: number /* u64 */;
    maxClaimFee: number /* u64 */;
    maxActionFee: number /* u64 */;
    minAirdropDuration: number /* i64 */;
    defaultAirdropDuration: number /* i64 */;
    maxAirdropDuration: number /* i64 */;
    updateGracePeriod: number /* i64 */;
    masterFeeBps: number /* u16 */;
    version: number /* u8 */;
    bump: number /* u8 */;
    padding: bytes;
    reserved: bytes;
}
```

This account has a fixed size of 432 bytes.