mod cmd;
mod file;
mod help;
mod todo;

use cmd::*;
use file::*;
use help::*;
use std::{fs, process::exit};
use todo::TodoList;

use crate::todo::FormatOptions;

fn main() {
    let file_path = get_file_path();
    let mut todo_list = load_from_file(&file_path)
        .or::<()>(Ok(TodoList::new()))
        .unwrap();

    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];
    let is_cmd_list = args.len() > 0 && args[0] == "list";
    if args.len() == 0 || is_cmd_list {
        if todo_list.items.len() == 0 {
            eprintln!(
                "There are no tasks. {}",
                if is_cmd_list {
                    String::new()
                } else {
                    short_help()
                }
            );
            exit(1);
        } else {
            println!("{}", todo_list.format(FormatOptions { use_color: true }));
            exit(0);
        }
    }
    match args[0].as_str() {
        "help" => {
            print_help();
            exit(0);
        }
        "--help" => {
            print_help();
            exit(0);
        }
        "reset" => {
            if let Ok(_) = fs::remove_file(&file_path) {
                eprintln!("deleted all tasks");
            }
            exit(0);
        }
        "add" => cmd_add(&mut todo_list, &args[1..]).unwrap(),
        "edit" => cmd_edit(&mut todo_list, &args[1..]).unwrap(),
        "del" => cmd_del(&mut todo_list, &args[1..]).unwrap(),
        "delete" => cmd_del(&mut todo_list, &args[1..]).unwrap(),
        "check" => cmd_check(&mut todo_list, &args[1..]).unwrap(),
        "uncheck" => cmd_uncheck(&mut todo_list, &args[1..]).unwrap(),
        _ => {
            eprintln!("Unknow command. {}", short_help());
            exit(1);
        }
    }
    save_to_file(&file_path, todo_list).unwrap();
}
