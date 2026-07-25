---
title: Create Bitmap
description: Overview of the Create Bitmap instruction
---

# Create Bitmap

## Instruction accounts

| Name               | Signer | Writable | Required |
| ------------------ | ------ | -------- | -------- |
| `airdropConfig`    | ❌      | ❌        | ✅        |
| `protocolTreasury` | ❌      | ✅        | ✅        |
| `airdropMaster`    | ❌      | ❌        | ✅        |
| `treasury`         | ❌      | ✅        | ✅        |
| `masterCreator`    | ❌      | ❌        | ✅        |
| `bitmap`           | ❌      | ✅        | ✅        |
| `airdrop`          | ❌      | ✅        | ✅        |
| `mint`             | ❌      | ❌        | ✅        |
| `authority`        | ✅      | ✅        | ✅        |
| `systemProgram`    | ❌      | ❌        | ✅        |

## Instruction arguments

```ts
type CreateBitmapInstruction = { discriminator: bytes; id: number /* u16 */; total: number /* u32 */; airdropId: number /* u64 */ }
```