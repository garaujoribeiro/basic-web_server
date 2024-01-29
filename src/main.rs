use std::{
    fs,
    io::{ BufRead, BufReader, Write },
    net::{ TcpListener, TcpStream },
    thread,
    time::Duration,
};

use basic_web_server::ThreadPool;


fn main() {
    let address: String = String::from("127.0.0.1:7878");
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind(&address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    handle_response(stream, status_line, path)
}

fn handle_response(mut stream: TcpStream, status_line: &str, path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
}
