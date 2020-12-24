use crate::data::Experience;
use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub experience: &'static Experience,
}

#[derive(Clone, Debug)]
pub struct ExperienceHeader {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ExperienceHeader {
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
        let experience = &self.props.experience;
        let tag_component = html! {
            <ul id="tag-list">
                { experience.tags.iter()
                    .map(|tag| html!{ <li>{tag}</li> })
                    .collect::<Html>()
                }
            </ul>
        };

        html! {
        <div>
            <h2 id={experience.title}>{experience.title}</h2>
            <h4><a
                target="_blank"
                rel="noopener noreferrer"
                href={experience.company_url}>{experience.company}</a></h4>
            <h6>{experience.start_date}{" - "}{experience.end_date}</h6>
            {tag_component}
            <br/>
        </div>
                }
    }
}
