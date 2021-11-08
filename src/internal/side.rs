use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "BUY")]
    Buy,
}

impl FromStr for Side {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SELL" => Ok(Self::Sell),
            "BUY" => Ok(Self::Buy),
            s => Err(format!("Invalid side {}", s)),
        }
    }
}
