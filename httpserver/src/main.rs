
mod server;
mod handler;
mod router;


fn main() {
    server::Server::new("localhost:3000").run();

}