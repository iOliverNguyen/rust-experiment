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
    if args.len() == 0 {
        return Err(format!("Invalid arguments. {}", short_help()));
    }
    let indexes = parse_args_as_indexes(todo_list.items.len(), args)?;
    let positions: Vec<_> = indexes
        .iter()
        .map(|idx| Some(Position::AtIndex(*idx - 1)))
        .collect();

    let results = todo_list.delete(&positions);
    let mut results: Vec<_> = results
        .iter()
        .filter_map(|x| match x {
            Ok(ActionResult::Deleted(TaskId(id))) => Some(*id as usize),
            _ => None,
        })
        .collect();
    results.sort();

    let mut count: usize = 0;
    let mut last: usize = 0;
    for id in results {
        if id != last {
            count += 1
        }
        last = id
    }
    println!("deleted {} tasks", count);
    Ok(())
}

pub fn cmd_check(todo_list: &mut TodoList, args: &[String]) -> Result<(), String> {
    if args.len() == 0 {
        return Err(format!("Invalid arguments. {}", short_help()));
    }
    let mut count: usize = 0;
    let indexes = parse_args_as_indexes(todo_list.items.len(), args)?;
    for index in indexes {
        let task = &todo_list.items[index - 1];
        if !task.done {
            count += 1;
            let mut task = task.clone();
            task.done = true;
            map_result(todo_list.edit(Some(Position::AtIndex(index - 1)), task))?;
        }
    }
    println!(
        "marked {} task{} as done",
        count,
        if count != 1 { "s" } else { "" }
    );
    Ok(())
}

pub fn cmd_uncheck(todo_list: &mut TodoList, args: &[String]) -> Result<(), String> {
    if args.len() == 0 {
        return Err(format!("Invalid arguments. {}", short_help()));
    }
    let mut count: usize = 0;
    let indexes = parse_args_as_indexes(todo_list.items.len(), args)?;
    for index in indexes {
        let task = &todo_list.items[index - 1];
        if task.done {
            count += 1;
            let mut task = task.clone();
            task.done = false;
            map_result(todo_list.edit(Some(Position::AtIndex(index - 1)), task))?;
        }
    }
    println!(
        "marked {} task{} as not done",
        count,
        if count != 1 { "s" } else { "" }
    );
    Ok(())
}

fn map_result(res: Result<ActionResult, Error>) -> Result<(), String> {
    match res {
        Ok(_) => Ok(()),
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

fn parse_args_as_indexes(max: usize, args: &[String]) -> Result<Vec<usize>, String> {
    let mut indexes = vec![];
    for arg in args {
        let index = arg
            .parse::<usize>()
            .map_err(|_| format!("Invalid arguments. {}", short_help()))?;
        if index <= 0 || index > max {
            return Err(format!("index {} is out of range (max {})", index, max));
        }
        indexes.push(index);
    }
    Ok(indexes)
}
