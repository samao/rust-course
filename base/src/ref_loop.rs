use log::info;

use self::List::{Cons, Nil};
use std::{
    cell::RefCell,
    marker::PhantomPinned,
    pin::Pin,
    ptr::NonNull,
    rc::{Rc, Weak},
};
/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-09 10:10:01
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-09 15:51:44
 */
pub fn loop_ref() {
    info!("loop_ref");
}

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    #[allow(dead_code)]
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.data);

        unsafe {
            let mut_ref: Pin<&mut Unmovable> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }

        boxed
    }
}

pub fn loop_ref_weak_node_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    });

    info!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        info!(
            "before down branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        info!(
            "before down leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        info!(
            "after branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        info!(
            "after leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    info!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    info!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    let u = User;
    {
        let _u = Box::new(&u);
        info!("block finished");
    }
    info!("block out side: {:?}", u);
}

#[derive(Debug)]
struct User;

impl Drop for User {
    fn drop(&mut self) {
        info!("User is droping");
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[allow(dead_code)]
fn loop_weak_ref() {
    let gadget_owner = Rc::new(Owner {
        name: "Gadget Man".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });

    {
        let gadget1 = Rc::new(Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        });

        gadget_owner
            .gadgets
            .borrow_mut()
            .push(Rc::downgrade(&gadget1));
    }
    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    });
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget2));
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        if let Some(gadget) = gadget_opt.upgrade() {
            info!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        } else {
            info!("Gadget is empty");
        }
    }
}

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

#[allow(dead_code)]
fn loop_ref_overflow() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    info!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    info!("a指向的节点 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    info!("在b创建后， a的rc计数 = {}", Rc::strong_count(&a));
    info!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    info!("b指向的节点 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    info!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    info!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    info!("a next item = {:?}", a.tail());
}

type Item<T> = RefCell<Rc<T>>;

#[derive(Debug)]
enum List {
    Cons(i32, Item<List>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&Item<Self>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
