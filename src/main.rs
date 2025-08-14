use std::{
    fs::{self},
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use web_server_chappter_20::ThreadPool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(_) => return,
    };

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(_) => return,
        };
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line: String;

    if let Some(line) = buf_reader.lines().next() {
        request_line = match line {
            Ok(line) => line,
            Err(_) => return,
        }
    } else {
        panic!("Unable to read request line")
    };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => return,
    };

    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n
        {contents}"
    );

    match stream.write_all(response.as_bytes()) {
        Ok(res) => res,
        Err(_) => panic!("Couldn't write to stream"),
    };
}
