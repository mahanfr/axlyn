use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
mod middleware;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handel_connection(stream);
        //println!("connection established")
    }
}

fn handel_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            // middleware::handle_request(&buf);
            // println!("Request: {:#?}", String::from_utf8_lossy(&buf));
            http_parser::parse_request(&buf).unwrap();
        }
        Err(_) => println!("Partial Http Request"),
    }
    let response = "HTTP/1.1 200 OK\r\n\r\nHello";
    stream.write_all(response.as_bytes()).unwrap();
}

pub mod http {
    use core::fmt;

    pub struct Header {
        name: String,
        value: String,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Error {
        BadRequestError,
        VersionError,
        StatusError,
        HeaderNameError,
        HeaderValueError,
        InvalidNewLineError,
        TokenError,
        TooManyHeadersError,
        InvalidChunkSize,
        PartialRequest
    }

    impl Error {
        #[inline]
        fn description(&self) -> &'static str {
            match *self {
                Error::BadRequestError => "Invalid Request",
                Error::VersionError => "Invalid Http Version",
                Error::StatusError => "Invalid Status Number",
                Error::HeaderNameError => "Invalid Header Name",
                Error::HeaderValueError => "Invalid Header Value",
                Error::InvalidNewLineError => "Invalid New Line",
                Error::TokenError => "Invalid Token",
                Error::TooManyHeadersError => "Too Many Headers",
                Error::InvalidChunkSize => "Invalid Chunk Size",
                Error::PartialRequest => "Request is Partial"
            }
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(self.description())
        }
    }

    pub struct Request {
        pub version: u8,
        pub url: &'static str,
        pub method: &'static str,
        pub is_partial: bool,
        pub headers: Vec<Header>,
        pub content: Vec<u8>,
    }
    pub struct Response {
        pub version: u8,
        pub status_code: u16,
        pub headers: Vec<Header>,
        pub content: Vec<u8>,
    }
}

mod http_parser {
    use std::{io::BufRead};

    use crate::http::Header;

    use super::http::{Error, Request};

    #[inline]
    fn is_token(b: u8) -> bool {
        b > 0x1F && b < 0x7F
    }

    static URI_MAP: [bool; 256] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, false, true, false, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    #[inline]
    fn is_uri_token(b: u8) -> bool {
        URI_MAP[b as usize]
    }

    static HEADER_NAME_MAP: [bool; 256] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, false, true, true, true, true, true, false, false, true, true, false, true, true, false, true, true, true, true, true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    #[inline]
    fn is_header_name_token(b: u8) -> bool {
        HEADER_NAME_MAP[b as usize]
    }

    static HEADER_VALUE_MAP: [bool; 256] = [
        false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ];

    #[inline]
    fn is_header_value_token(b: u8) -> bool {
        HEADER_VALUE_MAP[b as usize]
    }

    //TODO: -> Result<Request, Error>
    pub fn parse_request(buf: &[u8]) -> Result<Request, Error>{
        // all the data except spacers
        let mut stack: Vec<u8> = Vec::<u8>::new();
        let mut current_state: State = State::RequestLine;
        // to currently find space between data
        let mut spacers = String::new();
        // This indicates different types of data in http request
        #[derive(PartialEq)]
        enum State{
            RequestLine,
            Header,
            Content,
        }

        // reading every byte one by one
        for byte in buf{
            // if state is content everything goes to stack
            if current_state == State::Content{
                stack.push(*byte);
                continue;
            }

            // check for spacer chars and add them to spacer stack
            if *byte == b'\r' || *byte == b'\n'{
                spacers.push(*byte as char);
            }
            // if byte is something other than spacers
            else {
                if spacers.len() < 1{
                    stack.push(*byte);
                    continue;
                }
                if spacers.to_owned() == "\r\n\r\n"{
                    spacers.clear();
                    if current_state == State::RequestLine{
                        println!("RequestLine: {}",String::from_utf8_lossy(&stack));
                        parse_request_line(&stack);
                        stack.clear();
                        current_state = State::Content;
                    }
                    else if current_state == State::Header{
                        println!("Header: {}",String::from_utf8_lossy(&stack));
                        parse_header(&stack);
                        stack.clear();
                        current_state = State::Content;
                    }
                }
                else if spacers.to_owned() == "\r\n"{
                    spacers.clear();
                    if current_state == State::RequestLine{
                        println!("RequestLine: {}",String::from_utf8_lossy(&stack));
                        stack.clear();
                        current_state = State::Header;
                    }else{
                        println!("Header: {}",String::from_utf8_lossy(&stack));
                        stack.clear();
                    }
                }
                else{
                    spacers.clear();
                    return Err(Error::InvalidNewLineError)
                }
                stack.push(*byte)
            }
        }
        if current_state == State::Content {
            println!("Content: {}",String::from_utf8_lossy(&stack));
            return Ok(Request{
                version: 1,
                url: "abc",
                method: "GET",
                is_partial: false,
                headers: Vec::new(),
                content: stack,
            });
        }else if current_state == State::Header{
            return Err(Error::PartialRequest)
        }else {
            return Err(Error::PartialRequest)
        }
        
    }

    fn parse_header(stack: &[u8]) -> Result<Header,Error> {
        todo!()
    }

    fn parse_request_line(line:&[u8]) -> Result<(u8,String,String),Error>{
        let mut version = 1;
        let mut url:String = String::new();
        let mut method = String::new();

        let mut current_state = State::Method;
        let mut stack = String::new();
        #[derive(Debug,PartialEq,Eq)]
        enum State{
            Method,
            Url,
            Version
        }
        
        
        return Ok((version,url,method))
    }
    

}
