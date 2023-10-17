/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-22 15:15:08
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-09-22 15:47:17
 */
pub fn utf() {
    for c in "中国人".chars() {
        println!("char: {}", c);
    }
    for b in "中国人".bytes() {
        println!("bytes: {}", b)
    }

    let complex_str = "holla中国人नमस्ते";
    println!("slice: {}", utf8_slice::slice(complex_str, 0, 6));
    println!("len: {}", utf8_slice::len(complex_str));
    println!("till: {}", utf8_slice::till(complex_str, 10));
    println!("from: {}", utf8_slice::from(complex_str, 10));

    println!("all: {}", complex_str);

    for c in complex_str.chars() {
        println!("err char: {}", c);
    }
}
