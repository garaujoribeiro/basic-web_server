use std::{ fs, io::{ BufRead, BufReader, Write }, net::{ TcpListener, TcpStream } };

fn main() {
    let address: String = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(&address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let sucess_request = request_line == "GET / HTTP/1.1";

    let (status_line, path) = if sucess_request {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    handle_response(stream, status_line, path)
}

fn handle_response(mut stream: TcpStream, status_line: &str, path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
}
