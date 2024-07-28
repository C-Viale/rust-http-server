use std::{
  io::Write,
  net::{TcpListener, TcpStream},
  sync::{Arc, Mutex},
};

use crate::thread_pool::ThreadPool;

mod parse_request;
use parse_request::parse_request;

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

  pub fn listen(self, port: u16) {
    let host = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(host).unwrap();
    let pool = ThreadPool::new(4);

    let server = Arc::new(Mutex::new(self));

    for stream in listener.incoming() {
      let stream = stream.unwrap();
      let server = Arc::clone(&server);

      pool.execute(move || {
        let server = server.lock().unwrap();
        server.handle_connection(stream);
      });
    }
  }

  fn handle_connection(&self, mut stream: TcpStream) {
    let request = parse_request(&stream);
    let body = "{\"a\":\"b\"}".to_string();
    let mut response = String::new();

    if let Some(request_handler) = self
      .handlers
      .iter()
      .find(|h| h.path == request.path && request.method == h.method)
    {
      let res = (request_handler.handler)();
      response.push_str(&make_response(200, &res));
    } else {
      response.push_str(&make_response(404, &body));
    }

    stream.write_all(response.as_bytes()).unwrap();
  }
}

pub struct RequestHandler {
  pub method: &'static str,
  pub path: &'static str,
  pub handler: Box<dyn Fn() -> String + Send + 'static>,
}
