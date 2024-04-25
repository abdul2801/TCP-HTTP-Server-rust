use http::{httprequest::{self, HttpRequest}, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};

pub trait handler {
    fn handle<'a>(req: &'a HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        println!("Loading file: {}", file_name);
        use std::fs;
        let path = format!("{}/public/{}", env!("CARGO_MANIFEST_DIR"), file_name);
        fs::read_to_string(path).ok()
    }

}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
order_id: i32,
order_date: String,
order_status: String,
}
pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

impl handler for PageNotFoundHandler {
    fn handle<'a>(req: &'a HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl handler for StaticPageHandler {
    fn handle<'a>(req: &'a HttpRequest) -> HttpResponse {
        let httprequest::Resource::Path(s) = &req.resource;

        let path = s.split("/").collect::<Vec<&str>>();

        match path[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Some("OK".to_string())),
            pat  => match Self::load_file(pat) {
                
                Some(content) => {
                    let mut map = std::collections::HashMap::new();
                    if pat.ends_with(".html") {
                        map.insert("Content-Type", "text/html");
                    } else if pat.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if pat.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    }
                    HttpResponse::new("200", Some(map), Some(content))

                },
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
                
            } 
        }
    }

}

impl WebServiceHandler {
    fn load_json(file_name: &str) -> Vec<OrderStatus> {
        use std::fs;
        let path = format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), file_name);
        let content = fs::read_to_string(path).unwrap();
        let res = serde_json::from_str(content.as_str()).unwrap();
        res
        
    }
}
impl handler for WebServiceHandler {
    fn handle<'a>(req: &'a HttpRequest) -> HttpResponse {
        let httprequest::Resource::Path(s) = &req.resource;

        let path = s.split("/").collect::<Vec<&str>>();
        // /api/shipping/orders
        if path.len() <= 3 {
            return HttpResponse::new("404", None, Self::load_file("404.html"));
        }
        match path[2] {
            "shipping" if path.len() > 2 && path[3] == "orders" => {
                let orders = Self::load_json("orders.json");
                let mut map = std::collections::HashMap::new();
                map.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(map), Some(serde_json::to_string(&orders).unwrap()))
            },
            _ => {
                HttpResponse::new("404", None, Self::load_file("404.html"))

            }
        }
    }

}

