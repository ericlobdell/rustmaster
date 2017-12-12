use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
enum Currency { USD, EUR }

#[derive(Debug)]
struct CurrencyError {
    description: String
}

impl Display for CurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CurrencyError: {}", self.description)
    }
}

impl Error for CurrencyError {
    fn description(&self) -> &str {
        "CurrencyError"
    }
}

impl Currency {
    fn new(currency: &str) -> Result<Self,CurrencyError> {
        match currency {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => Err(CurrencyError {
                description: format!("{} is not a valid currency", currency)
            })
        }
    }
}

#[derive(Debug)]
struct Money {
    currency: Currency,
    amount: u64
}

#[derive(Debug)]
struct MoneyError {
    cause: CurrencyError
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoneyError due to {}", self.cause)
    }
}

impl Error for MoneyError {
    fn description(&self) -> &str {
        "MoneyError"
    }
    fn cause(&self) -> Option<&Error> {
        Some(&self.cause)
    }
}

impl Money {
    fn new(currency: &str, amount: u64) -> Result<Self,MoneyError> {
        let currency = match Currency::new(currency) {
            Ok(c) => c,
            Err(e) => return Err(MoneyError { 
                cause: e 
                })
        };

        Ok(Money {
            currency: currency,
            amount: amount
        })
    }
}

fn main() {
    let m1 = Money::new("EUR", 12345);
    let m2 = Money::new("FIR", 45678);

    let err = m2.unwrap_err();
    println!("{}", err);

    // println!("M1 is {:?}", m1);
    // println!("M2 is {:?}", m2);
}