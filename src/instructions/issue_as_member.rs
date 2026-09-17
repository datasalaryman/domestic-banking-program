use crate::state::{Banking, Member};
use quasar_lang::{cpi::Seed, prelude::*};
use quasar_spl::prelude::*;

#[derive(Accounts)]
pub struct IssueAsMember {
    pub authority: Signer,
    pub banking_authority: SystemAccount,
    #[account(
        address=Banking::seeds(banking_authority.address()),
        has_one(currency)
    )]
    pub banking: Banking,
    #[account(
        mut,
        address=Member::seeds(authority.address(), banking.address()),
        has_one(authority),
        has_one(banking)
    )]
    pub member: Member,
    #[account(mut)]
    pub currency: Account<Mint>,
    #[account(mut)]
    pub destination: Account<Token>,
    pub token_program: Program<TokenProgram>,
}

impl IssueAsMember {
    #[inline(always)]
    pub fn issue(&mut self, amount: u64, bumps: &IssueAsMemberBumps) -> Result<(), ProgramError> {
        require_keys_eq!(
            *self.destination.mint(),
            *self.currency.address(),
            ProgramError::InvalidAccountData
        );

        let clock = <Clock as quasar_lang::sysvars::Sysvar>::get()?;
        let current_timestamp =
            u64::try_from(clock.unix_timestamp.get()).map_err(|_| ProgramError::InvalidArgument)?;

        let mut until = self.member.until.get();
        let mut issuance_remaining = self.member.issuance_remaining.get();
        if current_timestamp > until {
            until = until
                .checked_add(self.member.period.get())
                .ok_or(ProgramError::ArithmeticOverflow)?;
            issuance_remaining = self.member.issuance_limit.get();
        }

        issuance_remaining = issuance_remaining
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        {
            let bump = [bumps.banking];
            let banking_signer = [
                Seed::from(b"banking" as &[u8]),
                Seed::from(self.banking_authority.address().as_ref()),
                Seed::from(bump.as_ref()),
            ];
            self.token_program
                .mint_to(&self.currency, &self.destination, &self.banking, amount)
                .invoke_signed(&banking_signer)?;
        }

        self.member.until = until.into();
        self.member.issuance_remaining = issuance_remaining.into();
        Ok(())
    }
}
