use std::{collections::HashMap, fmt::format, io::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    pub version : &'a str,
    pub status_code: &'a str,
    pub status_text : &'a str,
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub body: Option<String>,
}


impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
        version : "HTTP/1.1",
        status_code : "200",
        status_text : "OK",
        headers : None,
        body : None, 
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(status_code : &'a str , headers : Option<HashMap<&'a str, &'a str>> , body : Option<String>) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        response.status_code = status_code;
        response.headers = match &headers {
            Some(h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            "400" => "Bad Request",
            _ => "Not Found",
        };
        response.body = body;
        response

            
        }
        pub fn headers(&self) -> String{
            let map = self.headers.clone().unwrap();
            let mut res = "".into();
            for (k,v) in map {
                res = format!("{}{}: {}\r\n", res, k, v);
            }
            res
    
        }
    pub fn send_response(&self, stream: &mut std::net::TcpStream) -> Result<(), std::io::Error>{
        let res = String::from(self.clone());
        stream.write(res.as_bytes()).unwrap();
        Ok(())
    }
}
    
    

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        let res1 = res.clone();
        let length;
        if res1.body.is_some() {
            length = res1.body.as_ref().unwrap().len();
        } else {
            length = 0;
            
        }

        format!("{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}", res1.version, res1.status_code, res1.status_text, res1.headers(),length, res1.body.unwrap_or("".to_string()))
    }
} 
    

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_response() {
        let actual_res = HttpResponse::new("200", None, Some("body".into()));

        let res_expected = HttpResponse {
            version : "HTTP/1.1",
            status_code : "200",
            status_text : "OK",
            headers : Some({
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                h
            }),
            body : Some("body".into()),
        };

        assert_eq!(actual_res, res_expected);




    }
    #[test]
    fn test_string_response() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "text/html");
        headers.insert("Auth", "token");
        let res = HttpResponse::new("200", Some(headers), Some("body".into()));
        let actual_res = String::from(res);
        let expected_res = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nAuth: token\r\nContent-Length: 4\r\n\r\nbody".to_string();
        assert_eq!(actual_res, expected_res);
    }
}






