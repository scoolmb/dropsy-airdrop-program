import { FullySignedTransaction } from "@solana/kit";
import {
  FailedTransactionMetadata,
  TransactionMetadata,
} from "@solana/kit-plugin-litesvm";
import { expect } from "chai";

export function sendTransactionOrExpectThrowError(
  client: any,
  signedTransaction: any,
  logging = false,
  errorCode?: number,
) {
  const result = client.svm.sendTransaction(signedTransaction);

  if ("err" in result && typeof result.err === "function") {
    console.error("❌ Transaction failed");

    const err = result.err();
    console.error(err.toString());

    console.log("Logs:", result.meta().logs());

    if (errorCode !== undefined) {
      expect(err.toString()).to.include(errorCode.toString());
    }

    return result;
  }

  console.log("✅ Transaction succeeded");
  console.log("Signature:", result.signature());
  console.log("Compute units:", result.computeUnitsConsumed());
  console.log("Logs:", result.logs());

  if (errorCode !== undefined) {
    throw new Error(
      `Expected transaction to fail with error ${errorCode}, but it succeeded.`,
    );
  }

  return result;
}

export function expectThrowsErrorCode(
  response: TransactionMetadata | FailedTransactionMetadata,
  errorCode: number,
) {
  if (response instanceof FailedTransactionMetadata) {
    const message = response.err().toString();

    if (!message.toString().includes(errorCode.toString())) {
      throw new Error(
        `Unexpected error: ${message}. Expected error: ${errorCode}`,
      );
    }

    return;
  } else {
    throw new Error("Expected an error but didn't get one");
  }
}
