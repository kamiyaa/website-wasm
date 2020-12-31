use yew::prelude::*;

pub fn summary() -> Html {
    html! {
    <ul>
    <li>{ "- Taught tutorials of 30 students on various computer science topics" }</li>
    <li>{ "- Facilitated labs, office hours and study groups to help students understand advanced computer science topics" }</li>
    <li>{ "- Explained and clarified concepts such as polymorphism, inheritance, interfaces, pointers, memory allocations, generics and many more" }</li>
    </ul>
    }
}

pub fn content() -> Html {
    html! {
    <div>
    <p>{ "Courses taught:" }
    </p>
    <ul>
        <li><b>{ "CSCA08 - Introduction to Computer Science I" }</b></li>
        { "Using Python, students are introduced to the fundamental concepts of programming such as variables, conditional statements, loops, functions, lists, and dictionaries." }
        <br/>
        { "I was responsible for answering questions during anti-lectures, hosting office hours and marking exams." }
        <li><b>{ "CSCA48 - Introduction to Computer Science II" }</b></li>
        { "Students are introduced to more advanced programming concepts with C such as structs, linked data structures and recursion." }
        <br/>
        { "I taught a tutorial of 20-30 students; going over contents taught in lectures and coming up with additional examples to help students understand the course content better." }
        <li><b>{ "CSCB07 - Software Design" }</b></li>
        { "Utilizing Java's Object-Oriented programming style, students are introduced to the concepts of good design and design patterns." }
        <br/>
        { "I oversaw labs where students would apply techniques taught in class such as singleton, factory, builder, adapter, generics, interfaces and inheritance. I was also responsible for marking exams." }
        <li><b>{ "CSCB09 - Software Tools and System Design" }</b></li>
        { "Using C, students are taught systems programming. Introducing concepts such as Makefiles, header files, pointers (and double pointers), file I/O, processes (fork), pipes and sockets." }
        <br/>
        { "I was responsible for lab where students would apply techniques taught in class and marking exams." }
    </ul>
    </div>
        }
}
