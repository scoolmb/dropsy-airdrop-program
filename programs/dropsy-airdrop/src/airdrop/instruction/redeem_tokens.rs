use crate::airdrop::RedeemedTokens;
use crate::airdrop::TokensRedeemed;
use crate::constants::{AIRDROP_SEED, STATE_REDEEMED};
use crate::error::ErrorCode;
use crate::utils::{close_vault_account, now, transfer_tokens_from_vault};
use anchor_lang::prelude::*;

pub fn redeem_tokens(ctx: Context<RedeemedTokens>) -> Result<()> {
    let mut airdrop = ctx.accounts.airdrop.load_mut()?;
    let authority_key = ctx.accounts.authority.key();
    let mint_key = ctx.accounts.mint.key();

    let current_time = now()?;

    require!(current_time > airdrop.ends_at, ErrorCode::AirdropNotEnded);

    let remaining_balance = ctx.accounts.vault.amount;
    let bump = airdrop.bump;

    let signer_seeds: &[&[&[u8]]] = &[&[
        AIRDROP_SEED,
        mint_key.as_ref(),
        authority_key.as_ref(),
        &airdrop.id.to_le_bytes(),
        &[bump],
    ]];

    transfer_tokens_from_vault(
        &ctx.accounts.vault.to_account_info(),
        &ctx.accounts.destination_token_account.to_account_info(),
        &ctx.accounts.airdrop.to_account_info(),
        &ctx.accounts.token_program.to_account_info(),
        &ctx.accounts.mint.to_account_info(),
        signer_seeds,
        remaining_balance,
        ctx.accounts.mint.decimals,
    )?;

    close_vault_account(
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.airdrop.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        signer_seeds,
    )?;

    airdrop.state = STATE_REDEEMED;

    emit!(TokensRedeemed {
        airdrop: ctx.accounts.airdrop.key(),
        authority: airdrop.authority,
        timestamp: current_time,
    });

    msg!("{} redeemed succefully ", remaining_balance);

    Ok(())
}
