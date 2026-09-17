use crate::state::{Banking, BankingInner};
use quasar_lang::prelude::*;
use quasar_spl::prelude::*;
use quasar_spl::MintInitParams;

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
    pub currency: Uninit<Account<Mint>>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<SystemProgram>,
}

impl InitializeBanking {
    #[inline(always)]
    pub fn initialize_banking(&mut self, decimals: u8) -> Result<(), ProgramError> {
        self.currency.init(
            &self.payer,
            MintInitParams::Mint {
                decimals,
                authority: self.banking.address(),
                freeze_authority: Some(self.banking.address()),
                token_program: self.token_program.to_account_view(),
            },
        )?;

        self.banking.set_inner(BankingInner {
            authority: *self.payer.address(),
            currency: *self.currency.address(),
            decimals,
        });
        Ok(())
    }
}
