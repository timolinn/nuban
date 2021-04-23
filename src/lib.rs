//! This is a lightweight crate for verifying NUBAN numbers
//! for all Nigerian bank accounts as was directed by the CBN.

use std::collections::HashMap;
use std::{cell::Cell, sync::Once};

pub const BANKS: [(&'static str, &'static str); 24] = [
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
    ("058", "Guaranty Trust Bank"),
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
];

struct LazyBanks(Once, Cell<Option<HashMap<&'static str, &'static str>>>);

unsafe impl Sync for LazyBanks {}

static LAZY_BANKS: LazyBanks = LazyBanks(Once::new(), Cell::new(None));

#[derive(Eq, Clone, Debug, PartialEq)]
pub struct Nuban {
    bank_code: String,
    account_number: String,
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum Error {
    InvalidBankCode,
    InvalidAccountNumber,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Error::InvalidBankCode => "invalid bank code",
            Error::InvalidAccountNumber => "invalid account number",
        };
        write!(f, "{}", reason)
    }
}

impl Nuban {
    pub fn new(bank_code: &str, account_number: &str) -> Result<Self, Error> {
        #[rustfmt::skip] {
            if bank_code.len() != 3 || !Self::is_valid_bank(bank_code) { Err(Error::InvalidBankCode)? }
            if account_number.len() != 10 { Err(Error::InvalidAccountNumber)? }
        };

        Ok(Nuban {
            bank_code: bank_code.to_string(),
            account_number: account_number.to_string(),
        })
    }

    pub fn get_bank_name(&self) -> &str {
        Self::banks().get(self.bank_code()).copied().unwrap()
    }

    pub fn is_valid(&self) -> bool {
        let check_digit = self.account_number.chars().last().unwrap();
        let check_digit = check_digit.to_digit(10).unwrap() as u8;
        self.calculate_check_digit() == check_digit
    }

    pub fn is_valid_bank(bank_code: &str) -> bool {
        Self::banks().contains_key(bank_code)
    }

    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    pub fn bank_code(&self) -> &str {
        &self.bank_code
    }

    pub fn calculate_check_digit(&self) -> u8 {
        // The Approved NUBAN format: [ABC][DEFGHIJKL][M], where
        //   -       ABC : 3-digit Bank Code
        //   - DEFGHIJKL : NUBAN Account Serial Number
        //   -         M : NUBAN Check Digit
        // https://www.cbn.gov.ng/OUT/2011/CIRCULARS/BSPD/NUBAN%20PROPOSALS%20V%200%204-%2003%2009%202010.PDF
        // let numbers = format!("{}{}", , &self.account_number[..9]);

        let bank_code = self.bank_code.chars();
        let account_number = self.account_number.chars().take(9);
        let nuban_chars = bank_code.chain(account_number);
        let nuban_digits = nuban_chars.map(|num| num.to_digit(10).unwrap());
        let seed = [3, 7, 3, 3, 7, 3, 3, 7, 3, 3, 7, 3].iter();
        let check_sum: u32 = seed.zip(nuban_digits).map(|(l, r)| l * r).sum();
        match 10 - (check_sum % 10) as u8 {
            10 => 0,
            x => x,
        }
    }

    pub fn banks() -> &'static HashMap<&'static str, &'static str> {
        LAZY_BANKS
            .0
            .call_once(|| LAZY_BANKS.1.set(Some(BANKS.iter().copied().collect())));

        unsafe {
            if let Some(ref banks) = *LAZY_BANKS.1.as_ptr() {
                return banks;
            }
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_returns_new_nuban_instance() {
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
    fn test_returns_false_for_invalid_account() {
        let account = Nuban::new("058", "0982736625").unwrap();
        assert!(!account.is_valid());
    }

    #[test]
    fn test_returns_true_for_valid_account() {
        let account = Nuban::new("058", "0152792740").unwrap();
        assert!(account.is_valid());
    }

    #[test]
    fn test_calculate_check_digit() {
        let account = Nuban::new("058", "0152792740").unwrap();
        let correct_check_digit = account.calculate_check_digit();
        assert_eq!(correct_check_digit, 0);
    }

    #[test]
    fn test_get_bank_name() {
        let account = Nuban::new("058", "0152792740").unwrap();
        assert_eq!(account.get_bank_name(), String::from("Guaranty Trust Bank"));
    }
}
