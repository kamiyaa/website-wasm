use crate::routes::AppRoute;

pub struct URLRoute {
    pub name: &'static str,
    pub route: &'static str,
}

pub static HEADER_LINKS: [URLRoute; 2] = [
    URLRoute {
        name: "Experiences",
        route: "/#experiences",
    },
    URLRoute {
        name: "Projects",
        route: "/#projects",
    },
];
