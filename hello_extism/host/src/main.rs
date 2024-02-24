use extism::*;
use hello_extism_shared::*;
use std::{env, path::PathBuf};

fn find_root_dir() -> Option<PathBuf> {
    let dir = env::current_dir().unwrap();
    let mut dir = dir.as_path();
    while let Some(_) = dir.file_name() {
        let git_dir = dir.join(".git");
        if git_dir.exists() {
            return Some(dir.to_path_buf());
        }
        dir = dir.parent()?;
    }
    eprintln!("Can not find root directory with .git");
    None
}

fn main() {
    let root_dir = find_root_dir().unwrap();
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
