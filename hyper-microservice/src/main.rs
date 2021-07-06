use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

// Implementing Future trait result hyper::Response<Body> or hyper::Error
fn microservice_handler(req: Request<Body>) -> impl Future<Item=Response<Body>, Error=Error> {
    // r# is a multiline string in Rust
    const INDEX: &'static str = r#"
        <!doctype html>
        <html>
            <head>
                <title>Rust Microservice</title>
            </head>
            <body>
                <h3>Rust Microservice</h3>
            </body>
        </html>
    "#;

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            future::ok(response)
        },
    }
}

fn main() {
    
    // Binding local address
    let addr = ([0, 0, 0, 0], 8080).into();
    // Server instance, it returns Builder struct
    let builder = Server::bind(&addr);

    // Default handler to accept requests
    // Using service_fn as handlers for request results
    let server = builder.serve(|| service_fn(microservice_handler));

    // Dropping server errors
    let server = server.map_err(drop);

    hyper::rt::run(server)
}
