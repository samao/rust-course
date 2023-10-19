use std::{
    // collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::Context,
    time::{Duration, Instant},
};

use crossbeam::channel;
use futures::task::{self, ArcWake};
use my_redis::Delay;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-18 15:20:16
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-18 15:57:05
 */
fn main() {
    println!("mini-tokio");

    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_secs(3);
        let out = Delay { when, waker: None }.await;

        println!("final async = {}", out);
    });
    println!("will run mini tokio");
    mini_tokio.run();
}

struct MiniTokio {
    // tasks: VecDeque<Task>,
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        println!("Task schedule");
        self.executor.send(self.clone()).unwrap();
    }

    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();
        println!("Task poll");
        let _ = future.as_mut().poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });
        println!("Task spawn and send");
        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("task arc wake");
        arc_self.schedule();
    }
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        println!("create miniTokio");
        MiniTokio {
            // tasks: VecDeque::new(),
            scheduled,
            sender,
        }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // self.tasks.push_back(Box::pin(future));
        println!("add future to miniTokio");
        Task::spawn(future, &self.sender);
    }

    fn run(&mut self) {
        // let waker = futures::task::noop_waker();
        // let mut cx = Context::from_waker(&waker);

        // while let Some(mut task) = self.tasks.pop_front() {
        //     if task.as_mut().poll(&mut cx).is_pending() {
        //         self.tasks.push_back(task);
        //     }
        // }
        println!("excutor miniTokio@run");
        while let Ok(task) = self.scheduled.recv() {
            println!("excutor miniTokio will task poll in loop");
            task.poll();
        }
    }
}
