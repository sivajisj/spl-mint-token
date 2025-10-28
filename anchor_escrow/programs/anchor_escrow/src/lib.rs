use anchor_lang::prelude::*;

mod state;
mod errors;
mod instructions;
use instructions::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod anchor_escrow {
    use super::*;



    // ✅ Main escrow creation — stores details and deposits Token A
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, receive, amount)
    }

    // ✅ Accepts the escrow — transfers Token B, closes vault, releases Token A
    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::handler(ctx)
    }

    // ✅ Refunds escrow — returns Token A to maker, closes vault and escrow
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}





// ERROR: Custom program error: 0x1004

// PROGRAM LOGS:
//  22222222222222222222222222222222222222222222 invoke [1]
//  log: AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id.
//  22222222222222222222222222222222222222222222 consumed 3790 of 1400000 compute units
//  22222222222222222222222222222222222222222222 failed: custom program error: 0x1004
