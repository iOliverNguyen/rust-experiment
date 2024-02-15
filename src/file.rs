use crate::TodoList;

use serde_json;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path;

pub fn get_file_path() -> path::PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push("rustexp/todo0/todo.json");
    path
}

pub fn load_from_file(file_path: &path::Path) -> Result<TodoList, String> {
    let file_path_str = file_path.to_str().unwrap();
    let mut file = match fs::File::open(&file_path) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => fs::File::create(file_path)
                .map_err(|err| format!("failed to create file {:?}: {}", file_path_str, err)),
            _ => Err(format!("failed to open file {:?}: {}", file_path_str, err)),
        },
    }?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|err| format!("failed to read file {:?}: {}", file_path_str, err))?;

    if content.trim() == "" {
        return Ok(TodoList::new());
    }

    let todo_list: TodoList = serde_json::from_str(&content).map_err(|err| {
        format!(
            "failed to decode json from file {:?}: {}",
            file_path_str, err
        )
    })?;
    Ok(todo_list)
}

pub fn save_to_file(file_path: &path::Path, todo_list: TodoList) -> Result<(), String> {
    let file_path_str = file_path.to_str().unwrap();

    let todo_json = serde_json::to_string(&todo_list)
        .map_err(|err| format!("failed to encode json: {}", err))?;

    let mut file = fs::File::open(file_path_str)
        .map_err(|err| format!("failed to write to file {:?}: {}", file_path_str, err))?;

    file.set_len(0).unwrap(); // truncate
    file.write_all(&todo_json.as_bytes())
        .map_err(|err| format!("failed to write to file {:?}: {}", file_path_str, err))?;
    Ok(())
}
