use anchor_lang::prelude::*;
use anchor_lang::solana_program;

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

#[zero_copy]
pub struct Mint {
    pub mint_authority: Pubkey,
}

#[account(zero_copy)]
pub struct MintSlab {
    pub mints: [Mint; 300_000],
}

#[program]
pub mod speedy_token {
    use super::*;

    pub fn initialize_token_account_slab(_ctx: Context<InitializeTokenAccountSlab>) -> Result<()> {
        Ok(())
    }

    pub fn allocate_token_account(
        ctx: Context<AllocateTokenAccount>,
        authority: Pubkey,
        mint: u32,
        index: u32,
    ) -> Result<()> {
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

    pub fn initialize_mint_slab(_ctx: Context<InitializeMintSlab>) -> Result<()> {
        Ok(())
    }

    pub fn allocate_mint(
        ctx: Context<AllocateMint>,
        mint_authority: Pubkey,
        index: u32,
    ) -> Result<()> {
        let slab = &mut ctx.accounts.slab.load_mut()?;

        let mint = &mut slab.mints[index as usize];

        require!(
            mint.mint_authority == Pubkey::default(),
            TokenError::SpaceAlreadyTaken
        );

        mint.mint_authority = mint_authority;

        Ok(())
    }

    pub fn mint_to(ctx: Context<MintTo>, to: u32, amount: u64) -> Result<()> {
        let token_slab = &mut ctx.accounts.token_slab.load_mut()?;

        let to = &mut token_slab.token_accounts[to as usize];
        to.balance += amount;

        Ok(())
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Debug, Default)]
    pub struct TransferInstruction {
        from: u32,
        to: u32,
        amount: u64,
    }

    pub fn transfer(ctx: Context<Transfer>) -> Result<()> {
        let slab = &mut ctx.accounts.slab.load_mut()?;

        let ed25519_ixs = solana_program::sysvar::instructions::load_current_index_checked(&ctx.accounts.instructions.to_account_info())?;

        for i in 0..ed25519_ixs {
            let ed25519_ix = solana_program::sysvar::instructions::load_instruction_at_checked(
                i.into(),
                &ctx.accounts.instructions.to_account_info(),
            )?;
            let signed_transfer_ix = TransferInstruction::try_from_slice(&ed25519_ix.data[112..])?;

            let from = &mut slab.token_accounts[signed_transfer_ix.from as usize];
            from.balance -= signed_transfer_ix.amount;
            from.nonce += 1;

            let to = &mut slab.token_accounts[signed_transfer_ix.to as usize];
            to.balance += signed_transfer_ix.amount;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    token_slab: AccountLoader<'info, TokenAccountSlab>,
    #[account(mut)]
    mint_slab: AccountLoader<'info, MintSlab>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    slab: AccountLoader<'info, TokenAccountSlab>,
    /// CHECK: do what I say, silly compiler
    #[account(address = solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
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

#[derive(Accounts)]
pub struct InitializeMintSlab<'info> {
    #[account(zero)]
    slab: AccountLoader<'info, MintSlab>,
}

#[derive(Accounts)]
pub struct AllocateMint<'info> {
    #[account(mut)]
    slab: AccountLoader<'info, MintSlab>,
}

#[error_code]
pub enum TokenError {
    #[msg("This space has already been taken")]
    SpaceAlreadyTaken,
}
