use yew::prelude::*;

pub fn content() -> Html {
    html! {
    <div>
        <h5>{ "Background" }</h5>
        <p>
        { "At first, I wanted to learn " }<b>{ "C++" }</b>{ " because it had all the features of a modern language while still being comparable to C in performance. While researching about C++, I stumbled across Rust, the shiny new language that supposedly competes in the same space as C and C++. Curious and attracted by all the benefits it claimed, I decided to learn Rust instead. This project is encapsulates much of what I learned of the Rust programming Language." }
        <br/>
        { "Everyday, I use a popular terminal file manager called "}
        <b><a
            target="_blank"
            rel="noopener noreferrer"
            href="https://ranger.github.io/">{ "Ranger" }</a>
        </b>
        { ". It is written in Python and for the most part, it did its job well. However, one of the major issues I had using Ranger was its performance. Load times were a bit slow on large directory listings and file operations were very slow, especially operations that was across a MTP device (such as an Android phone)." }
        { "So with my newly learned Rust skills, I decided to write my own file manager that incorporates many of the features of Ranger, while keeping it lean and fast." }
        </p>
        <br/>
        <h5>{ "Challenges" }</h5>
        <p>{ "There were many challenges I faced along the way. The first was Rust's borrow checker. Compared to most other languages, Rust has a very strict programming style. Without a garbage collector and manual memory management, developers must let the Rust compiler know exactly when a value is no longer used and can be freed. Rust also will only allow either n immutable borrows or 1 mutable borrow at any given time." }
        { "In the beginning, it was very hard fighting with the borrow checker. For example:" }
        <div class="code-block">
            <code>
            { "let myStruct = myStruct::new();" }
            <br/>
            { "let myVar = &myStruct.fieldA;" }
            <br/>
            { "func(&mut myStruct);" }
            </code>
        </div>
        { "This code would not compile because we are passing myStruct as a mutable variable into while myVar holds an immutable reference to myStruct." }
        <br/>
        { "Over time, I came to like the Rust's borrow checker because it is very satisfying to know that if your code compiles, it has a very low chance of containing bugs." }
        <br/>
        { "Another challenge I had was concurrency. Being able to manage file operations in the background (i.e. Pasting jobs) while maintaining a responsive UI was very challenging. At first, I had a thread listening to stdin for user input and send that input to the main thread. IO workers would also send input to the main thread. But if I were to launch a terminal application, my input thread would be stealing all the inputs from the application. So I had to come up with a way to tell my input thread to stop reading. This led me to design a more complicated set up where the main thread also talks with the input thread. This allowed to main thread to essentially control the input thread and tell it exactly when it wants input." }
        </p>
        <br/>
        <h5>{ "Conclusion" }</h5>
        <p>{ "Overall, I am very proud of this project. I also really enjoyed programming in Rust. And although, some concepts are very hard to express in Rust (linked data structures), it's benefits are very attractive." }
        { "With the rise of other Companies adopting Rust, there will definitely be a place for Rust in the future." }
        </p>
        <ul>
            <li><a
                target="_blank"
                rel="noopener noreferrer"
                href="https://aws.amazon.com/blogs/opensource/aws-sponsorship-of-the-rust-project/">
                { "AWS' sponsorship of the Rust project" }
                </a>
            </li>
            <li><a
                target="_blank"
                rel="noopener noreferrer"
                href="https://blog.discordapp.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f">
                { "Why Discord is switching from Go to Rust" }
                </a>
            </li>
        </ul>
    </div>
        }
}
