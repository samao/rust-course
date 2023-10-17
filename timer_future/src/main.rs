use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
    time::Duration,
};

use futures::{
    future::BoxFuture,
    join, pin_mut, select,
    task::{waker_ref, ArcWake},
    try_join, Future, FutureExt, TryFutureExt,
};
use timer_future::TimerFuture;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-16 14:29:08
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-16 18:17:29
 */
#[tokio::main]
async fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("work!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    drop(spawner);

    executor.run();

    join!(enjoy_book(), enjoy_music());
    // join_all(vec![enjoy_book(), enjoy_music()]).await;
    println!("next_order");
    let _ = get_book_and_music().await;

    race_tasks().await;

    println!("fain all")
}

async fn task_one() {
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("task_one");
}
async fn task_two() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("task_two");
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    }
}

async fn get_book_and_music() -> Result<(u8, bool), String> {
    let boot_fut = get_book().map_err(|()| "book_error".to_string());
    let music_fut = get_music();

    try_join!(boot_fut, music_fut)
}

async fn get_book() -> Result<u8, ()> {
    tokio::time::sleep(Duration::from_secs(4)).await;
    Ok(10)
}

async fn get_music() -> Result<bool, String> {
    tokio::time::sleep(Duration::from_secs(6)).await;
    // Err("".to_string())
    Ok(true)
}

async fn enjoy_music() {
    println!("enjoy music");
}

async fn enjoy_book() {
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("enjoy book");
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (task_sender, ready_queue) = sync_channel(10_1000);

    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = Arc::clone(arc_self);

        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}
