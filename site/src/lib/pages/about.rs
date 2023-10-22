// src/lib/pages/about.rs

#![allow(non_snake_case)]

// dependencies
use crate::components::footer::SiteFooter;
use crate::components::header::SiteHeader;
use crate::components::navigation::NavBar;
use yew::{function_component, html, Html};

#[function_component]
pub fn AboutPage() -> Html {
    html! {
        <>
            <SiteHeader />
            <br />
            <NavBar />
            <br />
            <main class="content">
                <section >
                    <article>
                        <h2>{ "About Me" }</h2>
                        <p>{ "Hello, I'm Jeff. I'm a professional engineer by day and up-and-coming software developer by night. I live in Vancouver, Canada. Welcome to my corner of the world wide web. I grew up around computers and cut my teeth coding in SMARTBasic on the Coleco ADAM computer system. In University, I learned how to code in Turbo Pascal, with a little C thrown in for good measure.  In the mid to late nineties, I embraced the world wide web and made my first very simple web sites.  Most of what I make for the web now looks like it came from that era, but I digress..." }</p>
                        <p>{ "My day job doesn't involve software development, so I drifted away from programming for a long number of years. At the end of 2020, in the dark hours of COVID, I decided I wanted to pick up coding again." }</p>
                        <br />
                        <h3>{ "Why Rust?" }</h3>
                        <p>{ "I started with C++, but quickly realized (I sort of already knew) that it was a harsh mistress and I would never be truly confident with it. I also want to create things to view and consume on the web. As a fledgling, solo-developer, I needed something better...enter Rust. Rust is a language which enables people to build reliable and efficient software. I fell in love almost immediately with this notion." }</p>
                        <br />
                        <h3>{ "My Site" }</h3>
                        <p>{ "My hope is that this site serves as an inspiration of what Rust is capable of. It's truly a full stack language, and with up an coming services like Shuttle, whom I proudly support and host nearly everything with, is an unbeatable choice for developing your next big idea. I'm endeavouring to keep this site as pure as I can, in terms of the tech it's based on." }</p>
                        <br />
                    </article>
                </section>
            </main>
            <br />
            <SiteFooter />
        </>
    }
}
