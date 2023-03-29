use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct Dbid(Uuid);
impl Dbid {
    pub fn new() -> Dbid {
        Uuid::new_v4().into()
    }
    pub fn nil() -> Dbid {
        Self(Uuid::nil())
    }
}

impl Default for Dbid {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Dbid {
    type Err = uuid::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Dbid(Uuid::parse_str(id)?))
    }
}
