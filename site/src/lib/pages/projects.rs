// src/lib/pages/projects.rs

#![allow(non_snake_case)]

// dependencies
use crate::components::footer::SiteFooter;
use crate::components::header::SiteHeader;
use crate::components::navigation::NavBar;
use yew::{function_component, html, Html};

#[function_component]
pub fn ProjectsPage() -> Html {
    html! {
        <>
            <SiteHeader />
            <br />
            <NavBar />
            <br />
            <main class="content">
                <section>
                    <article>
                        <h2>{ "Current Projects" }</h2>
                        <h3>{ "Zero to Production in Rust"}</h3>
                        <p>{ "My Zero to Production in Rust, with Axum, adventure is complete! Final code is "} <a href="https://github.com/sentinel1909/crusty-rustacean-api" target="_blank">{ "here"}</a></p>
                        <p>{"Loads of further work needs to be done though, to extend the book code and turn it into a real service."}</p>
                    </article>
                    <br />
                    <article>
                        <h3>{ "Markdown to HTML Converter"}</h3>
                        <p>{ "I made a Rust command line tool which takes a file with markdown (including frontmatter) as input, splits out the frontmatter, and converts the markdown body into HTML, outputting it as a file."}</p>
                        <p>{ "Source code is "} <a href="https://github.com/sentinel1909/markdown-html-rs" target="_blank">{ " here"}</a></p>
                    </article>
                    <br />
                    <article>
                        <h3>{ "Rivet Head Music Diary"}</h3>
                        <p>{ "I'm making a music diary app for myself. I want a way to track and record what I listen to during my morning and afternoon commutes to and from work.  So far, the API portion is coming togther. There is no front end yet."}</p>
                        <p>{ "Source code is "} <a href="https://github.com/sentinel1909/rivet-head-app" target="_blank">{ " here"}</a></p>
                    </article>
                </section>
            </main>
            <br />
            <SiteFooter />
        </>
    }
}
