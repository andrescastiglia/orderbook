use serde::Serialize;
use std::str::FromStr;

/// Used to determine the type of the operation.
#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "DELETE")]
    Delete,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATE" => Ok(Self::Create),
            "DELETE" => Ok(Self::Delete),
            s => Err(format!("Invalid operation {}", s)),
        }
    }
}
