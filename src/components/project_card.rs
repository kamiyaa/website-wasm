use yew::prelude::*;

use crate::data::{AppRoute, Link, Project};

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
                         <img alt={project.name} src={project.preview_url}/>
                </div>
                <div class="flex_card_info">
                        <h3>{project.name}</h3>
                        <h5>{project.languages.join(", ")}</h5>
                        <p>{project.description}</p>
                </div>
            </Link>
        </div>
                }
    }
}
