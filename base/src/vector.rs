/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-09-25 14:33:46
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-09-25 14:44:29
 */
pub fn run() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip);
    }

    let v: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in v {
        ip.display();
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn show_addr(ip: IpAddr) {
    println!("{:?}", ip);
}

trait IpAddrTrait {
    fn display(&self);
}

struct V4(String);
impl IpAddrTrait for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0);
    }
}
struct V6(String);
impl IpAddrTrait for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0);
    }
}
