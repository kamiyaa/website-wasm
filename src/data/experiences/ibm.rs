use yew::prelude::*;

pub fn component() -> Html {
    html! {
    <div>
    <p>{ "Being part of the " }<b>{ "Runtimes" }</b>{ " team here at IBM, I was responsible for working " }<a target="_blank" href="https://www.eclipse.org/openj9/">{ "OpenJ9" }</a>
    { ", IBM's own open source implementation of Java." }
    </p>
    <br/>
    <h5><b>{ "Disassembler Work" }</b></h5>
    <p><b>{ "Background:" }</b>
    <br/>
    { "When a Java program running with OpenJ9 crashes, a "}<a target="_blank" href="https://en.wikipedia.org/wiki/Core_dump">{ "core dump" }</a>{ " is produced. This core dump allows developers of OpenJ9 to investiage the cause of the crash and come up with a solution to fix it. To analyze these core dumps, our team has developed a diassembler to read the core dumps for all IBM platforms. But in order to keep this tool up to date, a lot of work is required to add support for new instructions sets whenever new cpu architectures are released." }
    <br/>
    { "My first task was to port our internal disassembler to use "}
    <a target="_blank" href="http://www.capstone-engine.org/">{ "Capstone" }</a>
    { ", an open source disassembler library. This effort immediately added support for " }
    <b>{ "z13, z14 and Power9" }</b>{ " instructions to our disassembler, eliminating months of potential work needed to add those instructions." }
    <br/>
    { "Often when debugging a core dump, it requires a lot of time and effort to reach
certain points of the program. Developers wanted a way to easily restore sessions
and more powerful scripting support.
My next task was to alleviate this problem by adding scripting support
via " }<a target="_blank" href="https://www.lua.org/">{ "Lua" }</a>{". "}
    <b>{ "Lua" }</b>{ " is a great option to choose for our workflows because it was embeddable, allowing us to compile Lua only once and statically-linking it to our disassembler. This scripting support allowed users to quickly restore their debugging sessions and do much more powerful data manipulations." }
    <br/>
    { "More recently "}<a target="_blank" href="https://www.eclipse.org/openj9/">
    { "OpenJ9" }</a>{ " added support for "}<b>{ "AArch64" }</b>{ " and the team was given hardware to test with. I was responsible for adding and enabling AArch64 support to our disassembler." }
    <br/>
    { "Other features I've worked on for our internal disassembler:" }
    <ul>
    {
        [
            "Add support for demangling C++ names",
            "Refactor code base to be more maintainable",
            "Refactor build system to support cross compilations in the future"
        ].iter().map(|s| html!{<li>{s}</li>}).collect::<Html>()
    }
    </ul>

    { "All this required me to ensure it worked on all IBM-supported platforms, such as AIX, zOS, LinuxPPC and zLinux." }
    </p>
    <br/>
    <h5><b>{ "OpenJ9's Shared Class Cache" }</b></h5>
    <p>{ "One of the features OpenJ9 has is the "}<a target="_blank" href="https://developer.ibm.com/tutorials/j-class-sharing-openj9/">{ "Shared Classes Cache" }</a>
    { ". What it allows is for the OpenJ9 JVM to produce a cache of the Java classes used by the running program. By default, this feature is only enabled for Java std classes because it is still experimental. I was responsible for exploring and expanding its current usage. You can read more about it on the OpenJ9 blog post "}<a target="_blank" href="https://blog.openj9.org/2019/12/17/developing-with-appsody-and-openj9-scc/">{ "here" }</a>{ "." }
    </p>
    </div>
        }
}
