use std::process::{Command};
use std::env;
use std::fs;
use yaml_rust2::{YamlLoader};
use chrono::prelude::*;

fn main() {
    start();
}

fn start() {
    let args: Vec<String> = env::args().collect();
    let file_path = (&args[1]).clone();

    let raw_config = fs::read_to_string("config.yml")
        .expect("Can't read file config.yml!");

    let config = YamlLoader::load_from_str(raw_config.as_str())
        .expect("Can't load config from config.yml!");

    let config = config[0].clone();

    let mut default = None;

    let time = Local::now();
    let weekday = format!("{}", time.weekday());
    let naive_time = time.naive_local().time();

    for cfg in config.into_iter() {
        if cfg["default"].as_bool().or(Some(false)).unwrap() {
            default = Some(
                cfg["path"].clone().into_string().expect("No path in some cfg")
            );
        } else {
            let begin = NaiveTime::parse_from_str(
                cfg["time"][0][weekday.as_str()][0]
                    .clone()
                    .as_str()
                    .unwrap(),
                "%H:%M:%S"
            ).unwrap();
            let end = NaiveTime::parse_from_str(
                cfg["time"][0][weekday.as_str()][1]
                    .clone()
                    .as_str()
                    .unwrap(),
                "%H:%M:%S"
            ).unwrap();

            if naive_time >= begin && naive_time <= end {
                run(&cfg["path"].clone().into_string().unwrap(), &file_path);
                break;
            }
        }
    }

    if default != None {
        run(&default.unwrap(), &file_path);
    }
}

fn run (process_path: &String, file_path: &String) {
    Command::new(process_path)
        .arg(file_path)
        .output()
        .expect("Fail to execute the process!");
}