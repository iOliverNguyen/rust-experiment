# A simple todo app

_This app is for getting familiar with writing code in Rust._

## Goal

Implement a simple cli todo app.

- [x] Show list of todos
- [x] Create new tasks
- [x] Edit and delete tasks
- [x] Mark as done or not
- [x] Save to file

## Usage

```
$ todo0 --help

Usage:
  {}                : Show the list of tasks
  {} list           : Show the list of tasks
  {} reset          : Delete all tasks
  {} add hello foo  : Add a new task "hello foo" at the end
  {} add 2 foo bar  : Add a new task "foo bar" at position 2
  {} edit bar       : Edit the last task, set to "bar"
  {} edit 1 bar     : Edit the task at position 1, set to "bar"
  {} del 2 1        : Delete the second and first tasks
  {} del last       : Delete the last task
  {} del done       : Delete all tasks marked as done
  {} check 1 2      : Mark the tasks at position 1 and 2 as done
  {} uncheck 1 2    : Mark the tasks at position 1 and 2 as not done
```

## License

MIT
