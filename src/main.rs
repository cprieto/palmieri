use std::net::TcpListener;
use palmieri::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Unable to bind to random port");

    let port = listener.local_addr().unwrap().port();
    println!("Running at http://127.0.0.1:{port}");

    run(listener)?.await
}
