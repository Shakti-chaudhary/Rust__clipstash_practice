use crate::data::Dbid;
use derive_more::Constructer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Constructer, Deserialize, Serialize)]
pub struct ClipId(Dbid);

impl ClipId {
    pub fn into_inner(self) -> Self {
        Self(id)
    }
}

impl From<Dbid> for ClipId {
    fn from(id: Dbid) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(Dbid::nil())
    }
}
