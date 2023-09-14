use anchor_lang::prelude::*;

declare_id!("Hj1cWGvmqaTruSZ2vETEwGBQNtWaJrYMWNhWozeSB4BN");

#[program]
pub mod speedy_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
