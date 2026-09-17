use quasar_lang::prelude::*;

// Creates a new banking domain
#[account(discriminator = 1, set_inner)]
#[seeds(b"banking", authority: Address)]
pub struct Banking {
    pub authority: Address, 
    pub currency: Address,
}

// Creates a new membership to Banking
// Verified through Member address
#[account(discriminator = 2, set_inner)]
#[seeds(b"member", authority: Address, banking: Address)]
pub struct Member {
    pub authority: Address, 
    pub banking: Address, 
    pub issuance_limit: u64, 
    pub until: u64,
    pub issuance_remaining: u64, 
}
