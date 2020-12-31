use yew::prelude::*;

pub fn content() -> Html {
    html! {
    <div>
    <p>{ "This is the project for my personal website." }
    </p>
    <p>{ "Originally, I planned to write it using just HTML/CSS." }
    { "But after doing some research, I realized that being able to create "}
    <b>{ "React" }</b>
    { " applications are very important and useful. Taking advantage of this opportunity, I learned " }<b>{ "Javascript" }</b>{ " (and ES6) and React at the same time." }
    </p>
    <p>{ "One of the challenges I faced was just designing the website to be easy to navigate as well as appealing to users. Design is not something I had prior industry experience with so I had to do some research into designing user-friendly and intuitive interfaces." }
    { "Even after figuring out the design, translating it to sass/css was also a learning curve I had to overcome." }
    </p>
    </div>
        }
}
