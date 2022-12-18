use std::collections::BTreeMap;
use strum_macros::Display;
pub struct Router {
    pub route: String,
    children: BTreeMap<String, Router>,
    params_child_route: String,
    handlers: BTreeMap<String, fn(c: Context) -> Response>,
}

impl Router {

    pub(crate) fn new() -> Router{
        return Router {
            route: "".to_string(),
            children: BTreeMap::new(),
            params_child_route: "".to_string(),
            handlers: BTreeMap::new(),
            
            
        };
    }
    pub(crate) fn get_handler(&self,method_type: MethodType, route: &str){
        let mut route_segs: Vec<&str> = route.trim_end_matches('/').split('/').collect();
        if route_segs[0] != ""{
            route_segs.insert(0, "");
        }
        match self.search_route(method_type, route_segs, 0) {
            Some(_) => println!("exist!"),
            None => print!("not exist!")
        }
    }

    

    fn search_route(&self, method_type: MethodType, route_segs: Vec<&str>, cur_index: usize ) -> Option<&fn(Context) -> Response>{
        if cur_index == route_segs.len() - 1{
            for (key, value) in self.handlers.iter() {
                if *key == method_type.to_string(){
                    return Some(value);
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
            let str = &route_segs[..=cur_index+1];
            self.children.entry( route_segs[cur_index+1].to_string()).or_insert(Router {
                route: route_segs[cur_index+1].to_string(),
                children: BTreeMap::new(), 
                params_child_route: "".to_string(),
                handlers: BTreeMap::new()
                
            });
            let router = self.children.get_mut(route_segs[cur_index + 1]).unwrap();
            router.modify(method_type,route_segs, cur_index + 1,  handler);
        }
    }

}
#[derive(Debug, Display)]
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
    pub response_type: ResponseType

}