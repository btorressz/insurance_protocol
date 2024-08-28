import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from '@project-serum/anchor';
import {Provider, web3 } from '@project-serum/anchor';
import type { InsuranceProtocol } from "../target/types/insurance_protocol";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.InsuranceProtocol as anchor.Program<InsuranceProtocol>;



// Initialize the Anchor provider environment
//const provider = anchor.AnchorProvider.env();
//anchor.setProvider(provider);

const provider = Provider.env();  // Use `Provider` if Solana Playground is using an older version
anchor.setProvider(provider);


// IDL and program ID
const idl = await anchor.Program.fetchIdl("<your-program-id>", provider);
const program = new anchor.Program(idl, "<your-program-id>", provider);

async function main() {
    try {
        // Get balance of the wallet
        const balance = await provider.connection.getBalance(provider.wallet.publicKey);
        console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

        // Fetch the insurance pool account balance
        const insurancePoolPubkey = new web3.PublicKey("<your-insurance-pool-address>");
        const insurancePoolBalance = await provider.connection.getBalance(insurancePoolPubkey);
        console.log(`Insurance Pool balance: ${insurancePoolBalance / web3.LAMPORTS_PER_SOL} SOL`);

        // Fetch the insurance policy account data
        const policyPubkey = new web3.PublicKey("<your-policy-account>");
        const policyAccount = await program.account.insurancePolicy.fetch(policyPubkey);

        console.log("Insurance Policy data:");
        console.log(`User: ${policyAccount.user.toString()}`);
        console.log(`Premium Amount: ${policyAccount.premiumAmount}`);
        console.log(`Coverage Amount: ${policyAccount.coverageAmount}`);
        console.log(`Active: ${policyAccount.isActive}`);

    } catch (error) {
        console.error("Error occurred:", error);
    }
}

// Execute the function
main();

/*
console.log("My address:", program.provider.publicKey.toString());
const balance = await program.provider.connection.getBalance(program.provider.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

// Fetch the insurance pool account balance (replace with actual pool account)
const insurancePoolPubkey = new web3.PublicKey("<your-insurance-pool-address>");
const insurancePoolBalance = await program.provider.connection.getBalance(insurancePoolPubkey);
console.log(`Insurance Pool balance: ${insurancePoolBalance / web3.LAMPORTS_PER_SOL} SOL`);

// Fetch and display policy data (replace with actual policy account)
const policyPubkey = new web3.PublicKey("<your-policy-account>");
const policyAccount = await program.account.insurancePolicy.fetch(policyPubkey);

console.log("Insurance Policy data:");
console.log(`User: ${policyAccount.user.toString()}`);
console.log(`Premium Amount: ${policyAccount.premiumAmount}`);
console.log(`Coverage Amount: ${policyAccount.coverageAmount}`);
console.log(`Active: ${policyAccount.isActive}`);*/