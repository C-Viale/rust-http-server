use std::{
  io::Write,
  net::{TcpListener, TcpStream},
};

use crate::thread_pool::ThreadPool;

mod parse_request;
use parse_request::{parse_request, HttpRequest};

mod make_response;
use make_response::make_response;

pub struct HttpServer {
  handlers: Vec<RequestHandler>,
}

impl HttpServer {
  pub fn new() -> HttpServer {
    HttpServer { handlers: vec![] }
  }

  pub fn bind_handlers(&mut self, handlers: Vec<RequestHandler>) {
    self.handlers = handlers;
  }

  pub fn listen(&self, port: u16) {
    let host = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(host).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
      let stream = stream.unwrap();

      pool.execute(|| {
        handle_connection(stream);
      });
    }
  }
}

pub fn handle_connection(mut stream: TcpStream) {
  let request = parse_request(&stream);
  let body = "{\"a\":\"b\"}".to_string();
  let response = make_response(400, &body);

  stream.write_all(response.as_bytes()).unwrap();
}

// pub fn match_route(&request: HttpRequest) {}

pub struct RequestHandler {
  pub method: &'static str,
  pub path: &'static str,
}
