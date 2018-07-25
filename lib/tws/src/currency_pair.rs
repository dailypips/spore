use penny::Currency;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCurrencyPairError {
    kind: CurrencyPairErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CurrencyPairErrorKind {
    Base,
    Seconary,
}

impl ParseCurrencyPairError {
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        match self.kind {
            CurrencyPairErrorKind::Base => "cannot parse base currency from string",
            CurrencyPairErrorKind::Seconary => "cannot parse seconary currency from string",
        }
    }
}

impl fmt::Display for ParseCurrencyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
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
        let base = pair[0]
            .parse::<Currency>()
            .map_err(|_| ParseCurrencyPairError {
                kind: CurrencyPairErrorKind::Base,
            })?;
        let seconary = pair[1]
            .parse::<Currency>()
            .map_err(|_| ParseCurrencyPairError {
                kind: CurrencyPairErrorKind::Seconary,
            })?;

        Ok(CurrencyPair::new(base, seconary))
    }
}
