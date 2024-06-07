use std::collections::HashMap;
use std::ops::Neg;
use Result;

pub enum Error {
    NotEnoughFunds,
    UserDoesNotExists,
    BadConversion,
    Overflow,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64,
}

#[derive(Debug)]
pub struct Bank {
    pub name: String,
    pub users: HashMap<String, User>,
    pub credit_interst: u64,
    pub debit_interst: u64,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct BankBalance {
    pub liabilities: u64,
    pub assets: u64,
}

impl Bank {
    // fn user_index_by_name(&mut self, name: &str) -> Option<usize> {
    //     self.users.
    //     self.users.iter_mut().position(|user| user.name == name)
    // }

    pub fn calc_balance(&mut self) -> BankBalance {
        BankBalance {
            liabilities: self
                .users
                .values()
                .filter(|user| user.balance > 0)
                .map(|user| user.balance.unsigned_abs())
                .sum(),
            assets: self
                .users
                .values()
                .filter(|user| user.balance < 0)
                .map(|user| user.balance.unsigned_abs())
                .sum(),
        }
    }

    fn add_to_balance(&mut self, origin: &str, amount: i64) -> Result<(), Error> {
        let user = self.users.get_mut(origin).ok_or(Error::UserDoesNotExists)?;
        user.balance = user.balance.checked_add(amount).ok_or(Error::Overflow)?;
        Ok(())
    }

    pub fn transfer_funds(
        &mut self,
        amount: u64,
        origin: &str,
        destination: &str,
    ) -> Result<(), Error> {
        let amount: i64 = amount.try_into().map_err(|_| Error::BadConversion)?;
        match (self.users.get(origin), self.users.get(destination)) {
            (Some(p1), Some(_)) => {
                let origin_credit_line: i64 = p1
                    .credit_line
                    .try_into()
                    .map_err(|_| Error::BadConversion)?;
                if p1
                    .balance
                    .checked_sub(origin_credit_line)
                    .ok_or(Error::Overflow)?
                    >= origin_credit_line.neg()
                {
                    self.add_to_balance(origin, amount.neg())?;
                    self.add_to_balance(destination, amount)?;
                    Ok(())
                } else {
                    Err(Error::NotEnoughFunds)
                }
            }
            _ => Err(Error::UserDoesNotExists),
        }
    }

    pub fn accrue_interest(&mut self) {
        for user in self.users.values_mut().filter(|user| user.balance > 0) {
            user.balance += user.balance * self.debit_interst as i64 / 10_000;
        }
        for user in self.users.values_mut().filter(|user| user.balance < 0) {
            user.balance += user.balance * self.credit_interst as i64 / 10_000;
        }
    }

    pub fn merge_bank(&mut self, bank: Bank) {
        for (name, user) in bank.users {
            match self.users.get_mut(&name) {
                Some(existing_user) => {
                    existing_user.balance += user.balance;
                }
                None => {
                    self.users.insert(name, user);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test() -> Bank {
        let u1 = User {
            name: String::from("User 1"),
            credit_line: 100,
            balance: 1000,
        };
        let u2 = User {
            name: String::from("User 2"),
            credit_line: 100,
            balance: -100,
        };
        Bank {
            name: String::from("ABCD"),
            users: HashMap::from([(u1.name.to_string(), u1), (u2.name.to_string(), u2)]),
            credit_interst: 500,
            debit_interst: 100,
        }
    }

    #[test]
    fn test_get_transfer() {
        let mut bank = setup_test();
        assert!(bank.transfer_funds(200u64, "User 1", "User 2").is_ok());
        assert!(bank.transfer_funds(150u64, "User 2", "User 1").is_ok());
        assert!(bank.transfer_funds(300u64, "User 2", "User 1").is_err());
        assert!(bank.transfer_funds(200u64, "User 3", "User 1").is_err());
    }

    #[test]
    fn test_balance() {
        let mut bank = setup_test();
        assert_eq!(
            bank.calc_balance(),
            BankBalance {
                liabilities: 1000,
                assets: 100,
            }
        );
    }

    #[test]
    fn test_interest() {
        let mut bank = setup_test();
        bank.accrue_interest();
        assert_eq!(bank.users["User 1"].balance, 1010);
        assert_eq!(bank.users["User 2"].balance, -105);
    }

    #[test]
    fn test_merge() {
        let mut bank = setup_test();
        let mut bank2 = setup_test();
        let u3 = User {
            name: String::from("User 3"),
            credit_line: 100,
            balance: 1,
        };
        bank2.users.insert(u3.name.to_string(), u3.clone());
        bank.merge_bank(bank2);
        assert_eq!(bank.users["User 3"], u3);
        assert_eq!(bank.users["User 1"].balance, 2000);
        assert_eq!(bank.users["User 2"].balance, -200);
    }
}
