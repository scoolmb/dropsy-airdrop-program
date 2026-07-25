---
title: Tokens Deposited
description: Overview of the Tokens Deposited type
---

# Tokens Deposited

## Type definition

```ts
type TokensDeposited = {
    airdrop: Address;
    amount: number /* u64 */;
    newSupply: number /* u64 */;
    depositor: Address;
    timestamp: number /* i64 */;
}
```