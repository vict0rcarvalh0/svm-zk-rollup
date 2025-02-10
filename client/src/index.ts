import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import fs from "fs";

const connection = new Connection("https://api.devnet.solana.com");

// Program key and state account
const programId = new PublicKey("YourProgramID");
const stateAccount = new PublicKey("StateAccountPublicKey");

// Simulating a SP1 Proof
const proof = fs.readFileSync("proof.bin");

async function submitState() {
    const transaction = new Transaction();
    // TODO: Add instruction to submit proof
    console.log("Enviando prova...");
}
submitState();
