use crate::{
    airdrop::{Airdrop, AirdropInitialized, TokensDeposited},
    claim_map::{BitmapAccount, BitmapInitialized},
    utils::now,
};
use anchor_lang::prelude::*;

/*pub fn emit_initialized_master(address: Pubkey, authority: Pubkey, treasury: Pubkey) -> Result<()> {
    emit!(MasterInitialized {
        address,
        authority,
        treasury,
        timestamp: now()?,
    });
    Ok(())
}*/

pub fn emit_initalized_airdrop(airdrop: &Airdrop, airdrop_key: Pubkey) -> Result<()> {
    emit!(AirdropInitialized {
        airdrop: airdrop_key,
        mint: airdrop.mint,
        authority: airdrop.authority,
        master: airdrop.master,
        merkle_root: airdrop.merkle_root,
        start_time: airdrop.starts_at,
        end_time: airdrop.ends_at,
        timestamp: now()?,
    });
    Ok(())
}

pub fn emit_initalized_bitmap(bitmap: &BitmapAccount, bitmap_key: Pubkey) -> Result<()> {
    emit!(BitmapInitialized {
        airdrop: bitmap.airdrop,
        bitmap: bitmap_key,
        bitmap_id: bitmap.id,
        timestamp: now()?,
    });
    Ok(())
}

pub fn emit_deposited_tokens(airdrop: &Airdrop, amount: u64, airdrop_key: Pubkey) -> Result<()> {
    emit!(TokensDeposited {
        airdrop: airdrop_key,
        amount,
        new_supply: airdrop.supply,
        depositor: airdrop.authority,
        timestamp: now()?,
    });
    Ok(())
}
