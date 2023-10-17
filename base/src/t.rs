use std::fmt::Display;
use std::ops::Add;
use std::vec;

use log::info;

use crate::{un_safe_code::User, user};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-25 11:00:19
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-13 15:18:59
 */
pub fn run() {
    let p = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'c' };

    let p3 = p.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p = Point {
        x: 3.0f32,
        y: 4.0f32,
    };
    println!(
        "distance to origin: {}, summary: {}",
        p.distance(),
        p.summarize()
    );

    display_array(&[0; 5]);
    display_array(&["ok"; 3]);

    let s = String::from("GUUUUUUD!!!");
    println!("{}", s.summarize());

    let r = Rect { x: 10, y: 20 };
    let x = r + Rect { x: 100, y: 200 };
    println!("{:}", x);

    let meter = Meters(10);
    let millimeters = Millimeters(10) + meter;
    dbg!(millimeters);

    let millimeters = Millimeters(1510);
    let meter = Meters(10) + millimeters;
    dbg!(meter);

    collection();
}

struct Point<T, U> {
    x: T,
    y: U,
}

// #[derive(Debug)]
struct Rect<T> {
    x: T,
    y: T,
}

impl<T: Display> Display for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} {}>", self.x, self.y)
    }
}

impl<T: Add<T, Output = T>> Add for Rect<T> {
    type Output = Rect<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Rect {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, p: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: p.y }
    }
}

impl Point<f32, f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", arr);
}

impl<T: Display, U: Display> Summary for Point<T, U> {
    fn summarize(&self) -> String {
        format!("Point: ({}, {})", self.x, self.y)
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more...")
    }
}

pub fn draw(_ui: impl Summary) {
    _ui.summarize();
}

impl Summary for String {
    fn summarize(&self) -> String {
        format!("Hello: {}", self)
    }
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Millimeters) -> Self::Output {
        Meters(self.0 + ((rhs.0 / 1000) as u32))
    }
}

pub fn collection() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &mut v[0];
    println!("the first element is: {}", first);
    // v.push(6);
    *first = 10;
    println!("the first element is: {:?}", v);

    // let a: Vec<u64> = mvc!{1u64,2,3,4,5};
    let u = user!["Liya".to_string(), 77];
    info!("macro create user: {:?}", u);
}
