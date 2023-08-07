// src/lib/routes/mod.rs

// dependencies
use crate::pages::about::AboutPage;
use crate::pages::home::HomePage;
use crate::pages::projects::ProjectsPage;
use yew::{html, Html};
use yew_router::prelude::*;

// an enum to define routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/projects")]
    Projects,
}

// switch function to control which route is rendered
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage />
        },
        Route::About => html! {
            <AboutPage />
        },
        Route::Projects => html! {
            <ProjectsPage />
        },
    }
}
