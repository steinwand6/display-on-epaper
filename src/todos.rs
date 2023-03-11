use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_todos(task_file_path: &str) -> Vec<String> {
    let mut tasks = Vec::new();
    let task_list = File::open(task_file_path).unwrap();
    let reader = BufReader::new(task_list);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(" TODO ") {
            let line = line.replace(" TODO ", "").replace("*", " ");
            tasks.push(line);
        }
    }
    tasks
}
