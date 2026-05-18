use crate::claim_map::BitmapAccount;
use crate::constants::{BITMAP_SIZE, MAX_BITMAP_CLAIM};
use crate::error::ErrorCode;

use anchor_lang::prelude::*;

const _: () = assert!(MAX_BITMAP_CLAIM > 0, "MAX_BITMAP_CLAIM must be > 0");

fn relative_index(index: u64) -> usize {
    (index % MAX_BITMAP_CLAIM) as usize
}
pub fn calculate_bitmap_id(index: u64) -> u16 {
    (index / MAX_BITMAP_CLAIM) as u16
}
pub fn calculate_vested_bitmap_id(index: u64) -> u16 {
    (index / BITMAP_SIZE as u64) as u16
}
pub fn calculate_fee(amount: u64, fee_percent: u64) -> Result<(u64, u64)> {
    require!(fee_percent <= 100, ErrorCode::InvalidPercentage);

    let fee = amount
        .checked_mul(fee_percent)
        .ok_or(ErrorCode::Overflow)?
        .checked_div(100)
        .ok_or(ErrorCode::Overflow)?;

    let amount_after_fee = amount.checked_sub(fee).ok_or(ErrorCode::Overflow)?;

    Ok((fee, amount_after_fee))
}
pub fn is_claimed(bitmap: &[u8], index: u64) -> Result<bool> {
    let byte_index = (index / 8) as usize;
    let bit_index = (index % 8) as u8;
    require!(byte_index < bitmap.len(), ErrorCode::InvalidBitmapIndex);
    Ok(bitmap[byte_index] & (1 << bit_index) != 0)
}

pub fn set_claimed(bitmap: &mut [u8], index: u64) -> Result<()> {
    let byte_index = (index / 8) as usize;
    let bit_index = (index % 8) as u8;
    require!(byte_index < bitmap.len(), ErrorCode::InvalidBitmapIndex);
    bitmap[byte_index] |= 1 << bit_index;
    Ok(())
}

pub fn basic_claim(bitmap: &mut BitmapAccount, index: u64) -> Result<()> {
    let idx = relative_index(index);

    require!(
        !is_claimed(&bitmap.claimed_bitmap, idx as u64)?,
        ErrorCode::AlreadyClaimed
    );
    set_claimed(&mut bitmap.claimed_bitmap, idx as u64)?;

    Ok(())
}

/*pub fn vested_claim(
    bitmap: &mut BitmapAccount,
    vesting_account: &VestingAccount,
    index: u64, // user's global index
    start_time: i64,
    end_time: i64,
    total_amount: u64,
) -> Result<u64> {
    // Convert to relative index in bitmap
    let relative_idx = relative_index(index);
    let current_time = now()?;

    // Calculate total vested amount using VestingCalculator
    let vested_amount = VestingCalculator::calculate_vested_amount(
        vesting_account,
        total_amount,
        start_time,
        end_time,
        current_time,
    )?;

    // Get already claimed percentage (0..100)
    let already_claimed_percentage = bitmap.get_percentage_claimed(relative_idx)?;

    // Calculate total vested percentage (0..100)
    let vested_percentage = (vested_amount * 100 / total_amount) as u8;

    //  Compute eligible percentage
    let eligible_percentage = vested_percentage.saturating_sub(already_claimed_percentage);
    require!(eligible_percentage > 0, ErrorCode::AlreadyClaimed);

    // Compute eligible token amount
    let eligible_amount = total_amount
        .saturating_mul(eligible_percentage as u64)
        .checked_div(100)
        .ok_or(ErrorCode::Overflow)?;

    // Update bitmap with new claimed percentage
    let new_claimed_percentage = already_claimed_percentage.saturating_add(eligible_percentage);
    bitmap.set_percentage_claimed(relative_idx, new_claimed_percentage)?;

    Ok(eligible_amount)
}*/
