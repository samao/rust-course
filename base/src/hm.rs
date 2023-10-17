/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-25 15:04:22
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-09-25 17:54:57
 */
use std::{collections::HashMap, fmt::Display, sync::Arc, thread, time::Duration};

pub fn run() {
    let team_lsit = vec![("china", 100), ("USA", 10), ("Japan", 20), ("china", 120)];

    let mut team_score: HashMap<_, _> = team_lsit.into_iter().collect();
    team_score.insert("USA", 15);
    team_score.entry("Japan").and_modify(|val| *val *= 10);
    team_score.insert("Kore", 9);
    println!("{:?}", team_score);
    owner_move();

    println!("i128Max: {}", u128::MAX);

    type_transformer();

    _danger();
    let a = vec![(false, true), (false, true)];
    let ac: HashMap<_, _> = a.into_iter().collect();
    println!("{:?}", ac);
}

fn owner_move() {
    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);

    if let Some(uage) = handsome_boys.get("Sunface") {
        println!("Hash Map Has sunface: {}", uage)
    }

    //这个就转移了 println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁 -> {:?}", age, handsome_boys);
}

fn type_transformer() {
    let values = [1, 2];
    let p1 = values.as_ptr();
    let first_addr = p1 as usize;
    println!("不可变裸指针: {:?}", p1);
    let second_addr = first_addr + 4;
    let p2 = second_addr as *mut i32;

    unsafe {
        *p2 += 1;
    }
    println!("罗指针：{:?}", values);

    let b: u8 = 300u16.try_into().unwrap_or_else(|e| {
        println!("u16 -> u8 error: {:?}", e);
        99
    });
    println!("try into -> {}", b);

    println!("手动转换: {:?}", reinterpret(Foo { x: 100, y: 99 }));
}

struct Foo {
    x: u32,
    y: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Bar {
    a: u32,
    b: u32,
}

fn reinterpret(Foo { x: a, y: b }: Foo) -> Bar {
    Bar { a, b }
}

#[derive(Clone)]
struct Container<T>(Arc<T>);

// impl<T> Clone for Container<T>  {
//     fn clone(&self) -> Self {
//         Self(Arc::clone(&self.0))
//     }
// }

#[allow(dead_code, unused)]
fn clone_container<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cp = foo.clone();
    let bar_cp = bar.clone();
}

fn _foo() -> i32 {
    0
}

fn _danger() {
    let pointer = _foo as *const ();
    let fun_fc = unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };

    assert_eq!(0, fun_fc());

    let mut a = 10;

    let b = &a as *const i32;
    let c = &mut a as *mut i32;

    unsafe {
        *c += 100;
        println!("b is: {}, c is: {}, a is {}", *b, *c, a);
    }

    let builder = thread::Builder::new().name("HOHO THREAD".to_string());
    if let Err(worker) = builder.spawn(|| {
        assert!(false, "HOHO");
    }) {
        println!("worker run fail: {:?}", worker);
    };
    thread::sleep(Duration::from_secs(4));
    println!("worker failed!");

    let chicken = build::<Chicken>();
    let pig = build::<Pig>();

    println!("chicken is : {}, pig is: {:?}", chicken, pig);
}

trait Build {
    fn build() -> Self;
}
#[derive(Debug)]
struct Pig(u32);
impl Build for Pig {
    fn build() -> Self {
        Pig(10)
    }
}
struct Chicken(u32);
impl Build for Chicken {
    fn build() -> Self {
        Chicken(25)
    }
}
impl Display for Chicken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chicken<{}>", self.0)
    }
}

fn build<T: Build>() -> T {
    T::build()
}
