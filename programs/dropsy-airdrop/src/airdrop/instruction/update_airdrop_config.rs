use crate::airdrop::{AirdropConfigInitArgs, UpdatedConfig};
use anchor_lang::prelude::*;

pub fn update_airdrop_config(
    ctx: Context<UpdatedConfig>,
    args: AirdropConfigInitArgs,
) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;

    // verify config

    // update config
    config.update(args);

    msg!("Config {} updated successfully", ctx.accounts.config.key());

    Ok(())
}
