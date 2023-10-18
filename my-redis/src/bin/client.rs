use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-17 15:36:16
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-18 15:32:45
 */
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let res = resp_rx.await;
        println!("tx1 GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let res = resp_rx.await;
        println!("tx2 GOT = {:?}", res);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            println!("GOT = {:?}", cmd);
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    resp.send(res).unwrap();
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    resp.send(res).unwrap();
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
pub enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}
