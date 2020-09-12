use yew::prelude::*;

use super::PageTemplate;
use crate::components::{MenuStrip, ProjectHeader};
use crate::data::{AppRoute, AppRouteAnchor, Project};

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
        let bookmarks_component = html! {
            <AppRouteAnchor classes="article-topbar-link"
                route={AppRoute::Projects}>
            <div class="menu-strip-item">
                <div class="fa fa-angle-left fa-lg"/>
                <div style="display: inline; margin-left: 0.5rem;">
                { "Projects" }
                </div>
            </div>
            </AppRouteAnchor>
        };
        let content = (self.props.project.html)();

        html! {
        <PageTemplate>
        <div class="article">
            <MenuStrip>{bookmarks_component}</MenuStrip>
            <div class="article-content">
            <ProjectHeader project={&self.props.project}/>
            {content}
            </div>
        </div>
        </PageTemplate>
                }
    }
}
