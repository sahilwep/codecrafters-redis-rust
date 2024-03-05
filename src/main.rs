use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::str;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Server listening\n");

    loop {
        if let Ok((socket, _)) = listener.accept().await {
            tokio::spawn(handle_connection(socket));
        }
    }
}

async fn handle_connection(mut socket: TcpStream) {
    if let Ok(client_addr) = socket.peer_addr() {
        println!("Connected to {}", client_addr);
    }

    let mut buf = [0; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                if let Ok(client_addr) = socket.peer_addr() {
                    println!("Connection closed with {}", client_addr);
                }
                break;
            }
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
            Ok(n) => {
                if let Some(request) = parse(&buf[..n]) {
                    println!("\nREQUEST BEGIN ({})", socket.peer_addr().unwrap());
                    println!("\nPARSING\n{}", str::from_utf8(&buf[..n]).unwrap());
                    println!("\n{}", request);
                    let response = handle_request(&request);
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("Error writing response: {:?}", e);
                        break;
                    }
                }
            }
        }
        println!("\nREQUEST FINISHED ({})\n", socket.peer_addr().unwrap());
    }
}

fn handle_request(request: &Request) -> String {
    match &request.command[..] {
        "PING" => simple_string("PONG"),
        "ECHO" => simple_string(&request.args[0]),
        _ => simple_string("WTF"),
    }
}

const CRLF: &str = "\r\n";

fn simple_string(string: &str) -> String {
    format!("+{}{}", string, CRLF)
}

fn parse(message: &[u8]) -> Option<Request> {
    let mut slice = message;
    if slice.get(0)? != &b'*' {
        return None;
    }
    slice = &slice[1..];
    if slice.is_empty() {
        return None;
    }
    let array_len = slice.get(0)? - b'0';
    slice = &slice[1..];
    let mut elements: Vec<Vec<u8>> = Vec::with_capacity(array_len as usize);
    while !slice.is_empty() {
        match slice.get(0)? {
            b'\0' => break,
            b'\r' => slice = &slice[2..],
            b'$' => {
                slice = &slice[1..];
                let element_len = str::from_utf8(slice.get(..2)?.as_ref())?.parse().ok()?;
                slice = &slice[2..];
                let element = slice.get(..element_len)?.to_vec();
                elements.push(element);
                slice = &slice[element_len..];
            }
            _ => return None,
        }
    }
    Some(Request {
        command: String::from_utf8(elements.get(0)?.to_vec()).ok()?.to_uppercase(),
        args: elements.get(1..)?.iter().map(|x| String::from_utf8(x.to_vec()).ok()).collect::<Option<Vec<String>>>()?,
    })
}

struct Request {
    command: String,
    args: Vec<String>,
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "COMMAND: {}\nARGS ({}): [{}]",
            self.command,
            self.args.len(),
            self.args
                .iter()
                .fold(String::new(), |acc, x| acc + x + ", ")
        )
    }
}
