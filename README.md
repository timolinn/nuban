# NUBAN Verifier

This crate is still under development.

### What does it do?

It checks the validity of a Nigerian bank account number using the NUBAN guidelines.

```rust
    use nuban::Nuban;

    let account = Nuban::new(BANK_CODE, ACCOUNT_NUMBER);
    println!("{}", account.is_valid());
```

### Features

+ Get bank name using the code

### Progress?

This project is being developed using TDD, run `cargo test` to check progress. Once all existing tests are passing, then it's ready 😄.