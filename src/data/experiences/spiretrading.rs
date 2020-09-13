use yew::prelude::*;

pub fn component() -> Html {
    html! {
    <div>
    <p>{ "Normally I would write a page on what I did, but this time I decided to make a video instead." }
    <br/>
    { "I hope you find it entertaining and educational!" }
    <br/>
    </p>
    <div>
        <iframe width="853" height="480" src="https://www.youtube.com/embed/c1iEh6TlQeQ" frameborder="0" allow="accelerometer; encrypted-media; gyroscope; picture-in-picture; fullscreen"></iframe>
    </div>
    </div>
        }
}
