// src/lib/components/navigation/mod.rs

#![allow(non_snake_case)]

// dependencies
use crate::routes::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav>
          <ul class="nav-links">
            <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
            <li><Link<Route> to={Route::About}>{ "About" }</Link<Route>></li>
            <li><a href="https://blog.crusty-rustacean.dev" target="_blank">{ "Blog" }</a></li>
            <li><Link<Route> to={Route::Projects}>{ "Projects" }</Link<Route>></li>
            <li><a href="https://github.com/sentinel1909" target="_blank">{ "GitHub" }</a></li>
          </ul>
        </nav>
    }
}
