
use colored::*;
use serde_derive::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
#[derive(Deserialize, Clone)]
struct Task {
    name: String,
    working_dir: String,
    binary: String,
    args: Vec<String>,
    env: Option<HashMap<String, String>>,
    color: String,
    group: u32,
}

pub fn exec_stream<P: AsRef<std::path::Path>>(
    name: &str,
    binary: P,
    cwd: &str,
    color: String,
    args: Vec<String>,
    env: HashMap<String, String>,
    tx: mpsc::Sender<u32>,
) {
    let mut cmd = Command::new(binary.as_ref())
        .current_dir(cwd)
        .args(&args)
        .envs(env)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    tx.send(cmd.id()).unwrap();

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
                println!("{:15}| {}", name.color(color.clone()), line.color(c));
            }
        }
    }

    cmd.wait().unwrap();
}

fn get_tasks_for_group(group: u32, tasks: Vec<Task>) -> Vec<Task> {
    tasks
        .into_iter()
        .filter(|task| task.group == group)
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let tasks: Vec<Task> = serde_json::from_str(contents.as_str()).unwrap();
    let mut current_group = 0;
    let (_tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    while current_group < 5 {
        let group_tasks = get_tasks_for_group(current_group, tasks.clone());
        println!(
            "devconsole     | Starting {} tasks from group {}",
            group_tasks.len(),
            current_group
        );
        for task in group_tasks {
            let tx_clone = mpsc::Sender::clone(&_tx);
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
                    tx_clone,
                )
            }));
        }
        thread::sleep(Duration::from_secs(5));
        current_group += 1;
    }

    let pids: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    let pids_clone = Arc::clone(&pids);

    thread::spawn(move || {
        for pid in rx {
            //println!("storing PID: {}", pid);
            let mut pids = pids.lock().unwrap();
            pids.deref_mut().push(pid);
        }
    });

    ctrlc::set_handler(move || {
        let pids = pids_clone.lock().unwrap();
        let pids = pids.deref();
        for _pid in pids {
            //println!("terminating PID: {}", pid);
        }
    })
    .unwrap();

    for handle in handles {
        handle.join().unwrap();
    }
}
