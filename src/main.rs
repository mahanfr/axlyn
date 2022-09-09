use http_body::Full;
use hyper::body::Bytes;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use std::error::Error;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use tokio::join;
use tokio::net::TcpListener;

mod logger;
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

async fn index2(req: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    if req.uri().path() == "/" && req.method() == Method::GET {
        return Ok(
            get_file("resources/index.html").await.unwrap()
        )
    }else if req.uri().path().starts_with("/static") && req.method() == Method::GET {
        return Ok(
            get_file(format!("resources{}",req.uri().path()).as_str()).await.unwrap()
        )
    }else if req.uri().path().starts_with("/fonts") && req.method() == Method::GET {
        return Ok(
            get_file(format!("resources/static{}",req.uri().path()).as_str()).await.unwrap()
        )
    }else if req.uri().path() == "/favicon.ico" && req.method() == Method::GET {
        return Ok(
            get_file("resources/favicon.ico").await.unwrap()
        )
    }
    Ok(not_found().await)
}

async fn get_file(filename: &str) -> Result<Response<Full<Bytes>>,Box<dyn Error>> {
    if let Ok(contents) = tokio::fs::read(filename).await {
        let body = contents.into();
        return Ok(Response::new(Full::new(body)));
    }

    Ok(not_found().await)
}

async fn not_found() -> Response<Full<Bytes>>{
    if let Ok(contents) = tokio::fs::read("resources/404.html").await {
        let body = contents.into();
        return Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(body))
        .unwrap()
    }
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("not found".into()))
        .unwrap()
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
    let addr = SocketAddr::from(
        format!("{}:{}", CONFIG.server_addr, CONFIG.server_port)
            .parse::<SocketAddr>()
            .unwrap(),
    );
    let admin_addr = SocketAddr::from(
        format!("{}:{}", CONFIG.admin_addr, CONFIG.admin_port)
            .parse::<SocketAddr>()
            .unwrap(),
    );

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

    logger::debug("Application is in development stage");
    logger::info(format!("Listening on http://{} and http://{}", addr, admin_addr).as_str());

    if CONFIG.debug == true {
        logger::warning("Application is in development stage");
    }

    let _ret = join!(api_server, admin_server);
}
