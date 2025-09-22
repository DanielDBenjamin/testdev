use leptos::prelude::*;


#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="container">
            <h1 class="heading">"About Clock-It"</h1>
            <p>
                "Clock-It is a simple demo built with Leptos. It shows how to set up multiple pages"
                " using the built-in router and a bit of shared layout."
            </p>
            <p>
                "You can use this page to expand on the story of the app, add your own components,"
                " or link to external resources."
            </p>
        </section>
    }
}
