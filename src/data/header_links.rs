use super::AppRoute;

pub struct URLRoute {
    pub name: &'static str,
    pub route: AppRoute,
}

pub static HEADER_LINKS: [URLRoute; 2] = [
    URLRoute {
        name: "Experiences",
        route: AppRoute::Experiences,
    },
    URLRoute {
        name: "Projects",
        route: AppRoute::Projects,
    },
];
