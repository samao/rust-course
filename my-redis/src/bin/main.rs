use std::time::{Duration, Instant};

use my_redis::Delay;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::join;
/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-18 10:42:48
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-18 15:39:08
 */
#[tokio::main]
async fn main() -> io::Result<()> {
    create().await?;
    read().await?;
    Ok(())
}

async fn read() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);

    // run_delay().await;
    join!(run_delay(), run_parallel());

    Ok(())
}

async fn create() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;
    let n = file.write(b"https://www.baidu.com/s?ie=utf-8&f=3&rsv_bp=1&rsv_idx=1&tn=baidu&wd=%E6%B4%BB%E5%8A%9B%E8%A1%97app&fenlei=256&oq=%25E6%25B4%25BB%25E5%258A%259B%25E8%25A1%2597&rsv_pq=e4aa660e0005075e&rsv_t=d58edolWTHh0LhVKidwYw6CAxI%2BVePYqA2jDd7Cwjg3aXqWIsouVqt3q8e8&rqlang=cn&rsv_enter=1&rsv_dl=ts_0&rsv_btype=t&rsv_sug3=11&rsv_sug1=4&rsv_sug7=100&rsv_sug2=1&prefixsug=%25E6%25B4%25BB%25E5%258A%259B%25E8%25A1%2597&rsp=0&rsv_sug4=2067").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);

    Ok(())
}

async fn run_parallel() {
    println!("entry run parallel");
    Delay {
        when: Instant::now() + Duration::from_secs(6),
    }
    .await;
}

async fn run_delay() {
    println!("entry run delay");
    let when: Instant = Instant::now() + Duration::from_secs(3);
    let future = Delay { when };

    let out = future.await;

    println!("async return = {}", out);
}
