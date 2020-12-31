use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/experiences/{id}"]
    Experience(usize),
    #[to = "/projects/{id}"]
    Project(usize),
    #[to = "/home"]
    Home,
    #[to = "/"]
    Index,
}

pub type Link = RouterAnchor<AppRoute>;
