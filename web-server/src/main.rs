/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-17 10:51:31
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-17 11:59:50
 */
use std::{fs, time::Duration};

use async_std::{
    io::{Read, Write},
    net::TcpListener,
    prelude::*,
    task::{self, spawn},
};
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    // 阻塞迭代器
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream).await;
    // }

    listener
        .incoming()
        .for_each_concurrent(None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            spawn(handle_connection(tcpstream));
            //单线程 handle_connection(tcpstream).await;
        })
        .await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "static/hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "static/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "static/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    stream
        .write(format!("{}{}", status_line, contents).as_bytes())
        .await
        .unwrap();
    stream.flush().await.unwrap();
}

// #[allow(dead_code)]
// // 单线程实现
// fn handle_connection_bk(mut stream: std::net::TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     let get = b"GET / HTTP/1.1\r\n";

//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK\r\n\r\n", "static/hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "static/404.html")
//     };
//     let content = fs::read_to_string(filename).unwrap();

//     let response = format!("{}{}", status_line, content);
//     stream.write_all(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

#[cfg(test)]
mod test_connection {
    use std::task::Poll;

    use async_std::io::{Read, Write};

    use super::*;

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);

        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();

        let expected_contents = fs::read_to_string("static/hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);

        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
            buf: &mut [u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            let size: usize = std::cmp::min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(
            mut self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            self.write_data = Vec::from(buf);
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    impl std::marker::Unpin for MockTcpStream {}
}
