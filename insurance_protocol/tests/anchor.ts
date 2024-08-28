import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import type { InsuranceProtocol } from "../target/types/insurance_protocol";

describe("Insurance Protocol Tests", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.InsuranceProtocol as anchor.Program<InsuranceProtocol>;
  
  
  // Test for initializing the insurance pool
  it("Initialize the Insurance Pool", async () => {
    const poolAccountKp = new web3.Keypair();
    const poolAuthority = await web3.PublicKey.findProgramAddress(
      [Buffer.from("insurance_pool")],
      program.programId
    );
    
    const txHash = await program.methods
      .initializePool(0) // bump seed
      .accounts({
        insurancePool: poolAccountKp.publicKey,
        poolAuthority: poolAuthority[0],
        admin: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([poolAccountKp])
      .rpc();

    console.log(`Insurance Pool initialized. Tx: ${txHash}`);
    await program.provider.connection.confirmTransaction(txHash);
  });

  // Test for purchasing insurance
  it("Purchase Insurance", async () => {
    const policyAccountKp = new web3.Keypair();
    const insurancePoolKey = new web3.PublicKey("<your-insurance-pool-address>");
    
    const txHash = await program.methods
      .purchaseInsurance(new BN(1000), new BN(100), new BN(5000)) // deposit, premium, coverage
      .accounts({
        user: program.provider.publicKey,
        insurancePolicy: policyAccountKp.publicKey,
        insurancePool: insurancePoolKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([policyAccountKp])
      .rpc();

    console.log(`Insurance purchased. Tx: ${txHash}`);
    await program.provider.connection.confirmTransaction(txHash);
  });

  // Test for canceling a policy
  it("Cancel Insurance Policy", async () => {
    const insurancePoolKey = new web3.PublicKey("<your-insurance-pool-address>");
    const policyAccountKey = new web3.PublicKey("<your-policy-account>");

    const txHash = await program.methods
      .cancelPolicy()
      .accounts({
        insurancePolicy: policyAccountKey,
        insurancePool: insurancePoolKey,
        user: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Policy canceled. Tx: ${txHash}`);
    await program.provider.connection.confirmTransaction(txHash);
  });

  // Test for approving a claim
  it("Approve Claim", async () => {
    const insurancePoolKey = new web3.PublicKey("<your-insurance-pool-address>");
    const policyAccountKey = new web3.PublicKey("<your-policy-account>");

    const txHash = await program.methods
      .approveClaim()
      .accounts({
        insurancePolicy: policyAccountKey,
        insurancePool: insurancePoolKey,
        admin: program.provider.publicKey, // Admin account must approve
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Claim approved. Tx: ${txHash}`);
    await program.provider.connection.confirmTransaction(txHash);
  });
});
