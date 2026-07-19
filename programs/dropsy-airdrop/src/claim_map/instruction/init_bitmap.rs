use crate::claim_map::InitialisedBitmap;

use crate::error::ErrorCode;
use crate::utils::{emit_initalized_bitmap, validate_bitmap_data, validate_master_treasury};
use anchor_lang::prelude::*;

pub fn init_bitmap(
    ctx: Context<InitialisedBitmap>,
    id: u16,
    total: u32,
    //airdrop_id: u64,
) -> Result<()> {
    let airdrop_config = ctx.accounts.base.airdrop_config.load()?;
    let airdrop_master = ctx.accounts.base.airdrop_master.load()?;
    let mut airdrop = ctx.accounts.airdrop.load_mut()?;
    let mut bitmap_account = ctx.accounts.bitmap.load_init()?;

    validate_master_treasury(ctx.accounts.base.treasury.key(), airdrop_master.treasury)?;
    validate_master_treasury(
        ctx.accounts.base.protocol_treasury.key(),
        airdrop_config.protocol_treasury,
    )?;

    validate_bitmap_data(total, airdrop.version, airdrop.ends_at)?;

    bitmap_account.init(
        ctx.accounts.authority.key(),
        ctx.accounts.airdrop.key(),
        total,
        id,
        ctx.bumps.bitmap,
        airdrop.version,
    );

    airdrop.bitmap_count = airdrop
        .bitmap_count
        .checked_add(1)
        .ok_or(ErrorCode::TooManyBitmaps)?;

    /*let recipients: Vec<FeeRecipient> = vec![
        FeeRecipient {
            account: ctx.accounts.base.treasury.to_account_info(),
            allocation: airdrop_master.bitmap_creation_fee,
        },
        FeeRecipient {
            account: ctx.accounts.base.protocol_treasury.to_account_info(),
            allocation: airdrop_config.protocol_fee,
        },
    ];*/

    /*process_fee_recipients(
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        recipients,
    )?;*/

    emit_initalized_bitmap(&bitmap_account, ctx.accounts.bitmap.key())?;

    Ok(())
}
