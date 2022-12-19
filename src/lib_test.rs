#[cfg(test)]
use crate::{method::MethodType, handler::Handler, context::RumContext};

#[test]
fn check_basic_route() {

    let mut handler = Handler::new();
    handler.add_route(MethodType::GET, "/app/v1/test", |_:&mut RumContext|{ });
    let target_route = "/app/v1/test";
    let route_segs: Vec<&str> = target_route.trim_end_matches('/').split('/').collect();
    let result = handler.router.get_info_and_controller(MethodType::GET, &route_segs);
    assert_eq!(result.is_some(), true);
}

#[test]
fn check_route_with_params() {
    let mut handler = Handler::new();
    handler.add_route(MethodType::GET, "/app/v1/test/:param1/page", |_:&mut RumContext|{ });
    let target_route = "/app/v1/test/aaaaa/page";
    let route_segs: Vec<&str> = target_route.trim_end_matches('/').split('/').collect();
    let result = handler.router.get_info_and_controller(MethodType::GET, &route_segs);
    assert_eq!(result.is_some(), true);
}

#[test]
fn check_different_route() {
    let mut handler = Handler::new();
    handler.add_route(MethodType::GET, "/app/v1/test", |_:&mut RumContext|{ });
    let target_route = "/app/v1/test2";
    let route_segs: Vec<&str> = target_route.trim_end_matches('/').split('/').collect();
    let result = handler.router.get_info_and_controller(MethodType::GET, &route_segs);
    assert_eq!(result.is_some(), false);
}

#[test]
fn check_same_route_but_diffent_method() {
    let mut handler = Handler::new();
    handler.add_route(MethodType::POST, "/app/v1/test", |_:&mut RumContext|{ });
    let target_route = "/app/v1/test";
    let route_segs: Vec<&str> = target_route.trim_end_matches('/').split('/').collect();
    let result = handler.router.get_info_and_controller(MethodType::GET, &route_segs);
    assert_eq!(result.is_some(), false);
}

#[test]
#[should_panic]
fn check_set_same_controller_crash() {
    let mut handler = Handler::new();
    handler.add_route(MethodType::POST, "/app/v1/test", |_:&mut RumContext|{ });
    handler.add_route(MethodType::POST, "/app/v1/test", |_:&mut RumContext|{ });
}

#[test]
#[should_panic]
fn check_set_same_position_multiple_param_crash() {
    let mut handler = Handler::new();
    handler.add_route(MethodType::POST, "/app/v1/test/:param1/page", |_:&mut RumContext|{ });
    handler.add_route(MethodType::POST, "/app/v1/test/:param2/page", |_:&mut RumContext|{ });
}