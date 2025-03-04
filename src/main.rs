use zero2prod::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Should bind to socket address with random port");
    println!("listening on {:?}", listener.local_addr());
    run(listener)?.await
}
