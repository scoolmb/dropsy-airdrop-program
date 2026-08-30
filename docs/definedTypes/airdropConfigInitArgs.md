---
title: Airdrop Config Init Args
description: Overview of the Airdrop Config Init Args type
---

# Airdrop Config Init Args

## Type definition

```ts
type AirdropConfigInitArgs = {
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
  protocolTreasury: Option<Address>;
};
```
