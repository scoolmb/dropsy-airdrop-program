import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DropsyAirdrop } from "../target/types/dropsy_airdrop";
import { 
  PublicKey, 
  Keypair, 
  LAMPORTS_PER_SOL,
  SystemProgram,
  SYSVAR_RENT_PUBKEY
} from "@solana/web3.js";
import { assert, expect } from "chai";
import { InitializeAirdropConfigInstructionDataArgs } from "../clients/js/src/generated";


describe("Airdrop - Init Config", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DropsyAirdrop as Program<DropsyAirdrop>;

  // Test accounts
  let airdropMasterPda: PublicKey;
  let airdropMasterBump: number;
  let airdropConfigPda: PublicKey;
  let authority: Keypair;
  let mint: Keypair;
  let protocolTreasury: PublicKey; // Dropsy Treasury

  // Constants
  const AIRDROP_VERSION = 1;
  const PROTOCOL_FEE_BPS = 100; // 1%
  const MASTER_FEE_BPS = 200; // 2%

  before(async () => {
    // Generate test keypairs
    authority = Keypair.generate();
    mint = Keypair.generate();

    console.log("Authority PublicKey:", authority.publicKey.toString());
    console.log("Mint PublicKey:", mint.publicKey.toString());
    console.log("Protocol Treasury PublicKey:", protocolTreasury.toString());

    // Airdrop some SOL to authority for testing
    const sig = await provider.connection.requestAirdrop(
      authority.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    // Find PDA for AirdropMaster
    const [masterPda, masterBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("master")],
      program.programId
    );
    airdropMasterPda = masterPda;
    airdropMasterBump = masterBump;

    // Find PDA for AirdropConfig
    const [configPda, _bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("airdrop_config"),

      ],
      program.programId
    );
    airdropConfigPda = configPda;
  });
    protocolTreasury = new PublicKey("3CZmTu4u4JmU7dyYLeMo3pC2Gqm6DDfWkphshFA8vKpq");

  // Helper function to create mint account (simplified)
  async function createMint() {
    // In real tests, you'd use spl-token to create a real mint
    // For this example, we assume mint already exists
    return mint.publicKey;
  }

  describe(" Failure Cases", () => {
    it("Shouldn't initialize AirdropConfig with invalid Admin", async () => {

      console.log("start initAirdropConfig test:");
      const initArgs : any = {
        wlRoot: null, // Assuming no whitelist root for this test
        airdropMasterCreateFee: null, // Assuming no fee for this test
        maxClaimFee: null,
        maxActionFee: null,
        minAirdropDuration: null,
        defaultAirdropDuration: null,
        maxAirdropDuration: null,
        updateGracePeriod: null,
        protocolFee: null,
        masterFeeBps: null,
      };


      // Build the instruction
    
      try {
       const tx = await program.methods
        .initializeAirdropConfig(initArgs)
        .accounts({
          config: airdropConfigPda,
          protocolTreasury,
          authority: authority.publicKey, // Must match ADMIN constant
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
        assert.fail("Should have failed but succeeded");

      } catch (err) {
        console.log("Expected failure:", err);
        expect(err.message).to.include("Admin");
      }
    });
  }); 
});