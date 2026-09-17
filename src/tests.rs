use quasar_svm::{Account, Pubkey, QuasarSvm};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

const TOKEN_AMOUNT_OFFSET: usize = 64;
const TOKEN_STATE_OFFSET: usize = 108;
const MEMBER_UNTIL_OFFSET: usize = 81;
const MEMBER_ISSUANCE_REMAINING_OFFSET: usize = 89;

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

fn add_member_instruction(
    payer: Pubkey,
    banking: Pubkey,
    member_authority: Pubkey,
    member: Pubkey,
    issuance_limit: u64,
    period: u64,
) -> Instruction {
    let mut data = vec![1];
    data.extend_from_slice(&issuance_limit.to_le_bytes());
    data.extend_from_slice(&period.to_le_bytes());
    Instruction {
        program_id: Address::from(crate::ID.to_bytes()),
        accounts: vec![
            AccountMeta::new(Address::from(payer.to_bytes()), true),
            AccountMeta::new_readonly(Address::from(banking.to_bytes()), false),
            AccountMeta::new_readonly(Address::from(member_authority.to_bytes()), false),
            AccountMeta::new(Address::from(member.to_bytes()), false),
            AccountMeta::new_readonly(
                Address::from(quasar_svm::system_program::ID.to_bytes()),
                false,
            ),
        ],
        data,
    }
}

fn issue_as_banking_instruction(
    authority: Pubkey,
    banking: Pubkey,
    currency: Pubkey,
    destination: Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![2];
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction {
        program_id: Address::from(crate::ID.to_bytes()),
        accounts: vec![
            AccountMeta::new_readonly(Address::from(authority.to_bytes()), true),
            AccountMeta::new_readonly(Address::from(banking.to_bytes()), false),
            AccountMeta::new(Address::from(currency.to_bytes()), false),
            AccountMeta::new(Address::from(destination.to_bytes()), false),
            AccountMeta::new_readonly(
                Address::from(quasar_svm::SPL_TOKEN_PROGRAM_ID.to_bytes()),
                false,
            ),
        ],
        data,
    }
}

fn issue_as_member_instruction(
    authority: Pubkey,
    banking_authority: Pubkey,
    banking: Pubkey,
    member: Pubkey,
    currency: Pubkey,
    destination: Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![3];
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction {
        program_id: Address::from(crate::ID.to_bytes()),
        accounts: vec![
            AccountMeta::new_readonly(Address::from(authority.to_bytes()), true),
            AccountMeta::new_readonly(Address::from(banking_authority.to_bytes()), false),
            AccountMeta::new_readonly(Address::from(banking.to_bytes()), false),
            AccountMeta::new(Address::from(member.to_bytes()), false),
            AccountMeta::new(Address::from(currency.to_bytes()), false),
            AccountMeta::new(Address::from(destination.to_bytes()), false),
            AccountMeta::new_readonly(
                Address::from(quasar_svm::SPL_TOKEN_PROGRAM_ID.to_bytes()),
                false,
            ),
        ],
        data,
    }
}

fn system_account(address: Pubkey, lamports: u64) -> Account {
    Account {
        address,
        lamports,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    }
}

fn empty_account(address: Pubkey) -> Account {
    system_account(address, 0)
}

fn token_account(address: Pubkey, mint: Pubkey, owner: Pubkey) -> Account {
    let token = quasar_svm::token::TokenAccount {
        mint,
        owner,
        ..quasar_svm::token::TokenAccount::default()
    };
    let mut account = quasar_svm::token::create_keyed_token_account(&address, &token);
    account.data[TOKEN_STATE_OFFSET] = 1;
    account
}

fn read_u64(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap())
}

