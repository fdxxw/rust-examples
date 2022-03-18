use rust_embed::RustEmbed;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("server listener on http:://127.0.0.1:7878");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // handle_connection(stream);
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    // let contents = fs::read_to_string(filename).unwrap();
    let f = Asset::get(filename).unwrap();
    let contents = std::str::from_utf8(f.data.as_ref()).unwrap();
    let resp = format!("{}{}", status_line, contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let resp = format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );

    //     stream.write(resp.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(5));
    //     stream.write(buf)
    // } else {

    // }
}
