use std::{
    fs::{self},
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use web_server_chappter_20::ThreadPool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(_) => panic!("Error creating listener"),
    };

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(err) => panic!("error: {}", err),
        };
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = String::new();

    if let Some(request_line) = buf_reader.lines().next() {
        match request_line {
            Ok(line) => line,
            Err(_) => panic!("Could not read reuqest line"),
        }
    } else {
        panic!("Unable to read request line")
    };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
            Content-Length: {length}\r\n\r\n
            {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
