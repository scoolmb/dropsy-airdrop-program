use fuzz_accounts::*;
use trident_fuzz::fuzzing::{solana_sdk::msg, *};
mod fuzz_accounts;
mod types;
use types::*;

use crate::types::dropsy_airdrop::{
    InitializeMasterInstruction, InitializeMasterInstructionAccounts,
    InitializeMasterInstructionData,
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
        let authority = self.fuzz_accounts.authority.insert(&mut self.trident, None);
        self.trident.airdrop(&authority, 10 * LAMPORTS_PER_SOL);
    }

    #[flow]
    fn flow1(&mut self) {
        let protocol_fee = self.trident.random_from_range(0..u128::MAX) as u64;
        let master_fee_bps = self.trident.random_from_range(0..u16::MAX);

        let data = MasterInitArgs {
            protocol_fee,
            master_fee_bps,
        };

        let master = self.fuzz_accounts.master.insert(
            &mut self.trident,
            Some(PdaSeeds {
                seeds: &[b"master"],
                program_id: dropsy_airdrop::program_id(),
            }),
        );
        let authority = self
            .fuzz_accounts
            .authority
            .get(&mut self.trident)
            .expect("authority not initialized");

        msg!("authority key :  {} ", authority);
        msg!("master key :  {} ", master);
        let ix = InitializeMasterInstruction::data(InitializeMasterInstructionData::new(data))
            .accounts(InitializeMasterInstructionAccounts::new(
                master, authority, authority,
            ))
            .instruction();

        let account = self.trident.get_account(&authority);

        msg!("authority balance (fuzz validator): {}", account.lamports());

        // Send transaction
        let res = self.trident.process_transaction(&[ix], Some("Initialize"));

        msg!("response {} initialized successfully", res.logs());
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
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows
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
    FuzzTest::fuzz(50, 100);
}
