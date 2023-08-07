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
                        <p>{ "Hello, I'm Jeff. I'm a software developer in Vancouver. Welcome to my corner of the web. I grew up around computers and cut my teeth coding in SMARTBasic on the Coleco ADAM computer system. In University, I learned how to code in Turbo Pascal, with a little C thrown in for good measure.  In the mid to late nineties, I embraced the world wide web and made my first very simple web sites.  Most of what I make for the web now looks like it came from that era, but I digress..." }</p>
                        <p>{ "My day job doesn't involve software development, so I drifted away from programming for a long number of years. At the end of 2020, in the height of COVID, I decided I wanted to pick up coding again." }</p>
                        <br />
                        <h3>{ "Why Rust?" }</h3>
                        <p>{ "I started with C++, but quickly realized that I would never be confident with it. I also want to create things to view and consume on the web, which C++ isn't really the greatest for.  I know you can (with WebAssembly) but I never pursued that. I discovered Rust thanks to Paul Thurrott and Leo Laporte on the Windows Weekly podcast. I dove in head first and haven't looked back. Rust empowers you to build reliable and efficient software. I fell in love with this notion, along with the tooling, ecosystem, and community. " }</p>
                        <br />
                        <h3>{ "My Site" }</h3>
                        <p>{ "My site will never have ads, affiliate links, tracking, analytics, paywall or sponsored posts. I want to share what I learn about Rust and hopefully inspire others to pick it up. I'm a fish swimming up river here, because right in this moment the world feels the negative effects of reliable and efficient software every single day. I hope this site serves as an inspiration that we can and should do better. A secondary goal is to show the world that Rust is a full stack language, capable of making all the things." }</p>
                        <br />
                        <h3>{ "My Inspiration" }</h3>
                        <p>{"This site is heavily inspired by the wonderful work of "} <a href="https://taniarascia.com" target="_blank">{ "Tania Rascia." }</a>{ " I hope to create a valuable resource like she has, but with a Rust flavour."}</p>
                    </article>
                </section>
            </main>
            <br />
            <SiteFooter />
        </>
    }
}
