import {
  type Address,
  type Client,
  type Instruction,
  type Rpc,
  sendTransactionWithoutConfirmingFactory,
  signTransactionMessageWithSigners,
  TransactionSigner,
} from "@solana/kit";
import { createTransactionMessageFromInstructions } from "./client";
import { sendTransactionOrExpectThrowError } from "./svm";
import { DropsyClient, getClient } from "./common";

export const sendInstructions = async ({
  signer,
  instructions,
  client,
}: {
  signer: TransactionSigner;
  instructions: Instruction[];
  client: DropsyClient;
}) => {
  const transactionMessage = await createTransactionMessageFromInstructions(
    client.rpc,
    signer,
    instructions,
  );

  const signedTransaction = await signTransactionMessageWithSigners(
    transactionMessage,
  );

  const result = sendTransactionOrExpectThrowError(
    client,
    signedTransaction,
    true,
  );

  return result;
};
