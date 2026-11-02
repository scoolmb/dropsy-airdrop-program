use crate::utils::validate_whitelist;
use crate::{
    airdrop::{InitializedAirdropMaster, MasterAirdropInitArgs},
    utils::{validate_master_treasury},
};
use anchor_lang::prelude::*;

pub fn init_airdrop_master(
    ctx: Context<InitializedAirdropMaster>,
    args: MasterAirdropInitArgs,
) -> Result<()> {
    let mut airdrop_master = ctx.accounts.airdrop_master.load_init()?;
    let airdrop_config = ctx.accounts.config.load()?;


    validate_master_treasury(
        ctx.accounts.protocol_treasury.key(),
        airdrop_config.protocol_treasury,
    )?;

    args.validate(airdrop_config.max_claim_fee, airdrop_config.max_action_fee)?;

    validate_whitelist(
        ctx.accounts.master_creator.key(),
        airdrop_config.wl_root,
        args.wl_proof.as_ref(),   
    )?;

    airdrop_master.init(
        ctx.accounts.master_creator.key(),
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
