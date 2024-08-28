
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.InsuranceProtocol as anchor.Program<InsuranceProtocol>;
  
import type { InsuranceProtocol } from "../target/types/insurance_protocol";
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use solana_program_test::*;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use insurance_protocol::*;  
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::transport::TransportError;
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::Clock;
use std::str::FromStr;

#[tokio::test]
async fn test_initialize_pool() -> Result<(), TransportError> {
    let program = program_test();  // Initialize a program_test context
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    // Define test accounts
    let pool_key = Pubkey::new_unique();
    let admin_key = payer.pubkey();

    // Build the initialize_pool instruction
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: insurance_protocol::ID,
            accounts: insurance_protocol::accounts::InitializePool {
                insurance_pool: pool_key,
                admin: admin_key,
                system_program: system_program::ID,
            }.to_account_metas(None),
            data: insurance_protocol::instruction::InitializePool {
                bump: 1,
            }.data(),
        }],
        Some(&payer.pubkey()),
    );

    // Sign and send the transaction
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    // Confirm that the insurance pool is initialized properly
    // Fetch account to ensure it was created
    let insurance_pool_account = banks_client.get_account(pool_key).await.unwrap().unwrap();
    assert_eq!(insurance_pool_account.data.len(), InsurancePool::LEN);  // Check the pool size
    Ok(())
}

#[tokio::test]
async fn test_purchase_insurance() -> Result<(), TransportError> {
    let program = program_test();
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let user_key = payer.pubkey();
    let pool_key = Pubkey::new_unique();
    let policy_key = Pubkey::new_unique();

    // Build the purchase_insurance instruction
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: insurance_protocol::ID,
            accounts: insurance_protocol::accounts::PurchaseInsurance {
                user: user_key,
                insurance_policy: policy_key,
                insurance_pool: pool_key,
                system_program: system_program::ID,
            }.to_account_metas(None),
            data: insurance_protocol::instruction::PurchaseInsurance {
                deposit_amount: 1000,
                premium_amount: 100,
                coverage_amount: 5000,
            }.data(),
        }],
        Some(&payer.pubkey()),
    );

    // Sign and send the transaction
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    // Fetch and verify the insurance policy
    let insurance_policy_account = banks_client.get_account(policy_key).await.unwrap().unwrap();
    assert_eq!(insurance_policy_account.data.len(), InsurancePolicy::LEN);
    Ok(())
}

#[tokio::test]
async fn test_cancel_policy() -> Result<(), TransportError> {
    let program = program_test();
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let user_key = payer.pubkey();
    let pool_key = Pubkey::new_unique();
    let policy_key = Pubkey::new_unique();

    // Build the cancel_policy instruction
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: insurance_protocol::ID,
            accounts: insurance_protocol::accounts::CancelPolicy {
                user: user_key,
                insurance_policy: policy_key,
                insurance_pool: pool_key,
                system_program: system_program::ID,
            }.to_account_metas(None),
            data: insurance_protocol::instruction::CancelPolicy {}.data(),
        }],
        Some(&payer.pubkey()),
    );

    // Sign and send the transaction
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    // Verify the policy is no longer active
    let insurance_policy_account = banks_client.get_account(policy_key).await.unwrap().unwrap();
    let insurance_policy = InsurancePolicy::try_from_slice(&insurance_policy_account.data).unwrap();
    assert_eq!(insurance_policy.is_active, false);  // Policy should be marked inactive

    Ok(())
}

#[tokio::test]
async fn test_adjust_coverage() -> Result<(), TransportError> {
    let program = program_test();
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let user_key = payer.pubkey();
    let pool_key = Pubkey::new_unique();
    let policy_key = Pubkey::new_unique();

    // Build the adjust_coverage instruction
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: insurance_protocol::ID,
            accounts: insurance_protocol::accounts::AdjustCoverage {
                user: user_key,
                insurance_policy: policy_key,
            }.to_account_metas(None),
            data: insurance_protocol::instruction::AdjustCoverage {
                new_coverage_amount: 10000,
            }.data(),
        }],
        Some(&payer.pubkey()),
    );

    // Sign and send the transaction
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    // Fetch and verify the insurance policy's updated coverage
    let insurance_policy_account = banks_client.get_account(policy_key).await.unwrap().unwrap();
    let insurance_policy = InsurancePolicy::try_from_slice(&insurance_policy_account.data).unwrap();
    assert_eq!(insurance_policy.coverage_amount, 10000);  // Verify updated coverage

    Ok(())
}

#[tokio::test]
async fn test_stake_into_pool() -> Result<(), TransportError> {
    let program = program_test();
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let user_key = payer.pubkey();
    let pool_key = Pubkey::new_unique();
    let user_token_account_key = Pubkey::new_unique();
    let pool_token_account_key = Pubkey::new_unique();

    // Build the stake_into_pool instruction
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: insurance_protocol::ID,
            accounts: insurance_protocol::accounts::StakeIntoPool {
                user: user_key,
                user_token_account: user_token_account_key,
                pool_token_account: pool_token_account_key,
                insurance_pool: pool_key,
                token_program: token::ID,
            }.to_account_metas(None),
            data: insurance_protocol::instruction::StakeIntoPool {
                amount: 500,
            }.data(),
        }],
        Some(&payer.pubkey()),
    );

    // Sign and send the transaction
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    // Fetch the pool and verify the staked amount
    let insurance_pool_account = banks_client.get_account(pool_key).await.unwrap().unwrap();
    let insurance_pool = InsurancePool::try_from_slice(&insurance_pool_account.data).unwrap();
    assert_eq!(insurance_pool.total_premium_collected, 500);  // Verify the staked amount

    Ok(())
}

#[tokio::test]
async fn test_governance_voting() -> Result<(), TransportError> {
    let program = program_test();
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let user_key = payer.pubkey();
    let governance_key = Pubkey::new_unique();
    let vote_record_key = Pubkey::new_unique();

    // Build the submit_governance_vote instruction
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: insurance_protocol::ID,
            accounts: insurance_protocol::accounts::SubmitVote {
                governance: governance_key,
                vote_record: vote_record_key,
                user: user_key,
                system_program: system_program::ID,
            }.to_account_metas(None),
            data: insurance_protocol::instruction::SubmitVote {
                proposal_id: 1,
                vote: true,  // Voting 'Yes'
            }.data(),
        }],
        Some(&payer.pubkey()),
    );

    // Sign and send the transaction
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    // Fetch and verify the vote record
    let vote_record_account = banks_client.get_account(vote_record_key).await.unwrap().unwrap();
    let vote_record = VoteRecord::try_from_slice(&vote_record_account.data).unwrap();
    assert_eq!(vote_record.vote, true);  // Verify the vote was 'Yes'

    Ok(())
}
