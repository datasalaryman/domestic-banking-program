use quasar_lang::prelude::*;
use quasar_spl::prelude::*;
use crate::state::{Banking, BankingInner}; 

#[derive(Accounts)]
pub struct InitializeBanking {
    pub payer: Signer,
    #[account(
        init, 
        mut, 
        payer=payer, 
        address=Banking::seeds(payer.address())
    )]
    pub banking: Banking, 
    pub currency: Account<Mint>, 
    pub system_program: Program<SystemProgram>,
}

impl InitializeBanking {
    #[inline(always)]
    pub fn initialize_banking(&mut self) -> Result<(), ProgramError> {
        self.banking.set_inner(BankingInner {
            authority: *self.payer.address(), 
            currency: *self.currency.address(), 
        }); 
        Ok(())
    }
}
