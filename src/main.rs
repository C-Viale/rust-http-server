use std::{
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

mod thread_pool;
use thread_pool::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

  let pool = ThreadPool::new(4);

  // accept connections and process them serially
  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);

  let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  println!("{}", http_request.join(","));

  let status = "HTTP/1.1 200 OK";

  let content = "{\"a\":\"b\"}";
  let length = content.len();

  let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

  stream.write_all(response.as_bytes()).unwrap();
}
