use rand;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};

#[derive(PartialEq, Debug)]
pub enum ActionResult {
    Inserted(TaskId),
    Updated(TaskId),
    Deleted(TaskId),
}

#[derive(PartialEq, Debug)]
pub enum Error {
    NotFound,
    Validation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => f.write_str("task not found"),
            Self::Validation(s) => f.write_str(s),
        }
    }
}

pub enum Position {
    AtIndex(usize),
    ById(TaskId),
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TaskId(u64);

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub items: Vec<Task>,
}

impl TaskId {
    pub fn new() -> Self {
        TaskId(rand::random::<u64>())
    }
}

impl Task {
    // generate a new task with random id
    pub fn new(title: &str) -> Self {
        Task {
            id: TaskId::new(),
            title: title.to_string(),
            done: false,
        }
    }

    pub fn validate(task: &Self) -> Result<Self, Error> {
        // validate
        let title = task.title.trim();
        if title == "" {
            return Err(Error::Validation(String::from("title is empty")));
        }

        // generate id
        let task = match task.id {
            TaskId(0) => Task {
                id: TaskId::new(),
                ..task.clone()
            },
            _ => task.clone(),
        };
        Ok(task)
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_str = self.format();
        f.write_str(&format_str).unwrap();
        Ok(())
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { items: vec![] }
    }

    pub fn format(&self) -> String {
        if self.items.len() == 0 {
            return String::from("no items");
        }

        let mut buf = String::new();
        self.items.iter().enumerate().for_each(|(idx, task)| {
            if idx > 0 {
                buf.write_str("\n").unwrap();
            }
            buf.write_fmt(format_args!("{:>3}. {}", idx + 1, task.title))
                .unwrap();
        });
        buf
    }

    // add the task to the list (if the task.id already exist in the list, update it instead)
    pub fn add(&mut self, pos: Option<Position>, task: Task) -> Result<ActionResult, Error> {
        // generate id if empty
        let task = Task::validate(&task);
        if let Err(err) = task {
            return Err(err);
        }
        let task = task.unwrap();

        // find existing task by id and update the task
        if let Ok(result) = self.edit(None, task.clone()) {
            return Ok(result);
        }

        // add the task at the requested position
        let id = task.id;
        match pos {
            None => self.items.insert(self.items.len(), task),
            Some(Position::AtIndex(index)) => self.items.insert(index, task),
            Some(Position::ById(anchor_id)) => {
                let index = self.items.iter().position(|x| x.id == anchor_id);
                match index {
                    Some(index) => self.items.insert(index + 1, task),
                    None => self.items.insert(self.items.len(), task),
                }
            }
        };
        Ok(ActionResult::Inserted(id))
    }

    pub fn edit(&mut self, pos: Option<Position>, task: Task) -> Result<ActionResult, Error> {
        // validate
        let task = Task::validate(&task);
        if let Err(err) = task {
            return Err(err);
        }
        let task = task.unwrap();

        // find existing task by id and update the task
        let index = self.items.iter().position(|x| x.id == task.id);
        if let Some(index) = index {
            let id = task.id;
            if let Some(item) = self.items.get_mut(index) {
                *item = task;
            };
            return Ok(ActionResult::Updated(id));
        };

        // update the task
        match pos {
            None => Err(Error::NotFound),
            Some(Position::ById(id)) => self.edit(None, Task { id, ..task }),
            Some(Position::AtIndex(index)) => {
                let prev = self.items.get_mut(index);
                match prev {
                    None => Err(Error::NotFound),
                    Some(prev) => {
                        *prev = Task {
                            id: prev.id,
                            ..task
                        };
                        Ok(ActionResult::Updated(prev.id))
                    }
                }
            }
        }
    }

