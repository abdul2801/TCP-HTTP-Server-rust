use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    UNINITIALIZED,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNINITIALIZED,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    UNINITIALIZED,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::UNINITIALIZED,
        }
    }
}

#[derive(Debug , PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug , PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resource,
    pub version: Version,
    pub headers : HashMap<String,String>,
    pub msg_body : String,
}

impl From<&str> for HttpRequest {
  fn from(s : &str) -> Self { 
    let mut lines = s.lines();
    let mut headers = HashMap::new();
    let mut msg_body = String::new();
    let mut method = Method::UNINITIALIZED;
    let mut resource = Resource::Path(String::new());
    let mut version = Version::UNINITIALIZED;
    if let Some(request_line) = lines.next() {
      let mut parts = request_line.split_whitespace();
      if let Some(method_str) = parts.next() {
        method = Method::from(method_str);
      }
      if let Some(resource_str) = parts.next() {
        resource = Resource::Path(resource_str.to_string());
      }
      if let Some(version_str) = parts.next() {
        version = Version::from(version_str);
      }
    }
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            let mut parts = line.split(":");
            if let Some(key) = parts.next() {
                if let Some(value) = parts.next() {
                    // check for localhost:3000
                    headers.insert(key.to_string(), value.to_string());


                }
            }
        } else {
            msg_body = line.to_string();
        

        }
    }
    HttpRequest {
      method,
      resource,
      version,
      headers,
      msg_body,
    }
    
  }
}

mod tests {
    use super::*;
    #[test]
    fn test_method_from() {
        assert_eq!(Method::from("GET"), Method::GET);
        assert_eq!(Method::from("POST"), Method::POST);
        assert_eq!(Method::from("UNINITIALIZED"), Method::UNINITIALIZED);
    }
    #[test]
    fn test_version() {
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("UNINITIALIZED"), Version::UNINITIALIZED);
    }

    #[test]
    fn test_http_request_from() {
        let request = HttpRequest::from("GET / HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\nbody");
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.resource, Resource::Path("/".to_string()));
        assert_eq!(request.version, Version::V1_1);
        assert_eq!(request.headers.get("Host").unwrap(), " localhost");
        assert_eq!(request.headers.get("User-Agent").unwrap(), " curl/7.68.0");
        assert_eq!(request.headers.get("Accept").unwrap(), " */*");
        assert_eq!(request.msg_body, "body");
    }
}
