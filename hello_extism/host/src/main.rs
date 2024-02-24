use extism::*;
use hello_extism_shared::*;
use std::{env, path::PathBuf};

fn find_dir(target_dir: &str) -> Option<PathBuf> {
    let dir = env::current_dir().unwrap();
    let mut dir = dir.as_path();
    // let dir = dir.as_path();
    while let Some(file_name) = dir.file_name() {
        if file_name == target_dir {
            return Some(dir.to_path_buf());
        }
        dir = dir.parent()?;
    }
    eprintln!("Can not find dir {} in the path", target_dir);
    None
}

fn main() {
    let root_dir = find_dir("rust-experiment").unwrap();
    let guest_file = root_dir.join("target/wasm32-unknown-unknown/debug/hello_extism_guest.wasm");
    if !guest_file.exists() {
        panic!(
            r"guest file not found: {:?}

Hint: cd hello_extism/guest
      cargo build --target wasm32-unknown-unknown
",
            guest_file
        );
    }

    let manifest = Manifest::new([Wasm::file(guest_file)]);

    let mut guest = Plugin::new(manifest, [], true).unwrap();

    {
        let args = AddArgs {
            args: vec![1, 2, 3, 42],
        };
        let res = guest.call::<AddArgs, AddOut>("add", args).unwrap();
        println!(
            "add: {}{}",
            res.result,
            if res.overflow { " (overflow)" } else { "" }
        );
    }
    {
        let args = ListString {
            items: vec![
                "hello world".to_string(),
                "one two three".to_string(),
                "".to_string(),
            ],
        };
        let res = guest
            .call::<ListString, ListString>("capitalize", args)
            .unwrap();
        dbg!(res);
    }
}
