
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
      #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    //Token Accounts
    #[account(mint::token_program = token_program)]
    pub mint_a: IterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_b: IterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program

    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = Maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub Vault: InterfaceAccount<'info, TokenAccount>,

    //Programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}



impl<'info> Make<'info> {
    // #creating the Escrow
    fn populate_escrow(&mut self, seed: u64,amount: u64, bump: u8) ->Result<()> {
        self.escrow.set_inner(Escrow{
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: amount,
            bump
        });

        Ok(())
    }

    // Deposit the tokens
    fn deposit_tokens(&self, amount: u64) -> Result<()>{
        transfer_checker(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked{
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.maker.to_account_info()

                },
            ),
            amount, self.mint_a.decimals,
        )?;

        Ok(())
        
    }

pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
   //Validate the amount
   require_gt!(receive, 0, EscrowError::InvalidAmount);
   require_gt!(amount, 0, EscrowError::InvalidAmount);


   ctx.accounts.populate_escrow(seed, receive,ctx.bump.escrow)?;

   ctx.accounts.deposit_tokens(amount)?;
   Ok(())
}
}