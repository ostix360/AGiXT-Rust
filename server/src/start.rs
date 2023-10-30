use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::time;


pub async fn start() {
       // We'll bind to 127.0.0.1:3000
       let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

       // A `Service` is needed for every connection, so this
       // creates one from our `hello_world` function.
       let make_svc = make_service_fn(|_conn| async {
           // service_fn converts our function into a `Service`
           Ok::<_, Infallible>(service_fn(hello_world))
       });
   
       let server = Server::bind(&addr).serve(make_svc);

       let graceful = server.with_graceful_shutdown(shutdown_signal());
   
       // Run this server for... forever!
       if let Err(e) = graceful.await {
           eprintln!("server error: {}", e);
       }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Hello from my pc");
    time::sleep(Duration::from_secs(5)).await;
    Ok(Response::new("Hello, World".into()))
}