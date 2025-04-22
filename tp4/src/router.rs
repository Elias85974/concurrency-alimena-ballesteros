use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, Vec<Box<dyn Route + Send + Sync>>>,
}

pub trait Route: Send + Sync {
    fn execute(&self, params: Option<Vec<&str>>, body: Option<&str>) -> (u16, &str);
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

    pub fn execute_route(&self, method: &str, path: &str, params: Option<Vec<&str>>, body: Option<&str>) -> (u16, &str) {
        if let Some(routes) = self.routes.get(method) {
            for route in routes {
                if route.matches(path) {
                    return route.execute(params, body);
                }
            }
        }
        let mut response = String::new();
        self.routes.iter().for_each(|(key, values)| {
            values.iter().for_each(|value| {
                response.push_str(&format!("{} {:?}\n", key, value));
            });
        });
        (400, response.as_str())
    }
}