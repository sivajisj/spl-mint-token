use anchor_lang::prelude::*;

mod state;
mod errors;
mod instructions;
use instructions::*;

declare_id!("3acf7jULzuiDE41DzhKyT46pZ3FscRHBUxiHN4pgvv5y");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    #[instruction(discriminator = 0)]
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) ->Result<()> {
Ok(())
    }

    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()>{
Ok(())
    }
    
    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()>{
Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
