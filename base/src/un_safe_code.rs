use std::{fmt::Display, rc::Rc, slice::from_raw_parts, str::from_utf8_unchecked};

use log::info;

#[macro_export]
macro_rules! mvc {
    ( $( $x:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

#[macro_export]
macro_rules! user {
    ( $( $x:expr, $y:expr)? ) => {
        {
            $(
                User {
                    name: $x,
                    age: $y,
                }
            )?
        }
    };
}
/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-13 11:46:27
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-13 15:58:55
 */
pub fn run() {
    let u = user!("JUNE".to_string(), 99);
    info!("user is: {:?}", u);
    let mut v = Van(0);

    let u_ref_van = &v as *const Van;
    info!("RAW: {:?} = {}", u_ref_van, unsafe { *u_ref_van });
    let u_m_ref_van = &mut v as *mut Van;
    unsafe {
        *u_m_ref_van = Van(10);
    }
    info!("MUT: {:?} = {}", u_m_ref_van, unsafe { *u_m_ref_van });

    let (p, len) = get_value_form_ptr();
    let message = get_str_at_location(p, len);

    info!("The {} bytes at {:?} stored: {}", len, p, message);

    // 智能指针创建裸指针
    let a = Box::new(100);
    let ca: *const i32 = &*a;
    let cb = Box::into_raw(a);
    let b = Rc::new(10);
    let db: *const i32 = &*b;
    let dd = Rc::into_raw(b);
    info!("a position: {:?} = {:?} -> {}", ca, cb, unsafe { *ca });
    info!("b position: {:?} = {:?} -> {}", db, dd, unsafe { *dd });

    Sunfei::say_hi();
}

trait HelloMacro {
    fn say_hi();
}
struct Sunfei;
impl HelloMacro for Sunfei {
    fn say_hi() {
        info!("Sunfei say hi");
    }
}

struct Sunface;
impl HelloMacro for Sunface {
    fn say_hi() {
        info!("Sunface say hi");
    }
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Clone, Copy)]
struct Van(u8);
impl Display for Van {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Van={}", self.0)
    }
}

fn get_value_form_ptr() -> (*const u8, usize) {
    let v = "Hello world!";
    let pointer = v.as_ptr();
    let length = v.len();
    (pointer, length)
}

fn get_str_at_location(p: *const u8, len: usize) -> &'static str {
    let v = mvc![1u128, 2, 3, 4, 5];
    info!("{:?}", v);
    unsafe { from_utf8_unchecked(from_raw_parts(p, len)) }
}

#[no_mangle]
pub extern "C" fn say_hi() {
    info!("rust say_hi method call!");
}
