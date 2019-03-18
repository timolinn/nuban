# NUBAN Verifier

This crate is still under development.

### What does it do?

It checks the validity of a Nigerian bank account number using the NUBAN guidelines.

```rust
    use nuban::Nuban;

    let account = Nuban::new(BANK_CODE, ACCOUNT_NUMBER);
    println!("{}", account.is_valaid());
```