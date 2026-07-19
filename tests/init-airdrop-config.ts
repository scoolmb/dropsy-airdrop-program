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
  const wallet = provider.wallet as anchor.Wallet;

  // Test accounts
  let airdropMasterPda: PublicKey;
  let airdropMasterBump: number;
  let airdropConfigPda: PublicKey;
  let airdropConfigBump: number;
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
    const [configPda, configBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("airdrop_config"),

      ],
      program.programId
    );
    airdropConfigPda = configPda;
    airdropConfigBump = configBump;
  });
    protocolTreasury = new PublicKey("3CZmTu4u4JmU7dyYLeMo3pC2Gqm6DDfWkphshFA8vKpq");

  // Helper function to create mint account (simplified)
  async function createMint() {
    // In real tests, you'd use spl-token to create a real mint
    // For this example, we assume mint already exists
    return mint.publicKey;
  }

  describe("✅ Success Cases", () => {
    /*it("Should initialize AirdropMaster successfully", async () => {
      // First, initialize the master account
      const tx = await program.methods
        .initAirdropMaster()
        .accounts({
          authority: authority.publicKey,
          airdropMaster: airdropMasterPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      // Fetch the master account to verify
      const masterAccount = await program.account.airdropMaster.fetch(
        airdropMasterPda
      );

      expect(masterAccount.authority.toString()).to.equal(
        authority.publicKey.toString()
      );
      expect(masterAccount.airdropIdCounter).to.equal(0);
      expect(masterAccount.airdropClaimFee).to.equal(0);
      expect(masterAccount.protocolFeeBps).to.equal(0);
      expect(masterAccount.masterFeeBps).to.equal(0);

      console.log("✅ AirdropMaster initialized at:", airdropMasterPda.toString());
    });*/

    it("Should initialize AirdropConfig with valid parameters", async () => {
      // Get the mint address
      //const mintPubkey = await createMint();
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
    console.log("✅ AirdropConfig initialized successfully at:", airdropConfigPda.toString());

      // Fetch the config account
      const configAccount = await program.account.airdropConfig.fetch(
        airdropConfigPda
      );

      // Verify all fields
      expect(configAccount.authority.toString()).to.equal(
        authority.publicKey.toString()
      );
      expect(configAccount.version).to.equal(AIRDROP_VERSION);
      expect(configAccount.protocolFee).to.equal(PROTOCOL_FEE_BPS);
      expect(configAccount.masterFeeBps).to.equal(MASTER_FEE_BPS);
      /*expect(configAccount.isPaused).to.be.false;
      expect(configAccount.totalAirdropCount).to.equal(0);
      expect(configAccount.totalClaimedAmount.toString()).to.equal("0");
*/
      console.log("✅ AirdropConfig initialized at:", airdropConfigPda.toString());
    });

    /*it("Should allow multiple configs for different mints", async () => {
      // Create a second mint
      const secondMint = Keypair.generate();
      
      // Find PDA for second config
      const [secondConfigPda] = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          authority.publicKey.toBuffer(),
          secondMint.publicKey.toBuffer(),
        ],
        program.programId
      );

      // Initialize second config
      await program.methods
        .initAirdropConfig(
          AIRDROP_VERSION,
          PROTOCOL_FEE_BPS,
          MASTER_FEE_BPS
        )
        .accounts({
          authority: authority.publicKey,
          airdropMaster: airdropMasterPda,
          airdropConfig: secondConfigPda,
          mint: secondMint.publicKey,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([authority])
        .rpc();

      // Verify both configs exist
      const config1 = await program.account.airdropConfig.fetch(airdropConfigPda);
      const config2 = await program.account.airdropConfig.fetch(secondConfigPda);

      expect(config1.mint.toString()).to.not.equal(config2.mint.toString());
      console.log("✅ Multiple configs created successfully");
    });*/
  });

  /*describe("❌ Failure Cases", () => {
    it("Should fail if AirdropMaster doesn't exist", async () => {
      const fakeMaster = Keypair.generate();
      const fakeConfigPda = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          authority.publicKey.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        program.programId
      );

      try {
        await program.methods
          .initAirdropConfig(
            AIRDROP_VERSION,
            PROTOCOL_FEE_BPS,
            MASTER_FEE_BPS
          )
          .accounts({
            authority: authority.publicKey,
            airdropMaster: fakeMaster.publicKey, // Invalid master
            airdropConfig: fakeConfigPda[0],
            mint: mint.publicKey,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
          })
          .signers([authority])
          .rpc();
        
        assert.fail("Should have failed but succeeded");
      } catch (error) {
        expect(error.message).to.include("AccountNotInitialized");
        console.log("✅ Correctly failed: Master account not initialized");
      }
    });

    it("Should fail if config already exists", async () => {
      try {
        // Try to initialize the same config again
        await program.methods
          .initAirdropConfig(
            AIRDROP_VERSION,
            PROTOCOL_FEE_BPS,
            MASTER_FEE_BPS
          )
          .accounts({
            authority: authority.publicKey,
            airdropMaster: airdropMasterPda,
            airdropConfig: airdropConfigPda, // Already used
            mint: mint.publicKey,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
          })
          .signers([authority])
          .rpc();
        
        assert.fail("Should have failed but succeeded");
      } catch (error) {
        expect(error.message).to.include("already in use");
        console.log("✅ Correctly failed: Config already exists");
      }
    });

    it("Should fail if protocol_fee_bps + master_fee_bps > 10000 (100%)", async () => {
      const invalidProtocolFee = 6000; // 60%
      const invalidMasterFee = 5000; // 50% (total 110%)

      const newMint = Keypair.generate();
      const [newConfigPda] = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          authority.publicKey.toBuffer(),
          newMint.publicKey.toBuffer(),
        ],
        program.programId
      );

      try {
        await program.methods
          .initAirdropConfig(
            AIRDROP_VERSION,
            invalidProtocolFee,
            invalidMasterFee
          )
          .accounts({
            authority: authority.publicKey,
            airdropMaster: airdropMasterPda,
            airdropConfig: newConfigPda,
            mint: newMint.publicKey,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
          })
          .signers([authority])
          .rpc();
        
        assert.fail("Should have failed but succeeded");
      } catch (error) {
        expect(error.message).to.include("fees");
        expect(error.message).to.include("100%");
        console.log("✅ Correctly failed: Fees exceed 100%");
      }
    });

    it("Should fail if protocol_fee_bps is negative (should be u16)", async () => {
      // This is a type safety test - Anchor's IDL will enforce u16
      // We can't pass negative numbers in TypeScript, but we can test overflow
      
      const overflowFee = 65535; // Max u16
      const smallFee = 100;

      const newMint = Keypair.generate();
      const [newConfigPda] = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          authority.publicKey.toBuffer(),
          newMint.publicKey.toBuffer(),
        ],
        program.programId
      );

      // This should pass (u16 max is allowed), but logic may reject it
      // The contract should validate this
      try {
        await program.methods
          .initAirdropConfig(
            AIRDROP_VERSION,
            overflowFee,
            smallFee
          )
          .accounts({
            authority: authority.publicKey,
            airdropMaster: airdropMasterPda,
            airdropConfig: newConfigPda,
            mint: newMint.publicKey,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
          })
          .signers([authority])
          .rpc();
        
        // If it succeeded, we should check that the contract validated the fee
        const config = await program.account.airdropConfig.fetch(newConfigPda);
        expect(config.protocolFeeBps).to.be.below(10000);
        console.log("✅ Contract correctly validated fee bounds");
      } catch (error) {
        // Even better - contract rejected it
        expect(error.message).to.include("fee");
        console.log("✅ Correctly failed: Fee validation in contract");
      }
    });

    it("Should fail if wrong authority tries to initialize", async () => {
      const wrongAuthority = Keypair.generate();
      const newMint = Keypair.generate();
      
      // Airdrop some SOL to wrong authority
      await provider.connection.requestAirdrop(
        wrongAuthority.publicKey,
        2 * LAMPORTS_PER_SOL
      );

      const [newConfigPda] = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          wrongAuthority.publicKey.toBuffer(),
          newMint.publicKey.toBuffer(),
        ],
        program.programId
      );

      try {
        await program.methods
          .initAirdropConfig(
            AIRDROP_VERSION,
            PROTOCOL_FEE_BPS,
            MASTER_FEE_BPS
          )
          .accounts({
            authority: wrongAuthority.publicKey,
            airdropMaster: airdropMasterPda,
            airdropConfig: newConfigPda,
            mint: newMint.publicKey,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
          })
          .signers([wrongAuthority])
          .rpc();
        
        assert.fail("Should have failed but succeeded");
      } catch (error) {
        expect(error.message).to.include("ConstraintSeeds");
        console.log("✅ Correctly failed: Wrong authority");
      }
    });

    it("Should fail if mint is not a valid token mint", async () => {
      const invalidMint = Keypair.generate(); // Not a real token mint

      const [newConfigPda] = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          authority.publicKey.toBuffer(),
          invalidMint.publicKey.toBuffer(),
        ],
        program.programId
      );

      try {
        await program.methods
          .initAirdropConfig(
            AIRDROP_VERSION,
            PROTOCOL_FEE_BPS,
            MASTER_FEE_BPS
          )
          .accounts({
            authority: authority.publicKey,
            airdropMaster: airdropMasterPda,
            airdropConfig: newConfigPda,
            mint: invalidMint.publicKey,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
          })
          .signers([authority])
          .rpc();
        
        assert.fail("Should have failed but succeeded");
      } catch (error) {
        // This will fail because the mint account doesn't have the correct data
        expect(error.message).to.include("Account");
        console.log("✅ Correctly failed: Invalid mint account");
      }
    });
  });*/

  /*describe("🔍 Integration Tests", () => {
    it("Should maintain correct PDA derivation", async () => {
      // Test that the PDA is derived correctly
      const [derivedPda, derivedBump] = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          authority.publicKey.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        program.programId
      );

      expect(derivedPda.toString()).to.equal(airdropConfigPda.toString());
      console.log("✅ PDA derivation is correct");
    });

    it("Should store config data correctly", async () => {
      const configAccount = await program.account.airdropConfig.fetch(
        airdropConfigPda
      );

      // Verify all stored data
      expect(configAccount.version).to.equal(AIRDROP_VERSION);
      expect(configAccount.protocolFeeBps).to.equal(PROTOCOL_FEE_BPS);
      expect(configAccount.masterFeeBps).to.equal(MASTER_FEE_BPS);
      expect(configAccount.isPaused).to.be.false;
      expect(configAccount.totalAirdropCount).to.equal(0);
      expect(configAccount.totalClaimedAmount.toString()).to.equal("0");
      expect(configAccount.bump).to.equal(airdropConfigBump);

      console.log("✅ All config data stored correctly");
    });

    it("Should allow creating config after fee updates in master", async () => {
      // Update master fees first
      const newProtocolFee = 500; // 5%
      const newMasterFee = 300; // 3%
      
      // Assuming you have an update_fee method in your contract
      // If not, we skip this test
      
      const newMint = Keypair.generate();
      const [newConfigPda] = await PublicKey.findProgramAddress(
        [
          Buffer.from("config"),
          authority.publicKey.toBuffer(),
          newMint.publicKey.toBuffer(),
        ],
        program.programId
      );

      // Initialize with new fees
      await program.methods
        .initAirdropConfig(
          AIRDROP_VERSION,
          newProtocolFee,
          newMasterFee
        )
        .accounts({
          authority: authority.publicKey,
          airdropMaster: airdropMasterPda,
          airdropConfig: newConfigPda,
          mint: newMint.publicKey,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([authority])
        .rpc();

      const config = await program.account.airdropConfig.fetch(newConfigPda);
      expect(config.protocolFeeBps).to.equal(newProtocolFee);
      expect(config.masterFeeBps).to.equal(newMasterFee);
      
      console.log("✅ Config created with updated fees");
    });
  });*/
});