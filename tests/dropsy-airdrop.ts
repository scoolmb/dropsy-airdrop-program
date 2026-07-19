/*import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DropsyAirdrop } from "../target/types/dropsy_airdrop";

describe("dropsy-airdrop", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.dropsyAirdrop as Program<DropsyAirdrop>;

  it("Is initialized airdrop config and master !", async () => {
    // Add your test here.
    const tx = await program.methods.createAirdrop().accounts({
      airdropMaster: anchor.web3.Keypair.generate().publicKey,
      authority: anchor.web3.Keypair.generate().publicKey,
      treasury: anchor.web3.Keypair.generate().publicKey, 
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});*/
