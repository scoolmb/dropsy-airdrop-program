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
pub struct InitializedConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = AirdropConfig::LEN,
        seeds = [AIRDROP_CONFIG_SEED],
        bump,
    )]
    pub config: AccountLoader<'info, AirdropConfig>,

    /// CHECK: this is The Treasury
    pub protocol_treasury: UncheckedAccount<'info>,

    #[account(
        mut,
        signer,
        address = ADMIN @ ErrorCode::InvalidAdmin,
    )]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
