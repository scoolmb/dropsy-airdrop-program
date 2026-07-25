---
title: Claim Airdrop
description: Overview of the Claim Airdrop instruction
---

# Claim Airdrop

## Instruction accounts

| Name                      | Signer | Writable | Required |
| ------------------------- | ------ | -------- | -------- |
| `sourceTokenAccount`      | ❌     | ✅       | ✅       |
| `destinationTokenAccount` | ❌     | ✅       | ✅       |
| `claimer`                 | ✅     | ✅       | ✅       |
| `mint`                    | ❌     | ❌       | ✅       |
| `tokenProgram`            | ❌     | ❌       | ✅       |
| `associatedTokenProgram`  | ❌     | ❌       | ✅       |
| `systemProgram`           | ❌     | ❌       | ✅       |
| `airdropConfig`           | ❌     | ❌       | ✅       |
| `protocolTreasury`        | ❌     | ✅       | ✅       |
| `airdropMaster`           | ❌     | ❌       | ✅       |
| `treasury`                | ❌     | ✅       | ✅       |
| `masterCreator`           | ❌     | ❌       | ✅       |
| `airdrop`                 | ❌     | ✅       | ✅       |
| `authority`               | ❌     | ❌       | ✅       |
| `bitmap`                  | ❌     | ✅       | ✅       |

## Instruction arguments

```ts
type ClaimAirdropInstruction = {
  discriminator: bytes;
  index: number /* u64 */;
  proof: Array<bytes>;
  amount: number /* u64 */;
  claimMapIndex: number /* u16 */;
  airdropId: number /* u64 */;
};
```
