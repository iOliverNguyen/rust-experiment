use extism_convert::*;
use serde::*;

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Msgpack)]
pub struct AddArgs {
    pub args: Vec<i64>,
}

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Msgpack)]
pub struct AddOut {
    pub result: i64,
    pub overflow: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct ListStringV2 {
        pub items: Vec<String>,
        pub new_str: String,
        pub new_int: i32,
    }

    #[test]
    fn test_bincode() {
        let input = ListString {
            items: vec!["hello".to_string(), "world".to_string()],
        };
        let bytes = bincode::serialize(&input).unwrap();
        let output = bincode::deserialize(&bytes).unwrap();
        assert_eq!(input, output);
    }

    #[test] // ERROR
    fn test_bincode_v2() {
        let input = ListString {
            items: vec!["hello".to_string(), "world".to_string()],
        };
        let bytes = bincode::serialize(&input).unwrap();
        let output: ListStringV2 = bincode::deserialize(&bytes).unwrap();
        assert_eq!(output.items, input.items);
    }
}
