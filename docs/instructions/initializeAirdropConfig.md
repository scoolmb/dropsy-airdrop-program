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
type InitializeAirdropConfigInstruction = { discriminator: bytes; args: AirdropConfigInitArgs }
```

## See also

- [AirdropConfigInitArgs](../types/airdropConfigInitArgs.md)