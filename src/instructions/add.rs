use quasar_lang::prelude::*;
use crate::state::{Banking, Member, MemberInner}; 

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
    pub fn add_member(&mut self) -> Result<(), ProgramError> {
        self.member.set_inner(MemberInner {
            authority: *self.member_bank.address(), 
            banking: *self.banking.address(), 
            issuance_limit: 12, 
            until: 34,
            issuance_remaining: 2342, 
        });
        Ok(())
    }
}
