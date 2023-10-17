use std::{cell::RefCell, rc::Rc, sync::Arc, thread};

use log::{debug, info};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-08 11:55:11
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-09 10:08:06
 */
pub const ONE_DAY: u32 = 24 * 60 * 60 * 1000;

pub fn prt(num: u32) {
    info!("example ptr: {}", num);
    let u = User {
        name: "June.liu",
        age: 18,
    };
    prin_user(&u);
    let a = [10; 10];
    debug!(target: "PTR POSTION", "{:?}", a.as_ptr());
    let b = a;
    debug!(target: "PTR POSTION", "{:?}", b.as_ptr());

    ru_trait();
}

fn prin_user(user @ &User { name, age }: &User) {
    debug!("{:?} = name: {}, age: {}", user, name, age);
}

#[allow(dead_code)]
#[derive(Debug)]
struct User<'a> {
    name: &'a str,
    age: u8,
}

#[allow(dead_code)]
trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        info!("这是屏幕上第{}号按钮", self.id);
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        info!("这个选择框贼难用{}", self.id);
    }
}

fn ru_trait() {
    let elems: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { id: 1 }),
        Box::new(Button { id: 2 }),
        Box::new(Select { id: 2 }),
    ];

    for e in elems {
        e.draw();
    }

    let y = MBox::new(Box::new(10));
    assert_eq!(10, **y);

    let plus = add_one(&y);

    info!("{:?}", plus);

    let a = Rc::new("hello world".to_string());
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

    ref_mut();
}

#[derive(Debug)]
struct MBox<T>(T);

impl<T> MBox<T> {
    fn new(x: T) -> MBox<T> {
        MBox(x)
    }
}

impl<T> std::ops::Deref for MBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MBox<T> {
    fn drop(&mut self) {
        println!("droping");
    }
}

impl<T> std::ops::DerefMut for MBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn add_one(a: &i32) -> i32 {
    a + 1
}

#[allow(dead_code)]
#[derive(Debug)]
struct Owner {
    name: String,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn ref_mut() {
    let gadget_owner = Rc::new(Owner {
        name: "Gadget Man".to_string(),
    });
    info!("1.ref count: {}", Rc::strong_count(&gadget_owner));
    let gadget1 = Gadget {
        id: 10,
        owner: gadget_owner.clone(),
    };
    info!("2.ref count: {}", Rc::strong_count(&gadget_owner));
    let gadget2 = Gadget {
        id: 110,
        owner: gadget_owner.clone(),
    };
    info!("3.ref count: {}", Rc::strong_count(&gadget_owner));
    drop(gadget_owner);
    info!("4.ref count: {}", Rc::strong_count(&gadget1.owner));
    dbg!(gadget1);
    dbg!(gadget2);
    tread_ref();
}

fn tread_ref() {
    let s = Arc::new("多线程漫游者".to_string());
    let mut handles = Vec::new();
    for id in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            info!("{}: {}", id, s);
        });

        handles.push(Some(handle));
    }

    handles.iter_mut().for_each(|handle| match handle.take() {
        Some(handle) => {
            handle.join().unwrap();
        }
        None => (),
    });

    let c = RefCell::new("value".to_string());
    // let one = c.get();
    // c.set("val".to_string());
    // let two = c.get();
    // info!("{}, {}", one, two);
    {
        let mut a = c.borrow_mut();
        a.push_str("string");
    }
    {
        let b = c.borrow();
        info!("immutable: {}", b);
    }
    println!("all borrow: {:?}", c.borrow());
}
