use crate::{help::*, todo::*};

pub fn cmd_add(todo_list: &mut TodoList, args: &[String]) -> Result<(), String> {
    if args.len() == 0 {
        return Err(format!("Invalid arguments. {}", short_help()));
    };
    let index = args[0].parse::<usize>();
    match index {
        Err(_) => {
            let task = parse_task(args)?;
            map_result(todo_list.add(None, task))?;
            println!("added 1 task");
        }
        Ok(index) => {
            let task = parse_task(&args[1..])?;
            map_result(todo_list.add(Some(Position::AtIndex(index - 1)), task))?;
            println!("added 1 task");
        }
    };
    Ok(())
}

pub fn cmd_edit(todo_list: &mut TodoList, args: &[String]) -> Result<(), String> {
    if args.len() == 0 {
        return Err(format!("Invalid arguments. {}", short_help()));
    }
    let index = args[0].parse::<usize>();
    match index {
        Err(_) => {
            let task = parse_task(args)?;
            let len = todo_list.items.len();
            let index = if len > 0 { len } else { 1 };
            map_result(todo_list.edit(Some(Position::AtIndex(index - 1)), task))?;
            println!("edited 1 task");
        }
        Ok(index) => {
            let task = parse_task(&args[1..])?;
            map_result(todo_list.edit(Some(Position::AtIndex(index - 1)), task))?;
            println!("edited 1 task");
        }
    };
    Ok(())
}

pub fn cmd_del(todo_list: &mut TodoList, args: &[String]) -> Result<(), String> {
    todo!()
}

pub fn cmd_check(todo_list: &mut TodoList, args: &[String]) -> Result<(), String> {
    todo!()
}

pub fn cmd_uncheck(todo_list: &mut TodoList, args: &[String]) -> Result<(), String> {
    todo!()
}

fn map_result(res: Result<ActionResult, Error>) -> Result<(), String> {
    match res {
        Ok(res) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

fn parse_task(args: &[String]) -> Result<Task, String> {
    if args.len() == 0 {
        Err(format!("Missing task title. {}", short_help()))
    } else {
        let title = args[0..].join(" ");
        Ok(Task::new(&title))
    }
}
