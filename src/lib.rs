//! This is a lightweight crate for verifying NUBAN numbers
//! for all Nigerian bank accounts as was directed by the CBN.

use std::{cell::Cell, collections::HashMap, fmt, sync::Once};

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
pub struct Nuban<'a>(&'a str, &'a str, &'a str);

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

impl<'a> Nuban<'a> {
    pub fn new(bank_code: &'a str, account_number: &'a str) -> Result<Self, Error> {
        #[rustfmt::skip] {
            if !Self::is_valid_bank(bank_code) { Err(Error::InvalidBankCode)? }
            if account_number.len() != 10  { Err(Error::InvalidAccountNumber)? }
        };

        let check_digit = {
            let (account_number, check_digit) = account_number.split_at(9);
            match (
                check_digit.chars().next().unwrap().to_digit(10),
                Self::calculate_check_digit(bank_code, account_number),
            ) {
                (Some(l), r) if l != r => Err(Error::InvalidAccountNumber)?,
                _ => {}
            };
            check_digit
        };
        Ok(Nuban(bank_code, account_number, check_digit))
    }

    pub fn is_valid_bank(bank_code: &str) -> bool {
        bank_code.len() == 3 && Self::banks().contains_key(bank_code)
    }

    pub fn is_valid_account(bank_code: &str, account_number: &str) -> bool {
        Nuban::new(bank_code, account_number).is_err()
    }

    pub fn bank_code(&self) -> &str {
        self.0
    }

    pub fn bank_name(&self) -> &str {
        Self::banks().get(self.0).unwrap()
    }

    pub fn account_number(&self) -> &str {
        self.1
    }

    pub fn check_digit(&self) -> &str {
        self.2
    }

    fn calculate_check_digit(bank_code: &'a str, account_number: &'a str) -> u32 {
        // The Approved NUBAN format: [ABC][DEFGHIJKL][M], where
        //   -       ABC : 3-digit Bank Code
        //   - DEFGHIJKL : NUBAN Account Serial Number
        //   -         M : NUBAN Check Digit
        // https://www.cbn.gov.ng/OUT/2011/CIRCULARS/BSPD/NUBAN%20PROPOSALS%20V%200%204-%2003%2009%202010.PDF

        let nuban_chars = bank_code.chars().chain(account_number.chars());
        let nuban_digits = nuban_chars.map(|num| num.to_digit(10).unwrap());
        let seed = [3, 7, 3, 3, 7, 3, 3, 7, 3, 3, 7, 3].iter();
        let check_sum: u32 = seed.zip(nuban_digits).map(|(l, r)| l * r).sum();
        match 10 - (check_sum % 10) {
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
        let account = Nuban::new("058", "0152792740");
        assert_eq!(account.unwrap(), Nuban("058", "0152792740", "0"));
    }

    #[test]
    fn test_returns_false_for_invalid_account() {
        let account = Nuban::new("058", "0982736625");
        assert!(account.is_err());
    }

    #[test]
    fn test_returns_true_for_valid_account() {
        let account = Nuban::new("058", "0152792740");
        assert!(account.is_ok());
    }

    #[test]
    fn test_calculate_check_digit() {
        let correct_check_digit = Nuban::calculate_check_digit("058", "0152792740");
        assert_eq!(correct_check_digit, 0);
    }

    #[test]
    fn test_get_bank_name() {
        let account = Nuban::new("058", "0152792740").unwrap();
        assert_eq!(account.bank_name(), String::from("Guaranty Trust Bank"));
    }
}
