use std::io::{Result, prelude::*};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::{thread, time};

// 处理消息
fn process_stream(process_name: String, mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];


    // 读取消息buffer数据
    while match stream.read(&mut buffer) {
        Ok(n) if n == 0 => false,
        Ok(n) => {
            // buffer 转 字符串，发送内容
            let msg = String::from_utf8_lossy(&buffer[..n]);

            // 打印 消息
            println!(
                "[{}] [process_stream] [message] {:?}",
                if process_name == "server".to_string() {
                    "client".to_string()
                } else {
                    "server".to_string()
                },
                msg
            );

            // 服务端 对话
            if process_name == "server" {
                if msg == "hello" {
                    stream.write(&buffer[..n]).unwrap();
                }
                if msg == "My name is muniz" {
                    stream.write(b"I don't want to talk to you, bye").unwrap();
                }

                if msg == "stop" {
                    stream.write(b"stop").unwrap();
                }
            }

            // 客户端 对话
            if process_name == "client" {
                if msg == "hello" {
                    stream.write(b"My name is muniz").unwrap();
                }
                if msg == "I don't want to talk to you, bye" {
                    stream.write(b"stop").unwrap();
                }
                if msg == "stop" {
                    stream.shutdown(Shutdown::Both).unwrap();
                    println!("\n[end'''] oneblock tcp server\n");

                    std::process::exit(0);
                }
            }

            true
        }
        Err(_) => false,
    } {}

    Ok(())
}

// 服务端线程
fn worker_server() {
    // 绑定IP+端口，监听服务
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("[server] listener 127.0.0.1:3000 ");

    // 遍历每一个进入的 链接
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("[server] waiting for next message");

        // 调用集中处理消息体函数
        process_stream("server".to_string(), stream).unwrap();
    }
}

// 客户端线程
fn worker_client() {
    // 连接服务端
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    println!("[client] connect 127.0.0.1:3000 ");

    stream.write(b"hello").unwrap();

    // 调用集中处理消息体函数
    process_stream("client".to_string(), stream).unwrap();
}

fn main() {
    println!("\n[start'] oneblock tcp server\n");

    let handle_server = thread::spawn(worker_server);
    let handle_client = thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(300));
        worker_client();
    });

    handle_server.join().unwrap();
    handle_client.join().unwrap();
}
