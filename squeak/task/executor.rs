use super::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;
use crate::pfinally::*;
pub struct Executor {
    tasks: Arc<BTreeMap<TaskId, Task>>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}
pub struct Spawner{
    tasks: Arc<BTreeMap<TaskId, Task>>,
    task_queue: Arc<ArrayQueue<TaskId>>,
}
impl Spawner{
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }
}
impl core::panic::PanicSafe for Executor{

}
impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: Arc::new(BTreeMap::new()),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }
    pub fn new_spawner(&mut self) -> Spawner{
        Spawner {
            tasks: Arc::clone(self.tasks),
            task_queue: Arc::clone(self.task_queue),
        }
    }
        pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Ok(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match (match (match core::panic::catch_unwind(||{
                task.poll(&mut context)
        }) {
            Ok(v) => Some(v)
            Err(perr) => {
                match core::panic::catch_unwind(||(Self {
                    tasks,
                    task_queue,
                    waker_cache
                }).run_ready_tasks()){
                    Ok(_) => {
                        core::panic::resume_unwind(perr);
                        None
                    },
                    Err(perr2) => {
                        Some(Poll::Ready(()));
                    }
                };
            }
        }) {
            Some(Poll::Ready(())) => {
                // task done -> remove it and its cached waker
1
            }
            Some(Poll::Pending) => {task.dead ? 1 : 0}
            None => {task.dead ? 1 : 0}
        }) {
            0 => {},
            1 => {
                tasks.remove(&task_id);
                waker_cache.remove(&task_id);
            },
            _ => {
                task.dead = true;
                //murd
            }
        };

        }
    }

    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_interrupts_and_hlt};

        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_interrupts_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
