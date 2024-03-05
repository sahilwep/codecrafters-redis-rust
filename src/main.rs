use regex::Regex;
use std::fmt::{self, Display, Formatter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::vec::Vec;
static CMD: [&str; 4] = ["PING", "ECHO", "COMMANDS", "DOCS"];

struct Request {
    id: u8,
    cmd: String,
    options: Vec<String>,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "CMD: {:?}\tOPTIONS:{:?}", self.cmd, self.options)
    }
}

struct Answer {
    len: usize,
    ans: String,
}

/* No working
impl Display for Answer{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "*1\r\n{:?}\r{:?}\r\n", self.len, self.ans)
    }
}*/

fn send_answer(mut stream: &TcpStream, ans: Answer) {
    let res = format!("${:?}\r\n{:?}\r\n", ans.len, ans.ans).replace("\"", "");
    let res_as_bytes = res.as_bytes();
    match stream.write_all(res_as_bytes) {
        Ok(_) => {
            println!("[{:?}] {:?} is sent", stream.peer_addr(), res);
            println!("[{:?}] {:?} is sent", stream.peer_addr(), res.as_bytes());
        }
        Err(_) => {
            println!("[{:?}] {:?} failed to be send", stream.peer_addr(), res);
        }
    }
}

fn main() {

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        let handle_stream = thread::spawn(move || match stream {
            Ok(_stream) => {
                println!("Accepted new connection [{:?}]", _stream.peer_addr());
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let data = String::from_utf8_lossy(&buffer[..size]);
                println!(
                    "[{:?}] bytes received: {:?} content: {:?}",
                    stream.peer_addr(),
                    size,
                    data
                );
                let pong = b"+PONG\r\n";
                match stream.write_all(pong) {
                    Ok(_) => {
                        println!(
                            "[{:?}] {:?} message is sent",
                            stream.peer_addr()
                            ,String::from_utf8_lossy(pong)
                        );
                    }
                    Err(e) => {
                        println!("[{:?}] {:?} message failed to be sent, error: {:?}", stream.peer_addr(), String::from_utf8_lossy(pong), e );
                        return;
                    }
                }
                if size == 0 {
                    println!("Disconnection occured with {:?}", stream.peer_addr());
                    break;
                }
                let collected_data: Vec<String> = collect_input_data(buffer, size, &stream);
                let my_requests: Vec<Request> = parse_collected_data(collected_data);
                for request in my_requests {
                    println!("{}", request);
                    let ans = get_answer_from_request(request);
                    send_answer(&stream, ans);
                }
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
            }
        }
        thread::sleep(Duration::from_millis(5));
        thread::sleep(Duration::from_millis(500));
    }
}



fn collect_input_data(buffer: [u8; 1024], size: usize, stream: &TcpStream) -> Vec<String> {
    let data = String::from(String::from_utf8_lossy(&buffer[..size]));
    println!(
        "[{:?}] bytes received: {:?} content: {:?}",
        stream.peer_addr(),
        size,
        data
    );
    let mut collected_data: Vec<String> = Vec::new();
    for s in data.split("\r\n") {
        if !s.is_empty() {
            collected_data.push(String::from(s));
        }
    }
    return collected_data;
}


fn parse_collected_data(collected_data: Vec<String>) -> Vec<Request> {
    let mut my_requests: Vec<Request> = Vec::new();
    let mut index: usize = 0;
    let bulk_string = Regex::new(r"^\$[0-9]").unwrap();
    for cmd in &collected_data {
        if CMD.contains(&cmd.to_uppercase().as_str()) {
            let mut cmd_options: Vec<String> = Vec::new();
            for option in collected_data.iter().skip(index + 1) {
                if CMD.contains(&option.to_uppercase().as_str()) {
                    break;
                } else if bulk_string.is_match(option) {
                    continue;
                }
                cmd_options.push(String::from(option));
            }
            let new_request = Request {
                id: 0,
                cmd: cmd.to_uppercase(),
                options: cmd_options,
            };
            my_requests.push(new_request);
        }
        index += 1;
    }
    return my_requests;
}


fn get_answer_from_request(my_request: Request) -> Answer {
    match my_request.cmd.as_str() {
        "PING" => {
            return Answer {
                len: String::from("PONG").len(),
                ans: String::from("PONG"),
            };
        }
        "ECHO" => {
            return Answer {
                len: my_request.options.join(" ").len(),
                ans: my_request.options.join(" "),
            };
        }
        _ => {
            println!("Unknown command");
            return Answer {
                len: String::from("PONG").len(),
                ans: String::from("PONG"),
            };
        }
    }
}