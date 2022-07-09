// 引用类库 io、net 
use std::{io::{Read, Write}};
use std::{net::{TcpListener, TcpStream}};
// 引入 thread 类库用来多线程处理
use std::thread;
// 引入 str 库，用来转换byte与str类型
use std::str;

fn main() { 
    // IP:端口
    let ip = "127.0.0.1:8888".to_string();
    // 创建一个Tcp监听，通过字符串切片将ip传入，可以通过 lsof -i:8888 进行验证端口启动
    let listener = TcpListener::bind(&ip).unwrap();
    // 调用 incoming() 方法接收客户端的链接信息
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 如果链接成功，开启一个新的线程，之所以用多线程的原因是TCP客户请求可能有多个。
                thread::spawn(move|| {
                    // 将客户端处理信息解耦到 handle_client 函数中，并移交 stream 变量所有权
                    handle_client(stream);
                });
            }
            Err(e) => { 
                // panic直接退出程序。
                panic!("错误 {:?}", e)
            }
        }
    }

    // 关闭Tcp监听链接
    drop(listener);
}

/**
* @param stream: TcpStream  传入的输入流
*/
fn handle_client(mut stream: TcpStream) {
    // 定义一个存储用的数组，因为需要后续进行填充值所以声明为可变的 `mut`
    let mut buf = [0; 512];
    // 建立一个循环来反复读取客户的输入信息
    loop {
        // 通过read方法
        let bytes_read = stream.read(&mut buf).expect("读取出现错误，这里直接中断程序运行");

        if bytes_read == 0 {
            // 退出loop
            break;
        }

        // 将byte[] 转换为str 类型。
        let s = match str::from_utf8(&buf[..bytes_read]) {
            // 如果转换成功返回字符串值。
            Ok(v) => v,
            // 遇到转换错误输出错误信息，终止程序运行。
            Err(_e) => {
                // 输出调试信息。
                stream.write(b"Need utf-8 sequence.").unwrap();
                // 继续监听，虽然本次输入的字符流格式不是utf8 字符，但是不影响下次输入所以不需要 panic!
                continue;
            },
        };

       // 输出调试信息
       println!("客户端传信息:{}", s);

       // 不处理错误，用 unwrap 或 expect 直接将正确值取出来，如果出错了就直接让程序挂掉，其中 unwrap 在挂掉时是标准库内置的错误提示，而 expect 则让我们可以自己定义一个字符串在程序挂掉时显示。
        stream.write(b"Server: ").unwrap();
        stream.write(&buf[..bytes_read]).unwrap();
    }
}