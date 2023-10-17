/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-22 15:39:12
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-09-22 16:11:42
 */
#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn update() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // username 没有移动
    let user2 = User {
        username: String::from("Jack"),
        ..user1
    };

    // 继续移走username
    let user3 = User {
        email: String::from("aa@33"),
        username: user1.username,
        active: true,
        sign_in_count: 1,
    };
    println!("{:#?}\r\n{:#?}", user2, user3);
}
