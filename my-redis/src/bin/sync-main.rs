use std::{thread, time::Duration};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-17 15:36:16
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-18 15:32:45
 */
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("async hello world!");
        });
    println!("next");

    thread::sleep(Duration::from_secs(2));
}
