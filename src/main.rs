use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*,BufReader}
};
mod middleware;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handel_connection(stream);
        //println!("connection stablished")
    }
}

fn handel_connection(mut stream: TcpStream){
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let mut buf = [0u8;4096];
    match stream.read(&mut buf){
        Ok(_) => {
            // middleware::handle_request(&buf);
            println!("Request: {:#?}",String::from_utf8_lossy(&buf));
        },
        Err(_) => println!("Partial Http Request"),
    }

    // println!("Request: {:#?}",http_request);
    let response = "HTTP/1.1 200 OK\r\n\r\nHello";
    stream.write_all(response.as_bytes()).unwrap();
}

mod http_parser{
    use core::fmt;


    struct Header {
        name: String,
        value: String
    }

    #[derive(Copy,Clone,PartialEq,Eq,Debug)]
    enum Error{
        BadRequestError,
        VersionError,
        StatusError,
        HeaderNameError,
        HeaderValueError,
        InvalidNewLineError,
        TokenError,
        TooManyHeadersError,
        InvalidChunkSize,
    }

    impl Error{
        #[inline]
        fn description(&self) -> &'static str {
            match *self{
                Error::BadRequestError => "Invalid Request",
                Error::VersionError => "Invalid Http Version",
                Error::StatusError => "Invalid Status Number",
                Error::HeaderNameError => "Invalid Header Name",
                Error::HeaderValueError => "Invalid Header Value",
                Error::InvalidNewLineError => "Invalid New Line",
                Error::TokenError => "Invalid Token",
                Error::TooManyHeadersError => "Too Many Headers",
                Error::InvalidChunkSize => "Invalid Chunk Size",
            }
        }
    }

    impl fmt::Display for Error{
        fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
            f.write_str(self.description())
        }
    }

    struct Request {
        version: u8,
        url: &'static str,
        method: &'static str,
        is_partial: bool,
        headers: Vec<Header>,
        content: Vec<u8>
    }
    struct Response {
        version: u8,
        status_code: u16,
        headers: Vec<Header>,
        content: Vec<u8>
    }

    fn parse_request(buf:&[u8]) -> Result<Request,Error>{
        return Err(Error::BadRequestError)
    }
}