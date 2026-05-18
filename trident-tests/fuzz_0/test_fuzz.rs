use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

use crate::types::dropsy_airdrop::{
    InitializeInstruction, InitializeInstructionAccounts, InitializeInstructionData,
};

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Trident client for interacting with the Solana program
    trident: Trident,
    /// Storage for all account addresses used in fuzz testing
    fuzz_accounts: AccountAddresses,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: AccountAddresses::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // Perform any initialization here, this method will be executed
        // at the start of each iteration
    }

    #[flow]
    fn flow1(&mut self) {
        // 🔥 Random protocol fee
        //let protocol_fee = self.trident.random_from_range(0..u64::MAX);
        let protocol_fee = self.trident.random_from_range(0..u128::MAX) as u64;

        // Authority signer
        let authority = self.fuzz_accounts.authority.insert(&mut self.trident, None);

        self.trident.airdrop(&authority, 10 * LAMPORTS_PER_SOL);

        let ix = InitializeInstruction::data(InitializeInstructionData::new(protocol_fee))
            .accounts(InitializeInstructionAccounts::new(authority))
            .instruction();

        // Send transaction
        let res = self.trident.process_transaction(&[ix], Some("Initialize"));
        // Smart assertion: ensure protocol_fee > 10_000 fails
        if protocol_fee > 10_000 {
            // Should fail with InvalidFee error
            assert!(
                res.is_error(),
                "Expected error for protocol_fee: {}, but transaction succeeded",
                protocol_fee
            );
        } else {
            // Should succeed
            assert!(
                res.is_success(),
                "Expected success for protocol_fee: {}, but transaction failed: {:?}",
                protocol_fee,
                res.get_custom_error_code()
            );
        }
    }

    #[flow]
    fn flow2(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows
    }

    #[end]
    fn end(&mut self) {
        // Perform any cleanup here, this method will be executed
        // at the end of each iteration
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
