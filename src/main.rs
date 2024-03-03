use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind");    // create a listener server that listens on localhost port 6379
    println!("[+] TcpListener start at 127.0.0.1:6379");
    // handling incoming multiple incoming client through loop, this will make them in queue, while only one client will connect to our server.
    for stream in listener.incoming() {
        match stream {  // using match as it return in "Result", to handel handel success & failure 
            Ok(stream) => {
                // println!("{:?}",stream); // print this to know about the TcpStream details.
                println!("\n[+] Client has connected to server!");  // simple connected message will pop on server side.
                handle_client(stream);  // calling our function to handel our client
                println!("[+] Respond Successfully!");
            }
            Err(e) => {
                eprint!("Server can't connect to client: {}", e);   // handling error when our server fails to connect with client.
            }
        }
    }
}


// Create a function to handle our client
fn handle_client(mut stream: TcpStream) {

    let mut buf = [0; 512]; // Create an array of size 512, with default value '0', to store the incoming message that are comes from client.   NOTE : we take 512 max size, because as redis default size(it can be change manually) is 512mb to handel response from user.
    let _message = b"+PONG\r\n";    // Message by server to client . This is an array of string.
    // using loop to handel specific user until we respond them.
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");    // this is used to read message from client
        
        // println!("{bytes_read}");   // this is the number of bytes of string.

        // this states that if we have not received anything from the client then we returns.
        if bytes_read == 0 {
            return;
        }
        // println!("message from client : {:?}", buf);    // this is the ASCII value of an array buf, that stores the client mess.
        
        println!("[+] Responding to the client!");
        stream.write_all(&_message[0..bytes_read]).expect("Failed to write to client"); // this statement used to respond the Client.
    }
}