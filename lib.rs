// Declare the modules 
pub mod contracts;
pub mod api;
pub mod utils;

// Re-exporting commonly used types and functions for easier access
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

pub use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::program::invoke;
pub use anchor_lang::solana_program::account_info::AccountInfo;
pub use anchor_lang::solana_program::entrypoint::ProgramResult;
pub use anchor_lang::solana_program::msg;
pub use anchor_lang::solana_program::system_instruction;
pub use anchor_lang::solana_program::program_pack::Pack;