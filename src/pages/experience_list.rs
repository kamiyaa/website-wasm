use yew::prelude::*;

use super::PageTemplate;
use crate::components::{ExperienceCard, MenuStrip};
use crate::data::{AppRoute, ExperienceListType, EXTRACURRICULAR_LIST};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub list: &'static ExperienceListType,
}

#[derive(Clone, Debug)]
pub struct ExperienceList {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ExperienceList {
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
        let bookmarks = ["Experiences", "Extracurricular"];
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
                    <ExperienceCard experience={v} route={AppRoute::Experience(i)}/>
                }
            })
            .collect::<Html>();

        html! {
        <PageTemplate>
        <div class="article">
            <MenuStrip>{bookmarks_component}</MenuStrip>
            <div class="article-content">
            <h1 id="Experiences">{"Experiences"}</h1>
                <div class="flex_card_list">
                    {list_content}
                </div>
            <h1 id="Extracurricular">{"Extracurricular"}</h1>
                <div>
                {extracurricular_content()}
                </div>
            </div>
        </div>
        </PageTemplate>
                }
    }
}

fn extracurricular_content() -> Html {
    EXTRACURRICULAR_LIST
        .iter()
        .map(|e| {
            html! {
                <div>
                <a target="_blank" rel="noopener noreferrer"
                    href={e.url}><h4>{e.name}</h4></a>
                <h6><b>{e.title}</b></h6>
                {e.date}

                {(e.html)()}
                </div>
            }
        })
        .collect::<Html>()
}
