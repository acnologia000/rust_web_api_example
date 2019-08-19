use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod request_proc;
mod thread_pool;

fn main() {
    //Getting thread pool size from environment variable (to be set by user)
    //using 2 threads per route as default in case variable is not set
    //this can be hardcoded or read from a file , hence optional
    let pool_size: usize = match env::var("THREAD_POOL_SIZE_FOR_EACH_ROUTE") {
        // value of 'THREAD_POOL_SIZE_FOR_EACH_ROUTE' is returned as string that needs to
        // be parsed into usize ,in case of errors we are using default values
        Ok(var) => match var.parse() {
            Ok(val) => val,
            Err(_err) => {
                println!("> Parse Error :{}'THREAD_POOL_SIZE_FOR_EACH_ROUTE' can only have unsigned integer Value",_err);
                println!("> using default value for THREAD_POOL_SIZE_FOR_EACH_ROUTE");
                2
            }
        },
        Err(_s) => 2,
    };

    //Getting listening port  from environment variable (to be set by user)
    //using 0.0.0.0:7878 as defaut port in case variable is not set
    //this can be hardcoded or read from a file , hence optional
    let port = match env::var("RUST_SERVER_PORT") {
        Ok(var) => var,
        Err(_s) => {
            println!(
                "> failed at reading :{} 'RUST_SERVER_PORT' using default",
                _s
            );
            "0.0.0.0:7878".to_string()
        }
    };

    //spitting basic chatter to notify user that application is running and reporting settings being used
    // again totally optional but helpful
    println!("> edit 'RUST_SERVER_PORT' environment variable to change server listening port");
    println!(
        "> edit 'THREAD_POOL_SIZE_FOR_EACH_ROUTE' environment variable to change thread pool size"
    );
    println!(
        "> Using {} as thread pool size for each route \n> using {} as port",
        pool_size, port
    );

    // binding a listner on our designated port for listening for Tcp requests
    // binding to a port might fail in case if we are bining to port that needs
    // elivated privelleges to access or port is busy(being used by another application)
    // or port/Ip are unavailable or wrong , its a good idea to report details to user
    let listner = match TcpListener::bind(&port) {
        Ok(val) => val,
        Err(err) => panic!("> Binding failure : {}", err),
    };

    //declaring string pattern for all routes
    let home_route = "/".to_string();
    let route_2 = "/r2".to_string();
    let route_3 = "/r3".to_string();
    //making thread pool for each route
    let home_pool = thread_pool::ThreadPool::new(pool_size);
    let route2pool = thread_pool::ThreadPool::new(pool_size);
    let route3pool = thread_pool::ThreadPool::new(pool_size);
    //buffer to store request
    let mut req_buffer = [0; 512];

    // listening to incoming requests in an infinite loop
    // listner.incoming() waits until a request comes in
    // and returns a 'std::result::Result<std::net::TcpStream, std::io::Error>' whenever a request drops
    // which should be unwrapped/matched to get 'std::net::TcpStream' which contains our Tcp request
    // and acts a portal to send back the response for incoming Tcp request
    // assume 'std::net::TcpStream' as a special envelope that is used to recieve Tcp request
    // and send Tcp respose
    for stream in listner.incoming() {
        // getting actual Tcp::stream from Result type given by listener
        let mut stream = match stream {
            Ok(val) => val,
            Err(_err) => {
                println!("> Failed at Unwrapping Stream :{}", _err);
                continue;
            }
        };

        // stream does not returns Tcp request directly , instead it writes it into
        // empty byte array we provid
        match stream.read(&mut req_buffer) {
            Ok(_val) => {}
            Err(err) => println!("> Failed at reading Request into buffer :{}", err),
        };

        // parsing request (which is stored in req_buffer) from [u8] to more readable and usable data structure
        let request =
            request_proc::parse_request(&mut String::from_utf8_lossy(&req_buffer).to_string())
                .unwrap();
        // using match as case-switch to send requests to be executed in different thread-pools
        match request {
            // compairing refrance to path inside request to routes and
            // accordingly execute appropriate functions in designated thread pools
            ref path if path.path == home_route => home_pool.execute(|| home(stream, request)),
            ref path if path.path == route_2 => route2pool.execute(|| route1(stream)),
            ref path if path.path == route_3 => route3pool.execute(|| route2(stream)),

            // _ handles all the cases that cannot be handled in our defined paths
            // since we dont have what user is asking for so according to internet standard
            // we will return Error 404
            // we will send response by stream.write(b"some response") method in stream
            // response is always written as &[u8] (refrance to byte array)
            // stream.write returns an Result<usize> that should be checked as there is a real
            // possibility of respose writing failure
            // if everything goes well it returns number bytes sent as response (which is useless in most cases)
            _ => err(stream),
        }
    }
}

fn home(mut stream: TcpStream, request: request_proc::Request) {
    println!("{}", request.to_string());
    stream.write(format!("HTTP/1.1 200 OK \nContent-Type: text/html \r\n\r\n hello from home <br> request was {} ",request.to_string()).as_bytes()).expect("failed to write");
}

fn route1(mut stream: TcpStream) {
    stream
        .write("HTTP/1.1 200 OK \nContent-Type: text/html \r\n\r\n hello from route 1".as_bytes())
        .expect("failed to write");
}

fn route2(mut stream: TcpStream) {
    stream
        .write("HTTP/1.1 200 OK \nContent-Type: text/html \r\n\r\n hello from route 2".as_bytes())
        .expect("failed to write");
}

fn err(mut stream: TcpStream) {
    stream
        .write(
            "HTTP/1.1 404 Not Found \nContent-Type: text/html \r\n\r\n hello from route 2"
                .as_bytes(),
        )
        .expect("failed to write");
}
// set env var for speed optimization during release build <RUSTFLAGS="-C target-cpu=native">
