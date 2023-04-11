import * as web3 from "@solana/web3.js";
import { getAdder, add } from "./functions";


const connection = new web3.Connection("http://127.0.0.1:8899");

async function main() {
  // write your code here
  await add(1, connection);
  console.log(await getAdder(connection))
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
