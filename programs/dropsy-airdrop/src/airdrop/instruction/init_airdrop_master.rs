use crate::{
    airdrop::{InitializedAirdropMaster, MasterAirdropInitArgs},
    //error::ErrorCode,
    utils::{validate_master_treasury, verify_discount_proof},
};
use anchor_lang::prelude::*;

pub fn init_airdrop_master(
    ctx: Context<InitializedAirdropMaster>,
    args: MasterAirdropInitArgs,
) -> Result<()> {
    let mut airdrop_master = ctx.accounts.airdrop_master.load_init()?;
    let airdrop_config = ctx.accounts.config.load()?;
    //let affiliate_info = ctx.accounts.affiliate.as_ref().map(|a| a.as_ref());
    //let mut total_fee = airdrop_config.airdrop_master_create_fee;
    //let affiliate_percentage = 5;

    validate_master_treasury(
        ctx.accounts.protocol_treasury.key(),
        airdrop_config.protocol_treasury,
    )?;

    args.validate(airdrop_config.max_claim_fee, airdrop_config.max_action_fee)?;

    if let Some(proof) = args.discount_proof.as_ref() {
        if verify_discount_proof(ctx.accounts.creator.key(), airdrop_config.wl_root, proof) {
            // 50% discount
            //total_fee = total_fee.checked_div(2).ok_or(ErrorCode::Overflow)?;
        }
    }

    /*let proof = args
        .discount_proof
        .as_ref()
        .ok_or(ErrorCode::MintMismatch)?;

    // Verify discount proof
    let is_valid = verify_discount_proof(ctx.accounts.creator.key(), airdrop_config.wl_root, proof);
    if !is_valid {
        return Err(ErrorCode::InvalidAdmin.into());
    }

    // Apply discount (50%)
    total_fee = total_fee.checked_div(2).ok_or(ErrorCode::Overflow)?;*/

    airdrop_master.init(
        ctx.accounts.creator.key(),
        ctx.accounts.treasury.key(),
        args,
        ctx.bumps.airdrop_master,
    );
    /*
    process_fees(
        &ctx.accounts.creator.to_account_info(),
        affiliate_info,
        &ctx.accounts.protocol_treasury.to_account_info(),
        &ctx.accounts.system_program,
        total_fee,
        affiliate_percentage,
    )?;
    */
    msg!(
        "Airdrop Master {} initialized successfully",
        &ctx.accounts.airdrop_master.key()
    );
    Ok(())
}
