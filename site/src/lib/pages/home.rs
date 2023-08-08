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
                        <p>{ "I'm a software developer in Vancouver. I love "} <a href="https://www.rust-lang.org" target="_blank"> { "Rust "}</a> { " and have embraced it as my thing. I like heavy music, sci-fi, landscapes, sunrises and sunsets. I will write here about all of these things." }</p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Latest blog post"}</h2>
                        <p>{ "I've posted a short "} <a href="https://crusty-rustacean-blog.shuttleapp.rs/rust-learning-journal-week-1/" target="_blank"> {" piece"}</a> {" on my blog about how convert a struct into a vector so you can iterate over its fields."}</p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Rust short take of the day" }</h2>
                        <p>{ "Rust has a strong type system which can be leveraged to great effect." }</p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Sites I enjoy" }</h2>
                        <p><a href="https://apod.nasa.gov/apod/astropix.html" target="_blank">{ "Astronomy Picture of the Day" }</a></p>
                    </article>
                    <br />
                    <article>
                        <h2>{ "Newsletter"}</h2>
                        <p>{"My newsletter service has been soft launched. The sign-up page can be found here: "} <a href="https://newsletter.crusty-rustacean.dev" target="_blank"> {" Crusty Rustacean Newsletter"}</a></p>
                    </article>
                </section>
            </main>
            <br />
            <SiteFooter />
        </>
    }
}
