import * as web3 from '@solana/web3.js';

import { sayHi } from './functions';
const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  // write your code here
  await sayHi(
    connection,
    new web3.PublicKey('7cKiYqQhh12atTVhsZqKy4E12bN3kFbDdHqPo4FSuYwe')
  );
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
