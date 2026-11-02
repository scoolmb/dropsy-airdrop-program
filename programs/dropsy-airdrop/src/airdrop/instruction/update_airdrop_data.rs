
use crate::airdrop::{AirdropUpdateArgs, UpdatedAirdrop};
use crate::utils::{
    validate_airdrop_identity,
    validate_update_merkle_permissions,
};
use anchor_lang::prelude::*;

pub fn update_airdrop_data(
    ctx: Context<UpdatedAirdrop>,
    args: AirdropUpdateArgs,
) -> Result<()> {
    let signer = ctx.accounts.signer.key();
    let authority = ctx.accounts.authority.key();
    let mint = ctx.accounts.mint.key();

    {
        let airdrop = ctx.accounts.airdrop.load()?;

        validate_airdrop_identity(
            &airdrop,
            &authority,
            &mint,
            args.id,
            0,
        )?;

        validate_update_merkle_permissions(&airdrop, signer)?;
    }

    {
        let mut airdrop = ctx.accounts.airdrop.load_mut()?;
        airdrop.update_merkle_root(args.merkle_root);
    }

    Ok(())
}
