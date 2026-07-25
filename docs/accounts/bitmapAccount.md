---
title: Bitmap Account
description: Overview of the Bitmap Account account
---

# Bitmap Account

## Account data

```ts
type BitmapAccount = {
    discriminator: bytes;
    authority: Address;
    airdrop: Address;
    total: number /* u32 */;
    claimedBitmap: bytes;
    id: number /* u16 */;
    version: number /* u8 */;
    bump: number /* u8 */;
}
```

This account has a fixed size of 1080 bytes.