// src/lib/components/footer/mod.rs

#![allow(non_snake_case)]

// dependencies
use chrono::{Datelike, Local};
use yew::{function_component, html, Html};

#[function_component]
pub fn SiteFooter() -> Html {
    let year = Local::now().year();
    html! {
        <footer>
            <section class="footer-links">
                <br />
                <p>{ "\u{00A9} " } {year} { " Made by Jeffery D Mitchell | All Rights Reserved | Site created in "} <a href="https://webassembly.org/" target="_blank">{ "WebAssembly" }</a> { " with " }<a href="https://yew.rs" target="_blank">{ " Yew"}</a></p>
                <p>{ "Follow me on " }<a href="https://x.com/sentinel1909" target="_blank">{ "X" }</a></p>
                <p><a href="https://www.buymeacoffee.com/sentinel1909" target="_blank">{ "Buy me a coffee?" }</a></p>
                <p>{" Site hosting and custom domain courtesy of: " }<a href="https://shuttle.rs" target="_blank">{ "shuttle.rs" }</a></p>
                <p>{"Background image courtesy of Summer Li from "} <a href="https://pexels.com" target="_blank">{ "pexels.com"}</a></p>
                <p>{ "This site may contain trace amounts of JavaScript." }</p>
            </section>
        </footer>
    }
}
