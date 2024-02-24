use extism_pdk::*;
use hello_extism_shared::*;

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
        let mut res_item = String::new();
        for part in arg.split(" ") {
            res_item.push_str(&cap(part));
            res_item.push(' ');
        }
        res_item.pop();
        res.push(res_item);
    }
    Ok(ListString { items: res })
}
