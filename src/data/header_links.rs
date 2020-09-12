use super::AppRoute;

pub struct Link {
    pub name: &'static str,
    pub route: AppRoute,
}

pub static HEADER_LINKS: [Link; 2] = [
    Link {
        name: "Experiences",
        route: AppRoute::Experiences,
    },
    Link {
        name: "Projects",
        route: AppRoute::Projects,
    },
];
