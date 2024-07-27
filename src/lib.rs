use std::{io::Write, net::TcpStream};

mod parse_request;
use parse_request::parse_request;

mod make_response;
use make_response::make_response;

pub fn handle_connection(mut stream: TcpStream) {
  let request = parse_request(&stream);

  let body = "{\"a\":\"b\"}".to_string();
  let response = make_response(400, &body);

  stream.write_all(response.as_bytes()).unwrap();
}
