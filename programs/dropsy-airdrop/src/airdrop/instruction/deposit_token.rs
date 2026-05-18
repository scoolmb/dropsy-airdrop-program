use crate::airdrop::DepositTokens;
use crate::error::ErrorCode;
use crate::utils::{
    emit_deposited_tokens, now, transfer_tokens_to_pda, validate_mint, validate_token_account,
};
use anchor_lang::prelude::*;

pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
    let mut airdrop = ctx.accounts.airdrop.load_mut()?;
    let current_time = now()?;
    let ends_at = airdrop.ends_at;

    require!(current_time <= ends_at, ErrorCode::AirdropEnded);
    require!(amount > 0, ErrorCode::InvalidAmount);

    validate_mint(&ctx.accounts.mint)?;
    validate_token_account(
        &ctx.accounts.source_token_account,
        &ctx.accounts.authority.key(),
        &ctx.accounts.mint.key(),
        Some(amount),
    )?;

    airdrop.supply = airdrop
        .supply
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    transfer_tokens_to_pda(
        &ctx.accounts.source_token_account.to_account_info(),
        &ctx.accounts.vault.to_account_info(),
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.token_program.to_account_info(),
        &ctx.accounts.mint.to_account_info(),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    emit_deposited_tokens(&airdrop, amount, ctx.accounts.airdrop.key())?;

    msg!("{} tokens deposited to airdrop", amount);

    Ok(())
}
