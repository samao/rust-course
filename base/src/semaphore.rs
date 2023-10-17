use std::{sync::Arc, thread, time::Duration};

use log::info;
use tokio::sync::Semaphore;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-10 17:51:04
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-10 17:57:22
 */
pub async fn run() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = vec![];
    for id in 1..=5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            info!("in tokio thread");
            thread::sleep(Duration::from_secs((3 * id).min(10)));
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
