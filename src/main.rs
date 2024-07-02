use std::{io::{Read, Write}, net::{TcpListener , TcpStream}};
use std::fs;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    println!("listening on port 8000");
    println!("server : 127.0.0.1:8000");
    
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle(stream);
    }
}

fn handle(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else   {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = "404 Not Found";
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
