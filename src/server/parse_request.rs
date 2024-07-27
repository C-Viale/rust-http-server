use std::{
  io::{BufRead, BufReader},
  net::TcpStream,
  vec,
};

struct QueryParam {
  key: String,
  value: String,
}

pub struct HttpRequest {
  method: String,
  path: String,
  query: Vec<QueryParam>,
  content_type: String,
}

pub fn parse_request(stream: &TcpStream) -> HttpRequest {
  let buf_reader = BufReader::new(stream);

  let http_request: Vec<String> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  let first_part: Vec<&str> = http_request[0].split(" ").collect();

  let method = first_part[0].to_string();
  let full_path = first_part[1].to_string();

  let content_type_option = http_request.iter().find(|val| val.contains("Content-Type"));
  let mut content_type = String::new();

  match content_type_option {
    Some(val) => content_type.push_str(val),
    None => content_type.push_str("text/plain"),
  }

  let full_path_vector: Vec<_> = full_path.split('?').collect();
  let path = full_path_vector.get(0).unwrap().to_string();
  let mut query: Vec<QueryParam> = vec![];

  if let Some(full_query) = full_path_vector.get(1) {
    for q in full_query.split('&') {
      query.push(split_query_param(q));
    }
  }

  HttpRequest {
    method,
    path,
    content_type,
    query,
  }
}

fn split_query_param(str: &str) -> QueryParam {
  let vector: Vec<_> = str.split('=').collect();

  let key = vector.get(0).unwrap().to_string();
  let mut value = String::new();

  if let Some(idx) = vector.get(1) {
    value.push_str(idx)
  }

  println!("key {key}, value {value}");

  QueryParam { key, value }
}
