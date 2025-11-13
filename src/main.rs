mod http_request;
mod http_response;
mod server;
mod router;
mod handler;

use server::Server;
fn main() {
    let server = Server::new("127.0.0.1:8080");

    server.run();
}
