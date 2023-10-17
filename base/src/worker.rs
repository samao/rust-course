use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-27 11:04:56
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-09-27 13:25:52
 */
pub fn run() {
    {
        let pool = ThreadPool::new(4);
        for id in 1..=15 {
            println!("发送任务: {}", id);
            pool.excute(move |worker_id| {
                println!("Work: {}, thread: {}", id, worker_id);
            });
        }
        thread::sleep(Duration::from_secs(20));

        pool.excute(|worker_id| {
            println!("worker: {}, 执行任务 sleep", worker_id);
            thread::sleep(Duration::from_secs(10));
            println!("worker: {}, 执行任务", worker_id);
        });
    }

    let handle = thread::spawn(|| (false, 99));
    let (success, id) = handle.join().unwrap();
    println!("返回值测试{} -> {}", success, id);
}

type Job = Box<dyn FnOnce(usize) -> () + 'static + Send>;

#[allow(unused)]
struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        let (sender, reciver) = channel();
        let reciver = Arc::new(Mutex::new(reciver));
        for id in 1..=size {
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }
        ThreadPool {
            sender: Some(sender),
            workers,
        }
    }
    fn excute<T>(&self, f: T)
    where
        T: FnOnce(usize) -> () + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[allow(unused)]
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::Builder::new()
            .name(format!("线程<{}>", id))
            .spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                if let Ok(job) = job {
                    thread::sleep(Duration::from_secs(3));
                    println!("线程{}获得任务", id);
                    job(id);
                } else {
                    println!("worker {} disconnected; shutting down", id);
                    break;
                }
            })
            .unwrap();
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
