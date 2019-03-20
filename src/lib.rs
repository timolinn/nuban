#![allow(unused)]

use std::collections::HashMap;

fn calculate_check_digit(bank_code: &str, first_nine: &str) -> String {
    "9".to_string()
}

#[derive(PartialEq, Debug)]
pub struct Nuban {
    bank_code: String,
    account_number: String,
}

impl Nuban {
    pub fn new(bank_code: &str, account_number: &str) -> Result<Self, &'static str> {

        if bank_code.len() != 3 || account_number.len() != 10 {
            return Err("Validation Error: invalid bank code or account number");
        }

        Ok(Nuban {
            bank_code: bank_code.to_string(),
            account_number: account_number.to_string(),
        })
    }

    pub fn get_bank_name(&self) -> Result<&str, &str> {
        Ok("Guaranty trust bank")
    }

    /// Checks whether the account number is a valid NUBAN
    ///
    /// Returns Ok(true) for valid numbers
    /// Returns Err(false) for invalid numbers
    ///
    /// Usage example
    ///
    /// ```rust
    ///     use nuban::Nuban;
    ///
    ///     fn main() {
    ///         let nuban = Nuban::new("058", "0739082716");
    ///         if let nuban.is_valid() = Ok(true) {
    ///             println!("'{}' is a valid account number", nuban.account_number());
    ///         } else {
    ///             println!("'{}' is not a valid account number", nuban.account_number());
    ///         }
    ///     }
    /// ```
    pub fn is_valid(&self) -> Result<bool, bool> {
        let mut account_number = self.account_number.clone();
        let check_digit = account_number.pop().unwrap();
        let mut numbers = String::new();
        numbers.push_str(&self.bank_code);
        numbers.push_str(&account_number);

        // convert str to a list ints to enable calculation
        let number_ints = numbers.chars().map(|num| num.to_digit(10).unwrap());

        let multipliers = vec![3, 7, 3, 3, 7, 3, 3, 7, 3, 3, 7, 3];
        let mut check_sum = 0;
        for (idx, num) in number_ints.enumerate() {
            check_sum += num * multipliers[idx];
        }

        let mut correct_check_digit = 10 - (check_sum % 10);
        if correct_check_digit == 10 {
            correct_check_digit = 0;
        }

        if correct_check_digit == check_digit.to_digit(10).unwrap()  {
            Ok(true)
        } else {
            Err(false)
        }
    }

    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    pub fn bank_code(&self) -> &str {
        &self.bank_code
    }

    pub fn calculate_check_digit(&self) -> Result<u8, &str> {
        Ok(0)
    }

    pub fn check_digit(&self) -> Result<u8, u8> {
        Ok(0)
    }

    fn banks(&self) -> HashMap<&str, &str> {
        let banks: HashMap<&str, &str> = vec![
            ("044", "Access Bank"),
            ("014", "Afribank"),
            ("023", "Citibank"),
            ("063", "Diamond Bank"),
            ("050", "Ecobank"),
            ("040", "Equitorial Trust Bank"),
            ("011", "First Bank"),
            ("214", "FCMB"),
            ("070", "Fidelity"),
            ("085", "FinBank"),
            ("058", "Guarantee Trust Bank"),
            ("069", "Intercontinentl Bank"),
            ("056", "Oceanic Bank"),
            ("082", "BankPhb"),
            ("076", "Skye Bank"),
            ("084", "SpringBank"),
            ("221", "StanbicIBTC"),
            ("068", "Standard Chartered Bank"),
            ("232", "Sterling Bank"),
            ("033", "United Bank For Africa"),
            ("032", "Union Bank"),
            ("035", "Wema Bank"),
            ("057", "Zenith Bank"),
            ("215", "Unity Bank"),
        ]
        .into_iter()
        .collect();
        banks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_returns_new_nuban_instance() {
        let account = Nuban::new("058", "0982736625");
        assert_eq!(
            account.unwrap(),
            Nuban {
                bank_code: String::from("058"),
                account_number: String::from("0982736625")
            }
        );
    }

    #[test]
    fn it_returns_false_for_invalid_account() {
        let account = Nuban::new("058", "0982736625").unwrap();
        assert!(account.is_valid().is_err());
    }

    #[test]
    fn it_returns_true_for_valid_account() {
        let account = Nuban::new("058", "0152792740").unwrap();
        assert!(account.is_valid().is_ok());
    }
}
