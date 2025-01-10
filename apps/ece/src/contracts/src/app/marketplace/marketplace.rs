use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey::Pubkey, system_instruction, program::invoke};
use std::collections::HashMap;

declare_id!("YourProgramIDHere");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owner_id: Pubkey) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.owner_id = owner_id;
        contract.nft_factory = NFTFactory::default();
        contract.products = HashMap::new();
        Ok(())
    }

    pub fn list_product(ctx: Context<ListProduct>, id: u64, owner_id: Pubkey, metadata: String, price: u64) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        require!(ctx.accounts.owner.key() == contract.owner_id, ErrorCode::Unauthorized);

        let product = Product {
            id,
            owner_id,
            metadata,
            price,
        };
        contract.products.insert(id, product);
        msg!("Product {} listed by {}", id, owner_id);
        Ok(())
    }

    pub fn purchase_product(ctx: Context<PurchaseProduct>, product_id: u64) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let buyer_id = *ctx.accounts.buyer.key;
        let product = contract.products.get(&product_id).ok_or(ErrorCode::ProductNotFound)?;

        require!(ctx.accounts.buyer.lamports() >= product.price, ErrorCode::InsufficientFunds);

        let seller_id = product.owner_id;
        contract.products.remove(&product_id);
        let ix = system_instruction::transfer(&buyer_id, &seller_id, product.price);
        invoke(&ix, &ctx.accounts.buyer.to_account_info())?;
        msg!("Product {} purchased by {}", product.id, buyer_id);
        Ok(())
    }

    pub fn get_product(ctx: Context<GetProduct>, product_id: u64) -> Result<Product> {
        let contract = &ctx.accounts.contract;
        let product = contract.products.get(&product_id).ok_or(ErrorCode::ProductNotFound)?;
        Ok(product.clone())
    }
}

#[account]
pub struct Contract {
    pub owner_id: Pubkey,
    pub nft_factory: NFTFactory,
    pub products: HashMap<u64, Product>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Product {
    pub id: u64,
    pub owner_id: Pubkey,
    pub metadata: String,
    pub price: u64,
}

#[derive(Default)]
pub struct NFTFactory {
    pub nfts: HashMap<u64, NFT>,
    pub next_id: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NFT {
    pub id: u64,
    pub owner_id: Pubkey,
    pub metadata: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + std::mem::size_of::<Contract>())]
    pub contract: Account<'info, Contract>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListProduct<'info> {
    #[account(mut)]
    pub contract: Account<'info, Contract>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct PurchaseProduct<'info> {
    #[account(mut)]
    pub contract: Account<'info, Contract>,
    #[account(mut)]
    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetProduct<'info> {
    pub contract: Account<'info, Contract>,
}

#[error]
pub enum ErrorCode {
    #[msg("Unauthorized action.")]
    Unauthorized,
    #[msg("Product not found.")]
    ProductNotFound,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
}