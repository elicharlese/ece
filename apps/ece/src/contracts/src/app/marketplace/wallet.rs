use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey::Pubkey, program::invoke};
use std::collections::HashMap;

declare_id!("YourProgramIDHere");

#[account]
pub struct Marketplace {
    listings: HashMap<Pubkey, Item>,
    balances: HashMap<Pubkey, u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Item {
    id: String,
    owner_id: Pubkey,
    price: u64,
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            listings: HashMap::new(),
            balances: HashMap::new(),
        }
    }
}

#[program]
pub mod wallet {
    use super::*;

    pub fn new_marketplace(ctx: Context<InitializeMarketplace>) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.listings = HashMap::new();
        marketplace.balances = HashMap::new();
        Ok(())
    }

    pub fn list_item(ctx: Context<ListItem>, id: String, price: u64) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();
        let item = Item {
            id: id.clone(),
            owner_id,
            price,
        };
        ctx.accounts.marketplace.listings.insert(Pubkey::from_str(&id)?, item);
        msg!("Item {} listed by {}", id, owner_id);
        Ok(())
    }

    pub fn purchase_item(ctx: Context<PurchaseItem>, id: String) -> Result<()> {
        let buyer_id = ctx.accounts.buyer.key();
        let item = ctx.accounts.marketplace.listings.get(&Pubkey::from_str(&id)?).ok_or(ErrorCode::ItemNotFound)?;
        let buyer_balance = ctx.accounts.marketplace.balances.get(&buyer_id).unwrap_or(&0);

        require!(buyer_balance >= &item.price, ErrorCode::InsufficientFunds);

        ctx.accounts.marketplace.balances.insert(buyer_id, buyer_balance - item.price);
        ctx.accounts.marketplace.balances.insert(item.owner_id, ctx.accounts.marketplace.balances.get(&item.owner_id).unwrap_or(&0) + item.price);
        ctx.accounts.marketplace.listings.remove(&Pubkey::from_str(&id)?);

        msg!("Item {} purchased by {}", id, buyer_id);
        Ok(())
    }

    #[payable]
    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let account_id = ctx.accounts.user.key();
        let amount = ctx.accounts.user.lamports();
        let balance = ctx.accounts.marketplace.balances.get(&account_id).unwrap_or(&0);
        ctx.accounts.marketplace.balances.insert(account_id, balance + amount);
        msg!("Deposited {} to {}", amount, account_id);
        Ok(())
    }

    #[payable]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let account_id = ctx.accounts.user.key();
        let balance = ctx.accounts.marketplace.balances.get(&account_id).unwrap_or(&0);
        require!(balance >= &amount, ErrorCode::InsufficientFunds);
        ctx.accounts.marketplace.balances.insert(account_id, balance - amount);
        **invoke**(system_instruction::transfer(account_id, amount));
        msg!("Withdrew {} from {}", amount, account_id);
        Ok(())
    }

    pub fn balance_of(ctx: Context<BalanceOf>, account_id: Pubkey) -> Result<u64> {
        Ok(ctx.accounts.marketplace.balances.get(&account_id).unwrap_or(&0).clone())
    }

    pub fn get_info(ctx: Context<GetInfo>) -> Result<String> {
        // Placeholder for token info retrieval
        let token_info = "Token Info Placeholder"; // Replace with actual token info retrieval logic
        Ok(token_info.to_string())
    }
}

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<Marketplace>())]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListItem<'info> {
    pub marketplace: Account<'info, Marketplace>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct PurchaseItem<'info> {
    pub marketplace: Account<'info, Marketplace>,
    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub marketplace: Account<'info, Marketplace>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub marketplace: Account<'info, Marketplace>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct BalanceOf<'info> {
    pub marketplace: Account<'info, Marketplace>,
}

#[derive(Accounts)]
pub struct GetInfo<'info> {
    pub marketplace: Account<'info, Marketplace>,
}

#[error]
pub enum ErrorCode {
    #[msg("Item not found.")]
    ItemNotFound,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
}