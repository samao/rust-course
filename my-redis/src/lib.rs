/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-18 15:34:56
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-18 17:44:29
 */

use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use std::thread;
use std::time::Instant;

#[derive(Debug)]
pub struct Delay {
    pub when: Instant,
    pub waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());
            println!("delay poll go thread");
            
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    println!("delay thread sleep");
                    thread::sleep(when - now);
                }
                println!("delay wake");
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            println!("hello world.");
            Poll::Ready("Done")
        } else {
            Poll::Pending
        }
    }
}
