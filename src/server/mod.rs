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

/// templateディレクトリからhtml文字列を取得する
fn get_html_from_template(name: &str, html_buf: &mut String) {
    let path_str = format!("src/server/template/{}.html", name);
    let path = Path::new(&path_str);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);
    reader.read_to_string(html_buf).unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    #[allow(clippy::unused_io_amount)]
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let mut html_buf = String::new();
    let response = if buffer.starts_with(get) {
        get_html_from_template("index", &mut html_buf);
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", html_buf);
        response
    } else {
        get_html_from_template("404", &mut html_buf);
        format!("HTTP/1.1 404 Not Found\r\n\r\n{}", html_buf)
    };
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
