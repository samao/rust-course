use std::{fmt::Display, slice::from_raw_parts, str::from_utf8_unchecked, thread};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-25 15:05:37
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-07 16:06:02
 */
pub fn say_dadddy() {
    let s1 = String::from("long string is long");

    {
        let mut s2 = String::from("xyz");
        let s3 = &mut s2;
        s3.push_str("GUUD");
        println!(r#"The longest string is "{}""#, longest(&s1, &s2));
    }

    let exceprt;
    // {
    let content = "GUUUD GUUUD Study.".to_string();
    exceprt = Excerpt::build(false, &content);
    // }
    println!("need live long enough! {:?}", exceprt);

    let handle = thread::spawn(|| {
        println!("child thread created");

        let handle = thread::spawn(|| {
            println!("grand son thread created");
        });

        handle.join().unwrap();
    });

    handle.join().unwrap();

    run();
}

fn run() {
    let r1;
    let r2;
    {
        static STATIC_I32: i32 = 42;
        r1 = &STATIC_I32;
        let x = "&'static str";
        r2 = x;
    }
    println!("&'static i32: {}", r1);
    println!("&'static str: {}", r2);

    // error
    // let r3;
    // {
    //     let s1 = "String".to_string();
    //     static_bound(&s1);
    //     r3 = &s1;
    // }

    // println!("{}", r3);

    let (pointer, length) = get_memory_location("你好，不好，好不好啊？");
    // 超范围会读到别的数据
    let message = get_str_at_location(pointer, length);

    println!(
        "The {} bytes at 0x{:?} stored: {}",
        length, pointer, message
    );

    let mut x = "GUUD".to_string();
    let mut sum = |a: String| x.push_str(&a);
    let _sum = |a: String| println!("{a}");
    for i in 0..4 {
        sum(format!(" Boy_{}", i));
    }

    println!("结果： {}", x);
    let s = sum_once(move |or| format!("==={} MY: {}", x, or));

    println!("结果2： {}", s);
}

#[derive(Debug, Clone, Copy)]
struct Book;

fn sum_once<T>(pending: T) -> String
where
    T: FnOnce(String) -> String,
{
    pending("XXXX".to_string())
}

fn get_str_at_location(pointer: *const u8, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer, length)) }
}

fn get_memory_location(string: &str) -> (*const u8, usize) {
    let pointer = string.as_ptr();
    let length = string.len();
    (pointer, length)
}

#[allow(unused)]
fn static_bound<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

#[allow(unused)]
fn reborrow() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr = &*r;
    println!("{:?}", rr);
    r.go(10, 10);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn go(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

#[derive(Debug)]
struct Double<'a, 'b: 'a>(&'a str, &'b str);

#[allow(unused)]
#[derive(Debug)]
struct Excerpt<'a, T> {
    content: &'a str,
    ptr: T,
}

impl<'a, T> Excerpt<'a, T> {
    fn build(ptr: T, content: &'a str) -> Excerpt<'a, T> {
        Excerpt { ptr, content }
    }
}
