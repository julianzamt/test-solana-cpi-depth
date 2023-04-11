import * as web3 from '@solana/web3.js';
import { programId, SIGNER } from './constants';
import { packUInt8 } from './utils';

export const callProgram0 = async (
  connection: web3.Connection,
  program0Id: web3.PublicKey,
  num: number,
  signer: web3.Keypair = SIGNER
) => {
  let dataBuffer = Buffer.from("");

  const addProgramId = new web3.PublicKey("EQPKvHz9obWzVNVhg58rt4AY5545hkGzGbBdJXYCKfgF")
  const program3Id = new web3.PublicKey("9wSm68hecoru1grewLFPGTzvVKZ5vdMAF9bXwrUPBftp")
  const program2Id = new web3.PublicKey("HoHHtNBevaqhyFuPx9z3vzWFRx66Y94mn22BPK9h24wd")
  const program1Id = new web3.PublicKey("Fe9DGPwqKFxoamYoAEsCRiAg9Aa3W32dVPC55P4mUnga")

  dataBuffer = packUInt8(dataBuffer, num);

  let [adderAddress] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from('adder')],
    addProgramId
  );

  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: program0Id, isSigner: false, isWritable: false },
      { pubkey: program1Id, isSigner: false, isWritable: false },
      { pubkey: program2Id, isSigner: false, isWritable: false },
      { pubkey: program3Id, isSigner: false, isWritable: false },
      { pubkey: addProgramId, isSigner: false, isWritable: false },
      { pubkey: adderAddress, isSigner: false, isWritable: true },
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