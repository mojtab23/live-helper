use std::fmt::Debug;
use std::io::Bytes;
use std::str;

use futures::{future, Stream};
use hyper::{Body, Chunk, Client, header, Method, Request, Response, Server, StatusCode};
use hyper::header::{AUTHORIZATION, CONTENT_LENGTH};
use hyper::rt::Future;
use hyper::service::service_fn;

const PORT: u16 = 3000;
const NOTFOUND: &[u8] = b"404\nNot Found";

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

    return match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let auth_header_op = req.headers().get(AUTHORIZATION);
            println!("{:?}", req);
            match auth_header_op {
                Some(value) => println!("Authorization: {:?}", value),
                _ => ()
            }

            *response.body_mut() = Body::from("Ok");
            Box::new(future::ok(response))
        }
        (&Method::POST, "/") => {
            println!("{:?}", req);
            let (parts, body) = req.into_parts();
            let auth_header_op = parts.headers.get(AUTHORIZATION);

            match parts.headers.get(CONTENT_LENGTH) {
                Some(len) => {
                    println!("len: {:?}", len);
                }
                _ => ()
            };


            let request = body.concat2()
                .map(move |chunk| {
                    let body = chunk.iter()
                        .cloned()
                        .collect::<Vec<u8>>();
                    let result = str::from_utf8(body.as_slice());
                    match result {
                        Ok(string) => println!("fucking body:{}", string),
                        _ => ()
                    }
                    *response.body_mut() = Body::from(body);
                    response
                });

            match auth_header_op {
                Some(value) => println!("Authorization: {:?}", value),
                _ => ()
            }

//            *response.body_mut() = Body::from("Ok");
            Box::new(request)
        }
        _ => {
            let body = Body::from(NOTFOUND);
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = body;
            Box::new(future::ok(response))
        }
    };
}

//fn hello_world(_req: Request<Body>) -> Response<Body> {
//    Response::new(Body::from(PHRASE))
//}
