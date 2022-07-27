use std::io::prelude::*; // read and write a stream
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;


fn main() {
    // bind will create new tcp listener that listens to the given address+port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    // incoming will give us an iterator of streams
    for stream in listener.incoming() {
        let stream = stream.unwrap(); 

        handle_connection(stream);
    }
}

// When working with streams we are working with bytes and not strings
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Holds data from request
    stream.read(&mut buffer).unwrap(); //  read bytes from the TcpStream and put them in the buffer

    let get = b"GET / HTTP/1.1\r\n"; // b for coercing get to a byte string

    // If the server recieves a response that starts with GET than we're good
    // 'status_line' and 'filename' will be init based on the response
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html") 
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap(); // read html-body

    // response message based on whether we got a 200 or 404
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", // ensure valid HTTP-response
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap(); // write response to client
    stream.flush().unwrap(); // wait until all bytes are written to connection
}