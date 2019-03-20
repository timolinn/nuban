# NUBAN 🦀

[![Build Status](https://travis-ci.org/timolinn/nuban.svg?branch=master)](https://travis-ci.org/timolinn/nuban)

This is a lightweight Rust crate for verifying NUBAN numbers
for all Nigerian bank accounts as was directed by the CBN.

### What does it do?

It checks the validity of a Nigerian bank account number based on the NUBAN guidelines provided by the CBN.

```rust
    use nuban::Nuban;

    fn main() {
        // pass the bank code and account number as arguments
        let nuban = Nuban::new("058", "0739082716").unwrap();
        if let Ok(true) = nuban.is_valid() {
            println!("'{}' is a valid account number", nuban.account_number());
        } else {
            println!("'{}' is not a valid account number", nuban.account_number());
        }
    }
```

### How to use

Create a `NUBAN` instance using the conventional `new` method

```rust
    let nuban = Nuban::new("098", "1038489302").unwrap();
```

The `new` method returns a `Result` type. It returns an `Err()` type for invalid account number or bank code. Typical when the length of `BANK_CODE` is not equal to `3` or the length of `ACCOUNT_NUMBER` is not equal to `10`.

To check validity on a `NUBAN` instance:
```rust
    println!("{}", nuban.is_valid().unwrap());
```

Also, You have access to helpful methods when using this crate:

> `get_bank_name(&self)`

Returns the full name of the bank. Returns `Err("Bank not found")` for invalid codes.

> `calculate_check_digit(&self)`

Returns the correct check digit of the account number.

> `bank(&self)`

Returns a `HashMap` of all the banks with their codes.


### Extra Features

+ Get full bank name for valid NUBANs

```rust
    let nuban = Nuban::new("058", "0739082716").unwrap();
    println!("{}", nuban.get_bank_name());
```

## Contributing

- Create an issue if you spot any bug.
- Feel free to fork and fix or extend the feature.
- Run `cargo test` to ensure all tests are passing.
- Add tests for your new features if you can.
- Send in your pull request 🔥🔥

## Other

Like this crate? Please star this repo.

You can give me a shoutout on [Twitter](https://twitter.com/timolinn_)