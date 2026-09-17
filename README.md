# Domestic Banking Program

A Solana program for creating a domestic banking domain and issuing its currency. A banking authority controls an SPL token mint and can delegate bounded issuance rights to member authorities.

## What It Can Do

- Create a banking domain for a signing authority with an SPL token mint with the banking PDA as authority.
- Add member banks to a banking domain giving them an issuance allowance and a recurring time period.
- Mint currency directly as the banking authority.
- Let members mint to token accounts for the banking currency while enforcing their remaining allowance.

## Todo

- Removing members
- Modifying member limits
- Allowing for transfer of banking authority
- Other Token Program operations
- Token-2022 support

## Instructions

| Discriminator | Instruction | Who signs | Behavior |
| --- | --- | --- | --- |
| `0` | `initialize(decimals)` | Banking authority | Creates the banking PDA and currency mint. |
| `1` | `add(issuance_limit, period)` | Banking authority | Creates a member PDA with a renewable issuance allowance. `period` is expressed in seconds. |
| `2` | `issue_as_banking(amount)` | Banking authority | Mints currency to a token account owned by the banking authority. |
| `3` | `issue_as_member(amount)` | Member authority | Mints currency to a token account for the banking mint and deducts the amount from the member's current allowance. |

Amounts and limits are raw token units. For example, with six decimals, `1_000_000` units represent one token.

## Account Model

### Banking

Derived from:

```text
["banking", banking_authority]
```

Stores:

- Banking authority address
- Currency mint address
- Currency decimals

### Member

Derived from:

```text
["member", member_authority, banking_pda]
```

Stores:

- Member authority address
- Banking PDA address
- Issuance limit per period
- Period duration in seconds
- Current period expiration timestamp
- Remaining issuance allowance

When a member issues currency, the program verifies the member signer and banking relationship, checks that the destination uses the banking currency mint, and subtracts the amount from the current allowance. An expired period restores the allowance before issuance.

## Program ID

```text
9ZyWG6ZceKcHy9fXRGLDJqZPHNPJtcmQqHAkKVMxidhW
```

## Build And Test

This project uses [Quasar](https://github.com/blueshift-gg/quasar) and Rust-based `quasar-svm` tests.

```sh
quasar build
quasar test
```

The tests cover banking initialization, direct banking issuance, member allowance enforcement, and allowance renewal after a period expires.
