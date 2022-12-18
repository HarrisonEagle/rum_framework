use fnv::FnvHashMap;

pub struct RouterRootNode{
    routes: FnvHashMap<String, Router>
}


impl RouterRootNode{
    pub fn new() -> RouterRootNode{
        return RouterRootNode{
            routes: FnvHashMap::default()
        };
    }

    pub fn show_routes(&self){
        self.routes.keys();
        for value in self.routes.values() {
            value.show_routes("", 0);
        }
    }

    pub fn add(&mut self, method_type: MethodType, route: &str, handler: fn(Context) -> Response) {
        let route_segs: Vec<&str> = route.trim_end_matches('/').split('/').collect();
        if(route_segs.len() > 0){
            self.routes.entry( route_segs[0].to_string()).or_insert(Router {
                route: route_segs[0].to_string(),
                children: FnvHashMap::default(), 
                handlers: Vec::new()
            });
            let mut cur_index = 0;
            let router = self.routes.get_mut(route_segs[0]).unwrap();
            router.modify(route_segs, cur_index, method_type, handler);
        }
    }
}

pub struct Router {
    pub route: String,
    children: FnvHashMap<String, Router>,
    handlers: Vec<(MethodType, fn(c: Context) -> Response)>
}

impl Router {

    fn show_routes(&self, route: &str, depth: i32){
        if(self.handlers.len() > 0){
            println!("{}->/{}", depth, route);
        }
        if(self.children.len() > 0){
            for value in self.children.values()  {
                //println!("ee:{}:{}", route,route != "");
                let new_route = if route == "" { format!("{}",value.route) } else { format!("{}/{}",route, value.route) } ;
                value.show_routes(&new_route, depth + 1);
            }
        }
    }


    pub fn handler_conflict(self, method_type: MethodType) -> bool{
        for handler in self.handlers{
            if matches!(handler.0, method_type){
                return true;
            }
        }
        return false;
    }

    pub fn modify(&mut self, route_segs: Vec<&str>, cur_index: usize, method_type: MethodType, handler: fn(Context) -> Response){
        
        if(cur_index == route_segs.len()){
            //let mut iter = self.handlers.iter();
            //let matches = iter.find(|&e| matches!(&e.0, method_type));
            self.handlers.push((method_type, handler));
        }else{
            self.children.entry( route_segs[cur_index].to_string()).or_insert(Router {
                route: route_segs[cur_index].to_string(),
                children: FnvHashMap::default(), 
                handlers: Vec::new()
            });
            let router = self.children.get_mut(route_segs[cur_index]).unwrap();
            router.modify(route_segs, cur_index + 1, method_type, handler);
        }
    }
    

    pub fn insert_handler(&mut self, method: MethodType, handler: fn(Context) -> Response){
        self.handlers.push((method, handler));
    }


}
pub enum MethodType{
    GET,
    POST,
    PUT,
    DELETE
}

pub enum ResponseType{
    text,
    html,
    json
}

pub struct Context{


}

pub struct Response{
    pub response_type: ResponseType

}