
use std::collections::HashMap;
pub struct Request {
    http_version: String,
    method: String,
pub path: String,
    gzip:bool,
    brotli:bool,
    deflate:bool
}

impl Request {
   pub fn print(&self) {

        println!("http req struct -> ");
        println!("{}",self.http_version);
        println!("{}",self.method);
        println!("{}",self.path);
        println!("{}",self.gzip);
        println!("{}",self.brotli);
    }
}

// site version
// pub fn parse_request(request: &mut String) -> Result<Request, ()> {
//     let mut parts = request.split(" ");
//     let method = match parts.next() {
//         Some(method) => method.trim().to_string(),
//         None => return Err(()),
//     };
//     let path = match parts.next() {
//         Some(path) => path.trim().to_string(),
//         None => return Err(()),
//     };
//     let http_version = match parts.next() {
//         Some(version) => version.trim().to_string(),
//         None => return Err(()),
//     };
    

//     Ok( Request {
//         http_version: http_version,
//         method: method,
//         path: path,
//     } )
// }

//stupid version
#[inline(always)] 
pub fn parse_request(request: &mut String) -> Result<Request, ()> {
    let mut parts = request.split(" ");

    let method = parts.next().unwrap().to_string(); 
    let path =  parts.next().unwrap().to_string(); 
    let http_version = parts.next().unwrap().to_string(); 
    
    let (br,gzip,deflate) = proc_req(request);
    

    Ok( Request {
        http_version: http_version,
        method: method,
        path: path,
        gzip : gzip,
        brotli:br,
        deflate:deflate
    } )
}

pub fn proc_req(request: &mut String) -> (bool,bool,bool) {
    let parts:Vec<&str> = request.split(" ").collect();
    let mut br  = false ;
    let mut gz = false ;
    let mut deflate = false ;
    for i in 0..parts.len() {
        if parts[i].contains("Accept-Encoding") {
            if parts[i+1].contains("br") || parts[i+2].contains("br") || parts[i+3].contains("br") {
                br = true ;    
            }
            if parts[i+1].contains("gzip") || parts[i+2].contains("gzip") || parts[i+3].contains("gzip") {
                gz = true ;    
            }
            if parts[i+1].contains("deflate") || parts[i+2].contains("deflate") || parts[i+3].contains("deflate") {
                deflate = true ;    
            }
        }
    }

    (br,gz,deflate)
}

