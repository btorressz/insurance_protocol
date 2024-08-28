# insurance_protocol

This project implements a decentralized insurance protocol on the Solana blockchain using Anchor. It allows users to purchase insurance policies, pay premiums with tokens, stake into the insurance pool, cancel policies, and submit claims. Administrators can approve claims, withdraw premiums, and manage the protocol through governance votes.
This Project was created using Solana Playground IDE(https://beta.solpg.io/)

## Key Features

- **Insurance Pool Management: Initialize an insurance pool to collect premiums and manage payouts.**

- **Policy Purchase: Users can purchase insurance policies with defined coverage and premium amounts.**

- **Claims and Payouts: Users can submit claims for approval, and administrators can approve or reject these claims.**

- **Token-based Payments: Premiums and staking into the insurance pool are handled through SPL token transfers.**

- **Governance Voting: Allows voting on protocol upgrades and changes.**

  ## Smart Contract Architecture
- **lib.rs: Contains the core logic for insurance policy management, premium collection, claims processing, and staking into the pool.**
- **anchor.tests.ts: TypeScript-based test suite for running tests on Solana Playground, validating the functionality of the insurance protocol.**
- **client.ts: TypeScript code that interfaces with the program on Solana Playground or Phantom Wallet.**
- **tests.rs: Rust-based test suite designed to rigorously validate program logic and ensure the accuracy of each core function. (Currently under review; anchor.tests.ts is being used for testing at this stage).**

