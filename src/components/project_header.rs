use std::collections::hash_map::Entry;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{RequestInit, RequestMode, Response};
use yew::prelude::*;

use crate::data::{GithubMetrics, Project, GITHUB_METRICS};

#[derive(Clone, Debug)]
pub enum FetchState {
    Never,
    Error(String),
    Fetching,
    Success(GithubMetrics),
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub project: &'static Project,
}

#[derive(Clone, Debug)]
pub struct ProjectHeader {
    props: Props,
    fetch_state: FetchState,
    link: ComponentLink<Self>,
}

impl Component for ProjectHeader {
    type Message = FetchState;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            fetch_state: Self::Message::Never,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.fetch_state = msg;
        if let FetchState::Success(g) = &self.fetch_state {
            let mut map = GITHUB_METRICS.lock().unwrap();
            let key = format!(
                "{}/{}",
                self.props.project.owner, self.props.project.repo_name
            );
            match map.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(g.clone());
                }
                _ => {}
            };
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let project = &self.props.project;
        let tag_component = html! {
            <ul class="tag-list">
                { project.tags.iter()
                    .map(|tag| html!{ <li>{tag}</li> })
                    .collect::<Html>()
                }
            </ul>
        };
        if let FetchState::Never = self.fetch_state {
            let map = GITHUB_METRICS.lock().unwrap();
            let key = format!(
                "{}/{}",
                self.props.project.owner, self.props.project.repo_name
            );
            match map.get(key.as_str()) {
                Some(g) => {
                    let gg = g.clone();
                    self.link
                        .callback(move |_| FetchState::Success(gg))
                        .emit(());
                }
                None => {
                    self.link.callback(|_| FetchState::Fetching).emit(());
                    let project = self.props.project.clone();
                    let link = self.link.clone();
                    let js_future = async move {
                        let message: Self::Message = get_github_metrics(&project).await.into();
                        link.send_message(message);
                    };
                    spawn_local(js_future);
                }
            }
        }

        let metrics = match &self.fetch_state {
            FetchState::Success(g) => html! {
                <div style="display: inline;">
                    <div style="display: inline; margin-left: 0.4rem; margin-right: 0.2rem;">
                    <i class="fas fa-star fa-lg"/></div>
                    {g.stargazers_count}
                    <div style="display: inline; margin-left: 0.6rem; margin-right: 0.2rem;">
                    <i class="fas fa-code-branch fa-lg"/></div>
                    {g.forks_count}
                </div>
            },
            FetchState::Error(e) => html! {<div>{e}</div>},
            FetchState::Fetching => html! {<div>{"Fetching Github metrics"}</div>},
            FetchState::Never => html! {<div>{"Oops, something went wrong"}</div>},
        };

        html! {
        <div>
            <h3 id={project.name}>{project.name}</h3>
            <br/>
            <h6>{project.description}{", "}{project.languages.join(", ")}</h6>
            <div class="project-github">
                <a target="_blank" href={project.url} rel="noopener noreferrer">
                    <i class="fab fa-github fa-2x"/>
                </a>
            {metrics}
            </div>
            {tag_component}
            <a
                href={project.preview_url}
                rel="noopener noreferrer"
                target="_blank">
                <img class="project-image"
                    alt={project.name}
                    src={project.preview_thumbnail_url}/>
            </a>
            <br/>
        </div>
                }
    }
}

pub async fn get_github_metrics(project: &Project) -> FetchState {
    use web_sys::console;
    console::log_1(&"Fetching...".into());
    match web_sys::window() {
        None => FetchState::Error("Failed to get window".to_string()),
        Some(window) => {
            let mut opts = RequestInit::new();
            opts.method("GET");
            opts.mode(RequestMode::Cors);

            let api_url = format!(
                "https://api.github.com/repos/{}/{}",
                project.owner, project.repo_name
            );

            let req = match web_sys::Request::new_with_str_and_init(&api_url, &opts) {
                Ok(r) => r,
                Err(e) => {
                    return FetchState::Error(e.as_string().unwrap());
                }
            };
            let resp_val = match JsFuture::from(window.fetch_with_request(&req)).await {
                Ok(r) => r,
                Err(e) => {
                    return FetchState::Error(e.as_string().unwrap());
                }
            };
            let resp: Response = match resp_val.dyn_into() {
                Ok(r) => r,
                Err(e) => {
                    return FetchState::Error(e.as_string().unwrap());
                }
            };
            let json_val = match resp.json() {
                Ok(r) => r,
                Err(e) => {
                    return FetchState::Error(e.as_string().unwrap());
                }
            };
            let json = match JsFuture::from(json_val).await {
                Ok(r) => r,
                Err(e) => {
                    return FetchState::Error(e.as_string().unwrap());
                }
            };
            let github_metrics: serde_json::error::Result<GithubMetrics> = json.into_serde();
            match github_metrics {
                Ok(g) => FetchState::Success(g),
                Err(e) => FetchState::Error(e.to_string()),
            }
        }
    }
}
