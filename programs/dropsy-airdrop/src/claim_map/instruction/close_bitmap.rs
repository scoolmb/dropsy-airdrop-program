use crate::claim_map::{ClosedBitmap};
use crate::error::ErrorCode;
use crate::utils::{FeeRecipient,  process_fee_recipients, validate_master_treasury, validate_redeem_tokens_input};
use anchor_lang::prelude::*;

pub fn close_bitmap(ctx: Context<ClosedBitmap>, airdrop_id: u64, id: u16,) -> Result<()> {
    let airdrop_config = ctx.accounts.airdrop_config.load()?;
    let mut airdrop = ctx.accounts.airdrop.load_mut()?;

    let last_bitmap_id = airdrop
        .bitmap_count
        .checked_sub(1)
        .ok_or(ErrorCode::BitmapCountUnderflow)?;

    require!(
        id == last_bitmap_id,
        ErrorCode::InvalidBitmapIndex
    );

    validate_redeem_tokens_input(
            &ctx.accounts.authority.key(),
            &airdrop,
            &ctx.accounts.mint.key(),
            airdrop_id,
            0,
    )?;

    validate_master_treasury(
        ctx.accounts.protocol_treasury.key(),
        airdrop_config.protocol_treasury,
    )?;

    let recipients: Vec<FeeRecipient> = vec![
        FeeRecipient {
            account: ctx.accounts.protocol_treasury.to_account_info(),
            allocation: airdrop_config.protocol_fee,
        },
    ];

    process_fee_recipients(
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        recipients,
    )?;

    // update stats
    airdrop.bitmap_count = last_bitmap_id;

    Ok(())
}