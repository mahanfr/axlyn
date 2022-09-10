use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

pub async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut _paths: Vec<(&str, Method, bool)> = Vec::new();

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
