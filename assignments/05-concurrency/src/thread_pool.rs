use std::any::Any;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
pub struct ThreadPool<T> {
    handles: Vec<JoinHandle<()>>,
    transmitters: Vec<Sender<T>>,
    receivers: Vec<Receiver<T>>,
    pub num_threads: usize,
    pub task_amount: usize,
}

pub struct Task<T_Args, T_Outs> {
    pub func: fn(T_Args) -> T_Outs,
    pub inputs: T_Args,
    pub outputs: T_Outs,
    pub end_thread: bool,
}

// pub struct T_Args {
//     args: Vec<Box<dyn Any>>,
// }
// pub struct T_Outs {
//     args: Vec<Box<dyn Any>>,
// }

impl<T_Args: Send + Clone + 'static, T_Outs: Send + Clone + 'static>
    ThreadPool<Task<T_Args, T_Outs>>
{
    /// Create a new LocalThreadPool with num_threads threads.
    ///
    /// Errors:
    /// - If num_threads is 0, return an error
    pub fn new(num_threads: usize) -> Self {
        if num_threads < 1 {
            panic!("num_threads cannot be 0")
        }

        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        let mut receivers: Vec<Receiver<Task<T_Args, T_Outs>>> = Vec::new();
        let mut senders: Vec<Sender<Task<T_Args, T_Outs>>> = Vec::new();

        for idx in 0..num_threads {
            let (tx_to_child, rx_from_parent): (
                Sender<Task<T_Args, T_Outs>>,
                Receiver<Task<T_Args, T_Outs>>,
            ) = mpsc::channel();

            let (tx_to_parent, rx_from_child): (
                Sender<Task<T_Args, T_Outs>>,
                Receiver<Task<T_Args, T_Outs>>,
            ) = mpsc::channel();

            let tx_to_parent_clone = tx_to_parent.clone();
            let rx_from_parent = Arc::new(Mutex::new(rx_from_parent));

            let thread_join_handle = thread::spawn(move || {
                let mut receiver = rx_from_parent.lock().unwrap();
                loop {
                    let task_todo: Result<Task<T_Args, T_Outs>, mpsc::RecvError> = receiver.recv();

                    let mut task_recv: Task<T_Args, T_Outs> = match task_todo {
                        Ok(ok) => ok,
                        Err(e) => panic!("{}", e),
                    };

                    if task_recv.end_thread {
                        println!("ending thread");
                        break;
                    }

                    task_recv.outputs = (task_recv.func.clone())(task_recv.inputs.clone());

                    let _ = tx_to_parent_clone.send(task_recv);
                }
            });

            handles.push(thread_join_handle);
            senders.push(tx_to_child);
            receivers.push(rx_from_child);
        }

        ThreadPool {
            handles: handles,
            transmitters: senders,
            receivers: receivers,
            num_threads: num_threads,
            task_amount: 0,
        }
    }

    /// Execute the provided function on the thread pool
    ///
    /// Errors:
    /// - If we fail to send a message, report an error
    //pub fn execute<F>(&self, f: F) {

    pub fn execute(&mut self, f: Vec<Task<T_Args, T_Outs>>) {
        self.task_amount = f.len();
        if f.len() > self.num_threads {
            panic!("Too many tasks for threadpool");
        }

        for (idx, task) in f.into_iter().enumerate() {
            //assign tasks
            self.transmitters[idx].send(task).expect("Bad task send");
        }
    }
    /// Retrieve any results from the thread pool that have been computed
    pub fn get_results(&mut self) -> Vec<Task<T_Args, T_Outs>> {
        let mut outputs: Vec<Task<T_Args, T_Outs>> = Vec::with_capacity(self.task_amount);

        for idx in 0..self.task_amount {
            // self.handles[idx]
            //     .join()
            //     .expect("The thread being joined has panicked");

            //retrieve tasks
            //self.receivers[idx].send(task).expect("Bad task send");

            let task_result: Result<Task<T_Args, T_Outs>, mpsc::RecvError> =
                self.receivers[idx].recv();

            match task_result {
                Ok(ok) => outputs.push(ok),
                Err(e) => panic!("{}", e),
            };
        }
        outputs
    }

    pub fn end_pool(&mut self, f: Task<T_Args, T_Outs>) {
        println!("attempting to kill thread pool");
        for idx in 0..self.num_threads {
            let task: Task<T_Args, T_Outs> = Task {
                func: f.func,
                inputs: f.inputs.clone(),
                outputs: f.outputs.clone(),
                end_thread: true,
            };

            //assign tasks
            self.transmitters[idx]
                .send(task)
                .expect("Couldn't end task");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // A simple function for testing task execution
    fn sample_task(input: i32) -> i32 {
        input * 2 // Doubles the input for easy checking
    }

    #[test]
    fn test_thread_pool_initialization() {
        let num_threads = 4;
        let pool: ThreadPool<Task<i32, i32>> = ThreadPool::new(num_threads);
        assert_eq!(pool.num_threads, num_threads);
        assert_eq!(pool.task_amount, 0);
        assert_eq!(pool.handles.len(), num_threads);
        assert_eq!(pool.transmitters.len(), num_threads);
        assert_eq!(pool.receivers.len(), num_threads);
    }

    #[test]
    #[should_panic(expected = "num_threads cannot be 0")]
    fn test_thread_pool_initialization_zero_threads() {
        let _ = ThreadPool::<Task<i32, i32>>::new(0);
    }

    #[test]
    fn test_thread_pool_task_execution() {
        let num_threads = 2;
        let mut pool: ThreadPool<Task<i32, i32>> = ThreadPool::new(num_threads);

        // Create tasks to send to the pool
        let tasks = vec![
            Task {
                func: sample_task,
                inputs: 5,
                outputs: 0,
                end_thread: false,
            },
            Task {
                func: sample_task,
                inputs: 10,
                outputs: 0,
                end_thread: false,
            },
        ];

        pool.execute(tasks);

        // Get results
        let results = pool.get_results();
        assert_eq!(results.len(), 2);

        // Check that each task's output matches the expected result
        assert_eq!(results[0].outputs, 10);
        assert_eq!(results[1].outputs, 20);
    }

    #[test]
    fn test_thread_pool_execute_and_retrieve_results() {
        let num_threads = 3;
        let mut pool = ThreadPool::new(num_threads);

        // Create tasks
        let tasks = vec![
            Task {
                func: sample_task,
                inputs: 3,
                outputs: 0,
                end_thread: false,
            },
            Task {
                func: sample_task,
                inputs: 7,
                outputs: 0,
                end_thread: false,
            },
            Task {
                func: sample_task,
                inputs: 1,
                outputs: 0,
                end_thread: false,
            },
        ];

        pool.execute(tasks);

        // Retrieve results
        let results = pool.get_results();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].outputs, 6); // 3 * 2
        assert_eq!(results[1].outputs, 14); // 7 * 2
        assert_eq!(results[2].outputs, 2); // 1 * 2
    }
}
