// src/lib/app.rs

// dependencies
use crate::routes::{switch, Route};
use yew::prelude::*;
use yew_router::BrowserRouter;
use yew_router::Switch;

// root App component
#[function_component]
pub fn App() -> Html {
    html! {
      <div class={classes!("container")}>
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
      </div>
    }
}
