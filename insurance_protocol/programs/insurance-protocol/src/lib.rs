use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};  // Fixed import

declare_id!("3bv8Hi7JYzuTdPJxMut67P7qRoZdJrJM33pwKZFp82tz");

#[program]
pub mod insurance_protocol {
    use super::*;

    // Initialize the insurance pool by admin
    pub fn initialize_pool(ctx: Context<InitializePool>, bump: u8) -> Result<()> {
        let pool = &mut ctx.accounts.insurance_pool;
        pool.total_premium_collected = 0;
        pool.total_claims_paid = 0;
        pool.authority = *ctx.accounts.pool_authority.key;
        Ok(())
    }

    // Purchase insurance
    pub fn purchase_insurance(
        ctx: Context<PurchaseInsurance>,
        deposit_amount: u64,
        premium_amount: u64,
        coverage_amount: u64
    ) -> Result<()> {
        let policy = &mut ctx.accounts.insurance_policy;
        policy.user = *ctx.accounts.user.key;
        policy.deposit_amount = deposit_amount;
        policy.premium_amount = premium_amount;
        policy.coverage_amount = coverage_amount;
        policy.start_time = Clock::get()?.unix_timestamp;
        policy.end_time = Clock::get()?.unix_timestamp + 30 * 24 * 60 * 60; // 30-day coverage
        policy.is_active = true;

        let insurance_pool = &mut ctx.accounts.insurance_pool;
        insurance_pool.total_premium_collected += premium_amount;

        Ok(())
    }

    // Cancel insurance and refund pro-rated premium
    pub fn cancel_policy(ctx: Context<CancelPolicy>) -> Result<()> {
        let policy = &mut ctx.accounts.insurance_policy;
        require!(policy.is_active, InsuranceError::PolicyNotActive);

        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time < policy.end_time, InsuranceError::PolicyExpired);

        let remaining_time = policy.end_time - current_time;
        let total_duration = policy.end_time - policy.start_time;

        // Pro-rated refund calculation
        let refund_amount = (policy.premium_amount as u128 * remaining_time as u128 / total_duration as u128) as u64;

        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += refund_amount;
        **ctx.accounts.insurance_pool.to_account_info().try_borrow_mut_lamports()? -= refund_amount;

        policy.is_active = false; // Mark the policy as canceled

        Ok(())
    }

    // Submit and approve claim by admin
    pub fn approve_claim(ctx: Context<ApproveClaim>) -> Result<()> {
        let policy = &mut ctx.accounts.insurance_policy;
        require!(policy.is_active, InsuranceError::PolicyNotActive);

        let insurance_pool = &mut ctx.accounts.insurance_pool;
        require!(
            insurance_pool.total_claims_paid + policy.coverage_amount
                <= insurance_pool.total_premium_collected,
            InsuranceError::NotEnoughFunds
        );

        insurance_pool.total_claims_paid += policy.coverage_amount;
        policy.is_active = false; // Mark the policy as inactive after approval

        Ok(())
    }

    // Admin withdraws premium funds from the insurance pool
    pub fn withdraw_premium(ctx: Context<WithdrawPremium>, amount: u64) -> Result<()> {
        let insurance_pool = &mut ctx.accounts.insurance_pool;
        require!(
            insurance_pool.total_premium_collected - insurance_pool.total_claims_paid >= amount,
            InsuranceError::NotEnoughFunds
        );
        
        // Transfer lamports to admin (the insurance protocol administrator)
        **ctx.accounts.admin.to_account_info().try_borrow_mut_lamports()? += amount;
        **ctx.accounts.insurance_pool.to_account_info().try_borrow_mut_lamports()? -= amount;

        Ok(())
    }

    // Log policy actions (created, canceled, claimed)
    pub fn log_policy_action(ctx: Context<LogPolicyAction>, action: PolicyAction) -> Result<()> {
        let history = &mut ctx.accounts.policy_history;
        history.user = *ctx.accounts.user.key;
        history.policy = ctx.accounts.insurance_policy.key();
        history.action = action;
        history.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // Auto-expire policy if it has passed the expiration time
    pub fn process_policy_expiration(ctx: Context<ProcessExpiration>) -> Result<()> {
        let policy = &mut ctx.accounts.insurance_policy;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > policy.end_time && policy.is_active {
            policy.is_active = false;
            return err!(InsuranceError::PolicyExpired);
        }

        Ok(())
    }

    // Adjust coverage dynamically
    pub fn adjust_coverage(ctx: Context<AdjustCoverage>, new_coverage_amount: u64) -> Result<()> {
        let policy = &mut ctx.accounts.insurance_policy;
        require!(policy.is_active, InsuranceError::PolicyNotActive);
        policy.coverage_amount = new_coverage_amount;
        Ok(())
    }

    // Pay premium with token (SPL token support)
    pub fn pay_premium_with_token(ctx: Context<PayPremiumWithToken>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.insurance_pool;  // Fix: change to insurance_pool
        require!(pool.total_premium_collected > 0, InsuranceError::NotEnoughFunds);  // Additional check for premium pool

        let token_account = &mut ctx.accounts.user_token_account;
        let pool_token_account = &mut ctx.accounts.pool_token_account;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: token_account.to_account_info(),
                to: pool_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;  // Correct use of token::transfer

        pool.total_premium_collected += amount;

        Ok(())
    }

    // Stake into the insurance pool for liquidity
    pub fn stake_into_pool(ctx: Context<StakeIntoPool>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.insurance_pool;  // Fix: use insurance_pool here

        let token_account = &mut ctx.accounts.user_token_account;
        let pool_token_account = &mut ctx.accounts.pool_token_account;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: token_account.to_account_info(),
                to: pool_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;

        pool.total_premium_collected += amount;  // Record the stake in the pool

        Ok(())
    }

    // Governance voting on protocol changes
    pub fn submit_governance_vote(ctx: Context<SubmitVote>, proposal_id: u64, vote: bool) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        let vote_record = &mut ctx.accounts.vote_record;

        vote_record.user = *ctx.accounts.user.key;
        vote_record.proposal_id = proposal_id;
        vote_record.vote = vote;
        vote_record.timestamp = Clock::get()?.unix_timestamp;

        if vote {
            governance.yes_votes += 1;
        } else {
            governance.no_votes += 1;
        }

        Ok(())
    }
}

