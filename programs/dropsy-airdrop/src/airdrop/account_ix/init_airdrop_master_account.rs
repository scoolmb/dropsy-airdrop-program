use crate::airdrop::AirdropConfig;
use crate::constants::AIRDROP_CONFIG_SEED;
use crate::MasterAirdropInitArgs;
use crate::{airdrop::AirdropMaster, constants::AIRDROP_MASTER_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    args: MasterAirdropInitArgs
)]
pub struct InitializedAirdropMaster<'info> {
    #[account(
        seeds = [AIRDROP_CONFIG_SEED],
        bump,
    )]
    pub config: AccountLoader<'info, AirdropConfig>,

    #[account(
        init,
        payer = master_creator,
        space = AirdropMaster::LEN,
        seeds = [AIRDROP_MASTER_SEED, master_creator.key().as_ref()],
        bump,
    )]
    pub airdrop_master: AccountLoader<'info, AirdropMaster>,

    #[account(mut)]
    /// CHECK: treasury
    pub protocol_treasury: UncheckedAccount<'info>,

    /// CHECK: treasury for master
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: affiliate
    pub affiliate: Option<UncheckedAccount<'info>>,

    #[account(mut, signer)]
    pub master_creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}
