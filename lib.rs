use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

declare_id!("9PnkDkGEemAeKMnKdPX9sMJEcHfEyJe1rp2sdnNf7vNz");

#[program]
pub mod anchor_pool {
    use super::*;

    pub fn init_pool_wallet(ctx: Context<InitPoolWallet>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.total_sol = 0;
        pool.initialized = true;
        Ok(())
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.pool.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, amount)?;

        let stake_account = &mut ctx.accounts.stake_account;
        stake_account.amount += amount;

        let pool = &mut ctx.accounts.pool;
        pool.total_sol += amount;

        Ok(())
    }

    pub fn transfer_stake(ctx: Context<TransferStake>, amount: u64) -> Result<()> {
        let from = &mut ctx.accounts.from_stake;
        let to = &mut ctx.accounts.to_stake;

        require!(from.amount >= amount, ErrorCode::InsufficientStake);

        from.amount -= amount;
        to.amount += amount;

        Ok(())
    }

    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        transac_id: String,
        beneficiary: Pubkey,
        amt: u64,
        upi_id: String,
    ) -> Result<()> {
        let from = &mut ctx.accounts.from_stake;
        let to = &mut ctx.accounts.to_stake;

        require!(from.amount >= amt, ErrorCode::InsufficientStake);

        from.amount -= amt;
        to.amount += amt;
        let tx = &mut ctx.accounts.transaction;
        tx.transac_id = transac_id;
        tx.payee = ctx.accounts.payee.key();
        tx.beneficiary = beneficiary;
        tx.amt = amt;
        tx.upi_id = upi_id;
        tx.settlement = false;
        tx.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn update_transaction(
        ctx: Context<UpdateTransaction>,
        amt: u64,
        upi_id: String,
    ) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        require!(!tx.settlement, ErrorCode::AlreadySettled);
        tx.amt = amt;
        tx.upi_id = upi_id;
        Ok(())
    }

    pub fn mark_settlement_complete(
        ctx: Context<MarkSettlementComplete>,
        secret_key: Pubkey,
    ) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;

        // Derive expected PDA from seeds
        let binding = tx.amt.to_string();
        let seeds = &[
            binding.as_bytes(),
            tx.upi_id.as_bytes(),
            b"secretkey",
        ];
        let (expected_key, _bump) = Pubkey::find_program_address(seeds, ctx.program_id);

        require!(secret_key == expected_key, ErrorCode::InvalidSecretKey);

        tx.settlement = true;
        Ok(())
    }

    pub fn request_stake_refund(ctx: Context<RequestRefund>) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        require!(!tx.settlement, ErrorCode::AlreadySettled);

        let now = Clock::get()?.unix_timestamp;
        require!(now - tx.timestamp > 900, ErrorCode::TooSoonForRefund);

        let pool_info = ctx.accounts.pool.to_account_info();
        **pool_info.try_borrow_mut_lamports()? -= tx.amt;
        **ctx
            .accounts
            .payee
            .to_account_info()
            .try_borrow_mut_lamports()? += tx.amt;

        tx.settlement = true;
        Ok(())
    }

    pub fn register_node(
        ctx: Context<RegisterNode>,
        node_id: String,
        latitude: f64,
        longitude: f64,
        endpoint: String,
        secret_key: String,
    ) -> Result<()> {
        let node = &mut ctx.accounts.node;
        node.node_id = node_id;
        node.owner = ctx.accounts.owner.key();
        node.latitude = latitude;
        node.longitude = longitude;
        node.state = false;
        node.endpoint = endpoint;
        node.secretkey = secret_key;
        Ok(())
    }

    pub fn set_node_state(ctx: Context<SetNodeState>, active: bool) -> Result<()> {
        let node = &mut ctx.accounts.node;
        require!(
            ctx.accounts.owner.key() == node.owner,
            ErrorCode::Unauthorized
        );
        node.state = active;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitPoolWallet<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 1,
        seeds = [b"pool"],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(
        mut,
        seeds = [b"pool"],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferStake<'info> {
    #[account(
        mut,
        seeds = [b"stake", from.key().as_ref()],
        bump
    )]
    pub from_stake: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [b"stake", to.key().as_ref()],
        bump
    )]
    pub to_stake: Account<'info, StakeAccount>,

    pub from: Signer<'info>,
    /// CHECK: Only used for seed derivation
    pub to: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(transac_id: String)]
pub struct CreateTransaction<'info> {
    #[account(
        init,
        payer = payee,
        space = 8 + 64 + 32 + 32 + 8 + 64 + 1 + 8,
        seeds = [b"tx", transac_id.as_bytes()],
        bump
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(
        mut,
        seeds = [b"stake", payee.key().as_ref()],
        bump
    )]
    pub from_stake: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [b"stake", beneficiary.key().as_ref()],
        bump
    )]
    pub to_stake: Account<'info, StakeAccount>,

    #[account(mut)]
    pub payee: Signer<'info>,

    /// CHECK: Only used for seed derivation
    pub beneficiary: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTransaction<'info> {
    #[account(mut, has_one = payee)]
    pub transaction: Account<'info, Transaction>,
    pub payee: Signer<'info>,
}

#[derive(Accounts)]
pub struct MarkSettlementComplete<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
}

#[derive(Accounts)]
pub struct RequestRefund<'info> {
    #[account(mut, has_one = payee)]
    pub transaction: Account<'info, Transaction>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub payee: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(node_id: String)]
pub struct RegisterNode<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 64 + 32 + 8 + 8 + 1,
        seeds = [b"node", node_id.as_bytes()],
        bump
    )]
    pub node: Account<'info, Node>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetNodeState<'info> {
    #[account(mut, has_one = owner)]
    pub node: Account<'info, Node>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct FetchTxByUser {}

#[derive(Accounts)]
pub struct FetchAllNodes {}

#[account]
pub struct StakeAccount {
    pub amount: u64,
}

#[account]
pub struct Transaction {
    pub transac_id: String,
    pub payee: Pubkey,
    pub beneficiary: Pubkey,
    pub amt: u64,
    pub upi_id: String,
    pub settlement: bool,
    pub timestamp: i64,
}

#[account]
pub struct Node {
    pub node_id: String,
    pub owner: Pubkey,
    pub latitude: f64,
    pub longitude: f64,
    pub state: bool,
    pub endpoint: String,
    pub secretkey: String,
}

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub total_sol: u64,
    pub initialized: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough stake to transfer")]
    InsufficientStake,
    #[msg("Transaction already settled")]
    AlreadySettled,
    #[msg("Secret key does not match")]
    InvalidSecretKey,
    #[msg("Cannot refund yet, too early")]
    TooSoonForRefund,
    #[msg("Unauthorized action")]
    Unauthorized,
}
