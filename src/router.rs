use fnv::FnvHashMap;
use strum_macros::Display;
pub struct Router {
    pub route: String,
    children: FnvHashMap<String, Router>,
    params_child_route: String,
    handlers: FnvHashMap<String, fn(c: Context) -> Response>
}

impl Router {

    pub(crate) fn new() -> Router{
        return Router {
            route: "".to_string(),
            children: FnvHashMap::default(),
            params_child_route: "".to_string(),
            handlers: FnvHashMap::default(),
        };
    }

    pub(crate) fn set_group() {

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

    pub(crate) fn modify(&mut self, route_segs: Vec<&str>, cur_index: usize, method_type: MethodType, handler: fn(Context) -> Response){
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
                children: FnvHashMap::default(), 
                params_child_route: "".to_string(),
                handlers: FnvHashMap::default()
            });
            let router = self.children.get_mut(route_segs[cur_index + 1]).unwrap();
            router.modify(route_segs, cur_index + 1, method_type, handler);
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