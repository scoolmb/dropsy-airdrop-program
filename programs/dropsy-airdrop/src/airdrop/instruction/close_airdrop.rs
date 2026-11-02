use crate::airdrop::ClosedAirdrop;
use crate::constants::STATE_REDEEMED;
use crate::error::ErrorCode;
use crate::utils::{FeeRecipient, process_fee_recipients, validate_master_treasury};
use anchor_lang::prelude::*;

pub fn close_airdrop(ctx: Context<ClosedAirdrop>, _airdrop_id: u64,) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let airdrop_config = ctx.accounts.airdrop_config.load()?;
    {
        let airdrop = ctx.accounts.airdrop.load()?;
        require!(
            airdrop.id == _airdrop_id,
            ErrorCode::InvalidAirdropId
        );
        require!(
            airdrop.state == STATE_REDEEMED,
            ErrorCode::AirdropNotEnded
        );
        require!(
            airdrop.bitmap_count == 0,
            ErrorCode::ActiveBitmapsExist
        );
        require_keys_eq!(
            authority,
            airdrop.authority,
            ErrorCode::UnAuthorized
        );
        
    } 
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

    ctx.accounts
        .airdrop
        .close(ctx.accounts.authority.to_account_info())?;

    

    /*emit!(AirdropClosed {
        airdrop: ctx.accounts.airdrop.key(),
        authority: airdrop.authority,
        timestamp: now()?,
    });*/

    msg!("Airdrop {} closed succefully", ctx.accounts.airdrop.key());

    Ok(())
}