use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Hj1cWGvmqaTruSZ2vETEwGBQNtWaJrYMWNhWozeSB4BN");


// 48 bytes
#[zero_copy]
pub struct TokenAccount {
    pub authority: Pubkey,
    pub balance: u64,
    pub mint: u32,
    pub nonce: u32,
}

#[account(zero_copy)]
pub struct TokenAccountSlab {
    pub token_accounts: [TokenAccount; 200_000], 
}

//#[zero_copy]
//pub struct Mint {
//    pub mint_authority: Pubkey,
//}
//
//#[account(zero_copy)]
//pub struct MintSlab {
//    pub mints: [Mint; 

#[program]
pub mod speedy_token {
    use super::*;

    pub fn initialize_token_account_slab(ctx: Context<InitializeTokenAccountSlab>) -> Result<()> {
        let slab = &mut ctx.accounts.slab.load_init()?;
        Ok(())
    }

    pub fn allocate_token_account(ctx: Context<AllocateTokenAccount>, authority: Pubkey, mint: u32, index: u32) -> Result<()> {
        let slab = &mut ctx.accounts.slab.load_mut()?;

        // if you didn't want users to need to pass in indexes, you could do
        // slab allocation inside the program
        let token_account = &mut slab.token_accounts[index as usize];

        require!(token_account.nonce == 0, TokenError::SpaceAlreadyTaken);

        token_account.authority = authority;
        token_account.mint = mint;
        token_account.nonce = 1;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeTokenAccountSlab<'info> {
    #[account(zero)]
    slab: AccountLoader<'info, TokenAccountSlab>,
}

#[derive(Accounts)]
pub struct AllocateTokenAccount<'info> {
    #[account(mut)]
    slab: AccountLoader<'info, TokenAccountSlab>,
}

#[error_code]
pub enum TokenError {
    #[msg("This space has already been taken")]
    SpaceAlreadyTaken,
}
