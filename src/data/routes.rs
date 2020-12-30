use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/experiences/{id}"]
    Experience(usize),
    #[to = "/experiences"]
    Experiences,
    #[to = "/projects/{id}"]
    Project(usize),
    #[to = "/projects"]
    Projects,
    #[to = "/home"]
    Home,
    #[to = "/"]
    Index,
}

pub type Link = RouterAnchor<AppRoute>;
