use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

const HOST: &'static str = "127.0.0.1"; // Serverのlistenアドレス
const PORT: u32 = 7878; // サーバーのlistenポート

fn main() {
    let listener = TcpListener::bind(format!("{HOST}:{PORT}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // リクエストが来るたびにスレッドを起動、下で定義されたhandle_connectionを実行する
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Hello, World."),
        _ => ("HTTP/1.1 404 NOT FOUND", "Not Found"),
    };
    let length = content.len();
    let response = format! {"{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"};

    stream.write_all(response.as_bytes()).unwrap();
}
