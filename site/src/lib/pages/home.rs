// src/lib/pages/home.rs

// dependencies
use crate::components::footer::SiteFooter;
use crate::components::header::SiteHeader;
use crate::components::navigation::NavBar;
use yew::{function_component, html, Html};

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <>
            <SiteHeader />
            <br />
            <NavBar />
            <br />
            <main class="content">
                <section>
                    <article>
                        <h2>{ "Welcome to my site" }</h2>
                        <p>{ "I'm a self-taught software developer in Vancouver. I love the Rust programming language, it's my primary focus. I can build in good 'ol HTML/CSS/JavaScript. I'm a fan of typing though, so I prefer TypeScript over plain JavaScript. My motto is: Rust on the back, TypeScript on the front." }</p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Rust short take of the day" }</h2>
                        <p>{ "Traits in Rust are a tool for implementing shared functionality, either within your own program or within someone else's" }</p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Newsletter"}</h2>
                        <p>{"My newsletter service has been soft launched. The sign-up page can be found here: "} <a href="https://newsletter.crusty-rustacean.dev" rel="noreferrer" target="_blank"> {" Crusty Rustacean Newsletter"}</a></p>
                    </article>
                </section>
            </main>
            <br />
            <SiteFooter />
        </>
    }
}
