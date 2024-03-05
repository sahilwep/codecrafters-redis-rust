use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    str,
};
    
    
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Server listening\n");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("Connected to {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let client_addr = socket.peer_addr().unwrap();
    println!("Connected to {client_addr}");
    loop {
        let mut buf = [0; 1024];
        match socket.read(&mut buf).await {
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
            Ok(0) => {
                println!("Connection closed with {client_addr}");
                return;
            }
            Ok(n) => {
                println!("\nREQUEST BEGIN ({client_addr})");
                println!("\nPARSING\n{}", str::from_utf8(&buf[..n]).unwrap());
                let parsed = parse(&buf).unwrap();
                println!("\n{parsed}");
                let response = handle_request(parsed);
                socket.write_all(response.as_bytes()).await.unwrap();
            }
        }
        println!("\nREQUEST FINISHED ({client_addr})\n");
    }
}


fn handle_request(request: Request) -> String {
    match &request.command[..] {
        "PING" => simple_string("PONG"),
        "ECHO" => simple_string(&request.args[0]),
        _ => simple_string("WTF"),
    }
}

const CRLF: &str = "\r\n";

fn simple_string(string: &str) -> String {
    "+".to_owned() + string + CRLF
}

fn parse(message: &[u8]) -> Option<Request> {
    let mut slice = message;
    if slice[0] != b'*' {
        panic!()
    };
    // TODO: handle identifier
    println!("start {}", slice[0] as char);
    slice = &slice[1..];
    // TODO: handle multi-digit length
    println!("array len {}", slice[0] as char);
    slice = &slice[1..];
    let mut elements: Vec<Vec<u8>> = vec![];
    while !slice.is_empty() {
        match slice[0] {
            b'\0' => {
                break;
            }
            b'\r' => {
                slice = &slice[2..];
            }
            b'$' => {
                slice = &slice[1..];
                let mut element_len: usize = 0;
                while slice[0] != b'\r' {
                    element_len = element_len * 10 + usize::from(slice[0] - b'0');
                    slice = &slice[1..];
                }
                slice = &slice[2..];
                let element = slice[0..element_len].to_vec();
                elements.push(element);
                slice = &slice[element_len..];
            }
            _ => {
                panic!();
            }
        }
    }
    match elements.as_slice() {
        [first, rest @ ..] => Some(Request {
            command: String::from_utf8(first.to_vec()).unwrap().to_uppercase(),
            args: rest
                .iter()
                .map(|x| String::from_utf8(x.to_vec()).unwrap())
                .collect(),
        }),
        _ => None,
    }
}

struct Request {
    command: String,
    args: Vec<String>,
}


impl Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "COMMAND: {}\nARGS ({}): [{}]",
            self.command,
            self.args.len(),
            self.args
                .iter()
                .fold(String::new(), |acc, x| acc + &x + ", ")
        )
    }
}