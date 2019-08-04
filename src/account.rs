mod account {
    use uuid::Uuid;
    use crate::money::money::*;

    pub struct Account {
        id: Uuid,
        pub balance: Money,
    }

    impl Account {
        pub fn new(balance: Money) -> Account {
            Account { id: Uuid::new_v4(), balance: balance }
        }

        pub fn deposit(&mut self, amount: Money) {
            self.balance = Money::new((self.balance+amount).amount, amount.currency)
        }

        pub fn withdraw(&mut self, amount: Money) {
            self.balance = Money::new((self.balance-amount).amount, amount.currency)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use account::*;
    use crate::money::money::*;

    #[test]
    fn it_should_deposit_correct_amount_onto_the_account() {
        let mut account = Account::new(Money::zero("PLN"));

        account.deposit(Money::new(2, "PLN"));

        assert_eq!(account.balance, Money::new(2, "PLN"));
    }

    #[test]
    fn it_should_withdraw_correct_amount_from_the_account() {
        let mut account = Account::new(Money::new(4,"PLN"));

        account.withdraw(Money::new(1, "PLN"));

        assert_eq!(account.balance, Money::new(3, "PLN"));
    }

}