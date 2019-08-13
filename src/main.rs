use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
mod thread_pool;
mod request_proc;
fn main() {
    let listner  = TcpListener::bind("0.0.0.0:7878").expect("binding Failure");
    //declaring string pattern for all routes 
    let _home_route = String::from_utf8_lossy(b"/").to_string();
    let _route_2 = String::from_utf8_lossy(b"/r2").to_string();
    //making thread pool for each route
    let home_pool = thread_pool::ThreadPool::new(3);
    let route2pool = thread_pool::ThreadPool::new(3);
    let route3pool = thread_pool::ThreadPool::new(3);
    let mut req_buffer = [0;512];
    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        stream.read(&mut req_buffer).unwrap();
        let request = request_proc::parse_request(&mut String::from_utf8_lossy(&req_buffer).to_string());
        let request = request.unwrap();
        match request.path {
        ref path if path == &_home_route => home_pool.execute(|| home(stream)),
        ref path if path == &_route_2 => route2pool.execute(||{route1(stream)}),
        _ => {stream.write(b"Error 404").unwrap();}
        }
        
        
    }
}

fn home(mut stream :TcpStream) {
   stream.write("HTTP/1.1 200 OK \nContent-Type: text/html \r\n\r\n hello from home".as_bytes()).expect("failed to write");
}

fn route1(mut stream :TcpStream) {
   stream.write("HTTP/1.1 200 OK \nContent-Type: text/html \r\n\r\n hello from route 1".as_bytes()).expect("failed to write");
}
fn route2(mut stream :TcpStream) {
   stream.write("HTTP/1.1 200 OK \nContent-Type: text/html \r\n\r\n hello from route 2".as_bytes()).expect("failed to write");
}