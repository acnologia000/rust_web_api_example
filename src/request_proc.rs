use std::fmt;

pub enum RequestType {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    PATCH,
    OPTIONS,
}

pub struct Request {
    pub http_version: String,
    pub method: RequestType,
    pub path: String,

    pub gzip: bool,
    pub brotli: bool,
    pub deflate: bool,
}

pub fn parse_request(request: &mut String) -> Result<Request, ()> {
    let mut parts = request.split(" ");

    let method = get_request_type(parts.next().unwrap().to_string()).unwrap_or(RequestType::GET);
    let path = parts.next().unwrap().to_string();
    let http_version = parts.next().unwrap().to_string();

    let (br, gzip, deflate) = proc_req(request);

    Ok(Request {
        http_version: http_version,
        method: method,
        path: path,
        gzip: gzip,
        brotli: br,
        deflate: deflate,
    })
}

#[inline(always)]
pub fn proc_req(request: &mut String) -> (bool, bool, bool) {
    let parts: Vec<&str> = request.split(" ").collect();
    let mut br = false;
    let mut gz = false;
    let mut deflate = false;

    for i in 0..parts.len() {
        if parts[i].contains("Accept-Encoding") {
            if parts[i + 1].contains("br")
                || parts[i + 2].contains("br")
                || parts[i + 3].contains("br")
            {
                br = true;
            }
            if parts[i + 1].contains("gzip")
                || parts[i + 2].contains("gzip")
                || parts[i + 3].contains("gzip")
            {
                gz = true;
            }
            if parts[i + 1].contains("deflate")
                || parts[i + 2].contains("deflate")
                || parts[i + 3].contains("deflate")
            {
                deflate = true;
            }
        }
    }

    (br, gz, deflate)
}

impl Request {
    pub fn to_string(&self) -> String {
        return format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.http_version, self.method, self.path, self.gzip, self.brotli, self.deflate
        );
    }
}

pub fn get_request_type(req: String) -> Option<RequestType> {
    if req.to_uppercase().contains("GET") {
        return Some(RequestType::GET);
    } else if req.to_uppercase().contains("POST") {
        return Some(RequestType::POST);
    } else if req.to_uppercase().contains("DELETE") {
        return Some(RequestType::DELETE);
    } else if req.to_uppercase().contains("PATCH") {
        return Some(RequestType::PATCH);
    } else if req.to_uppercase().contains("PUT") {
        return Some(RequestType::PUT);
    } else if req.to_uppercase().contains("HEAD") {
        return Some(RequestType::HEAD);
    } else if req.to_uppercase().contains("OPTIONS") {
        return Some(RequestType::OPTIONS);
    }
    None
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestType::GET => f.write_str("GET"),
            RequestType::POST => f.write_str("POST"),
            RequestType::PATCH => f.write_str("PATCH"),
            RequestType::DELETE => f.write_str("DELETE"),
            RequestType::PUT => f.write_str("PUT"),
            RequestType::HEAD => f.write_str("HEAD"),
            RequestType::OPTIONS => f.write_str("OPTIONS"),
        }
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
