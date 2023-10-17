/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-17 14:40:36
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-17 16:34:50
 */

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    println!(
        "OS: {}#{}",
        sys_info::os_type().unwrap_or_default(),
        sys_info::os_release().unwrap_or_default()
    );
    println!("HOST: {}", sys_info::hostname().unwrap_or_default());
    println!("DISK: {:?}", sys_info::disk_info().unwrap());
    println!("MEM: {:?}", sys_info::mem_info().unwrap());
    println!("CPU: {}", sys_info::cpu_num().unwrap_or_default());
    println!("CPU_SPEED: {:?} MHz", sys_info::cpu_speed().unwrap());

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = Arc::clone(&db);
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // let mut db = HashMap::new();

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT:{:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
