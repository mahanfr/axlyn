use colored::Colorize;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use tokio::join;
use tokio::net::TcpListener;

mod utils;

#[macro_use]
extern crate lazy_static;

// TODO: check this line before publishing
const DEV: bool = true;

lazy_static! {
    static ref PATHS: Vec<(&'static str, Method, bool)> = {
        let mut v = Vec::new();
        read_path_config(&mut v)
    };
    #[derive(Debug)]
    static ref CONFIG : utils::AppConfig = utils::get_app_config().unwrap();
}

async fn index1(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut _paths: Vec<(&str, Method, bool)> = Vec::new();
    _paths = PATHS.to_vec();

    for path in _paths {
        if req.uri().path() == path.0 && req.method() == path.1 {
            return Ok(Response::new("Hello world!".into()));
        } else if req.uri().path() == path.0 && req.method() != path.1 {
            return Ok(Response::builder()
                .status(StatusCode::NOT_ACCEPTABLE)
                .body("Method Is Not Acceptable!".into())
                .unwrap());
        }
    }
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found!".into())
        .unwrap())
}

async fn index2(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Admin server".into()))
}

fn read_path_config(
    list: &mut Vec<(&'static str, Method, bool)>,
) -> Vec<(&'static str, Method, bool)> {
    let file = File::open("./routes.conf").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let ln = Box::leak(Box::new(line.unwrap()));
        let values: Vec<&'static str> = ln.split_whitespace().collect();
        let path = values[0];
        let method = Method::from_bytes(values[1].as_bytes()).unwrap();
        list.push((path, method, false))
    }
    list.to_vec()
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let admin_addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let api_server = async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();

            tokio::task::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(index1))
                    .await
                {
                    println!("Error Serving connection: {:?}", err);
                }
            });
        }
    };

    let admin_server = async move {
        let listener = TcpListener::bind(admin_addr).await.unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();

            tokio::task::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(index2))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    };

    if DEV == true {
        println!(
            "{} Application is in development stage",
            " DEV-Warning ".on_yellow().bold()
        );
    }
    println!(
        "{} Listening on http://{} and http://{}",
        " Info ".on_bright_cyan().bold(),
        addr,
        admin_addr
    );

    if CONFIG.debug == true {
        println!(
            "{} Application is in development stage",
            " Warning ".on_yellow().bold()
        );
    }

    let _ret = join!(api_server, admin_server);
}
