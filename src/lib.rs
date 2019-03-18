use std::collections::HashMap;

pub struct Nuban {
    bank_code: u8,
    account_number: u32,
}

impl Nuban {
    pub fn new(bank_code: u8, account_number: u32) -> Nuban {
        Nuban {
            bank_code,
            account_number
        }
    }

    pub fn get_bank_name(&self) -> &str {
        "Guaranty trust bank"
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    pub fn account_number(&self) -> u32 {
        self.account_number
    }

    pub fn bank_code(&self) -> u8 {
        self.bank_code
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
            ].into_iter().collect();
        banks
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
