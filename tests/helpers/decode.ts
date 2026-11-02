import { address, Address, BaseAccount, ReadonlyUint8Array } from "@solana/kit";
import {
  AIRDROP_CONFIG_DISCRIMINATOR,
  AIRDROP_DISCRIMINATOR,
  AIRDROP_MASTER_DISCRIMINATOR,
  BITMAP_ACCOUNT_DISCRIMINATOR,
  DROPSY_AIRDROP_PROGRAM_ADDRESS,
  getAirdropCodec,
  getAirdropConfigCodec,
  getAirdropMasterCodec,
  getBitmapAccountCodec,
} from "../../clients/js/src/generated";
import {
  toUiAirdrop,
  toUiAirdropMaster,
  toUiBitmapAccount,
} from "../utils/helper";
import { toUiAirdropConfig } from "../types/airdrop-config.js";

export function discriminatorEquals(
  a: ReadonlyUint8Array,
  b: ReadonlyUint8Array,
) {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return false;
  return true;
}

export type AccountInfo = BaseAccount & {
  readonly address: Address<Address<string>>;
  readonly data: Uint8Array<ArrayBufferLike>;
} & {
  readonly exists: true;
};

export const ACCOUNT_TYPES = [
  "Airdrop",
  "Airdrop Master",
  "Airdrop Config",
  "ClaimMap",
  "Affiliate",
  "Wallet",
  "Program",
  "Dropsy Program",
  "Token",
  "Unknown",
] as const;

export type AccountType = (typeof ACCOUNT_TYPES)[number];

export interface DecodedAccount {
  type: AccountType;
  data: string;
  address: string;
  rawData?: Uint8Array;
  discriminator?: Uint8Array;
  program: string;
}

export interface DecoderResult {
  type: AccountType;
  data: any;
  rawData: Uint8Array;
  discriminator: Uint8Array;
}

/**
 * Detects the account type based on discriminator and account properties
 */
export function detectAccountType(accountInfo: AccountInfo): AccountType {
  // Check if it's executable (program)
  if (accountInfo.executable) {
    // You can add special program detection here
    // if (address === DROPSY_PROGRAM_ADDRESS) return "Dropsy Program";
    return "Program";
  }
  if (accountInfo.programAddress === "11111111111111111111111111111111") {
    return "Wallet";
  }
  if (
    accountInfo.programAddress === "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
  ) {
    return "Token";
  }
  if (
    accountInfo.programAddress === "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
  ) {
    return "Token";
  }

  const discriminator = accountInfo.data.slice(0, 8);
  // Check Dropsy discriminators
  if (discriminatorEquals(discriminator, AIRDROP_DISCRIMINATOR)) {
    return "Airdrop";
  } else if (discriminatorEquals(discriminator, BITMAP_ACCOUNT_DISCRIMINATOR)) {
    return "ClaimMap";
  } else if (discriminatorEquals(discriminator, AIRDROP_MASTER_DISCRIMINATOR)) {
    return "Airdrop Master";
  } else if (discriminatorEquals(discriminator, AIRDROP_CONFIG_DISCRIMINATOR)) {
    return "Airdrop Config";
  }

  // Default to Unknown if not a program and no Dropsy discriminator
  return "Unknown";
}

/**
 * Decodes account data based on its type
 */
export function decodeAccountData(
  type: AccountType,
  rawData: ReadonlyUint8Array,
  pubkey: string,
): any {
  try {
    switch (type) {
      case "Airdrop":
        const airdropCodec = getAirdropCodec();
        return toUiAirdrop(airdropCodec.decode(rawData), address(pubkey));

      case "Airdrop Config":
        const airdropConfigCodec = getAirdropConfigCodec();
        return toUiAirdropConfig(
          airdropConfigCodec.decode(rawData),
          address(pubkey),
        );

      case "Airdrop Master":
        const airdropMasterCodec = getAirdropMasterCodec();
        return toUiAirdropMaster(
          airdropMasterCodec.decode(rawData),
          address(pubkey),
        );

      case "ClaimMap":
        const claimMapCodec = getBitmapAccountCodec();
        return toUiBitmapAccount(
          claimMapCodec.decode(rawData),
          address(pubkey),
        );

      case "Wallet":
      case "Program":
      case "Dropsy Program":
      case "Unknown":
      default:
        // Return raw data or null for non-Dropsy accounts
        return null;
    }
  } catch (error) {
    console.error(`Failed to decode ${type} account:`, error);
    throw new Error(`Failed to decode ${type} account data`);
  }
}

/**
 * Main function to decode any Dropsy account
 */
export function decodeDropsyAccount(
  accountInfo: AccountInfo,
  address: string,
): DecodedAccount {
  if (!accountInfo.exists) {
    throw new Error("Account does not exist");
  }

  const rawData = accountInfo.data;
  const program = accountInfo.programAddress;

  // Detect account type
  const type = detectAccountType(accountInfo);

  // Decode data based on type
  let decodedData: any = null;
  let decodingError: string | null = null;

  try {
    decodedData = decodeAccountData(type, rawData, address);
  } catch (error: any) {
    decodingError = error.message;
    console.warn(`Could not decode ${type} account:`, error);
  }

  return {
    type,
    data: decodedData,
    address,
    program,
  };
}

/**
 * Extracts just the discriminator for quick type detection
 */
export function getAccountDiscriminator(
  accountInfo: AccountInfo,
): Uint8Array | null {
  if (!accountInfo.exists) return null;
  return accountInfo.data.slice(0, 8);
}

/**
 * Quick type detection without full decoding
 */
export function getAccountTypeQuick(accountInfo: AccountInfo): AccountType {
  if (!accountInfo.exists) {
    throw new Error("Account does not exist");
  }

  return detectAccountType(accountInfo);
}

/*export function decodeDropsyTransaction(transaction: any): any {
  if (!transaction) {
    throw new Error("Transaction does not exist");
  }

  const message = transaction.transaction.message;

  const signer =
    message.accountKeys.find((account) => account.signer)?.pubkey ?? "";

  const dropsyInstructions = message.instructions.filter(
    (instruction) => instruction.programId === DROPSY_AIRDROP_PROGRAM_ADDRESS,
  );

  const decodedInstructions = dropsyInstructions.map((instruction) =>
    decodeDropsyInstruction(instruction),
  );

  return {
    signature: transaction.signature,
    signer,
    type: decodedInstructions[0]?.instructionType ?? "unknown",
    timestamp: transaction.meta?.createdAt ?? null,
  };
}*/
