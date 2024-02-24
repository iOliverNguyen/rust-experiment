use bincode;
use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromBytes)]
#[encoding(Json)]
struct AddArgs {
    args: Vec<i64>,
}

#[derive(Serialize, Deserialize, ToBytes)]
#[encoding(Json)]
struct AddOut {
    result: i64,
    overflow: bool,
}

#[derive(Serialize, Deserialize)]
struct ListString {
    items: Vec<String>,
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

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    Ok(format!("Hello {}!", name))
}

#[plugin_fn]
pub fn add(args: AddArgs) -> FnResult<AddOut> {
    let mut result: i64 = 0;
    for arg in args.args.iter() {
        match result.checked_add(*arg) {
            Some(r) => result = r,
            None => {
                return Ok(AddOut {
                    result,
                    overflow: false,
                })
            }
        }
    }
    Ok(AddOut {
        result,
        overflow: false,
    })
}

#[plugin_fn]
pub fn capitalize(args: ListString) -> FnResult<ListString> {
    fn cap(s: &str) -> String {
        let mut out = String::new();
        let mut prev = ' ';
        for ch in s.chars() {
            if prev == ' ' {
                for ch in ch.to_uppercase() {
                    out.push(ch);
                }
            } else {
                out.push(ch);
            };
            prev = ch;
        }
        out
    }

    let mut res = vec![];
    for arg in args.items.iter() {
        for part in arg.split(" ") {
            res.push(cap(part));
        }
    }
    Ok(ListString { items: res })
}
