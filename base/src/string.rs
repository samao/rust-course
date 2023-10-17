/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-22 11:12:56
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-09-25 15:06:07
 */
mod clear;
mod concat;
mod delete;
mod insert;
mod push;
mod replace;
mod utf8;

pub fn opration() {
    translate();
    push::push();
    insert::insert();
    replace::replace();
    delete::delete();
    clear::clear();
    concat::concat();
    utf8::utf();
}

fn translate() {
    let long_string = "String literals\
        can span multiple lines.\
        The linebreak and indentation here ->\
        <- can be escaped too!";
    println!("{}", long_string);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let quotes = r#"{"status": 0, "message": "not exist"}"#;
    println!("{}", quotes);
    let foramt = String::from(r#"{"status": 0, "message": "not exist"}"#);
    println!("{}", foramt);

    println!(r#"{{"status": {}, "message": "{}"}}"#, 403, "没有访问权限");
}
