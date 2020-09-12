use yew::prelude::*;

use super::PageTemplate;
use crate::components::{ContributionList, MenuStrip, ProjectCard};
use crate::data::{AppRoute, ProjectListType, CONTRIBUTION_LIST};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub list: &'static ProjectListType,
}

#[derive(Clone, Debug)]
pub struct ProjectsList {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ProjectsList {
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
        let bookmarks = ["Projects", "Contributions"];
        let bookmarks_component = bookmarks
            .iter()
            .map(|b| {
                html! {
                    <a class="article-topbar-link" href={format!("#{}", b)}>{b}</a>
                }
            })
            .collect::<Html>();

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
        <PageTemplate>
        <div class="article">
            <MenuStrip>{bookmarks_component}</MenuStrip>
            <div class="article-content">
            <h1 id="Projects">{"Projects"}</h1>
                <div class="flex_card_list">
                    {list_content}
                </div>
            <h1 id="Contributions">{"Contributions"}</h1>
                <div>
                    <ContributionList list={&CONTRIBUTION_LIST}/>
                </div>
            </div>
        </div>
        </PageTemplate>
                }
    }
}
