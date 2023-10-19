use std::time::Duration;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-17 15:36:16
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-18 15:32:45
 */
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx.send("one");
    });

    let (tx1, rx1) = oneshot::channel();
    tokio::spawn(async {
        let _ = tx1.send("two");
    });

    tokio::select! {
        val = rx => {
            println!("rx completed first with {:?}", val);
        },
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
    }

    let ret_task = tokio::select! {
        n = one() => {
            println!("one is = {}", n);
            TaskType::Str("one return".to_string())
        },
        n = two() => {
            println!("two is = {}", n);
            TaskType::Bool(false)
        }
    };

    println!("async task run at: {:?}", ret_task);

    // select else 分支

    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel::<Option<u32>>(128);

    tokio::spawn(async move {
        let _ = tx2.send(Some(10));
        println!("{:?}", tx2);
    });

    tokio::spawn(async move {
        let _ = tx1.send(Some("value"));
    });

    tokio::select! {
        v = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        v = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("Both channels closed");
        }
    }

    tokio::time::sleep(Duration::from_secs(1)).await;

    // {
    //     let (tx1, mut rx1) = mpsc::channel::<String>(128);
    //     let (tx2, mut rx2) = mpsc::channel::<String>(128);
    //     let (tx3, mut rx3) = mpsc::channel::<String>(128);

    //     loop {
    //         let msg = tokio::select! {
    //             Some(msg) = rx1.recv() => msg,
    //             Some(msg) = rx2.recv() => msg,
    //             Some(msg) = rx3.recv() => msg,
    //             else => { break }
    //         };

    //         println!("Got {}", msg);
    //     }

    //     println!("All channels have been closed.");
    // }

    loop {
        tokio::select! {
            _ = work() => {
                println!("work done");
            },
            _ = game() => {
                println!("game done");
                break
            },
            else => {
                println!("break done");
                break
            }
        }
    }

    // ref .await must pin
    let sleep = async {
        println!("sleeping");
        false
    };
    tokio::pin!(sleep);

    let info = tokio::select! {
        val = &mut sleep => {val}
    };

    println!("final = {}", info);

}

async fn work() {
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("work");
}

async fn game() {
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("game");
}

#[derive(Debug)]
enum TaskType {
    Str(String),
    Bool(bool),
}

async fn one() -> u8 {
    10
}

async fn two() -> u8 {
    100
}
