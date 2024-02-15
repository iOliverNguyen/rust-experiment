const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub fn short_help() -> String {
    format!(
        r#"Use "{} help" or "{} --help" to see the usage."#,
        APP_NAME, APP_NAME
    )
}

pub fn print_help() {
    let app = APP_NAME;
    println!(
        r#"
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
"#,
        app, app, app, app, app, app, app, app, app, app, app, app
    );
}
