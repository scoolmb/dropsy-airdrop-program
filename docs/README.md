---
title: Dropsy Airdrop Program
description: Overview of the Dropsy Airdrop Program
---

# Dropsy Airdrop Program

## Information

- Address: `3b97gLF94DDpa7sJQ8orbE3Ab6JQWBsDQTdQ5azoZqcz`
- Version: `0.1.0`

## Accounts

- [Airdrop](accounts/airdrop.md)
- [Airdrop Config](accounts/airdropConfig.md)
- [Airdrop Master](accounts/airdropMaster.md)
- [Bitmap Account](accounts/bitmapAccount.md)

## Instructions

- [Claim Airdrop](instructions/claimAirdrop.md)
- [Create Airdrop](instructions/createAirdrop.md)
- [Create Bitmap](instructions/createBitmap.md)
- [Deposit Tokens](instructions/depositTokens.md)
- [Initialize Airdrop Config](instructions/initializeAirdropConfig.md)
- [Initialize Airdrop Master](instructions/initializeAirdropMaster.md)

## PDAs

_This program has no PDAs._

## Defined types

- [Airdrop Initialized](definedTypes/airdropInitialized.md)
- [Bitmap Closed](definedTypes/bitmapClosed.md)
- [Bitmap Initialized](definedTypes/bitmapInitialized.md)
- [Tokens Deposited](definedTypes/tokensDeposited.md)
- [Tokens Redeemed](definedTypes/tokensRedeemed.md)

## Errors

| Name                            | Code   | Message                                                         |
| ------------------------------- | ------ | --------------------------------------------------------------- |
| Value Out Of Range              | `6001` | Value out of range                                              |
| Value Below The Minimum         | `6002` | Value below the minimum                                         |
| Value Exceeds Maximum           | `6003` | Value exceeds the maximum                                       |
| Invalid Percentage              | `6004` | Invalid percentage (must be 0-100)                              |
| Invalid Timestamp               | `6006` | Invalid timestamp or duration                                   |
| Non Zero Value Required         | `6007` | Number must be non-zero                                         |
| Invalid Pub Key                 | `6008` | Invalid Pubkey provided                                         |
| Un Authorized                   | `6009` | Invalid Authority Pubkey                                        |
| Invalid Mutability              | `6010` | Invalid mutability Value                                        |
| Invalid Delegate Permission     | `6011` | Invalid delegate permission Value                               |
| Claim Fee Too High              | `6012` | Claim fee exceeds maximum allowed                               |
| Action Fee Too High             | `6013` | Action fee exceeds maximum allowed                              |
| Airdrop Not Started             | `6100` | Airdrop has not started yet                                     |
| Airdrop Ended                   | `6101` | Airdrop has already ended                                       |
| Airdrop Not Ended               | `6102` | Airdrop not yet ended                                           |
| Duration Too Short              | `6103` | Airdrop duration must be at least 24 hours                      |
| Invalid End Time                | `6104` | Airdrop must end at least 24 hours in the future                |
| Invalid Vesting Schedule        | `6105` | Vesting schedule is invalid                                     |
| Immutable Airdrop               | `6106` | Airdrop is immutable                                            |
| Immutable Field                 | `6107` | The target field is immutable                                   |
| Update Cutoff Time Passed       | `6108` | Airdrop updates are only allowed until cutoff time before start |
| Invalid Admin                   | `6200` | This Request Requires Admin Privileges                          |
| Owner Mismatch                  | `6201` | Airdrop owner mismatch                                          |
| Invalid Vault Authority         | `6202` | Invalid vault authority                                         |
| Invalid Destination Owner       | `6203` | Destination account owner is not the signer                     |
| Invalid Affiliate Pda           | `6204` | Mismatched affiliate PDA                                        |
| Unauthorized                    | `6205` | Not Authorized                                                  |
| Invalid Owner                   | `6206` | Transaction sender is not the owner                             |
| Invalid Mint                    | `6300` | Mint does not match stored state                                |
| Mint Mismatch                   | `6301` | Provided mint doesn't match expected mint                       |
| Vault Mint Mismatch             | `6302` | Vault mint doesn't match airdrop mint                           |
| Destination Mint Mismatch       | `6303` | Destination mint doesn't match                                  |
| Mint Is Frozen                  | `6304` | Mint is frozen                                                  |
| Invalid Mint Owner              | `6305` | Invalid token program owner                                     |
| Mint Is Not Initialized         | `6306` | Mint must be initialized                                        |
| Mint Has Freeze Authority       | `6307` | Mint has freeze authority                                       |
| Nft Not Allowed                 | `6308` | Fungible tokens only (no NFTs)                                  |
| Vault Has Delegate              | `6401` | Vault has delegate set                                          |
| Vault Frozen                    | `6402` | Vault is frozen                                                 |
| Vault Has Close Authority       | `6403` | Vault has close authority                                       |
| Invalid Airdrop Pda             | `6404` | Invalid vault account                                           |
| Vault Not Initialized           | `6405` | Vault not initialized                                           |
| Insufficient Vault Funds        | `6406` | Insufficient vault funds                                        |
| Source Has Delegate             | `6450` | Source has delegate set                                         |
| Source Account Frozen           | `6451` | Source account is frozen                                        |
| Source Has Close Authority      | `6452` | Source has close authority                                      |
| Invalid Proof                   | `6500` | Invalid merkle proof                                            |
| Already Claimed                 | `6501` | Tokens already claimed                                          |
| Missing Bitmap Pda              | `6502` | Missing bitmap PDA                                              |
| Invalid Bitmap Account          | `6503` | Invalid bitmap account                                          |
| Invalid Bitmap Index            | `6504` | Invalid bitmap index                                            |
| Too Many Bitmaps                | `6505` | Too many bitmap accounts                                        |
| Bitmap Too Large                | `6506` | Bitmap size exceeds limit                                       |
| Invalid Total                   | `6507` | Invalid total claimers                                          |
| Bitmap Airdrop Mismatch         | `6508` | Bitmap/airdrop mismatch                                         |
| Active Bitmaps Exist            | `6509` | Active bitmaps exist                                            |
| Bitmap Count Underflow          | `6510` | Bitmaps already closed                                          |
| Insufficient Deposit            | `6600` | Insufficient SOL deposit                                        |
| Create Fee Too High             | `6601` | Create fee too high (>0.05 SOL)                                 |
| Insufficient Funds For Fee      | `6603` | Insufficient funds for fee                                      |
| Invalid Fee Vault               | `6604` | Invalid fee vault                                               |
| Invalid Fee Vault Owner         | `6605` | Invalid fee vault owner                                         |
| Invalid Fee Vault Curve         | `6606` | Fee vault not in curve                                          |
| Invalid Pda                     | `6700` | Invalid PDA account                                             |
| Invalid Amount                  | `6701` | Invalid amount                                                  |
| Overflow                        | `6702` | Arithmetic overflow                                             |
| Vault Not Rent Exempt           | `6703` | Vault not rent exempt                                           |
| Invalid Treasury Account        | `6704` | invalid treasury account                                        |
| Invalid Airdrop Version         | `6705` | invalid airdrop version                                         |
| Invalid Vesting Type            | `6707` | Invalid vesting type                                            |
| Vesting Account Required        | `6708` | vesting account required for vested airdrop                     |
| Invalid Parent Account          | `6709` | Invalid Parent account provided                                 |
| Invalid Airdrop Vesting Version | `6710` | Airdrop version don't support vesting                           |
