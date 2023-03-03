import * as web3 from '@solana/web3.js';
import { programId, SIGNER } from './constants';

export const sayHi = async (
  connection: web3.Connection,
  programBPubkey: web3.PublicKey,
  signer: web3.Keypair = SIGNER
) => {
  let dataBuffer = Buffer.from('');

  let [accumulatorAddress] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from('accumulator')],
    programBPubkey
  );

  console.log('PDA: ', accumulatorAddress.toString());

  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: programBPubkey, isSigner: false, isWritable: false },
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

export const packUInt32 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 4);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeUInt32LE(data, buf.length);
  return newBuffer;
};