    pub fn delete(&mut self, pos: Option<Position>) -> Result<ActionResult, Error> {
        match pos {
            None => Err(Error::Validation(String::from("invalid position"))),
            Some(Position::AtIndex(index)) => {
                if index >= self.items.len() {
                    return Err(Error::NotFound);
                }
                let task = self.items.get(index);
                match task {
                    None => Err(Error::NotFound),
                    Some(task) => {
                        let id = task.id;
                        self.items.splice(index..=index, vec![]);
                        Ok(ActionResult::Deleted(id))
                    }
                }
            }
            Some(Position::ById(id)) => {
                let index = self.items.iter().position(|x| x.id == id);
                match index {
                    None => Err(Error::NotFound),
                    Some(index) => {
                        self.items.splice(index..=index, vec![]);
                        Ok(ActionResult::Deleted(id))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tasks<'a>(list: &'a TodoList) -> Vec<&'a str> {
        list.items.iter().map(|x| x.title.as_str()).collect()
    }

    #[test]
    fn add_task() {
        let mut list = TodoList::new();

        let task = Task::new("A");
        let task_id = task.id;
        let res = list.add(None, task);
        assert_eq!(ActionResult::Inserted(task_id), res.unwrap());
        assert_eq!(get_tasks(&list), vec!["A"],);

        let task = Task::new("B");
        let task_id = task.id;
        let res = list.add(Some(Position::AtIndex(0)), task);
        assert_eq!(ActionResult::Inserted(task_id), res.unwrap());
        assert_eq!(get_tasks(&list), vec!["B", "A"],);

        let task = Task::new("C");
        let task_id = task.id;
        let res = list.add(Some(Position::AtIndex(1)), task);
        assert_eq!(ActionResult::Inserted(task_id), res.unwrap());
        assert_eq!(get_tasks(&list), vec!["B", "C", "A"],);

        let task = Task::new("D");
        let task_id = task.id;
        let res = list.add(None, task);
        assert_eq!(ActionResult::Inserted(task_id), res.unwrap());
        assert_eq!(get_tasks(&list), vec!["B", "C", "A", "D"],);

        let task = Task::new("E");
        let task_id = task.id;
        let pos = Some(Position::ById(list.items[1].id));
        let res = list.add(pos, task);
        assert_eq!(ActionResult::Inserted(task_id), res.unwrap());
        assert_eq!(get_tasks(&list), vec!["B", "C", "E", "A", "D"],);
    }

    #[test]
    fn edit_task() {
        let mut list = TodoList::new();
        list.add(None, Task::new("A")).unwrap();
        list.add(None, Task::new("B")).unwrap();
        list.add(None, Task::new("C")).unwrap();
        assert_eq!(get_tasks(&list), vec!["A", "B", "C"],);

        // update non existent item
        let task = Task::new("not exist");
        let res = list.edit(None, task);
        assert_eq!(res.unwrap_err(), Error::NotFound);

        // update by task id
        let task_id = list.items[1].id;
        let task = Task {
            id: task_id,
            title: String::from("B0"),
            done: true,
        };
        let res = list.edit(None, task);
        assert_eq!(res.unwrap(), ActionResult::Updated(task_id));
        assert_eq!(get_tasks(&list), vec!["A", "B0", "C"]);
        assert_eq!(
            list.items[1],
            Task {
                id: task_id,
                title: String::from("B0"),
                done: true
            }
        );

        // update by index
        let task_id = list.items[2].id;
        let mut task = Task::new("C0");
        task.done = true;

        let res = list.edit(Some(Position::AtIndex(2)), task);
        assert_eq!(res.unwrap(), ActionResult::Updated(task_id));
        assert_eq!(get_tasks(&list), vec!["A", "B0", "C0"]);
        assert_eq!(
            list.items[2],
            Task {
                id: task_id,
                title: String::from("C0"),
                done: true
            }
        );
    }

    #[test]
    fn delete_task() {
        let mut list = TodoList::new();
        list.add(None, Task::new("A")).unwrap();
        list.add(None, Task::new("B")).unwrap();
        list.add(None, Task::new("C")).unwrap();
        list.add(None, Task::new("D")).unwrap();
        assert_eq!(get_tasks(&list), vec!["A", "B", "C", "D"],);

        // delete by id
        let task_id = list.items[1].id;
        let res = list.delete(Some(Position::ById(task_id)));
        assert_eq!(res.unwrap(), ActionResult::Deleted(task_id));
        assert_eq!(get_tasks(&list), vec!["A", "C", "D"]);

        // delete by index
        let task_id = list.items[1].id;
        let res = list.delete(Some(Position::AtIndex(1)));
        assert_eq!(res.unwrap(), ActionResult::Deleted(task_id));
        assert_eq!(get_tasks(&list), vec!["A", "D"]);
    }
}
