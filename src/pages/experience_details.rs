use yew::prelude::*;

use super::PageTemplate;
use crate::components::{ExperienceHeader, MenuStrip};
use crate::data::Experience;
use crate::routes::{AppRoute, Link};

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
        html! {
        <PageTemplate>
            <div class="article">
                <div class="article-content">
                <ExperienceHeader experience={&self.props.experience}/>
                {(self.props.experience.content)()}
                </div>
            </div>
        </PageTemplate>
                }
    }
}
