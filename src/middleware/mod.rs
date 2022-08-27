pub fn handle_request(buffer:&[u8]) {
    let mut headers = [httparse::EMPTY_HEADER;64];
    let mut req = httparse::Request::new(&mut headers);
    // let buf: &[u8] = &buffer[..];
    let request =  req.parse(buffer);
    match request {
        Ok(res) => {
           if !res.is_partial(){
            println!("{:?}", req)
           }
        },
        Err(msg) => println!("{}",msg),
    }
}