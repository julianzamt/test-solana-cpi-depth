import * as web3 from '@solana/web3.js';
import { programBId } from './constants';

import { getAccumulator, sayHi } from './functions';
const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  // write your code here
  await sayHi(
    connection,
    //new web3.PublicKey('8ooGcuGYUXHejEovJPE8Hkbzr7EUbbXsakmfaVSs8rjE')
    programBId
  );

  console.log(await getAccumulator(connection));
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
