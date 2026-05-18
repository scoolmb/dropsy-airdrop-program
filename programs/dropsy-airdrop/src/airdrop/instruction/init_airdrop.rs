use crate::airdrop::{AirdropInitArgs, InitializedAirdrop};

use crate::error::ErrorCode;
use crate::utils::{
    emit_initalized_airdrop, prepare_airdrop_data, process_fee_recipients,
    validate_master_treasury, FeeRecipient,
};
use anchor_lang::prelude::*;

pub fn init_airdrop(ctx: Context<InitializedAirdrop>, args: AirdropInitArgs) -> Result<()> {
    let airdrop_master = ctx.accounts.base.airdrop_master.load()?;
    let airdrop_config = ctx.accounts.base.airdrop_config.load()?;
    let mut airdrop = ctx.accounts.airdrop.load_init()?;
    let treasury_info = ctx.accounts.base.treasury.to_account_info();

    validate_master_treasury(ctx.accounts.base.treasury.key(), airdrop_master.treasury)?;
    validate_master_treasury(
        ctx.accounts.base.protocol_treasury.key(),
        airdrop_config.protocol_treasury,
    )?;

    let init_data = prepare_airdrop_data(
        args,
        &ctx.accounts.mint,
        airdrop_config.min_airdrop_duration,
        airdrop_config.max_airdrop_duration,
    )?;

    let master_share = airdrop_master.airdrop_creation_fee / 2;
    let protocol_share = airdrop_master
        .airdrop_creation_fee
        .checked_sub(master_share)
        .ok_or(ErrorCode::Overflow)?;

    let recipients: Vec<FeeRecipient> = vec![
        FeeRecipient {
            account: treasury_info,
            allocation: master_share,
        },
        FeeRecipient {
            account: ctx.accounts.base.protocol_treasury.to_account_info(),
            allocation: protocol_share,
        },
    ];

    process_fee_recipients(
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        airdrop_master.airdrop_creation_fee,
        recipients,
    )?;

    airdrop.init(
        ctx.accounts.base.airdrop_master.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.mint.key(),
        init_data,
        ctx.bumps.airdrop,
    )?;

    emit_initalized_airdrop(&airdrop, ctx.accounts.airdrop.key())?;

    Ok(())
}
