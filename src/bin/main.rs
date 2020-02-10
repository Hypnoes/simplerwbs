use simplerwbs::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get_v10 = b"GET / HTTP/1.0\r\n";
    let get_v11 = b"GET / HTTP/1.1\r\n";
    let get_v20 = b"GET / HTTP/2.0\r\n";

    let (status_line, filename) = if buffer.starts_with(get_v10) ||
        buffer.starts_with(get_v11) || buffer.starts_with(get_v20) {
        ("HTTP/2.0 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/2.0 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}