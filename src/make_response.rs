pub fn make_response(status: u16, body: &String) -> String {
  let message = get_status_message(status);
  let length = body.len();

  return format!("HTTP/1.1 {status} {message}\r\nContent-Length: {length}\r\n\r\n{body}");
}

fn get_status_message(status: u16) -> String {
  let message = match status {
    200 => "OK",
    201 => "Created",
    400 => "Bad Request",
    401 => "Unauthorized",
    _ => "OK",
  };

  return message.to_string();
}