fn initialize_bank(svm: &mut QuasarSvm) -> (Pubkey, Pubkey, Pubkey) {
    let authority = Pubkey::new_unique();
    let currency = Pubkey::new_unique();
    let (banking, _) = Pubkey::find_program_address(&[b"banking", authority.as_ref()], &crate::ID);
    let instruction = initialize_instruction(
        Address::from(authority.to_bytes()),
        Address::from(banking.to_bytes()),
        Address::from(currency.to_bytes()),
        Address::from(quasar_svm::SPL_TOKEN_PROGRAM_ID.to_bytes()),
        Address::from(quasar_svm::system_program::ID.to_bytes()),
        6,
    );
    let result = svm.process_instruction(
        &instruction,
        &[
            system_account(authority, 10_000_000_000),
            empty_account(banking),
            empty_account(currency),
        ],
    );
    result.assert_success();
    (authority, banking, currency)
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
    assert_eq!(&mint.data[4..36], banking.as_ref());
}

#[test]
fn test_issue_as_banking() {
    let mut svm = setup();
    let (authority, banking, currency) = initialize_bank(&mut svm);
    let destination = Pubkey::new_unique();
    let instruction =
        issue_as_banking_instruction(authority, banking, currency, destination, 1_000_000_000);

    let result = svm.process_instruction(
        &instruction,
        &[token_account(destination, currency, authority)],
    );

    result.assert_success();
    let destination = result.account(&destination).unwrap();
    assert_eq!(
        read_u64(&destination.data, TOKEN_AMOUNT_OFFSET),
        1_000_000_000
    );
}

#[test]
fn test_issue_as_member_enforces_and_resets_limit() {
    let mut svm = setup();
    svm.warp_to_timestamp(1_000);
    let (banking_authority, banking, currency) = initialize_bank(&mut svm);
    let member_authority = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();
    let (member, _) = Pubkey::find_program_address(
        &[b"member", member_authority.as_ref(), banking.as_ref()],
        &crate::ID,
    );
    let add_member = add_member_instruction(
        banking_authority,
        banking,
        member_authority,
        member,
        100,
        50,
    );
    let add_result = svm.process_instruction(
        &add_member,
        &[
            system_account(member_authority, 1_000_000),
            empty_account(member),
        ],
    );
    add_result.assert_success();
    let member_account = add_result.account(&member).unwrap();
    assert_eq!(read_u64(&member_account.data, MEMBER_UNTIL_OFFSET), 1_050);
    assert_eq!(
        read_u64(&member_account.data, MEMBER_ISSUANCE_REMAINING_OFFSET),
        100
    );
    assert_eq!(svm.sysvars.clock.unix_timestamp, 1_000);

    let first_issue = issue_as_member_instruction(
        member_authority,
        banking_authority,
        banking,
        member,
        currency,
        recipient,
        60,
    );
    let first_result = svm.process_instruction(
        &first_issue,
        &[token_account(recipient, currency, Pubkey::new_unique())],
    );
    first_result.assert_success();
    let member_account = first_result.account(&member).unwrap();
    assert_eq!(
        read_u64(&member_account.data, MEMBER_ISSUANCE_REMAINING_OFFSET),
        40
    );

    let over_limit = issue_as_member_instruction(
        member_authority,
        banking_authority,
        banking,
        member,
        currency,
        recipient,
        41,
    );
    svm.warp_to_timestamp(1_050);
    assert!(svm.process_instruction(&over_limit, &[]).is_err());

    svm.warp_to_timestamp(1_051);
    let after_reset = issue_as_member_instruction(
        member_authority,
        banking_authority,
        banking,
        member,
        currency,
        recipient,
        75,
    );
    let reset_result = svm.process_instruction(&after_reset, &[]);
    reset_result.assert_success();
    let member_account = reset_result.account(&member).unwrap();
    assert_eq!(read_u64(&member_account.data, MEMBER_UNTIL_OFFSET), 1_100);
    assert_eq!(
        read_u64(&member_account.data, MEMBER_ISSUANCE_REMAINING_OFFSET),
        25
    );
    let recipient_account = reset_result.account(&recipient).unwrap();
    assert_eq!(read_u64(&recipient_account.data, TOKEN_AMOUNT_OFFSET), 135);
}
