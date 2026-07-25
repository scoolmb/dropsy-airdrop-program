---
title: Airdrop Initialized
description: Overview of the Airdrop Initialized type
---

# Airdrop Initialized

## Type definition

```ts
type AirdropInitialized = {
    airdrop: Address;
    mint: Address;
    authority: Address;
    master: Address;
    merkleRoot: bytes;
    startTime: number /* i64 */;
    endTime: number /* i64 */;
    timestamp: number /* i64 */;
}
```