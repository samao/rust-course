use futures::executor::block_on;
use log::info;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-13 17:00:54
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-13 18:00:54
 */
pub fn run() {
    //    let song = block_on(learn_song());
    //    block_on(sing_song(song));
    //    info!("唱完");
    //    block_on(dance());
    block_on(async_main());
}
struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    info!("learn song");
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    info!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "你存在我深深的脑海里~ ~"
    );
}

async fn dance() {
    info!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn kill_dog() {
    info!("kill dog");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    let f3 = kill_dog();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2, f3);

    closure_inner();
}

fn closure_inner() {
    info!("closure inner");

    let say_bye = |msg: &str| {
        info!("say bye: {}", msg);

        10u8
    };

    let result = run_next(say_bye);
    info!("all end: {}", result);
}

fn run_next(cb: impl Fn(&str) -> u8) -> u8 {
    info!("run next!");
    let mut a = cb("八戒");
    info!("run next end {}", a);
    a += 100;
    a
}
