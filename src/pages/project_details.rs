use yew::prelude::*;

use super::PageTemplate;
use crate::components::{MenuStrip, ProjectHeader};
use crate::data::Project;
use crate::routes::{AppRoute, Link};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub project: &'static Project,
}

#[derive(Clone, Debug)]
pub struct ProjectDetails {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ProjectDetails {
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
        html! {
        <PageTemplate>
            <div class="article">
                <div class="article-content">
                <ProjectHeader project={&self.props.project}/>
                {(self.props.project.content)()}
                </div>
            </div>
        </PageTemplate>
                }
    }
}