// Define the structure for an insurance policy
#[account]
pub struct InsurancePolicy {
    pub user: Pubkey,
    pub deposit_amount: u64,
    pub coverage_amount: u64,
    pub premium_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub is_active: bool,
}

// Define the structure for the insurance pool that holds total premiums collected and claims paid
#[account]
pub struct InsurancePool {
    pub total_premium_collected: u64,
    pub total_claims_paid: u64,
    pub authority: Pubkey, // Using a PDA to manage the pool
}

impl InsurancePolicy {
    const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 1;
}

impl InsurancePool {
    const LEN: usize = 8 + 8 + 32;
}

// Define the structure for tracking policy history
#[account]
pub struct PolicyHistory {
    pub user: Pubkey,
    pub policy: Pubkey,
    pub action: PolicyAction,
    pub timestamp: i64,
}

// Define policy actions (e.g., created, canceled, claimed, expired)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PolicyAction {
    Created,
    Canceled,
    Claimed,
    Expired,
}

// Define governance structure and vote records
#[account]
pub struct Governance {
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_proposals: u64,
}

#[account]
pub struct VoteRecord {
    pub user: Pubkey,
    pub proposal_id: u64,
    pub vote: bool,
    pub timestamp: i64,
}

impl VoteRecord {
    const LEN: usize = 32 + 8 + 1 + 8;
}

// Error Handling
#[error_code]
pub enum InsuranceError {
    #[msg("The policy is not active.")]
    PolicyNotActive,
    #[msg("The policy has expired.")]
    PolicyExpired,
    #[msg("The insurance pool has insufficient funds.")]
    NotEnoughFunds,
}

// Contexts for instructions

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = admin, space = 8 + InsurancePool::LEN)]
    pub insurance_pool: Account<'info, InsurancePool>,
    #[account(
        seeds = [b"insurance_pool".as_ref()], 
        bump
    )]
    pub pool_authority: AccountInfo<'info>, // PDA controlling the insurance pool
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseInsurance<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + InsurancePolicy::LEN
    )]
    pub insurance_policy: Account<'info, InsurancePolicy>,
    #[account(mut)]
    pub insurance_pool: Account<'info, InsurancePool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelPolicy<'info> {
    #[account(mut)]
    pub insurance_policy: Account<'info, InsurancePolicy>,
    #[account(mut)]
    pub insurance_pool: Account<'info, InsurancePool>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveClaim<'info> {
    #[account(mut)]
    pub insurance_policy: Account<'info, InsurancePolicy>,
    #[account(mut)]
    pub insurance_pool: Account<'info, InsurancePool>,
    pub admin: Signer<'info>, // Only admin can approve claims
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawPremium<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,  // Admin account (needs to be a designated admin)
    #[account(mut)]
    pub insurance_pool: Account<'info, InsurancePool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogPolicyAction<'info> {
    #[account(mut)]
    pub policy_history: Account<'info, PolicyHistory>,
    pub insurance_policy: Account<'info, InsurancePolicy>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProcessExpiration<'info> {
    #[account(mut)]
    pub insurance_policy: Account<'info, InsurancePolicy>,
}

#[derive(Accounts)]
pub struct AdjustCoverage<'info> {
    #[account(mut)]
    pub insurance_policy: Account<'info, InsurancePolicy>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct PayPremiumWithToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub insurance_pool: Account<'info, InsurancePool>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct StakeIntoPool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub insurance_pool: Account<'info, InsurancePool>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SubmitVote<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    #[account(init, payer = user, space = 8 + VoteRecord::LEN)]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(mut)]  // Fix for the missing 'mut'
    pub user: Signer<'info>,  // Mark user as mutable (payer)
    pub system_program: Program<'info, System>,
}
