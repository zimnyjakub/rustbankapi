use std::cmp;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Money {
    pub amount: i64,
    pub currency: &'static str,
}

impl Money {
    pub fn new(amount: i64, currency: &'static str) -> Money {
        Money {
            amount: amount,
            currency: currency,
        }
    }

    pub fn zero(currency: &'static str) -> Money {
        Money {
            amount: 0,
            currency: currency,
        }
    }

    fn check_currency(&self, other: &Money) {
        if self.currency != other.currency {
            panic!("Currencies do not match");
        }
    }
}

impl ops::Add for Money {
    type Output = Money;
    fn add(self, _rhs: Money) -> Money {
        self.check_currency(&_rhs);

        Money {
            amount: self.amount + _rhs.amount,
            currency: self.currency,
        }
    }
}

impl ops::Sub for Money {
    type Output = Money;

    fn sub(self, _rhs: Money) -> Money {
        self.check_currency(&_rhs);
        if _rhs.amount > self.amount {
            panic!("Subtracting more than you have")
        }

        Money {
            amount: self.amount - _rhs.amount,
            currency: self.currency,
        }
    }
}

impl ops::Mul<i64> for Money {
    type Output = Money;

    fn mul(self, _rhs: i64) -> Money {
        Money {
            amount: self.amount * _rhs,
            currency: self.currency,
        }
    }
}

impl cmp::PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two_money_objects_correctly() {
        let money = Money::new(1, "PLN");
        let expected = Money::new(2, "PLN");

        assert_eq!(money + money, expected)
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_currencies_dont_match_when_adding() {
        let money1 = Money::new(1, "PLN");
        let money2 = Money::new(1, "GBP");

        let _expected = money1 + money2;
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_currencies_dont_match_when_subtracting() {
        let money1 = Money::new(1, "PLN");
        let money2 = Money::new(1, "GBP");

        let _expected = money1 - money2;
    }

    #[test]
    fn it_subtracts_money_objects_correctly() {
        let money1 = Money::new(2, "PLN");
        let money2 = Money::new(1, "PLN");
        let expected = Money::new(1, "PLN");

        assert_eq!(money1 - money2, expected)
    }

    #[test]
    fn it_multiplies_money_correctly() {
        let money = Money::new(2, "PLN");
        let expected = Money::new(4, "PLN");

        assert_eq!(money * 2, expected)
    }
}
