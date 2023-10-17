/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-07 15:42:34
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-08 10:53:52
 */
use log::{self, debug, info, trace};
use std::{collections::HashMap, fmt::Display};

pub struct MLoger;

impl log::Log for MLoger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        println!("{}:{} - {}", record.level(), record.target(), record.args());
    }
}

pub trait SayHi {
    fn bye(&self);
}

struct Dot(u8);

impl SayHi for Dot {
    fn bye(&self) {
        info!("bye bye dot. {}", self.0);
    }
}

fn shave_the_yak() {
    trace!("Commencing yak shaving");

    Dot(10).bye();
    let range_count = (1..=10).count();
    let arr = [123; [0; 10].len()];

    debug!("数组: {:?} = {}", arr, range_count);
}

pub fn run() {
    // let mut count = 0;
    // loop {
    //     println!("run in count: {}", count);
    //     match count {
    //         5 => break,
    //         _ => {
    //             count += 1;
    //         }
    //     }
    // }
    // log::set_logger(&MLoger).unwrap();
    // log::set_max_level(log::LevelFilter::Trace);
    shave_the_yak();
    let values = vec![1, 2, 3, 4];
    match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => {
                    info!("手动迭代器：{}", x);
                }
                None => break,
            }
        },
    };

    let names = ["sunface", "sunfei", "june.liu", "june.liu"];
    let ages = [18; 4];
    let folks = names
        .into_iter()
        .zip(ages.into_iter())
        .collect::<HashMap<_, _>>();

    debug!("{:?}", folks);

    let fit_shoes = shoes_in_size(
        vec![
            Shoe {
                size: 10,
                style: "star".to_string(),
            },
            Shoe {
                size: 11,
                style: "lining".to_string(),
            },
            Shoe {
                size: 10,
                style: "adiaos".to_string(),
            },
            Shoe {
                size: 11,
                style: "adidas".to_string(),
            },
        ],
        10,
    );

    info!("fit_shoes: {:?}", fit_shoes);
    // println!("fit_shoes: {}", fit_shoes.get(0).unwrap());

    run_iter();
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Display for Shoe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shoe: {} [{}]", self.style, self.size)
    }
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    cnt: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { cnt: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cnt < 5 {
            self.cnt += 1;
            Some(self.cnt)
        } else {
            None
        }
    }
}

fn run_iter() {
    let mut counter = Counter::new();
    for id in 1..6 {
        assert_eq!(counter.next(), Some(id));
    }
    assert_eq!(counter.next(), None);

    let sum = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum::<u32>();
    assert_eq!(sum, 18);

    let v = vec![1u64, 2, 3, 4, 5];
    let val = v
        .iter()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(_, v)| v)
        .fold(10, |sum, cur| sum + cur);

    assert_eq!(val, 19);

    // let a = 10u8;
    let b = 1500u16;
    let _b_: u8 = match b.try_into() {
        Ok(v) => v,
        Err(err) => {
            info!("Fail in to: {:?}", err);
            0
        }
    };
}
