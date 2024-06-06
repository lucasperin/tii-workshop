use Result;

pub enum Error {
    NotEnoughFunds,
    UserDoesNotExists,
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
    pub users: Vec<User>,
    pub credit_interst: u64,
    pub debit_interst: u64,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct BankBalance {
    pub liabilities: u64,
    pub assets: u64,
}

impl Bank {
    fn user_index_by_name(&mut self, name: &str) -> Option<usize> {
        self.users.iter_mut().position(|user| user.name == name)
    }

    pub fn calc_balance(&mut self) -> BankBalance {
        BankBalance {
            liabilities: self
                .users
                .iter()
                .filter(|user| user.balance > 0)
                .map(|user| user.balance.unsigned_abs())
                .sum(),
            assets: self
                .users
                .iter()
                .filter(|user| user.balance < 0)
                .map(|user| user.balance.unsigned_abs())
                .sum(),
        }
    }

    pub fn transfer_funds(
        &mut self,
        amount: u64,
        origin: &str,
        destination: &str,
    ) -> Result<(), Error> {
        let p1 = self.user_index_by_name(origin);
        let p2 = self.user_index_by_name(destination);
        match (p1, p2) {
            (Some(p1), Some(p2)) => {
                if self.users[p1].balance >= amount as i64 {
                    self.users[p1].balance -= amount as i64;
                    self.users[p2].balance += amount as i64;
                    Ok(())
                } else {
                    Err(Error::NotEnoughFunds)
                }
            }
            _ => Err(Error::UserDoesNotExists),
        }
    }

    pub fn accrue_interes(&mut self) {
        for user in self.users.iter_mut().filter(|user| user.balance > 0) {
            user.balance += user.balance * self.debit_interst as i64 / 10_000;
        }
        for user in self.users.iter_mut().filter(|user| user.balance < 0) {
            user.balance += user.balance * self.credit_interst as i64 / 10_000;
        }
    }

    pub fn merge_bank(&mut self, bank: Bank) {
        for user in bank.users {
            match self.user_index_by_name(user.name.as_str()) {
                Some(position) => {
                    self.users[position].balance += user.balance;
                }
                None => {
                    self.users.push(user);
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
            users: vec![u1, u2],
            credit_interst: 500,
            debit_interst: 100,
        }
    }

    #[test]
    fn test_get_user() {
        let mut bank = setup_test();
        let u1 = bank.user_index_by_name("User 1");
        assert!(u1.is_some());
        let u2 = bank.user_index_by_name("User 2");
        assert!(u2.is_some());
        let u_invalid = bank.user_index_by_name("User 3");
        assert!(u_invalid.is_none());
    }

    #[test]
    fn test_get_transfer() {
        let mut bank = setup_test();
        assert!(bank.transfer_funds(200u64, "User 1", "User 2").is_ok());
        assert!(bank.transfer_funds(200u64, "User 2", "User 1").is_err());
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
        bank.accrue_interes();
        assert_eq!(bank.users[0].balance, 1010);
        assert_eq!(bank.users[1].balance, -105);
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
        bank2.users.push(u3.clone());
        bank.merge_bank(bank2);
        assert_eq!(bank.users[2], u3);
        assert_eq!(bank.users[0].balance, 2000);
        assert_eq!(bank.users[1].balance, -200);
    }
}
