use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let status_line = "HTTP/1.1 200 OK";
    let html = format!(include_str!("./index.html"), http_request);
    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{html}", html.len());
    stream.write_all(response.as_bytes()).unwrap();
}