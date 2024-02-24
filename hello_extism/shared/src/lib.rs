use extism_convert::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct AddArgs {
    pub args: Vec<i64>,
}

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct AddOut {
    pub result: i64,
    pub overflow: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListString {
    pub items: Vec<String>,
}

impl FromBytesOwned for ListString {
    fn from_bytes_owned(data: &[u8]) -> Result<Self, Error> {
        let out: Self = bincode::deserialize(data)?;
        Ok(out)
    }
}

impl<'a> ToBytes<'a> for ListString {
    type Bytes = Vec<u8>;

    fn to_bytes(&self) -> Result<Self::Bytes, Error> {
        let out = bincode::serialize(self)?;
        Ok(out)
    }
}
