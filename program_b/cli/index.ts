import * as web3 from "@solana/web3.js";

import { getAccumulator, add } from "./functions";
import { Accumulator } from "./types";

const connection = new web3.Connection("http://127.0.0.1:8899");

async function main() {
  // write your code here
  await add(connection);
  console.log(await getAccumulator(connection))

  await add(connection);
  console.log(await getAccumulator(connection))
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
