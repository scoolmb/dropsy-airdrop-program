---
title: Initialize Airdrop Master
description: Overview of the Initialize Airdrop Master instruction
---

# Initialize Airdrop Master

## Instruction accounts

| Name               | Signer | Writable | Required |
| ------------------ | ------ | -------- | -------- |
| `config`           | ❌      | ❌        | ✅        |
| `airdropMaster`    | ❌      | ✅        | ✅        |
| `protocolTreasury` | ❌      | ✅        | ✅        |
| `treasury`         | ❌      | ❌        | ✅        |
| `affiliate`        | ❌      | ✅        | ❌        |
| `masterCreator`    | ✅      | ✅        | ✅        |
| `systemProgram`    | ❌      | ❌        | ✅        |

## Instruction arguments

```ts
type InitializeAirdropMasterInstruction = {
    discriminator: bytes;
    airdropUpdateFee: Option<number /* u64 */>;
    airdropCreationFee: Option<number /* u64 */>;
    airdropClaimFee: Option<number /* u64 */>;
    airdropDelegateFee: Option<number /* u64 */>;
    bitmapCreationFee: Option<number /* u64 */>;
    discountProof: Option<Array<bytes>>;
}
```