use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use bytes::buf;    // using thread to make this asynchronous

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind");    // create a listener server that listens on localhost port 6379
    println!("[+] TcpListener start at 127.0.0.1:6379");
    // handling incoming multiple incoming client through loop, this will make them in queue, while only one client will connect to our server.
    for stream in listener.incoming() {
        match stream {  // using match as it return in "Result", to handel handel success & failure 
            Ok(stream) => {
                // thread implementation
                thread::spawn( || {
                    println!("\n[+] Client has connected to server!");  // simple connected message will pop on server side.
                    // println!("{:?}",stream); // print this to know about the TcpStream details.
                    handle_client(stream);  // calling our function to handel our client
                    println!("[+] Respond Successfully!");
                });
            }
            Err(e) => {
                eprint!("Server can't connect to client: {}", e);   // handling error when our server fails to connect with client.
            }
        }
    }
}


// Create a function to handle our client
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let byte_read = stream.read(&mut buf).unwrap();     // used to calculate length of incoming string

        if byte_read == 0 {
            return;
        }
        let request = String::from_utf8_lossy(&buf[..]);    // request is the incoming string from client.

        // Performing regex : spiting our string by "\r\n".
        request.split("\r\n").for_each(|line| {
            if line == "ping" {     // matching ping with line
                stream.write(b"+PONG\r\n").unwrap();    // writing it to the client.
            } else {
                stream.write_all(&buf[0..byte_read]).unwrap();
            }
        });
    }
}
