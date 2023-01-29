# NUBAN 🦀

[![Build Status](https://travis-ci.org/timolinn/nuban.svg?branch=master)](https://travis-ci.org/timolinn/nuban)
![crates.io](https://img.shields.io/crates/v/nuban.svg)

This is a lightweight Rust crate for verifying NUBAN numbers
for all Nigerian bank accounts as was directed by the CBN.

## What does it do?

It checks the validity of a Nigerian bank account number based on the NUBAN guidelines provided by the CBN.

### How to use

```rust
    use nuban::Nuban;

    fn main() {
        // pass the bank code and account number as arguments
        let nuban = Nuban::new("058", "0739082716").unwrap();
        if let Ok(true) = nuban.is_valid_account() {
            println!("'{}' is a valid account number", nuban.account_number());
        } else {
            println!("'{}' is not a valid account number", nuban.account_number());
        }
    }
```

Create a `NUBAN` instance using the conventional `new` method

```rust
    let nuban = Nuban::new("098", "1038489302");
```

To check validity on a `NUBAN` instance:

```rust
    println!("{}", nuban.is_valid_account().unwrap());
```

### Contributing

- Create an issue if you spot any bug.
- Feel free to fork and fix or extend the feature.
- Run `cargo test` to ensure all tests are passing.
- Add tests for your new features if you can.
- Send in your pull request 🔥🔥

### Other

Like this crate? Please star this repo.

You can give me a shoutout on [Twitter](https://twitter.com/timolinn_)
