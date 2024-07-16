use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

struct ConnectionContext {
    data_map: HashMap<String, String>,
}

impl ConnectionContext {
    fn new() -> Self {
        Self {
            data_map: HashMap::new(),
        }
    }
    fn set(&mut self, key: String, value: String) {
        self.data_map.insert(key, value);
    }
    fn get(&self, key: String) -> String {
        self.data_map.get(&key).unwrap().to_string()
    }
}

fn tokenize_command(command: &str) -> Vec<&str> {
    let commands: Vec<&str> = command.split("\r\n").collect();
    let mut tokens: Vec<&str> = Vec::new();
    if commands.len() == 0 {
        return Vec::new();
    }
    let first_token = commands[0];
    if first_token.chars().nth(0).unwrap() == '*' {
        let num_in_array = first_token[1..first_token.len()].parse::<i32>().unwrap();
        if num_in_array * 2 + 1 <= commands.len() as i32 {
            for i in (1..num_in_array * 2 + 1).step_by(2) {
                let token_type = commands[i as usize];
                let value = commands[i as usize + 1];
                if token_type.chars().nth(0).unwrap() == '$' {
                    let num_chars = token_type[1..token_type.len()].parse::<i32>().unwrap();
                    if num_chars == value.len() as i32 {
                        tokens.push(value);
                    }
                }
            }
        }
    }
    return tokens;
}

fn make_response_str(responses: Vec<String>) -> String {
    if responses.len() == 0 {
        return "*0\r\n".to_string();
    } else if responses.len() == 1 {
        return responses[0].to_owned();
    } else {
        let mut str = String::new();
        str += "*";
        str += &responses.len().to_string();
        str += "\r\n";
        for resp in responses {
            str += &resp;
            str += "\r\n";
        }
        return str;
    }
}


fn handle_command(command: &str, context: &mut ConnectionContext) -> Vec<u8> {
    let tokens = tokenize_command(command);
    if tokens.len() == 0 {
        return b"*0\r\n".to_vec();
    }
    let mut i = 0;
    let mut responses: Vec<String> = Vec::new();
    while i < tokens.len() {
        let command = tokens[i].to_lowercase();
        if command == "ping" {
            responses.push("+PONG\r\n".to_owned());
            i += 1;
        } else if command == "echo" {
            if i + 1 >= tokens.len() {
                responses.push("*-1\r\n".to_owned());
                i += 1;
                continue;
            }
            let mut resp_str = String::new();
            resp_str += "$";
            resp_str += &tokens[i + 1].len().to_string();
            resp_str += "\r\n";
            resp_str += tokens[i + 1];
            resp_str += "\r\n";
            responses.push(resp_str);
            i += 2;
        } else if command == "set" {
            println!("Set command called");
            if i + 2 >= tokens.len() {
                responses.push("*-1\r\n".to_owned());
                i += 1;
                continue;
            }
            context.set(tokens[i + 1].to_owned(), tokens[i + 2].to_owned());
            responses.push("+OK\r\n".to_owned());
            i += 3;
        } else if command == "get" {
            println!("get command called");
            if i + 1 >= tokens.len() {
                responses.push("*-1\r\n".to_owned());
                i += 1;
                continue;
            }
            let value = context.get(tokens[i + 1].to_owned());
            let mut resp_str = String::new();
            resp_str += "$";
            resp_str += &value.len().to_string();
            resp_str += "\r\n";
            resp_str += &value;
            resp_str += "\r\n";
            responses.push(resp_str);
            i += 2;
        } else {
            i += 1;
        }
    }
    let response_str = make_response_str(responses);
    // println!("{}", response_str);
    return response_str.as_bytes().to_vec();
}


fn handle_client(mut stream: std::net::TcpStream) {
    let mut connection_context = ConnectionContext::new();
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        let bytes_read: usize = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        let str = std::str::from_utf8(&buf[..bytes_read]).unwrap();
        let response = handle_command(str, &mut connection_context);
        stream.write_all(&response).unwrap();
        stream.flush().unwrap();
    }
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                thread::spawn(move || {
                    handle_client(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}