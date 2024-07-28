use http_server::{
  self, handlers,
  server::{HttpServer, RequestHandler},
};

fn main() {
  let mut server = HttpServer::new();

  let server_handlers = vec![RequestHandler {
    method: "GET",
    path: "/users",
    handler: Box::new(handlers::get_users),
  }];

  server.bind_handlers(server_handlers);
  server.listen(3000);
}
