use std::{
    cell::RefCell,
    sync::{Arc, Barrier, Condvar, Mutex, Once},
    thread,
    time::Duration,
};

use log::info;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-09 15:47:03
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-09 17:44:24
 */
pub fn run() {
    let handle = thread::spawn(|| {
        thread::spawn(|| {
            info!("I am a new thread.");
            thread::sleep(Duration::from_millis(10));
        })
    });

    handle.join().unwrap();
    info!("Child thread is finish!");

    thread::sleep(Duration::from_millis(100));

    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(3));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            info!("before await");
            b.wait();
            info!("after await");
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }

    th_loc();
}

fn th_loc() {
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    })
    .join()
    .unwrap();

    FOO.with(|f| {
        info!("result: {}", f.borrow());
        assert_eq!(*f.borrow(), 2);
    });

    condvar_thread();
}

fn condvar_thread() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        info!("changing started");
        thread::sleep(Duration::from_secs(1));
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        info!("wait");
        started = cvar.wait(started).unwrap();
    }

    info!("started changed: {}", started);

    call_once();
}

fn call_once() {
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();
    info!("start: {}", INIT.is_completed());
    let mut handles = Vec::new();
    for id in 10..13 {
        handles.push(thread::spawn(move || {
            INIT.call_once(|| {
                unsafe {
                    VAL = id + 10;
                }
                info!("excuted! = {} -> {}", id, unsafe { VAL });
            });

            info!("thread {} get: {}", id, unsafe { VAL })
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    info!("final: {} = {}", unsafe { VAL }, INIT.is_completed());
}
