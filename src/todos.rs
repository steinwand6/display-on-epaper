use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use image::DynamicImage;
use regex::Regex;

use crate::utils;
use crate::FontSetting;

fn get_todotasks(task_file_path: &str) -> Vec<String> {
    let mut tasks = Vec::new();
    let task_list = File::open(task_file_path).unwrap();
    let reader = BufReader::new(task_list);

    let re_first_level = Regex::new(r"^\* ").unwrap();
    let re_maintask = Regex::new(r"^\*{2} TODO").unwrap();
    let re_subtask_todo = Regex::new(r"^\*\*\*+ TODO").unwrap();
    let re_subtask_done = Regex::new(r"^\*\*\*+ DONE").unwrap();
    let mut in_project = false;
    for line in reader.lines() {
        let mut line = line.unwrap();

        if re_first_level.is_match(&line) {
            in_project = line.contains("Projects");
        }
        if !in_project {
            continue;
        }

        if re_maintask.is_match(&line) {
            line = line.replace("** TODO ", "- ");
            tasks.push(line);
        } else if re_subtask_todo.is_match(&line) {
            line = line.replace(" TODO ", "[ ]").replace("*", " ");
            tasks.push(line);
        } else if re_subtask_done.is_match(&line) {
            line = line.replace(" DONE ", "[x]").replace("*", " ");
            tasks.push(line);
        }
    }
    tasks
}

pub fn draw_todotasks(
    image: &mut DynamicImage,
    task_file_path: &str,
    font_setting: &FontSetting,
    x: i32,
    y: i32,
) {
    let todotasks = get_todotasks(task_file_path);
    utils::draw_texts_on_image(image, x, y, todotasks, font_setting);
}
