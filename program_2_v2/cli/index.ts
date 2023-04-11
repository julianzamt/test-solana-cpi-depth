import * as web3 from '@solana/web3.js';

import { callProgram3 } from './functions';
import { unpackUInt8 } from './utils';
const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  // write your code here
  const program3Id = new web3.PublicKey("F1TLnYDDwhH4EzGiGRQua1Faq67XXFN1HyTtrvZfhC4E")
  const counterProgramId = new web3.PublicKey("3DqyLaKd4fodLFarh7N53HNYyYNmpuZZk614P7mWs9d8")

  await callProgram3(
    program3Id,
    counterProgramId,
    connection,
  );

  // Print Counter.number in console
  let counterInfo = await connection.getAccountInfo(
    new web3.PublicKey('84DqB1CwaV7C8RsQBTQmtiHe4tn4dPd6GcDgkpLeet4S'),
    'processed'
  );

  let data = counterInfo ? counterInfo.data : null;
  if (!data) {
    throw new Error('No data retrieved');
  }
 
  const [num, _] = unpackUInt8(data);

  console.log('Counter number is: ', num);

}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
