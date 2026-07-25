---
title: Create Airdrop
description: Overview of the Create Airdrop instruction
---

# Create Airdrop

## Instruction accounts

| Name               | Signer | Writable | Required |
| ------------------ | ------ | -------- | -------- |
| `airdropConfig`    | ❌      | ❌        | ✅        |
| `protocolTreasury` | ❌      | ✅        | ✅        |
| `airdropMaster`    | ❌      | ❌        | ✅        |
| `treasury`         | ❌      | ✅        | ✅        |
| `masterCreator`    | ❌      | ❌        | ✅        |
| `airdrop`          | ❌      | ✅        | ✅        |
| `mint`             | ❌      | ❌        | ✅        |
| `authority`        | ✅      | ✅        | ✅        |
| `systemProgram`    | ❌      | ❌        | ✅        |

## Instruction arguments

```ts
type CreateAirdropInstruction = {
    discriminator: bytes;
    id: number /* u64 */;
    merkleRoot: Option<bytes>;
    startsAt: Option<number /* i64 */>;
    endsAt: Option<number /* i64 */>;
    version: Option<number /* u8 */>;
    delegateAuthority: Option<Address>;
}
```