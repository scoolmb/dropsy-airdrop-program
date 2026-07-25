---
title: Initialize Airdrop Config
description: Overview of the Initialize Airdrop Config instruction
---

# Initialize Airdrop Config

## Instruction accounts

| Name               | Signer | Writable | Required |
| ------------------ | ------ | -------- | -------- |
| `config`           | ❌      | ✅        | ✅        |
| `protocolTreasury` | ❌      | ❌        | ✅        |
| `authority`        | ✅      | ✅        | ✅        |
| `systemProgram`    | ❌      | ❌        | ✅        |

## Instruction arguments

```ts
type InitializeAirdropConfigInstruction = {
    discriminator: bytes;
    wlRoot: Option<bytes>;
    airdropMasterCreateFee: Option<number /* u64 */>;
    maxClaimFee: Option<number /* u64 */>;
    maxActionFee: Option<number /* u64 */>;
    minAirdropDuration: Option<number /* i64 */>;
    defaultAirdropDuration: Option<number /* i64 */>;
    maxAirdropDuration: Option<number /* i64 */>;
    updateGracePeriod: Option<number /* i64 */>;
    protocolFee: Option<number /* u64 */>;
    masterFeeBps: Option<number /* u16 */>;
}
```