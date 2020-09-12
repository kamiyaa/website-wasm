use yew::prelude::*;

pub fn component() -> Html {
    html! {
    <div>
    <p>{ "This was my first exposure to Rust's WebAssembly facilities. From this I've learned lots about how Rust/WebAssembly interfaces with Javascript. I also learned more about webpack and how to package Javascript apps to be deployable on the web." }
    </p>
    <a
        target="_blank"
        rel="noopener noreferrer"
        href="https://kamiyaa.github.io/game-of-life/">
        <button>{ "Try it here!" }</button></a>
    </div>
        }
}
