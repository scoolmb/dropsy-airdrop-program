use anchor_lang::prelude::Pubkey;
use sha2::{Digest, Sha256};

use crate::airdrop::AirdropClaimArgs;

pub fn hash_two(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let (first, second) = if left <= right {
        (left, right)
    } else {
        (right, left)
    };
    let mut hasher = Sha256::new();
    hasher.update(&first);
    hasher.update(&second);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

pub fn hash_leaf(index: u64, wallet: &Pubkey, amount: u64) -> [u8; 32] {
    let data = format!("{}:{}:{}", index, wallet, amount);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

pub fn hash_wallet_leaf(wallet: &Pubkey) -> [u8; 32] {
    // Use same pattern as hash_leaf - just hash the string representation
    let data = wallet.to_string(); // "Vote111111111111111111111111111111111111111"
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.finalize().into()
}

pub fn verify_discount_proof(wallet: Pubkey, root: [u8; 32], proof: &Vec<[u8; 32]>) -> bool {
    let mut computed_hash = hash_wallet_leaf(&wallet);

    for sibling in proof.iter() {
        computed_hash = hash_two(computed_hash, *sibling);
    }

    computed_hash == root
}

pub fn verify_merkle_proof(wallet: Pubkey, root: [u8; 32], args: &AirdropClaimArgs) -> bool {
    let leaf = hash_leaf(args.index, &wallet, args.amount);
    let mut computed_hash = leaf;
    for sibling in args.proof.iter() {
        computed_hash = hash_two(computed_hash, *sibling);
    }
    computed_hash == root
}
