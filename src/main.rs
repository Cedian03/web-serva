mod res;
mod req;

use std::{collections::HashMap, error::Error, fs::read_to_string, io::{Read, Write}, net, str};

use req::Request;
use res::Response;

fn handle_client(mut stream: net::TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf)?;

    let req: Request = str::from_utf8(&buf[0..n])?.parse()?;

    let res = match read_to_string(format!(".{}", req.rl_target)) {
        Ok(body) => Response::new(("HTTP/1.1".to_owned(), "200".to_owned(), "OK".to_owned()), HashMap::default(), body),
        Err(_) => Response::new(("HTTP/1.1".to_owned(), "404".to_owned(), "Not Found".to_owned()), HashMap::default(), "".to_owned()),
    };

    stream.write(res.to_string().as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = net::TcpListener::bind("127.0.0.1:8000")?;
    println!("Listening on: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        let _ = handle_client(stream?);
    }

    Ok(())
}
