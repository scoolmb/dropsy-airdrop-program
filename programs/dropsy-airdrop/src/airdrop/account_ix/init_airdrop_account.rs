use anchor_lang::{prelude::*, system_program};
use anchor_spl::token_interface::Mint;
use crate::AirdropInitArgs;
use crate::airdrop::AirdropConfig;
use crate::constants::AIRDROP_CONFIG_SEED;
use crate::error::ErrorCode;
use crate::{airdrop::{Airdrop, AirdropMaster}, constants::{AIRDROP_MASTER_SEED, AIRDROP_SEED}, };

#[derive(Accounts)]
pub struct BaseAirdropInitStruct<'info> {
   
    #[account(
        seeds = [AIRDROP_CONFIG_SEED],
        bump,
        has_one = protocol_treasury @ ErrorCode::InvalidTreasuryAccount,
    )]
    pub airdrop_config: AccountLoader<'info, AirdropConfig>,

    #[account(mut)]
    /// CHECK: Treasury
    pub protocol_treasury: UncheckedAccount<'info>,

     #[account(
        seeds = [AIRDROP_MASTER_SEED, master_creator.key().as_ref()],
        bump,
        has_one = treasury @ ErrorCode::InvalidTreasuryAccount,
    )]
    pub airdrop_master: AccountLoader<'info, AirdropMaster>,

    #[account(mut)]
    /// CHECK: Treasury
    pub treasury: UncheckedAccount<'info>,

    /// CHECK: creator
    pub master_creator: UncheckedAccount<'info>,

}


#[derive(Accounts)]
#[instruction(
    args: AirdropInitArgs
)]
pub struct InitializedAirdrop<'info> {
     pub base : BaseAirdropInitStruct<'info>,

    // airdrop pda to be initialized
    #[account(
        init,
        payer = authority,
        seeds = [
            AIRDROP_SEED, 
            authority.key().as_ref(),
            mint.key().as_ref(), 
        ],
        space = Airdrop::LEN,
        bump,
    )]
    pub airdrop: AccountLoader<'info, Airdrop>,

    pub mint: InterfaceAccount<'info, Mint>,
    // authority
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
