// site/src/main.rs

// dependencies
use leptos::*;

// the <App /> component
#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <header>
                <h1>"jeff-mitchell.dev"</h1>
            </header>
            <main>
                <section>
                    <article>
                        <h2>"Welcome to my site!"</h2>
                        <p>"I'm a self-taught software developer in Vancouver. I love the Rust programming language."</p>
                    </article>
                </section>
            </main>
            <footer>
                <p>"\u{00A9} Jeffery D. Mitchell"</p>
            </footer>
        </div>
    }
}

// main function
fn main() {
    mount_to_body(|| view! { 
        <App />
     })
}
