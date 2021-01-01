use yew::prelude::*;

use crate::components::ContributionCard;
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
            .map(|v| html! { <ContributionCard contribution={v}/> })
            .collect::<Html>();

        html! {
        <div class="flex_card_list">
            {list_content}
        </div>
                }
    }
}
