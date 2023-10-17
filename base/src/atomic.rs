/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-11 12:20:47
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-11 17:55:47
 */

use std::{
    ops::Sub,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc, Mutex, OnceLock,
    },
    thread::{self, JoinHandle},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use log::info;

const N_TIMES: u64 = 10_000_000;
const N_THREADS: usize = 10;
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

pub fn run() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    // let time = SystemTime::now();
    info!(
        "{:?}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    );
    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    info!("{:?}", Instant::now().sub(s));

    ptr_send();
}

const DELAY: u8 = 100;

fn ptr_send() {
    let pp = MBox(5 as *mut u8);
    let p = &MBox(5 as *mut u8);
    let v = Arc::new(Mutex::new(p));
    let handle = thread::spawn(move || {
        info!("线程转移裸指针<Send>：{:?}", pp);
        let v = v.lock().unwrap();
        info!("线程转移裸指针引用<Sync>: {:?}", v);
        info!("4.{:?}", DELAY);
    });

    info!("1.{:?}", DELAY);
    const DELAY: &str = "GUUUUUD";
    info!("2.{:?}", DELAY);
    next();
    handle.join().unwrap();
}

static mut REQUEST_RECV: u128 = 0;

fn next() {
    info!("3.{}", DELAY);

    unsafe {
        REQUEST_RECV += 1;
    };

    unsafe {
        assert_eq!(REQUEST_RECV, 1);
    }

    thread::spawn(|| unsafe {
        info!("static: {:?}", REQUEST_RECV);
    })
    .join()
    .unwrap();

    // static mut REQUEST_RECV: u128 = 9;
    atomic_add();
}

#[derive(Debug)]
struct MBox(*mut u8);
unsafe impl Send for MBox {}
unsafe impl Sync for MBox {}

static TOTAL: AtomicUsize = AtomicUsize::new(0);

fn atomic_add() {
    for _ in 0..100 {
        TOTAL.fetch_add(10, Ordering::Relaxed);
    }

    info!("total: {:?}", TOTAL);
    leak();
}

#[derive(Debug)]
struct Config(u8);

static mut CONFIG: Option<&mut Config> = None;

fn leak() {
    let c = Box::new(Config(0));

    unsafe {
        CONFIG = Some(Box::leak(c));
        info!("{:?}", CONFIG);
        CONFIG = init();
        info!("{:?}", CONFIG);
    }

    once_cell();
}

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config(10));
    Some(Box::leak(c))
}

fn once_cell() {
    let handle = thread::spawn(|| {
        let logger = Logger::global(0);
        logger.log("thread message".to_string());
    });

    let logger = Logger::global(1);
    logger.log("some message".to_string());

    let logger1 = Logger::global(2);
    logger1.log("other message".to_string());

    handle.join().unwrap();
}

// 全局单例
#[derive(Debug)]
struct Logger;
static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global(id: u8) -> &'static Logger {
        LOGGER.get_or_init(|| {
            info!("Logger is being created... {}", id);
            Logger
        })
    }

    fn log(&self, message: String) {
        info!("{}", message);
    }
}
