/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-22 16:11:48
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-09-27 15:30:12
 */
pub fn fill() {
    out_of_array_range();
    let arr: [String; 5] = core::array::from_fn(|i| format!("hello -> {}", i));
    println!("{:?}", arr);

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let z = v.iter().filter(|x| matches!(x, MyEnum::Foo));

    println!("{:?} {:?}", v.iter(), z);

    let foo = 'f';
    assert!(matches!(foo, 'a'..='z'));
    assert!(matches!('A', 'a'..='z' | 'A'..='Z'));
    let x = Some(4);
    assert!(matches!(x, Some(b) if b > 2));

    let school = Point { x: 100, y: 200 };

    assert!(matches!(
        school,
        Point {
            x: 0..=200,
            y: 100..=200
        }
    ));

    go(&school);
    bind();
}

#[derive(Debug)]
enum Message {
    Hello { id: i32 },
    World(String),
}

impl Message {
    fn call(&self) {
        println!("Message can call method! -> {:?}", self);
    }
}

fn bind() {
    let msg = Message::Hello { id: 110 };

    msg.call();

    Message::Hello { id: 220 }.call();

    Message::World(String::from(r#"I can say "你好""#)).call();

    match msg {
        Message::Hello { id: range @ 3..=10 } => {
            println!("3->10: {}", range);
        }
        Message::Hello { id: 11..=15 } => {
            println!("11->15");
        }
        Message::Hello { id } => {
            println!("other: {}", id);
        }
        _ => (),
    }
}

fn go(Point { x, y }: &Point) {
    println!("kid go to point ({}, {})", x, y);
    let p @ Point { x: px, y: py } = Point { x: 10, y: 12 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 100, y: 20 };
    if let p @ Point { x: 10..=200, y } = point {
        println!("on x=10 point, y={}: {:?}", y, p);
    } else {
        println!("not on x = 10 point");
    }

    match 2 {
        x @ (1 | 2) => {
            println!("match {}", x);
        }
        _ => (),
    }

    let distance = Point { x: 0, y: 0 }.distance(&Point { x: 10, y: 10 });

    println!("distance: {}", distance);
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn distance(&self, p: &Self) -> f64 {
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt()
    }
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

fn out_of_array_range() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // let element = a[index];
    if let Some(element) = a.get(index) {
        println!(
            "The value of the element at index {} is: {}",
            index, element
        );
    } else {
        println!("超出数组索引了");
    }
}
