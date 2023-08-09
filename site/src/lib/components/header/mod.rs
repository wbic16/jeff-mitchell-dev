// src/lib/components/mod.rs

#![allow(non_snake_case)]

// dependencies
use yew::{function_component, html, Html};

#[function_component]
pub fn SiteHeader() -> Html {
    html! {
        <header>
          <h1>{ "Jeff Mitchell" }</h1>
          <h2>{ "Self-taught developer and army of one" }</h2>
          <h3>{ "Also known as the Crusty-Rustacean" }</h3>
        </header>
    }
}
