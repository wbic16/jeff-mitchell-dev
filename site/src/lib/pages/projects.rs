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
                        <p>{ "My Zero to Production in Rust, with Axum, adventure is complete! Final code is "} <a href="https://github.com/sentinel1909/crusty-rustacean-api" target="_blank">{ "here"}</a></p>
                        <p>{"Loads of further work needs to be done though, to extend the book code and turn it into a real service."}</p>
                    </article>
                </section>
            </main>
            <br />
            <SiteFooter />
        </>
    }
}
