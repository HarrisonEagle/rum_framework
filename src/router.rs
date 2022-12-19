use std::collections::BTreeMap;
extern crate mime;
use crate::context::RumContext;
use crate::method::MethodType;

pub struct Router {
    pub route: String,
    children: BTreeMap<String, Router>,
    params_child_route: String,
    handlers: BTreeMap<String, fn(c: &mut RumContext)>,
    full_route: Vec<String>,
}

impl Router {
    pub(crate) fn new() -> Router {
        return Router {
            route: String::new(),
            children: BTreeMap::new(),
            params_child_route: String::new(),
            handlers: BTreeMap::new(),
            full_route: vec![String::new()],
        };
    }

    pub(crate) fn get_full_route(
        &self,
        method_type: MethodType,
        route_segs: &[&str],
    ) -> Option<(&[String], &fn(&mut RumContext))> {
        return match self.search_route(method_type, route_segs, 0) {
            Some(result) => Some(result),
            None => None,
        };
    }

    // exec middlewares and route handler
    pub(crate) fn exec_middleware(&self, full_route: &[String], cur_index: usize) {
        if cur_index == full_route.len() - 1 {
        } else {
            match self.children.get(&(full_route[cur_index + 1])) {
                Some(handler) => {
                    handler.exec_middleware(full_route, cur_index + 1);
                }
                None => {}
            };
        }
    }

    fn search_route(
        &self,
        method_type: MethodType,
        route_segs: &[&str],
        cur_index: usize,
    ) -> Option<(&[String], &fn(&mut RumContext))> {
        if cur_index == route_segs.len() - 1 {
            for (key, value) in self.handlers.iter() {
                if *key == method_type.to_string() {
                    return Some((&self.full_route[..], value));
                }
            }
        } else {
            return match self.children.get(route_segs[cur_index + 1]) {
                None => {
                    if self.params_child_route != "" {
                        match self.children.get(&self.params_child_route) {
                            None => None,
                            Some(router) => {
                                router.search_route(method_type, route_segs, cur_index + 1)
                            }
                        }
                    } else {
                        None
                    }
                }
                Some(router) => router.search_route(method_type, route_segs, cur_index + 1),
            };
        }
        return None;
    }

    pub(crate) fn show_routes(&self, route: &str) {
        if self.handlers.len() > 0 {
            for handler in &self.handlers {
                println!("|{}| /{}", handler.0, route);
            }
        }
        for value in self.children.values() {
            let new_route = if route == "" {
                format!("{}", value.route)
            } else {
                format!("{}/{}", route, value.route)
            };
            value.show_routes(&new_route);
        }
    }

    pub(crate) fn modify(
        &mut self,
        method_type: MethodType,
        route_segs: Vec<&str>,
        cur_index: usize,
        handler: fn(&mut RumContext),
    ) {
        let method_type_str = method_type.to_string();
        if cur_index == route_segs.len() - 1 {
            if self.handlers.contains_key(&method_type_str) {
                panic!(
                    "Error: Method->{} Route->{} already exists",
                    method_type, self.route
                );
            }
            self.handlers.insert(method_type_str, handler);
        } else {
            let new_seg = route_segs[cur_index + 1];
            if new_seg.starts_with(":") {
                if self.params_child_route != "" && new_seg != self.params_child_route {
                    panic!(
                        "Error: params {} conflict with params {}",
                        new_seg, self.params_child_route
                    );
                }
                self.params_child_route = new_seg.to_string();
            }
            self.children.entry(new_seg.to_string()).or_insert(Router {
                route: new_seg.to_string(),
                children: BTreeMap::new(),
                params_child_route: String::new(),
                handlers: BTreeMap::new(),
                full_route: (&route_segs[..=cur_index + 1])
                    .to_vec()
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
            });
            let router = self.children.get_mut(route_segs[cur_index + 1]).unwrap();
            router.modify(method_type, route_segs, cur_index + 1, handler);
        }
    }
}
