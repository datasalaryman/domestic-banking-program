use crate::state::{Banking, Member, MemberInner};
use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct AddMember {
    pub payer: Signer,
    #[account(
        address=Banking::seeds(payer.address())
    )]
    pub banking: Banking,
    pub member_bank: SystemAccount,
    #[account(
        init,
        mut,
        payer=payer,
        address=Member::seeds(member_bank.address(), banking.address())
    )]
    pub member: Member,
    pub system_program: Program<SystemProgram>,
}

impl AddMember {
    #[inline(always)]
    pub fn add_member(
        &mut self,
        issuance_limit: u64,
        period: u64,
    ) -> Result<(), ProgramError> {
        let clock = <Clock as quasar_lang::sysvars::Sysvar>::get()?;
        let current_timestamp = u64::try_from(clock.unix_timestamp.get())
            .map_err(|_| ProgramError::InvalidArgument)?;
        let until = current_timestamp
            .checked_add(period)
            .ok_or(ProgramError::InvalidArgument)?;

        self.member.set_inner(MemberInner {
            authority: *self.member_bank.address(),
            banking: *self.banking.address(),
            issuance_limit,
            period,
            until,
            issuance_remaining: issuance_limit,
        });
        Ok(())
    }
}
