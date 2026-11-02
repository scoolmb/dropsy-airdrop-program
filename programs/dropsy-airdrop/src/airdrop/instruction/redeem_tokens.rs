use crate::airdrop::RedeemedTokens;
use crate::constants::{AIRDROP_SEED, STATE_REDEEMED};
use crate::error::ErrorCode;
use crate::utils::FeeRecipient;
use crate::utils::process_fee_recipients;
use crate::utils::validate_master_treasury;
use crate::utils::validate_redeem_tokens_input;
use crate::utils::validate_token_account;
use crate::utils::{close_vault_account,  transfer_tokens_from_vault};
use anchor_lang::prelude::*;

pub fn redeem_tokens(
    ctx: Context<RedeemedTokens>,
    airdrop_id: u64,
) -> Result<()> {
    let airdrop_config = ctx.accounts.airdrop_config.load()?;
    let mint_key = ctx.accounts.mint.key();

    let remaining_balance = ctx.accounts.vault.amount;

    validate_token_account(
        &ctx.accounts.vault,
        &ctx.accounts.airdrop.key(),
        &ctx.accounts.mint.key(),
        Some(remaining_balance),
    )?;
    validate_master_treasury(
        ctx.accounts.protocol_treasury.key(),
        airdrop_config.protocol_treasury,
    )?;

    // Read the required fields, then release the account borrow.
    let (bump, id, airdrop_authority) = {
        let airdrop = ctx.accounts.airdrop.load()?;
        require!(
            airdrop.state != STATE_REDEEMED,
            ErrorCode::AirdropAlreadyRedeemed
        );

        validate_redeem_tokens_input(
            &ctx.accounts.authority.key(),
            &airdrop,
            &ctx.accounts.mint.key(),
            airdrop_id,
            0,
        )?;

        (airdrop.bump, airdrop.id, airdrop.authority)
    };



    let signer_seeds: &[&[&[u8]]] = &[&[
        AIRDROP_SEED,
        airdrop_authority.as_ref(),
        mint_key.as_ref(),
        &id.to_le_bytes(),
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

    // Reborrow only after the CPI/helper calls have completed.
    {
        let mut airdrop = ctx.accounts.airdrop.load_mut()?;
        airdrop.state = STATE_REDEEMED;
    }

    msg!("{} redeemed successfully", remaining_balance);

    Ok(())
}