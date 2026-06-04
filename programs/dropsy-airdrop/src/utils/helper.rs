use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{
    airdrop::{AirdropInitArgs, AirdropInitData},
    error::ErrorCode,
    utils::{validate_airdrop_inputs, validate_mint},
};

pub fn now() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp)
}

pub fn calculate_fees(total_fee: u64, affiliate_percentage: u8) -> Result<(u64, u64)> {
    // Validate percentage (0-100)
    require!(affiliate_percentage <= 100, ErrorCode::InvalidPercentage);

    // Calculate affiliate fee
    let affiliate_fee = total_fee
        .checked_mul(affiliate_percentage as u64)
        .ok_or(ErrorCode::Overflow)?
        .checked_div(100)
        .ok_or(ErrorCode::Overflow)?;

    // Protocol fee is what remains after affiliate fee
    let protocol_fee = total_fee
        .checked_sub(affiliate_fee)
        .ok_or(ErrorCode::Overflow)?;

    Ok((affiliate_fee, protocol_fee))
}

pub fn prepare_airdrop_data(
    args: AirdropInitArgs,
    mint: &InterfaceAccount<Mint>,
    minimum_duration: i64,
    default_duration: i64,
) -> Result<AirdropInitData> {
    validate_mint(mint)?;

    let now = now()?;
    let starts = args.starts_at.unwrap_or(now);
    let ends = args.ends_at.unwrap_or(starts + default_duration);
    let merkle_root = args.merkle_root.unwrap_or([0u8; 32]);
    let version = args.version.unwrap_or(0);

    validate_airdrop_inputs(starts, ends, version, minimum_duration)?;

    Ok(AirdropInitData {
        id: args.id,
        starts_at: starts,
        ends_at: ends,
        version,
        merkle_root,
    })
}
