use crate::state::Banking;
use quasar_lang::prelude::*;
use quasar_spl::prelude::*;

#[derive(Accounts)]
pub struct IssueAsBanking {
    pub authority: Signer,
    #[account(
        address=Banking::seeds(authority.address()),
        has_one(currency)
    )]
    pub banking: Banking,
    #[account(mut)]
    pub currency: Account<Mint>,
    #[account(mut, token(mint=currency, authority=authority, token_program=token_program))]
    pub destination: Account<Token>,
    pub token_program: Program<TokenProgram>,
}

impl IssueAsBanking {
    #[inline(always)]
    pub fn issue(&self, amount: u64, bumps: &IssueAsBankingBumps) -> Result<(), ProgramError> {
        let banking_signer = self.banking_signer(bumps);
        self.token_program
            .mint_to(&self.currency, &self.destination, &self.banking, amount)
            .invoke_signed(&banking_signer)
    }
}
