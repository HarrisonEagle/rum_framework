use std::collections::BTreeMap;
use strum_macros::Display;
extern crate mime;
use strum_macros::EnumString;
use crate::status_code;

use std::str::from_utf8_unchecked;
use std::fs::File;
use std::fs::metadata;
use std::io::Read;
use std::io::BufReader;

pub struct Router {
    pub route: String,
    children: BTreeMap<String, Router>,
    params_child_route: String,
    handlers: BTreeMap<String, fn(c: Context) -> Response>,
    full_route: Vec<String>,
}

impl Router {

    pub(crate) fn new() -> Router{
        return Router {
            route: "".to_string(),
            children: BTreeMap::new(),
            params_child_route: "".to_string(),
            handlers: BTreeMap::new(),
            full_route: vec!["".to_string()]
        };
    }

    pub(crate) fn get_full_route(&self,method_type: MethodType, route: &str) -> Option<(&[String], &fn(Context) -> Response)>{
        let mut route_segs: Vec<&str> = route.trim_end_matches('/').split('/').collect();
        if route_segs[0] != ""{
            route_segs.insert(0, "");
        }
        return match self.search_route(method_type, route_segs, 0) {
            Some(result) => { Some(result) },
            None => { None },
        }
    }

    // exec middlewares and route handler
    pub(crate) fn exec_middleware(&self, full_route: &[String], cur_index: usize) {
        if cur_index == full_route.len() - 1 {
           
        }else{
            match self.children.get(&(full_route[cur_index + 1])) {
                Some(handler) => { handler.exec_middleware(full_route, cur_index + 1); },
                None => { } ,
            };
        }
    }

    fn search_route(&self, method_type: MethodType, route_segs: Vec<&str>, cur_index: usize ) -> Option<(&[String], &fn(Context) -> Response)>{
        if cur_index == route_segs.len() - 1{
            for (key, value) in self.handlers.iter() {
                if *key == method_type.to_string(){
                    return Some((&self.full_route[..], value));
                }
            }
        }else {
            return match self.children.get(route_segs[cur_index+1]) {
                None => if self.params_child_route != "" {
                     match self.children.get(&self.params_child_route)  {
                         None => { None },
                         Some(router) => router.search_route(method_type, route_segs, cur_index + 1),
                     }
                } else { None },
                Some(router) => router.search_route(method_type, route_segs, cur_index + 1),
            };
        }
        return None; 
    }

    pub(crate) fn show_routes(&self, route: &str){
        if self.handlers.len() > 0 {
            for handler in &self.handlers{
                println!("|{}| /{}",handler.0, route);
            }
        }
        for value in self.children.values()  {
            let new_route = if route == "" { format!("{}",value.route) } else { format!("{}/{}",route, value.route) } ;
            value.show_routes(&new_route);
        }
    }

    pub(crate) fn modify(&mut self, method_type: MethodType, route_segs: Vec<&str>, cur_index: usize, handler: fn(Context) -> Response){
        let method_type_str = method_type.to_string();
        if cur_index == route_segs.len() - 1 {
            if self.handlers.contains_key(&method_type_str) {
                // TODO: THROW ERROR
                panic!("Error: Method->{} Route->{} already exists", method_type, self.route );
            }
            self.handlers.insert(method_type_str, handler);
        }else{
            let new_seg = route_segs[cur_index+1].to_string();
            if new_seg.starts_with(":") {
                if self.params_child_route != "" && new_seg != self.params_child_route{
                    panic!("Error: params {} conflict with params {}", new_seg ,self.params_child_route);
                }
                self.params_child_route = new_seg;
            }
            self.children.entry( route_segs[cur_index+1].to_string()).or_insert(Router {
                route: route_segs[cur_index+1].to_string(),
                children: BTreeMap::new(), 
                params_child_route: "".to_string(),
                handlers: BTreeMap::new(),
                full_route: (&route_segs[..=cur_index+1]).to_vec().iter().map(|s| s.to_string()).collect()
            });
            let router = self.children.get_mut(route_segs[cur_index + 1]).unwrap();
            router.modify(method_type,route_segs, cur_index + 1,  handler);
        }
    }

}
#[derive(Debug, Display, EnumString)]
pub enum MethodType{
    GET,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

pub enum ResponseType{
    Text,
    Html,
    Json
}

pub struct Context{


}

pub struct Response{
    pub(crate) http_status: String,
    pub(crate) response_type: ResponseType,
    pub(crate) content_type: String,
    pub(crate) response_body: String
}

impl Response {
    pub fn HTML(status_code: i32, response_body: String) -> Response{
        return Response {
            http_status: status_code::from_status_code(status_code),
            content_type: mime::HTML.to_string(),
            response_type: ResponseType::Html,
            response_body: response_body,
        }
    }

    pub fn FILE(status_code: i32, file_path: &str) -> Response{
        // Read file into vector.
        println!("Searching:{}", file_path);
        return match File::open(file_path){
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                // Read file into vector.
                reader.read_to_end(&mut buffer);
                let body = unsafe {from_utf8_unchecked(&buffer).to_string() };
                Response {
                    http_status: status_code::from_status_code(status_code),
                    content_type: match mime_guess::from_path(file_path).first(){
                        Some(mime) => { mime.to_string() },
                        None => { mime::TEXT.to_string() },
                    },
                    response_type: ResponseType::Html,
                    response_body: body,
                }
            },
            Err(e) => {
                Response {
                    http_status: status_code::from_status_code(status_code::NOT_FOUND),
                    content_type: mime::HTML.to_string(),
                    response_type: ResponseType::Html,
                    response_body: "Not Found".to_string(),
                }
            },
        };
    }
}