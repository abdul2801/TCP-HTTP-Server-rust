use std::{io::Write, net::TcpStream};

use http::httprequest::{HttpRequest, Method, Resource};

use crate::handler::{handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req : HttpRequest, stream : &mut TcpStream) {
        match req.method {
            Method::GET => match &req.resource {
                Resource::Path(path) => {
                    let route = path.split("/").collect::<Vec<&str>>();
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        
                        },
                        _ => {
                            let resp = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }

            },
            _  => {
                let resp = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
       


    }
}