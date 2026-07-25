---
title: Deposit Tokens
description: Overview of the Deposit Tokens instruction
---

# Deposit Tokens

## Instruction accounts

| Name                     | Signer | Writable | Required |
| ------------------------ | ------ | -------- | -------- |
| `sourceTokenAccount`     | ❌      | ✅        | ✅        |
| `vault`                  | ❌      | ✅        | ✅        |
| `airdrop`                | ❌      | ✅        | ✅        |
| `mint`                   | ❌      | ❌        | ✅        |
| `authority`              | ✅      | ✅        | ✅        |
| `tokenProgram`           | ❌      | ❌        | ✅        |
| `associatedTokenProgram` | ❌      | ❌        | ✅        |
| `systemProgram`          | ❌      | ❌        | ✅        |

## Instruction arguments

```ts
type DepositTokensInstruction = { discriminator: bytes; amount: number /* u64 */; airdropId: number /* u64 */ }
```