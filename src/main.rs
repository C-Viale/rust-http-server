use http_server::{
  self,
  server::{HttpServer, RequestHandler},
};

fn main() {
  let server = HttpServer::new();

  let handlers = vec![RequestHandler {
    method: "GET",
    path: "/teste",
  }];

  // server.clone().bind_handlers(handlers);
  server.listen(3000);
}
