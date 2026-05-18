use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    /* ---------------------------- */
    /*       Validation Errors      */
    /*           001-099            */
    /* ---------------------------- */
    #[msg("Value out of range")]
    ValueOutOfRange = 001,
    #[msg("Value below the minimum")]
    ValueBelowTheMinimum = 002,
    #[msg("Value exceeds the maximum")]
    ValueExceedsMaximum = 003,
    #[msg("Invalid percentage (must be 0-100)")]
    InvalidPercentage = 004,
    #[msg("Invalid timestamp or duration")]
    InvalidTimestamp = 006,
    #[msg("Number must be non-zero")]
    NonZeroValueRequired = 007,
    #[msg("Invalid Pubkey provided")]
    InvalidPubKey = 008,
    #[msg("Invalid Authority Pubkey")]
    UnAuthorized = 009,
    #[msg("Invalid mutability Value")]
    InvalidMutability = 010,
    #[msg("Invalid delegate permission Value")]
    InvalidDelegatePermission = 011,

    #[msg("Claim fee exceeds maximum allowed")]
    ClaimFeeTooHigh = 012,

    #[msg("Action fee exceeds maximum allowed")]
    ActionFeeTooHigh = 013,

    /* ---------------------------- */
    /*         Time & State         */
    /*           100-199            */
    /* ---------------------------- */
    #[msg("Airdrop has not started yet")]
    AirdropNotStarted = 100,
    #[msg("Airdrop has already ended")]
    AirdropEnded = 101,
    #[msg("Airdrop not yet ended")]
    AirdropNotEnded = 102,
    #[msg("Airdrop duration must be at least 24 hours")]
    DurationTooShort = 103,
    #[msg("Airdrop must end at least 24 hours in the future")]
    InvalidEndTime = 104,
    #[msg("Vesting schedule is invalid")]
    InvalidVestingSchedule = 105,
    #[msg("Airdrop is immutable")]
    ImmutableAirdrop = 106,
    #[msg("The target field is immutable")]
    ImmutableField = 107,
    #[msg("Airdrop updates are only allowed until cutoff time before start")]
    UpdateCutoffTimePassed = 108,

    /* ---------------------------- */
    /*       Authorization          */
    /*           200-299            */
    /* ---------------------------- */
    #[msg("This Request Requires Admin Privileges")]
    InvalidAdmin = 200,
    #[msg("Airdrop owner mismatch")]
    OwnerMismatch = 201,
    #[msg("Invalid vault authority")]
    InvalidVaultAuthority = 202,
    #[msg("Destination account owner is not the signer")]
    InvalidDestinationOwner = 203,
    #[msg("Mismatched affiliate PDA")]
    InvalidAffiliatePda = 204,
    #[msg("Not Authorized")]
    Unauthorized = 205,
    #[msg("Transaction sender is not the owner")]
    InvalidOwner = 206,

    /* ---------------------------- */
    /*          Token Mint          */
    /*           300-399            */
    /* ---------------------------- */
    #[msg("Mint does not match stored state")]
    InvalidMint = 300,
    #[msg("Provided mint doesn't match expected mint")]
    MintMismatch = 301,
    #[msg("Vault mint doesn't match airdrop mint")]
    VaultMintMismatch = 302,
    #[msg("Destination mint doesn't match")]
    DestinationMintMismatch = 303,
    #[msg("Mint is frozen")]
    MintIsFrozen = 304,
    #[msg("Invalid token program owner")]
    InvalidMintOwner = 305,
    #[msg("Mint must be initialized")]
    MintIsNotInitialized = 306,
    #[msg("Mint has freeze authority")]
    MintHasFreezeAuthority = 307,
    #[msg("Fungible tokens only (no NFTs)")]
    NftNotAllowed = 308,

    /* ---------------------------- */
    /*    Token Account States      */
    /*           400-499            */
    /* ---------------------------- */
    // Vault Errors
    #[msg("Vault has delegate set")]
    VaultHasDelegate = 401,
    #[msg("Vault is frozen")]
    VaultFrozen = 402,
    #[msg("Vault has close authority")]
    VaultHasCloseAuthority = 403,
    #[msg("Invalid vault account")]
    InvalidAirdropPda = 404,
    #[msg("Vault not initialized")]
    VaultNotInitialized = 405,
    #[msg("Insufficient vault funds")]
    InsufficientVaultFunds = 406,

    // Source Account Errors
    #[msg("Source has delegate set")]
    SourceHasDelegate = 450,
    #[msg("Source account is frozen")]
    SourceAccountFrozen = 451,
    #[msg("Source has close authority")]
    SourceHasCloseAuthority = 452,

    /* ---------------------------- */
    /*       Claims & Bitmaps       */
    /*           500-599            */
    /* ---------------------------- */
    #[msg("Invalid merkle proof")]
    InvalidProof = 500,
    #[msg("Tokens already claimed")]
    AlreadyClaimed = 501,
    #[msg("Missing bitmap PDA")]
    MissingBitmapPda = 502,
    #[msg("Invalid bitmap account")]
    InvalidBitmapAccount = 503,
    #[msg("Invalid bitmap index")]
    InvalidBitmapIndex = 504,
    #[msg("Too many bitmap accounts")]
    TooManyBitmaps = 505,
    #[msg("Bitmap size exceeds limit")]
    BitmapTooLarge = 506,
    #[msg("Invalid total claimers")]
    InvalidTotal = 507,
    #[msg("Bitmap/airdrop mismatch")]
    BitmapAirdropMismatch = 508,
    #[msg("Active bitmaps exist")]
    ActiveBitmapsExist = 509,
    #[msg("Bitmaps already closed")]
    BitmapCountUnderflow = 510,

    /* ---------------------------- */
    /*         Fee Handling         */
    /*           600-699            */
    /* ---------------------------- */
    #[msg("Insufficient SOL deposit")]
    InsufficientDeposit = 600,
    #[msg("Create fee too high (>0.05 SOL)")]
    CreateFeeTooHigh = 601,

    #[msg("Insufficient funds for fee")]
    InsufficientFundsForFee = 603,
    #[msg("Invalid fee vault")]
    InvalidFeeVault = 604,
    #[msg("Invalid fee vault owner")]
    InvalidFeeVaultOwner = 605,
    #[msg("Fee vault not in curve")]
    InvalidFeeVaultCurve = 606,

    /* ---------------------------- */
    /*       General Validation     */
    /*           700-799            */
    /* ---------------------------- */
    #[msg("Invalid PDA account")]
    InvalidPda = 700,
    #[msg("Invalid amount")]
    InvalidAmount = 701,
    #[msg("Arithmetic overflow")]
    Overflow = 702,
    #[msg("Vault not rent exempt")]
    VaultNotRentExempt = 703,
    #[msg("invalid treasury account")]
    InvalidTreasuryAccount = 704,
    #[msg("invalid airdrop version")]
    InvalidAirdropVersion = 705,
    #[msg("Invalid vesting type")]
    InvalidVestingType = 707,
    #[msg("vesting account required for vested airdrop")]
    VestingAccountRequired = 708,
    #[msg("Invalid Parent account provided")]
    InvalidParentAccount = 709,
    #[msg("Airdrop version don't support vesting")]
    InvalidAirdropVestingVersion = 710,
}
