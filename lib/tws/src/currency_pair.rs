#![feature(slice_patterns)]
use failure::Fail;
use penny::Currency;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Fail, Clone, PartialEq, Eq)]
pub enum ParseCurrencyPairError {
    #[fail(
        display = "cannot parse currency pair from string, currency pair fomat error, example:'EUR.USD' or 'EUR/USD'"
    )]
    ParseFormatError,
    #[fail(display = "cannot parse base currency from string")]
    ParseBaseCurrencyError,
    #[fail(display = "cannot parse seconary currency from string")]
    ParseSeconaryCurrencyError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrencyPair {
    base: Currency,
    seconary: Currency,
}

impl CurrencyPair {
    pub fn new(base: Currency, seconary: Currency) -> Self {
        CurrencyPair { base, seconary }
    }

    pub fn base(&self) -> Currency {
        self.base
    }

    pub fn seconary(&self) -> Currency {
        self.seconary
    }
}

impl FromStr for CurrencyPair {
    type Err = ParseCurrencyPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<&str> = s.trim().split(|c| c == '.' || c == '/').collect();
        match pair[..] {
            [base, seconary] => Ok(CurrencyPair::new(
                base.parse::<Currency>()
                    .map_err(|_| ParseCurrencyPairError::ParseBaseCurrencyError)?,
                seconary
                    .parse::<Currency>()
                    .map_err(|_| ParseCurrencyPairError::ParseSeconaryCurrencyError)?,
            )),
            _ => Err(ParseCurrencyPairError::ParseFormatError),
        }
    }
}
