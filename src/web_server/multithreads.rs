use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use threadpool::ThreadPool;

#[allow(dead_code)]
pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
    let pool = ThreadPool::new(16);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    std::thread::sleep(std::time::Duration::from_secs(5));

    let status_line = "HTTP/1.1 200 OK";
    let html = format!(include_str!("./index.html"), http_request);
    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{html}", html.len());
    stream.write_all(response.as_bytes()).unwrap();
}