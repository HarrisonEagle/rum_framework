use std::collections::BTreeMap;
extern crate mime;
use crate::context::RumContext;
use crate::method::MethodType;

pub struct Router {
    pub route: String,
    children: BTreeMap<String, Router>,
    params_child_route: String,
    controllers: BTreeMap<String, fn(c: &mut RumContext)>,
    full_route: Vec<String>,
    middlewares: Vec<fn(c: &mut RumContext)>,
}

impl Router {
    pub(crate) fn new() -> Router {
        return Router {
            route: String::new(),
            children: BTreeMap::new(),
            params_child_route: String::new(),
            controllers: BTreeMap::new(),
            full_route: Vec::new(),
            middlewares: Vec::new(),
        };
    }

    pub(crate) fn get_info_and_controller(
        &self,
        method_type: MethodType,
        route_segs: &[&str],
    ) -> Option<(&[String], &fn(&mut RumContext))> {
        return match self.search_route(route_segs, 0) {
            Some((full_route, router)) => match router.get_controller(method_type) {
                Some(controller) => Some((full_route, controller)),
                None => None,
            },
            None => None,
        };
    }

    pub(crate) fn exec_middleware(
        &self,
        full_route: &[String],
        cur_index: usize,
        context: &mut RumContext,
    ) {
        for middleware in &self.middlewares {
            middleware(context);
            if context.has_response() {
                return;
            }
        }
        if cur_index + 1 < full_route.len() {
            match self.children.get(&full_route[cur_index + 1]) {
                Some(router) => router.exec_middleware(full_route, cur_index + 1, context),
                None => (),
            }
        }
    }

    pub(crate) fn add_middleware(&mut self, controllers: Vec<fn(&mut RumContext)>) {
        for controller in controllers {
            self.middlewares.push(controller);
        }
    }

    pub(crate) fn get_controller(&self, method_type: MethodType) -> Option<&fn(&mut RumContext)> {
        return self.controllers.get(&method_type.to_string());
    }

    pub(crate) fn search_route(
        &self,
        route_segs: &[&str],
        cur_index: usize,
    ) -> Option<(&[String], &Router)> {
        if cur_index == route_segs.len() - 1 {
            return Some((&self.full_route[..], &self));
        } else {
            return match self.children.get(route_segs[cur_index + 1]) {
                None => {
                    if self.params_child_route != "" {
                        match self.children.get(&self.params_child_route) {
                            None => None,
                            Some(router) => router.search_route(route_segs, cur_index + 1),
                        }
                    } else {
                        None
                    }
                }
                Some(router) => router.search_route(route_segs, cur_index + 1),
            };
        }
    }

    pub(crate) fn search_route_mut(
        &mut self,
        route_segs: &[&str],
        cur_index: usize,
    ) -> Option<&mut Router> {
        if cur_index == route_segs.len() - 1 {
            return Some(self);
        } else {
            return match self.children.get_mut(route_segs[cur_index + 1]) {
                Some(router) => router.search_route_mut(route_segs, cur_index + 1),
                None => None,
            };
        }
    }

    pub(crate) fn show_routes(&self, route: &str) {
        if self.controllers.len() > 0 {
            for controller in &self.controllers {
                println!("|{}| /{}", controller.0, route);
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

    pub(crate) fn add_controller(
        &mut self,
        method_type: MethodType,
        controller: fn(&mut RumContext),
    ) {
        let method_type_str = method_type.to_string();
        if self.controllers.contains_key(&method_type_str) {
            panic!(
                "Error: Method->{} Route->{} already exists",
                method_type, self.route
            );
        }
        self.controllers.insert(method_type_str, controller);
    }

    pub(crate) fn modify(&mut self, route_segs: Vec<&str>, cur_index: usize) -> &mut Router {
        if cur_index == route_segs.len() - 1 {
            return self;
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
                controllers: BTreeMap::new(),
                middlewares: Vec::new(),
                full_route: (&route_segs[..=cur_index + 1])
                    .to_vec()
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
            });
            let router = self.children.get_mut(route_segs[cur_index + 1]).unwrap();
            return router.modify(route_segs, cur_index + 1);
        }
    }
}
