use crate::airdrop::{Airdrop, AirdropClaimArgs};
use crate::constants::{BITMAP_SEED, MAX_BITMAP_CLAIM, VERSION_VESTED};
use crate::error::ErrorCode;
use crate::utils::{
    calculate_bitmap_id, calculate_vested_bitmap_id, now, validate_timing, validate_version,
    verify_merkle_proof,
};
use anchor_lang::prelude::*;

pub fn validate_master_treasury(treasury_key: Pubkey, master_treasury: Pubkey) -> Result<()> {
    require_keys_eq!(
        treasury_key,
        master_treasury,
        ErrorCode::InvalidTreasuryAccount
    );
    Ok(())
}

pub fn validate_airdrop_inputs(
    starts_time: i64,
    ends_time: i64,
    version: u8,
    minimum_duration: i64,
) -> Result<()> {
    validate_timing(starts_time, ends_time, minimum_duration)?;
    validate_version(version)?;

    Ok(())
}

pub fn validate_bitmap_data(total: u32, version: u8, ends_at: i64) -> Result<()> {
    let current_time = now()?;
    let max_bitmap_claim = match version {
        0 => MAX_BITMAP_CLAIM,     // Version 0: 1 bit per user
        1 => MAX_BITMAP_CLAIM / 8, // Version 1: 1 byte per user
        _ => MAX_BITMAP_CLAIM / 8, // default fallback
    };

    require!(total > 0, ErrorCode::InvalidTotal);
    require!(total as u64 <= max_bitmap_claim, ErrorCode::BitmapTooLarge);
    require!(current_time <= ends_at, ErrorCode::AirdropEnded);

    Ok(())
}

pub fn validate_vesting_airdrop(
    airdrop_authority: Pubkey,
    authority: Pubkey,
    version: u8,
) -> Result<()> {
    require_keys_eq!(airdrop_authority, authority, ErrorCode::Unauthorized);

    require!(
        version == VERSION_VESTED,
        ErrorCode::InvalidAirdropVestingVersion
    );

    Ok(())
}

pub fn validate_airdrop_claim(
    claimer: &Pubkey,
    airdrop: &Airdrop,
    args: &AirdropClaimArgs,
    expected_version: u8,
) -> Result<()> {
    let current_time = now()?;
    let current_time_ms = now()? * 1000;
    msg!("CLAIMER: {}", claimer);
    msg!("CURRENT TIME: {}", current_time);
    msg!("CURRENT TIME MS: {}", current_time_ms);
    msg!("AIRDROP STARTS_AT: {}", airdrop.starts_at);
    msg!("AIRDROP ENDS_AT: {}", airdrop.ends_at);
    msg!("AIRDROP VERSION: {}", airdrop.version);
    msg!("EXPECTED VERSION: {}", expected_version);
    require!(
        current_time >= airdrop.starts_at,
        ErrorCode::AirdropNotStarted
    );
    require!(current_time < airdrop.ends_at, ErrorCode::AirdropEnded);
    require!(
        airdrop.version == expected_version,
        ErrorCode::InvalidAirdropVersion
    );
    require!(
        verify_merkle_proof(*claimer, airdrop.merkle_root, args),
        ErrorCode::InvalidProof
    );
    Ok(())
}

pub fn validate_bitmap(
    bitmap_key: &Pubkey,
    airdrop_key: &Pubkey,
    index: u64,
    program_id: &Pubkey,
    is_vested: bool,
) -> Result<()> {
    let bitmap_id = if is_vested {
        calculate_vested_bitmap_id(index)
    } else {
        calculate_bitmap_id(index)
    };

    let seeds = &[BITMAP_SEED, airdrop_key.as_ref(), &bitmap_id.to_le_bytes()];
    let (expected_key, _) = Pubkey::find_program_address(seeds, program_id);
    require_keys_eq!(expected_key, *bitmap_key, ErrorCode::InvalidBitmapAccount);
    Ok(())
}
