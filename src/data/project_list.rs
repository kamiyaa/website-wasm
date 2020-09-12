use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use yew::prelude::*;

use super::projects::{game_of_life, joshuto, ruiji, space_farmer_bot, website};

lazy_static! {
    pub static ref GITHUB_METRICS: Mutex<HashMap<String, GithubMetrics>> =
        Mutex::new(HashMap::new());
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GithubMetrics {
    pub stargazers_count: usize,
    pub forks_count: usize,
}

#[derive(Clone, Debug)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub languages: &'static [&'static str],
    pub owner: &'static str,
    pub repo_name: &'static str,
    pub preview_url: &'static str,
    pub preview_thumbnail_url: &'static str,
    pub tags: &'static [&'static str],
    pub html: fn() -> Html,
}

pub type ProjectListType = [Project; 5];

pub static PROJECT_LIST: ProjectListType = [
    Project {
        name: "Joshuto",
        description: "Terminal File Manager",
        url: "https://github.com/kamiyaa/joshuto",
        languages: &["Rust"],
        owner: "kamiyaa",
        repo_name: "joshuto",
        preview_url: "https://github.com/kamiyaa/joshuto/raw/master/screenshot.png",
        preview_thumbnail_url: "https://github.com/kamiyaa/joshuto/raw/master/screenshot.png",
        tags: &[
            "Rust",
            "Termion",
            "Tui-rs",
            "Terminal",
            "Concurrency",
            "Traits",
            "Boxing",
            "Toml",
        ],
        html: joshuto::component,
    },
    Project {
        name: "Ruiji",
        description: "Reverse Image Search Tool",
        url: "https://github.com/kamiyaa/ruiji",
        languages: &["C"],
        owner: "kamiyaa",
        repo_name: "ruiji",
        preview_url: "https://github.com/kamiyaa/ruiji/raw/master/ruiji_screenshot.png",
        preview_thumbnail_url: "https://github.com/kamiyaa/ruiji/raw/master/ruiji_screenshot.png",
        tags: &["C", "Curl", "Make", "Meson"],
        html: ruiji::component,
    },
    Project {
        name: "Game of Life",
        description: "Game of Life Simulator",
        url: "https://github.com/kamiyaa/game-of-life",
        languages: &["Rust", "Javascript", "WebAssembly"],
        owner: "kamiyaa",
        repo_name: "game-of-life",
        preview_url: "https://github.com/kamiyaa/game-of-life/raw/master/screenshot.png",
        preview_thumbnail_url: "https://github.com/kamiyaa/game-of-life/raw/master/screenshot.png",
        tags: &["Rust", "WebAssembly", "Javascript", "Webpack"],
        html: game_of_life::component,
    },
    Project {
        name: "SpaceFarmerBot",
        description: "Discord Bot",
        url: "https://github.com/Tusk98/SpaceFarmerBot",
        languages: &["Golang"],
        owner: "Tusk98",
        repo_name: "SpaceFarmerBot",
        preview_url:
            "https://raw.githubusercontent.com/Tusk98/SpaceFarmerBot/master/SpaceFarmer.jpg",
        preview_thumbnail_url:
            "https://raw.githubusercontent.com/Tusk98/SpaceFarmerBot/master/SpaceFarmer.jpg",
        tags: &["Golang", "Discord", "Json"],
        html: space_farmer_bot::component,
    },
    Project {
        name: "Website",
        description: "My Personal Website",
        url: "https://github.com/kamiyaa/kamiyaa.github.io",
        languages: &["React.js", "Sass"],
        owner: "kamiyaa",
        repo_name: "kamiyaa.github.io",
        preview_url: "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/preview.png",
        preview_thumbnail_url:
            "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/preview.png",
        tags: &["Javascript", "React", "React Hooks", "Sass", "npm"],
        html: website::component,
    },
];
