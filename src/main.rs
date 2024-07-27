use std::net::TcpListener;

mod thread_pool;
use thread_pool::ThreadPool;

use http_server::handle_connection;

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
