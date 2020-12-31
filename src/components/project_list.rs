use yew::prelude::*;

use crate::components::{ContributionList, MenuStrip, ProjectCard};
use crate::data::{ProjectListType, CONTRIBUTION_LIST};
use crate::routes::AppRoute;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub list: &'static ProjectListType,
}

#[derive(Clone, Debug)]
pub struct ProjectList {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ProjectList {
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
        let list_content = self
            .props
            .list
            .iter()
            .enumerate()
            .map(|(i, v)| {
                html! {
                    <ProjectCard project={v} route={AppRoute::Project(i)}/>
                }
            })
            .collect::<Html>();

        html! {
        <div class="flex_card_list">
            {list_content}
        </div>
            }
    }
}
