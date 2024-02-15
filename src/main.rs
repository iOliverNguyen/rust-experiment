mod cmd;
mod file;
mod todo;

use cmd::*;
use file::*;
use std::process::exit;
use todo::TodoList;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let file_path = get_file_path();
    let mut todo_list = load_from_file(&file_path).unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 0 {
        if todo_list.items.len() == 0 {
            eprintln!("There are no tasks. {}", short_help());
            exit(1);
        } else {
            println!("{}", &todo_list);
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
        "add" => cmd_add(&mut todo_list, &args[1..]),
        "edit" => cmd_edit(&mut todo_list, &args[1..]),
        "del" => cmd_del(&mut todo_list, &args[1..]),
        "delete" => cmd_del(&mut todo_list, &args[1..]),
        "check" => cmd_check(&mut todo_list, &args[1..]),
        "uncheck" => cmd_uncheck(&mut todo_list, &args[1..]),
        _ => {
            eprintln!("Unknow command. {}", short_help());
            exit(1);
        }
    }
    save_to_file(&file_path, todo_list).unwrap();
}

fn short_help() -> String {
    format!(
        r#"Use "{} help" or "{} --help" to see the usage."#,
        APP_NAME, APP_NAME
    )
}

fn print_help() {
    println!(
        r#"
Usage:
  todo                : Show the list of tasks
  todo add 'foo'      : Add a new task at the end
  todo add 2 'foo'    : Add a new task at position 2
  todo edit 1 'bar'   : Edit the task at position 1
  todo del 2 1        : Delete the second and first tasks
  todo del done       : Delete all tasks marked as done
  todo check 1 2      : Mark the tasks at position 1 and 2 as done
  todo uncheck 1 2    : Mark the tasks at position 1 and 2 as not done
"#
    );
}
