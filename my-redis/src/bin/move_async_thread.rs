use std::{
    pin::Pin,
    task::Poll,
    time::{Duration, Instant}, sync::Arc, thread,
};

use futures::{future::poll_fn, Future};
use my_redis::Delay;
use tokio::sync::Notify;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-18 17:03:48
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-18 17:49:49
 */
#[tokio::main]
async fn main() {
    let mut delay = Some(Delay {
        when: Instant::now() + Duration::from_secs(10),
        waker: None,
    });

    poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());

        tokio::spawn(async move {
            delay.await;
        });
        Poll::Ready(())
    })
    .await;

    println!("===notify===");
    let when = Instant::now() + Duration::from_secs(5);
    let notify = Arc::new(Notify::new());

    let thread_notify = Arc::clone(&notify);
    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            println!("sleeping");
            thread::sleep(when - now);
        }
        println!("notify one");
        thread_notify.notify_one();
    });

    println!("waiting");
    notify.notified().await;

    println!("finished");
}
