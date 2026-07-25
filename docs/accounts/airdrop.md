---
title: Airdrop
description: Overview of the Airdrop account
---

# Airdrop

## Account data

```ts
type Airdrop = {
    discriminator: bytes;
    master: Address;
    authority: Address;
    mint: Address;
    delegateAuthority: Address;
    merkleRoot: bytes;
    id: number /* u64 */;
    supply: number /* u64 */;
    boost: number /* u64 */;
    startsAt: number /* i64 */;
    endsAt: number /* i64 */;
    bitmapCount: number /* u16 */;
    state: number /* u8 */;
    version: number /* u8 */;
    delegatePermissions: number /* u8 */;
    bump: number /* u8 */;
    padding: bytes;
}
```

This account has a fixed size of 216 bytes.