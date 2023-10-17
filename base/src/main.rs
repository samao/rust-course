/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-22 11:11:20
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-13 17:54:15
 */
use base::{iter::MLoger, *};
// use log::info;

#[tokio::main]
async fn main() {
    log::set_logger(&MLoger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    // worker::run();
    // string::opration();
    // structs::update();
    // arrays::fill();
    // t::run();
    // vector::run();
    // hm::run();
    // a::say_a();
    // grand::say_dadddy();
    // iter::run();
    // dst::dst();
    // ptr::prt(ptr::ONE_DAY);
    // ref_loop::loop_ref();
    // thread::run()
    // channel::run();
    // semaphore::run().await;
    // atomic::run();
    // complex::up();
    // un_safe_code::run();
    async_code::run();
}
