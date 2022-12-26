use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::path::Path;
use regex::Regex;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let re = Regex::new(r"^(.*) (.*) (.*)$").unwrap();
    let caps = re.captures(&request_line).unwrap();
    let pathname = caps.get(2).map_or("", |m| m.as_str());
    let mut filename = "index";
    if pathname != "/" {
        filename = &pathname[1..];
    }

    let mut status_line = "HTTP/1.1 200 OK";
    let mut contents = String::new();

    let full_file_path = format!("{}{}{}", "public/", filename, ".html");
    if Path::new(&full_file_path).exists() {
        status_line = "HTTP/1.1 200 OK";
        contents = fs::read_to_string(&full_file_path).unwrap();
    }

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}