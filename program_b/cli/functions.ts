import * as web3 from "@solana/web3.js";
import { Buffer } from "buffer";
import { programId, SIGNER } from "./constants";
import * as utils from "./utils";
import { Accumulator } from "./types";

export const add = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let instructionNumber = 0;

  let dataBuffer = Buffer.from("");

  dataBuffer = utils.packUInt8(dataBuffer, instructionNumber);

  let [accumulatorAddress, _accumulatorBump] =
    web3.PublicKey.findProgramAddressSync(
      [Buffer.from("accumulator")],
      programId
    );

  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: accumulatorAddress, isSigner: false, isWritable: true },
      { pubkey: signer.publicKey, isSigner: true, isWritable: false },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: true,
      },
    ],
    data: dataBuffer,
  });

  let txReceipt = await web3.sendAndConfirmTransaction(
    connection,
    new web3.Transaction().add(instruction),
    [signer]
  );
  return txReceipt;
};

/******* GETTERS ********/

export const getAccumulator = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let [AccumulatorAddress, _AccumulatorBump] =
    web3.PublicKey.findProgramAddressSync(
      [Buffer.from("accumulator")],
      programId
    );
  let AccumulatorInfo = await connection.getAccountInfo(
    AccumulatorAddress,
    "processed"
  );
  let data = AccumulatorInfo ? AccumulatorInfo.data : null;
  if (!data) {
    throw new Error("No data retrieved");
  }
  let AccumulatorStruct = Accumulator.decode(data);
  return AccumulatorStruct;
};
