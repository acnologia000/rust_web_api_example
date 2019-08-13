use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::env;

mod thread_pool;
mod request_proc;

fn main() {

    //Getting thread pool size from environment variable (to be set by user)
    //using 2 threads per route as default in case variable is not set
    //this can be hardcoded or read from a file , hence optional
    let pool_size : usize = match env::var("THREAD_POOL_SIZE_FOR_EACH_ROUTE"){
        Ok(var)=> var.parse().expect("THREAD_POOL_SIZE_FOR_EACH_ROUTE can only be a unsigned integer") ,
        Err(_s)=>2
    };

    //Getting listening port  from environment variable (to be set by user)
    //using 0.0.0.0:7878 as defaut port in case variable is not set
    //this can be hardcoded or read from a file , hence optional
    let port = match env::var("RUST_SERVER_PORT"){
        Ok(var)=> var ,
        Err(_s)=>"0.0.0.0:7878".to_string()
    };

    //spitting basic chatter to notify user that application is running and reporting settings being used 
    // again totally optional but helpful  
    println!("> edit 'RUST_SERVER_PORT' environment variable to change server listening port");
    println!("> edit 'THREAD_POOL_SIZE_FOR_EACH_ROUTE' environment variable to change thread pool size");
    println!("> Using {} as default thread pool size for each route \n> using {} as port",pool_size,port);

    // binding a listner on our designated port for listening for Tcp requests 
    let listner  = TcpListener::bind(&port).expect("binding Failure");
    
    //declaring string pattern for all routes 
    let home_route = String::from_utf8_lossy(b"/").to_string();
    let route_2 = String::from_utf8_lossy(b"/r2").to_string();
    let route_3 = String::from_utf8_lossy(b"/r3").to_string();
    
    //making thread pool for each route
    let home_pool  = thread_pool::ThreadPool::new(pool_size);
    let route2pool = thread_pool::ThreadPool::new(pool_size);
    let route3pool = thread_pool::ThreadPool::new(pool_size);
    
    //buffer to store request
    let mut req_buffer = [0;512];

    // listening to incoming requests in an infinite loop
    // listner.incoming() returns a 'std::result::Result<std::net::TcpStream, std::io::Error>'
    // which should be unwrapped/matched to get 'std::net::TcpStream' which contains our Tcp request
    // and acts a portal to send back the response for incoming Tcp request 
    // assume 'std::net::TcpStream' as a special envelope that is used to recieve Tcp request 
    // and send Tcp respose 
    for stream in listner.incoming() {
        
        let mut stream = stream.unwrap();
        stream.read(&mut req_buffer).unwrap();
        let request = request_proc::parse_request(&mut String::from_utf8_lossy(&req_buffer).to_string()).unwrap();
        
        match request.path {
        ref path if path == &home_route => home_pool.execute(|| home(stream)),
        ref path if path == &route_2 => route2pool.execute(|| {route1(stream)}),
        ref path if path == &route_3 => route3pool.execute(|| {route2(stream)}),
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