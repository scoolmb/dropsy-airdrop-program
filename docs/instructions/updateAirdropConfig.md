---
title: Update Airdrop Config
description: Overview of the Update Airdrop Config instruction
---

# Update Airdrop Config

## Instruction accounts

| Name        | Signer | Writable | Required |
| ----------- | ------ | -------- | -------- |
| `config`    | ❌      | ✅        | ✅        |
| `authority` | ✅      | ✅        | ✅        |

## Instruction arguments

```ts
type UpdateAirdropConfigInstruction = { discriminator: bytes; args: AirdropConfigInitArgs }
```

## See also

- [AirdropConfigInitArgs](../types/airdropConfigInitArgs.md)