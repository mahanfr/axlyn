use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
mod utils;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PATHS: Vec<(&'static str, Method, bool)> = {
        let mut v = Vec::new();
        read_path_config(&mut v)
    };
}


async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut _paths : Vec<(&str,Method,bool)> = Vec::new();
    _paths = PATHS.to_vec();

    for path in _paths{
        if req.uri().path() == path.0 && req.method() == path.1 {
            return Ok(Response::new("Hello world!".into()))
        }else if req.uri().path() == path.0 && req.method() != path.1{
            return Ok(Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body("Method Is Not Acceptable!".into())
            .unwrap())
        }
    }
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found!".into())
        .unwrap())
}

fn read_path_config(list : &mut Vec<(&'static str, Method, bool)>) -> Vec<(&'static str, Method, bool)> {
    let file = File::open("./routes.conf").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let ln = Box::leak(Box::new(line.unwrap()));
        let values : Vec<&'static str> = ln.split_whitespace().collect();
        let path = values[0];
        let method = Method::from_bytes(values[1].as_bytes()).unwrap();
        list.push((path,method,false))
    }
    list.to_vec()
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    // unsafe{
    //     PATHS.push(("/",Method::GET,false));
    //     PATHS.push(("/path",Method::GET,false));
    // }
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
