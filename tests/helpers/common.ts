import {
  address,
  createClient,
  createKeyPairSignerFromBytes,
  generateKeyPairSigner,
  lamports,
} from "@solana/kit";
import { litesvm } from "@solana/kit-plugin-litesvm";
import { signer } from "@solana/kit-plugin-signer";
import { tokenProgram } from "@solana-program/token";
import { systemProgram } from "@solana-program/system";
import {
  findAssociatedTokenPda,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import * as fs from "node:fs";
import path from "node:path";

export async function getClient() {
  // const payer = //await generateKeyPairSigner();
  const payer = await createKeyPairSignerFromBytes(
    new Uint8Array(
      // prettier-ignore
      [124, 7, 78, 179, 222, 186, 151, 219, 91, 182, 224, 162, 163, 191, 157, 121, 36, 144, 24, 9, 158, 207, 90, 170, 236, 125, 104, 120, 52, 111, 154, 105, 69, 119, 213, 71, 205, 197, 107, 146, 92, 13, 133, 155, 225, 110, 27, 133, 147, 177, 174, 34, 119, 143, 219, 80, 194, 190, 126, 212, 146, 248, 221, 72],
    ),
  );
  const client = createClient()
    .use(signer(payer))
    .use(litesvm())
    .use(tokenProgram())
    .use(systemProgram());

  // Fund payer with 5 SOL
  client.svm.airdrop(payer.address, lamports(5_0000_000_000n));

  const programId = address("drophxkcW5TRfbMQgo45Yti3q8jVxuxvuNufMnu8WiE");
  const soPath = path.join(
    process.cwd(),
    "target",
    "deploy",
    "dropsy_airdrop.so",
  );

  if (!fs.existsSync(soPath)) {
    throw new Error(`Dropsy program not found: ${soPath}`);
  }
  if (fs.existsSync(soPath)) {
    client.svm.addProgramFromFile(programId, soPath);
    console.log("Program loaded:", programId);
    // Verify program account
    const programAccount = client.svm.getAccount(programId);
    if (programAccount.exists) {
      console.log("  Executable:", programAccount.executable);
      console.log("  Data size:", programAccount.data.length, "bytes");
    }
  } else {
    console.log("Program file not found:", soPath);
    console.log("Build with: cargo build-sbf");
  }

  return client;
}

export type DropsyClient = Awaited<ReturnType<typeof getClient>>;

export const createAndMintToken = async ({
  client,
  decimals = 9,
  amount = 1_000_000_000_000n,
}: {
  client: DropsyClient;
  decimals?: number;
  amount?: bigint;
}) => {
  const newMint = await generateKeyPairSigner();

  await client.token.instructions
    .createMint({
      newMint,
      decimals,
      mintAuthority: client.payer.address,
    })
    .sendTransaction();

  // Mint tokens to payer ATA
  await client.token.instructions
    .mintToATA({
      mint: newMint.address,
      owner: client.payer.address,
      mintAuthority: client.payer.address,
      amount,
      decimals,
    })
    .sendTransaction();

  // Find payer ATA
  const [payerAta] = await findAssociatedTokenPda({
    owner: client.payer.address,
    mint: newMint.address,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
  });

  // Fetch ATA account
  const payerAccount = await client.token.accounts.token.fetch(payerAta);
  const mintAccount = await client.token.accounts.mint.fetch(newMint.address);
  console.log(mintAccount.data.supply); // 0n
  console.log(mintAccount.data.decimals);

  console.log(payerAccount.data.amount); // 600_000_000_000n
  console.log(payerAccount.data.mint); // newMint.address
  console.log(payerAccount.data.owner);

  return {
    mint: newMint,
    mintAddress: newMint.address,
    payerAta,
    payerAccount,
    mintAccount,
  };
};

export const toMerkleRootBytes = (merkleRoot: string): ReadonlyUint8Array => {
  const hex = merkleRoot.replace(/^0x/i, "");

  if (!/^[0-9a-fA-F]{64}$/.test(hex)) {
    throw new Error("Invalid Merkle root: expected 32 bytes of hexadecimal");
  }

  return Uint8Array.from(
    hex.match(/.{2}/g)!.map((byte) => parseInt(byte, 16)),
  ) as ReadonlyUint8Array;
};
