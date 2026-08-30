use anchor_lang::prelude::*;

pub mod airdrop;

pub mod claim_map;
pub mod constants;
pub mod error;
pub mod utils;

use airdrop::*;
use claim_map::*;
//use solana_security_txt::security_txt;

/*security_txt! {
    name: "Dropsy Airdrop",
    project_url: "https://dropsy.org",
    contacts: "scoolmabrouki@gmail.com",
    policy: "https://dropsy.org",
    preferred_languages: "en",
    source_code: "https://github.com",
    encryption: "https://dropsy.org",
    acknowledgements: "We thank all whitehat researchers."
}*/

// test env
//declare_id!("3b97gLF94DDpa7sJQ8orbE3Ab6JQWBsDQTdQ5azoZqcz");
declare_id!("drophxkcW5TRfbMQgo45Yti3q8jVxuxvuNufMnu8WiE");

#[program]
pub mod dropsy_airdrop {
    use super::*;

    pub fn initialize_airdrop_config(
        ctx: Context<InitializedConfig>,
        args: AirdropConfigInitArgs,
    ) -> Result<()> {
        airdrop::instruction::init_airdrop_config(ctx, args)
    }
    pub fn update_airdrop_config(
        ctx: Context<UpdatedConfig>,
        args: AirdropConfigInitArgs,
    ) -> Result<()> {
        airdrop::instruction::update_airdrop_config(ctx, args)
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

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64, airdrop_id: u64) -> Result<()> {
        airdrop::instruction::deposit_tokens(ctx, amount, airdrop_id)
    }

    pub fn claim_airdrop(ctx: Context<ClaimTokens>, args: AirdropClaimArgs) -> Result<()> {
        airdrop::instruction::claim_tokens(ctx, args)
    }

    pub fn create_bitmap(
        ctx: Context<InitialisedBitmap>,
        id: u16,
        total: u32,
        airdrop_id: u64,
    ) -> Result<()> {
        claim_map::instruction::init_bitmap(ctx, id, total, airdrop_id)
    }
}
