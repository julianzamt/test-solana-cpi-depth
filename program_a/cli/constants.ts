import * as web3 from "@solana/web3.js";

export const SECRET = [76,162,125,143,185,221,94,126,13,154,196,145,9,73,222,135,134,0,170,173,72,241,212,229,59,123,37,51,50,136,194,123,225,148,61,186,204,104,23,15,231,14,249,70,177,248,237,61,37,218,7,154,240,34,142,197,227,252,100,235,130,240,246,213]; // your local private test-key. Try: $cat ~/.config/solana/id.json
export const PROGRAM_ID = "2hdfj3sbhBK2qSYFiy7gNXLufcDiGD2ZNaNUtYEEXN9e"; // Whatever returned from solana deploy
export const PROGRAM_B_ID = "8ooGcuGYUXHejEovJPE8Hkbzr7EUbbXsakmfaVSs8rjE";
export const KEY: Uint8Array = Uint8Array.from(SECRET);
export const programId = new web3.PublicKey(PROGRAM_ID);
export const programBId = new web3.PublicKey(PROGRAM_B_ID);
export const SIGNER: web3.Keypair = web3.Keypair.fromSecretKey(KEY);
