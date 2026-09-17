use quasar_svm::{Account, Pubkey, QuasarSvm};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/domestic_banking_program.so").unwrap();
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), &elf)
}

fn initialize_instruction(
    payer: Address,
    banking: Address,
    currency: Address,
    token_program: Address,
    system_program: Address,
    decimals: u8,
) -> Instruction {
    Instruction {
        program_id: Address::from(crate::ID.to_bytes()),
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(banking, false),
            AccountMeta::new(currency, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
        ],
        data: vec![0, decimals],
    }
}

#[test]
fn test_initialize() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let currency = Pubkey::new_unique();
    let (banking, _) = Pubkey::find_program_address(&[b"banking", payer.as_ref()], &crate::ID);

    let instruction = initialize_instruction(
        Address::from(payer.to_bytes()),
        Address::from(banking.to_bytes()),
        Address::from(currency.to_bytes()),
        Address::from(quasar_svm::SPL_TOKEN_PROGRAM_ID.to_bytes()),
        Address::from(quasar_svm::system_program::ID.to_bytes()),
        6,
    );

    let result = svm.process_instruction(
        &instruction,
        &[
            Account {
                address: payer,
                lamports: 10_000_000_000,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
            Account {
                address: banking,
                lamports: 0,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
            Account {
                address: currency,
                lamports: 0,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
        ],
    );

    result.assert_success();
    let mint = result.account(&currency).unwrap();
    assert_eq!(mint.owner, quasar_svm::SPL_TOKEN_PROGRAM_ID);
    assert_eq!(mint.data[44], 6);
}
