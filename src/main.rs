use futures::future;
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use hyper::header::AUTHORIZATION;
use hyper::rt::Future;
use hyper::service::service_fn;

const PORT: u16 = 8080;

// Just a simple type alias
type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;


fn main() {
    println!("Server running on port: {}", PORT);
    // This is our socket address...
    let addr = ([0, 0, 0, 0], PORT).into();
// A `Service` is needed for every connection, so this
// creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn(echo)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

// Run this server for... forever!
    hyper::rt::run(server);
}

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let auth_header_op = req.headers().get(AUTHORIZATION);
            println!("{:?}", req);
            match auth_header_op {
                Some(value) => println!("Authorization: {:?}", value),
                _ => ()
            }

            *response.body_mut() = Body::from("Ok");
        }
        (&Method::POST, "/") => {
            let auth_header_op = req.headers().get(AUTHORIZATION);
            println!("{:?}", req);
            match auth_header_op {
                Some(value) => println!("Authorization: {:?}", value),
                _ => ()
            }

            *response.body_mut() = Body::from("Ok");
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

//fn hello_world(_req: Request<Body>) -> Response<Body> {
//    Response::new(Body::from(PHRASE))
//}
