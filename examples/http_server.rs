// Simple HTTP server for Nova
// Compile: rustc --target wasm32-wasi server.rs -o server.wasm
// Run: nova run server.wasm

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\
                   Content-Type: text/html\r\n\
                   \r\n\
                   <!DOCTYPE html>\
                   <html>\
                   <head><title>Nova Server</title></head>\
                   <body>\
                   <h1>🚀 Running on Nova!</h1>\
                   <p>This is a WebAssembly server running in Nova container.</p>\
                   </body>\
                   </html>";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("🚀 Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
