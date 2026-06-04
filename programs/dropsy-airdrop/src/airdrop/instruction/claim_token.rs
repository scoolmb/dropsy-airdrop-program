use crate::airdrop::{AirdropClaimArgs, ClaimTokens};
use crate::constants::{AIRDROP_SEED, VERSION_BASIC};
use crate::error::ErrorCode;
use crate::utils::{
    basic_claim, process_fee_recipients, transfer_tokens_from_vault, validate_airdrop_claim,
    validate_bitmap, validate_master_treasury, validate_token_account, FeeRecipient,
};

use anchor_lang::prelude::*;

pub fn claim_tokens(ctx: Context<ClaimTokens>, args: AirdropClaimArgs) -> Result<()> {
    let airdrop_master = ctx.accounts.base.airdrop_master.load()?;
    let airdrop_config = ctx.accounts.base.airdrop_config.load()?;
    let claimer_key = ctx.accounts.claim_base.claimer.key();
    let airdrop = ctx.accounts.airdrop.load()?;
    let mut bitmap = ctx.accounts.bitmap.load_mut()?;

    validate_master_treasury(ctx.accounts.base.treasury.key(), airdrop_master.treasury)?;
    validate_master_treasury(
        ctx.accounts.base.protocol_treasury.key(),
        airdrop_config.protocol_treasury,
    )?;
    validate_airdrop_claim(&claimer_key, &airdrop, &args, VERSION_BASIC)?;
    validate_bitmap(
        &ctx.accounts.bitmap.key(),
        &ctx.accounts.airdrop.key(),
        args.index,
        ctx.program_id,
        false,
    )?;
    validate_token_account(
        &ctx.accounts.claim_base.source_token_account,
        &ctx.accounts.airdrop.key(),
        &ctx.accounts.claim_base.mint.key(),
        Some(args.amount),
    )?;
    basic_claim(&mut bitmap, args.index)?;

    let signer_seeds: &[&[&[u8]]] = &[&[
        AIRDROP_SEED,
        airdrop.authority.as_ref(),
        airdrop.mint.as_ref(),
        &airdrop.id.to_le_bytes(),
        &[airdrop.bump],
    ]];

    let decimals = ctx.accounts.claim_base.mint.decimals;
    let amount_in_base_units = args
        .amount
        .checked_mul(10u64.pow(decimals as u32))
        .ok_or(ErrorCode::Overflow)?;

    transfer_tokens_from_vault(
        &ctx.accounts
            .claim_base
            .source_token_account
            .to_account_info(),
        &ctx.accounts
            .claim_base
            .destination_token_account
            .to_account_info(),
        &ctx.accounts.airdrop.to_account_info(),
        &ctx.accounts.claim_base.token_program.to_account_info(),
        &ctx.accounts.claim_base.mint.to_account_info(),
        signer_seeds,
        amount_in_base_units,
        decimals,
    )?;

    let master_share = airdrop_master.airdrop_claim_fee / 2;
    let protocol_share = airdrop_master
        .airdrop_claim_fee
        .checked_sub(master_share)
        .ok_or(ErrorCode::Overflow)?;

    let recipients: Vec<FeeRecipient> = vec![
        FeeRecipient {
            account: ctx.accounts.base.treasury.to_account_info(),
            allocation: master_share,
        },
        FeeRecipient {
            account: ctx.accounts.base.protocol_treasury.to_account_info(),
            allocation: protocol_share,
        },
    ];

    process_fee_recipients(
        &ctx.accounts.claim_base.claimer.to_account_info(),
        &ctx.accounts.claim_base.system_program.to_account_info(),
        airdrop_master.airdrop_claim_fee,
        recipients,
    )?;

    msg!(
        "{} Tokens claimed successfully by {}",
        args.amount,
        claimer_key
    );

    Ok(())
}
