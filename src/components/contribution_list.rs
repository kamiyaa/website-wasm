use yew::prelude::*;

use crate::data::ContributionListType;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub list: &'static ContributionListType,
}

pub struct ContributionList {
    props: Props,
}

impl Component for ContributionList {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
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
            .map(|v| {
                html! {
                    <div class="horizontal_card">
                        <a
                            target="_blank"
                            rel="noopener noreferrer"
                            href={v.url}>
                            <img
                                class="horizontal_card_icon"
                                alt={v.name}
                                src={v.icon_url}
                            />
                        </a>
                        <div class="horizontal_card_info">

                            <h3 id={v.name}>{v.name}</h3>
                            <h5>{v.languages.join(", ")}</h5>
                            <p>{v.description}</p>
                            {(v.html)()}
                        </div>
                    </div>
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
