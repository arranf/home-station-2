use std::collections::HashMap;
use std::ops::Deref;

use crate::{Route, RouteId};

/// A simple `RouteId` key based router for storing and getting `Route`s.
pub struct Router<State> {
    routes: HashMap<RouteId, Box<dyn Route<State>>>,
}

impl<State> Router<State> {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Adds a `Route` to the `Router`
    pub fn add(&mut self, id: RouteId, route: impl Route<State> + 'static) {
        self.routes.insert(id, Box::new(route));
    }

    /// Gets a `Route` using a `RouteId`
    pub fn get(&self, id: &str) -> Option<&dyn Route<State>> {
        self.routes.get(id).map(Deref::deref)
    }
}
