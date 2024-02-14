use rand;

#[derive(PartialEq, Debug)]
enum ActionResult {
    Insert(TaskId),
    Update(TaskId),
    Delete(TaskId),
}

#[derive(Debug)]
enum Error {
    NotFound,
    Validation(String),
}

enum Position {
    AtIndex(usize),
    ById(TaskId),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct TaskId(u64);

#[derive(Clone)]
struct Task {
    pub id: TaskId,
    pub title: String,
    pub done: bool,
}

struct TodoList {
    pub items: Vec<Task>,
}

impl TaskId {
    pub fn new() -> Self {
        TaskId(rand::random::<u64>())
    }
}

impl Task {
    // generate a new task with random id
    pub fn new(title: String) -> Self {
        Task {
            id: TaskId::new(),
            title,
            done: false,
        }
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { items: vec![] }
    }

    fn validate(task: Task) -> Result<Task, Error> {
        // validate
        let title = task.title.trim();
        if title == "" {
            return Err(Error::Validation(String::from("title is empty")));
        }

        // generate id
        let task = match task.id {
            TaskId(0) => Task {
                id: TaskId::new(),
                ..task
            },
            _ => task,
        };
        Ok(task)
    }

    // add the task to the list (if the task.id already exist in the list, update it instead)
    pub fn add(&mut self, pos: Option<Position>, task: Task) -> Result<ActionResult, Error> {
        // generate id if empty
        let task = Self::validate(task);
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
        Ok(ActionResult::Insert(id))
    }

    pub fn edit(&mut self, pos: Option<Position>, task: Task) -> Result<ActionResult, Error> {
        // validate
        let task = Self::validate(task);
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
            return Ok(ActionResult::Update(id));
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
                        Ok(ActionResult::Update(prev.id))
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
                        Ok(ActionResult::Delete(id))
                    }
                }
            }
            Some(Position::ById(id)) => {
                let index = self.items.iter().position(|x| x.id == id);
                match index {
                    None => Err(Error::NotFound),
                    Some(index) => {
                        self.items.splice(index..=index, vec![]);
                        Ok(ActionResult::Delete(id))
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

        let task = Task::new(String::from("A"));
        let task_id = task.id;
        let res = list.add(None, task);
        assert_eq!(ActionResult::Insert(task_id), res.unwrap());
        assert_eq!(vec!["A"], get_tasks(&list));

        let task = Task::new(String::from("B"));
        let task_id = task.id;
        let res = list.add(Some(Position::AtIndex(0)), task);
        assert_eq!(ActionResult::Insert(task_id), res.unwrap());
        assert_eq!(vec!["B", "A"], get_tasks(&list));

        let task = Task::new(String::from("C"));
        let task_id = task.id;
        let res = list.add(Some(Position::AtIndex(1)), task);
        assert_eq!(ActionResult::Insert(task_id), res.unwrap());
        assert_eq!(vec!["B", "C", "A"], get_tasks(&list));

        let task = Task::new(String::from("D"));
        let task_id = task.id;
        let res = list.add(None, task);
        assert_eq!(ActionResult::Insert(task_id), res.unwrap());
        assert_eq!(vec!["B", "C", "A", "D"], get_tasks(&list));

        let task = Task::new(String::from("E"));
        let task_id = task.id;
        let pos = Some(Position::ById(list.items[1].id));
        let res = list.add(pos, task);
        assert_eq!(ActionResult::Insert(task_id), res.unwrap());
        assert_eq!(vec!["B", "C", "E", "A", "D"], get_tasks(&list));
    }
}
