use crate::{
    constants::{VERSION_BASIC, VERSION_VESTED},
    error::ErrorCode,
    utils::now,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

/// Validate that a value is greater than or equal to `min`.
pub fn validate_min(value: u64, min: u64, field: &str) -> Result<()> {
    require!(value >= min, ErrorCode::ValueBelowTheMinimum);
    msg!("Validation passed: {} >= {} (value={})", field, min, value);
    Ok(())
}

/// Validate that a value is less than or equal to `max`.
pub fn validate_max(value: u64, max: u64, field: &str) -> Result<()> {
    require!(value <= max, ErrorCode::ValueExceedsMaximum);
    msg!("Validation passed: {} <= {} (value={})", field, max, value);
    Ok(())
}

/// Validate that a value is within a range [min, max].
pub fn validate_range(value: u64, min: u64, max: u64, field: &str) -> Result<()> {
    require!(value >= min && value <= max, ErrorCode::ValueOutOfRange);
    msg!(
        "Validation passed: {} in [{}-{}] (value={})",
        field,
        min,
        max,
        value
    );
    Ok(())
}

/// Validate that a percentage (u8) is between 0 and 100.
pub fn validate_percentage(value: u8, field: &str) -> Result<()> {
    require!(value <= 100, ErrorCode::InvalidPercentage);
    msg!("Validation passed: {} <= 100 (value={})", field, value);
    Ok(())
}

/// Validate that a timestamp or duration is non-zero and positive.
pub fn validate_timestamp(value: i64, field: &str) -> Result<()> {
    require!(value > 0, ErrorCode::InvalidTimestamp);
    msg!("Validation passed: {} > 0 (value={})", field, value);
    Ok(())
}

/// Validate that a number is non-zero.
pub fn validate_non_zero(value: u64, field: &str) -> Result<()> {
    require!(value != 0, ErrorCode::NonZeroValueRequired);
    msg!("Validation passed: {} != 0 (value={})", field, value);
    Ok(())
}

pub fn validate_pubkey(value: Pubkey, expected: Pubkey, field: &str) -> Result<()> {
    require_keys_eq!(value, expected, ErrorCode::InvalidPubKey);
    msg!(
        "Validation passed: {} matches expected Pubkey (value={})",
        field,
        value
    );
    Ok(())
}

pub fn validate_dropsy_token(value: Pubkey, expected: Pubkey) -> Result<()> {
    require_keys_eq!(value, expected, ErrorCode::MintMismatch);
    Ok(())
}

pub fn validate_mint(mint: &Mint) -> Result<()> {
    // Must NOT have freeze authority
    /*require!(
        mint.freeze_authority.is_none(),
        ErrorCode::MintHasFreezeAuthority
    );*/

    // Cannot be NFT (decimals == 0)
    require!(mint.decimals > 0, ErrorCode::NftNotAllowed);

    // Must be initialized
    require!(mint.is_initialized, ErrorCode::MintIsNotInitialized);

    Ok(())
}

pub fn validate_token_account(
    ata: &TokenAccount,
    authority: &Pubkey,
    mint: &Pubkey,
    amount: Option<u64>,
) -> Result<()> {
    require!(ata.owner == *authority, ErrorCode::InvalidOwner);
    require!(ata.mint == *mint, ErrorCode::MintMismatch);
    require!(ata.delegate.is_none(), ErrorCode::SourceHasDelegate);
    require!(!ata.is_frozen(), ErrorCode::SourceAccountFrozen);
    require!(
        ata.close_authority.is_none(),
        ErrorCode::SourceHasCloseAuthority
    );
    if let Some(required_amount) = amount {
        require!(
            ata.amount >= required_amount,
            ErrorCode::InsufficientVaultFunds
        );
    }

    Ok(())
}

pub fn validate_version(version: u8) -> Result<()> {
    require!(
        version == VERSION_BASIC || version == VERSION_VESTED,
        ErrorCode::InvalidAirdropVersion
    );
    Ok(())
}

pub fn validate_timing(starts_at: i64, ends_at: i64, minimum_duration: i64) -> Result<()> {
    // Use saturating math for safety
    require!(
        ends_at >= starts_at.saturating_add(minimum_duration),
        ErrorCode::DurationTooShort
    );

    let current_time = now()?;
    require!(
        ends_at.saturating_sub(minimum_duration) >= current_time,
        ErrorCode::InvalidEndTime
    );

    Ok(())
}
