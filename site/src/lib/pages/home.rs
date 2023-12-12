// src/lib/pages/home.rs

// dependencies
use crate::components::footer::SiteFooter;
use crate::components::header::SiteHeader;
use crate::components::navigation::NavBar;
use yew::{function_component, classes, html, Html};

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <>
            <SiteHeader />
            <br />
            <NavBar />
            <br />
            <main class={classes!("content")}>
                <section>
                    <article>
                        <h2>{ "Welcome to my site" }</h2>
                        <p>{ "I'm a self-taught software developer in Vancouver. I love the Rust programming language. I can build in good 'ol HTML/CSS/JavaScript as well. I'm a fan of strong typing though, so I prefer TypeScript if I can't use Rust for any frontend business. My motto is: Rust on the back, TypeScript on the front." }</p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Rust short take of the day" }</h2>
                        <p>{ "Traits in Rust are a tool for implementing shared functionality. You can write your own traits, which let's others implement functionality in their programs, or you can implement other's traits in your own programs." }</p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Rust crate of the month" }</h2>
                        <p>{ "The Rust crate of the month is " } <a href="https://crates.io/crates/nom" rel="noreferrer" target="_blank"> {"Nom"}</a></p>
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
