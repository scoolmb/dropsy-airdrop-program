use anchor_lang::prelude::*;

pub mod airdrop;

pub mod claim_map;
pub mod constants;
pub mod error;
pub mod utils;

use airdrop::*;
use claim_map::*;

declare_id!("BWd3s27cPuinNkZYqZvRbdfvpGyP9ff5rJZk4WhuNwDw");

#[program]
pub mod dropsy_airdrop {
    use super::*;

    pub fn initialize_airdrop_config(
        ctx: Context<InitializedConfig>,
        args: AirdropConfigInitArgs,
    ) -> Result<()> {
        airdrop::instruction::init_airdrop_config(ctx, args)
    }
    pub fn initialize_airdrop_master(
        ctx: Context<InitializedAirdropMaster>,
        args: MasterAirdropInitArgs,
    ) -> Result<()> {
        airdrop::instruction::init_airdrop_master(ctx, args)
    }

    pub fn create_airdrop(ctx: Context<InitializedAirdrop>, args: AirdropInitArgs) -> Result<()> {
        airdrop::instruction::init_airdrop(ctx, args)
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        airdrop::instruction::deposit_tokens(ctx, amount)
    }

    pub fn claim_airdrop(ctx: Context<ClaimTokens>, args: AirdropClaimArgs) -> Result<()> {
        airdrop::instruction::claim_tokens(ctx, args)
    }

    pub fn create_bitmap(ctx: Context<InitialisedBitmap>, id: u16, total: u32) -> Result<()> {
        claim_map::instruction::init_bitmap(ctx, id, total)
    }

    /*pub fn redeem_tokens(ctx: Context<RedeemedTokens>) -> Result<()> {
        airdrop::instruction::redeem_tokens(ctx)
    }*/
}
