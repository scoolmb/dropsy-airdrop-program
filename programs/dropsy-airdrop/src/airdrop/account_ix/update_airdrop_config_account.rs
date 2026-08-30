use anchor_lang::prelude::*;

use crate::{
    airdrop::{AirdropConfig, AirdropConfigInitArgs},
    constants::{ADMIN, AIRDROP_CONFIG_SEED},
    error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(
    args: AirdropConfigInitArgs
)]
pub struct UpdatedConfig<'info> {
    #[account(
        mut,
        seeds = [AIRDROP_CONFIG_SEED],
        bump, // check in load 
    )]
    pub config: AccountLoader<'info, AirdropConfig>,

    #[account(
        mut,
        signer,
        address = ADMIN @ ErrorCode::InvalidAdmin,
    )]
    pub authority: Signer<'info>,
}
