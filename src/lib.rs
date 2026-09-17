#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod errors;
mod instructions;
mod state;
use instructions::*;

declare_id!("9ZyWG6ZceKcHy9fXRGLDJqZPHNPJtcmQqHAkKVMxidhW");

#[program]
mod domestic_banking_program {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<InitializeBanking>, decimals: u8) -> Result<(), ProgramError> {
        ctx.accounts.initialize_banking(decimals)
    }

    #[instruction(discriminator = 1)]
    pub fn add(ctx: Ctx<AddMember>) -> Result<(), ProgramError> {
        ctx.accounts.add_member()
    }
}

#[cfg(test)]
mod tests;
