use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, Vec<Box<dyn Route>>>,
}

pub trait Route {
    fn execute(&self, params: Vec<&str>) -> String;
    fn matches(&self, path: &str) -> bool;
}

impl Router {
    pub fn new() -> Router {
        Router { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, method: &str, route: Box<dyn Route>) {
        self.routes
            .entry(method.to_string())
            .or_insert_with(Vec::new)
            .push(route);
    }

    pub fn execute_route(&self, method: &str, path: &str, params: Vec<&str>) -> Option<String> {
        if let Some(routes) = self.routes.get(method) {
            for route in routes {
                if route.matches(path) {
                    return Some(route.execute(params));
                }
            }
        }
        None
    }
}