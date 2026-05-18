use crate::airdrop::{AirdropConfigInitArgs, InitializedConfig};
use anchor_lang::prelude::*;

pub fn init_airdrop_config(
    ctx: Context<InitializedConfig>,
    args: AirdropConfigInitArgs,
) -> Result<()> {
    let mut config = ctx.accounts.config.load_init()?;

    config.init(
        ctx.accounts.authority.key(),
        ctx.accounts.protocol_treasury.key(),
        args,
        ctx.bumps.config,
    );

    msg!(
        "Config {} initialized successfully",
        ctx.accounts.config.key()
    );

    Ok(())
}
