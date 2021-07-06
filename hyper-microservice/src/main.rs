use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn main() {
    
    // Binding local address
    let addr = ([0, 0, 0, 0], 8080).into();
    // Server instance, it returns Builder struct
    let builder = Server::bind(&addr);
    // Default handler to accept requests
    let server = builder.serve(|| {
        service_fn_ok(|_| {
            Response::new(Body::from("Rust microservice..."))
        })
    });

    // Dropping server errors
    let server = server.map_err(drop);

    hyper::rt::run(server)
}
