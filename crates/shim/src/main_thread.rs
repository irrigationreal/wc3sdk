#![cfg(windows)]

use std::collections::VecDeque;
use std::sync::{mpsc, Mutex, OnceLock};

pub type Task = Box<dyn FnOnce() + Send + 'static>;

static QUEUE: OnceLock<Mutex<VecDeque<Task>>> = OnceLock::new();
static MAIN_THREAD_ID: OnceLock<u32> = OnceLock::new();

fn queue() -> &'static Mutex<VecDeque<Task>> {
    QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

pub fn enqueue(task: Task) {
    let mut q = queue().lock().expect("queue lock");
    q.push_back(task);
}

pub fn mark_main_thread() {
    let _ = MAIN_THREAD_ID.set(unsafe { GetCurrentThreadId() });
}

pub fn is_main_thread() -> bool {
    let Some(id) = MAIN_THREAD_ID.get() else { return false };
    unsafe { GetCurrentThreadId() == *id }
}

pub fn run_on_main<F, T>(f: F) -> Result<T, anyhow::Error>
where
    F: FnOnce() -> Result<T, anyhow::Error> + Send + 'static,
    T: Send + 'static,
{
    if is_main_thread() {
        return f();
    }

    let (tx, rx) = mpsc::channel();
    enqueue(Box::new(move || {
        let result = f();
        let _ = tx.send(result);
    }));
    rx.recv().unwrap_or_else(|err| Err(anyhow::anyhow!(err)))
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

extern "system" {
    fn GetCurrentThreadId() -> u32;
}
