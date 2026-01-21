#![cfg(windows)]

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

pub type Task = Box<dyn FnOnce() + Send + 'static>;

static QUEUE: OnceLock<Mutex<VecDeque<Task>>> = OnceLock::new();

fn queue() -> &'static Mutex<VecDeque<Task>> {
    QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

pub fn enqueue(task: Task) {
    let mut q = queue().lock().expect("queue lock");
    q.push_back(task);
}

pub fn pump(max_tasks: usize) {
    let mut ran = 0usize;
    loop {
        let task = {
            let mut q = queue().lock().expect("queue lock");
            q.pop_front()
        };
        let Some(task) = task else { break };
        task();
        ran += 1;
        if max_tasks != 0 && ran >= max_tasks {
            break;
        }
    }
}
