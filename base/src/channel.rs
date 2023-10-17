use std::{
    sync::{mpsc, Arc, Condvar, Mutex, RwLock},
    thread,
    time::Duration,
};

use log::info;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-09 17:47:07
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-10 12:16:13
 */
pub fn run() {
    let (tx, rx) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(8)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => info!("received {} apples", count),
            Fruit::Orange(flavor) => info!("received {} oranges", flavor),
        }
    }

    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
            info!("thread {:?} finished", i);
        });
    }
    drop(tx);
    for x in rx {
        info!("GOT: {}", x);
    }
    info!("finished iterating");

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        info!("num set end");
    }
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    info!("num1 set end");
    drop(num1);
    info!("m = {:?}", m);

    mutil_thread();
}

fn mutil_thread() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    info!("result: {}", *counter.lock().unwrap());
    r_w_lock();
}

enum Fruit {
    Apple(u8),
    Orange(String),
}

fn r_w_lock() {
    let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
        info!("2");
    }
    // 同一时间多读锁或者单写锁
    {
        info!("4");
        let mut w = lock.write().unwrap();
        *w = 10;
        assert_eq!(*w, 10);
        info!("write: {}", w);
        // let mut w2 = lock.write().unwrap();
        // *w2 += 10;
    }

    condvar();
}

fn condvar() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = thread::spawn(move || {
        info!("inner get lock");
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                info!("inner release lock wait: {}", lock);
                lock = ccond.wait(lock).unwrap();
            }

            *lock = false;
            counter += 1;
            info!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;

    loop {
        info!("outside sleep");
        thread::sleep(Duration::from_millis(1000));
        info!("outside try get lock");
        *flag.lock().unwrap() = true;
        info!("outside get lock");
        counter += 1;

        if counter > 3 {
            break;
        }
        info!("outside counter: {}", counter);
        cond.notify_one();
    }

    hdl.join().unwrap();
    info!("final: {:?}", flag);
}
