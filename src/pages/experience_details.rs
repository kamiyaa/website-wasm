use yew::prelude::*;

use super::PageTemplate;
use crate::components::{ExperienceHeader, MenuStrip};
use crate::data::{AppRoute, Link, Experience};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub experience: &'static Experience,
}

#[derive(Clone, Debug)]
pub struct ExperienceDetails {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ExperienceDetails {
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
            <Link classes="article-topbar-link"
                route={AppRoute::Experiences}>
            <div class="menu-strip-item">
                <div class="fa fa-angle-left fa-lg"/>
                <div style="display: inline; margin-left: 0.5rem;">
                { "Experiences" }
                </div>
            </div>
            </Link>
        };
        let content = (self.props.experience.html)();

        html! {
        <PageTemplate>
        <div class="article">
            <MenuStrip>{bookmarks_component}</MenuStrip>
            <div class="article-content">
            <ExperienceHeader experience={&self.props.experience}/>
            {content}
            </div>
        </div>
        </PageTemplate>
                }
    }
}
