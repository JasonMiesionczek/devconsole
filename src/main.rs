use colored::*;
use serde_derive::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

#[derive(Deserialize)]
struct Task {
    name: String,
    working_dir: String,
    binary: String,
    args: Vec<String>,
    env: Option<HashMap<String, String>>,
    color: String,
}

pub fn exec_stream<P: AsRef<Path>>(
    name: &str,
    binary: P,
    cwd: &str,
    color: String,
    args: Vec<String>,
    env: HashMap<String, String>,
) {
    let mut cmd = Command::new(binary.as_ref())
        .current_dir(cwd)
        .args(&args)
        .envs(env)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);

        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                let mut c = color.clone();
                if line.contains("ERROR") {
                    c = String::from("red");
                } else if line.contains("WARN") {
                    c = String::from("yellow");
                }
                println!("{}|\t{}", name.color(color.clone()), line.color(c));
            }
        }
    }

    cmd.wait().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let tasks: Vec<Task> = serde_json::from_str(contents.as_str()).unwrap();

    let mut handles = Vec::new();
    for task in tasks {
        handles.push(thread::spawn(move || {
            let args = task.args.into_iter().collect();
            let env = match task.env {
                Some(e) => e,
                None => HashMap::new(),
            };
            exec_stream(
                task.name.as_str(),
                task.binary.as_str(),
                task.working_dir.as_str(),
                task.color,
                args,
                env,
            )
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
