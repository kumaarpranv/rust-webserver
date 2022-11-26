use std::{fs, net::TcpListener, net::TcpStream, io::prelude::*, io::BufReader};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader:BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = match fs::read_to_string("./templates/index.html") {
            Result::Ok(content) => content,
            Result::Err(err) => panic!("found error! {:?}", err),
        };
        let length = contents.len();
        let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
    else {
    }
}
