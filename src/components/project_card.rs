use yew::prelude::*;

use crate::data::Project;
use crate::routes::{AppRoute, Link};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub project: &'static Project,
    pub route: AppRoute,
}

#[derive(Clone, Debug)]
pub struct ProjectCard {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ProjectCard {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let project = &self.props.project;
        html! {
        <div class="flex_card">
            <Link route={self.props.route.clone()}>
                <div class="flex_card_img_container">
                    <div class="flex_card_img">
                         <img alt={project.name} src={project.preview_url}/>
                    </div>
                </div>
                <div class="flex_card_info">
                        <h4>{project.name}</h4>
                        <h6>{project.languages.join(", ")}</h6>
                        <p>{project.description}</p>
                </div>
            </Link>
        </div>
                }
    }
}
