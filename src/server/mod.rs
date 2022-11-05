use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

use crate::APP_CONFIG;

pub(crate) fn http_server() {
    let port = APP_CONFIG.get().unwrap().server_port;
    // TODO: handle error of result
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("server can be used on port {}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    #[allow(clippy::unused_io_amount)]
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let response = if buffer.starts_with(get) {
        let mut html_buf = String::new();
        // let current_dir = env::current_exe().unwrap();
        let path = Path::new("src/server/asset/index.html");
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut html_buf).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", html_buf);
        response
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n<html>Not Found</html>".to_string()
    };
    #[allow(clippy::unused_io_amount)]
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
