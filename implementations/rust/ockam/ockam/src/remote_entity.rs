use crate::Route;

pub struct RemoteEntity {
    pub route: Route,
}

impl RemoteEntity {
    pub fn create<R: Into<Route>>(route: R) -> Self {
        RemoteEntity {
            route: route.into(),
        }
    }
}
