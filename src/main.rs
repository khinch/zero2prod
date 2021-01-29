use zero2prod::run;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    println!("Attempting to bind to address: 127.0.0.1:{}", listener.local_addr().unwrap().port());
    run(listener)?.await
}