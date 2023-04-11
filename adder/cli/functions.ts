import * as web3 from "@solana/web3.js";
import { Buffer } from "buffer";
import { programId, SIGNER } from "./constants";
import * as utils from "./utils";
import { Adder } from "./types";

export const add = async (
  num: number, 
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let instructionNumber = 0;

  let dataBuffer = Buffer.from("");

  dataBuffer = utils.packUInt8(dataBuffer, instructionNumber);
  dataBuffer = utils.packUInt8(dataBuffer, num);

  let [adderAddress, _adderBump] =
    web3.PublicKey.findProgramAddressSync(
      [Buffer.from("adder")],
      programId
    );

  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: adderAddress, isSigner: false, isWritable: true },
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

export const getAdder = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let [adderAddress, _adderBump] =
    web3.PublicKey.findProgramAddressSync(
      [Buffer.from("adder")],
      programId
    );

  console.log("adder ADDRESS: ", adderAddress.toString());

  let adderInfo = await connection.getAccountInfo(
    adderAddress,
    "processed"
  );

  
  
  let data = adderInfo ? adderInfo.data : null;
  if (!data) {
    throw new Error("No data retrieved");
  }

  let adderStruct = Adder.decode(data);
  return adderStruct;
};
