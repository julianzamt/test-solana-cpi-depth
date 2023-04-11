import * as web3 from '@solana/web3.js';
import { programId, SIGNER } from './constants';
import { packUInt8 } from './utils';

export const callCounter = async (
  counterProgramId: web3.PublicKey,
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let dataBuffer = Buffer.from("");

  // Ix Number
  dataBuffer = packUInt8(dataBuffer, 0);

  let [counterAddress] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from('counter')],
    counterProgramId
  );

  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: counterProgramId, isSigner: false, isWritable: false },
      { pubkey: counterAddress, isSigner: false, isWritable: true },
      { pubkey: signer.publicKey, isSigner: true, isWritable: true },
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