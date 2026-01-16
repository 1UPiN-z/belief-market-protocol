use anchor_lang::prelude::*;

declare_id!("YourProgramIDGoeshereReplaceAfterDeploy");

#[program]
pub mod belief_market_protocol {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, invitor: Pubkey, referrer: Pubkey) -> Result<()> {
        // TODO: Create and store user PDA with invitor/referrer
        Ok(())
    }

    pub fn create_market(
        ctx: Context<CreateMarket>,
        outcomes: Vec<String>, // capped by length (e.g., 10–16 entries)
        trading_fee_bps: u16,  // set by creator (10 – 500 = 0.1% – 5%)
        resolve_at: i64        // unix timestamp deadline
    ) -> Result<()> {
        // TODO: Derive PDA, store market info, pools zeroed, set fee/creator
        Ok(())
    }

    pub fn buy_shares(ctx: Context<Trade>, market: Pubkey, outcome_idx: u8, amount: u64) -> Result<()> {
        // TODO: Add funds to pool, mint shares for specific outcome
        Ok(())
    }

    pub fn sell_shares(ctx: Context<Trade>, market: Pubkey, outcome_idx: u8, shares: u64) -> Result<()> {
        // TODO: Burn shares, return USDC from respective pool
        Ok(())
    }

    pub fn resolve_market(ctx: Context<ResolveMarket>, market: Pubkey) -> Result<()> {
        // TODO: Set winning outcome (largest pool at resolve_at time)
        Ok(())
    }

    pub fn redeem_winnings(ctx: Context<Redeem>, market: Pubkey, outcome_idx: u8) -> Result<()> {
        // TODO: Winners withdraw USDC based on shares held in winning outcome
        Ok(())
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>, market: Pubkey) -> Result<()> {
        // TODO: Platform/invitor/referrer/creator withdraws shares of protocol fees
        Ok(())
    }
}

// PDA structs, account structures and context definitions go below.

#[account]
pub struct Market {
    pub creator: Pubkey,
    pub invitor: Pubkey,
    pub referrer: Pubkey,
    pub resolve_at: i64,
    pub resolved: bool,
    pub winning_outcome: u8,
    pub trading_fee_bps: u16,
    pub outcomes: Vec<Outcome>, // capped Vec
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Outcome {
    pub label: String,
    pub pool: u64,  // USDC pooled on this outcome
    pub shares: u64,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub invitor: Pubkey,
    pub referrer: Pubkey,
    pub total_markets_created: u32,
}
