use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="container">
            <h2 class="heading">"Welcome"</h2>
            <p>
                "This is the home page of Clock-It. Use it as a starting point for your project"
                " and extend it with your own components, data fetching, and styling."
            </p>
            <p>
                "Click the About link in the navigation bar to see how routing works."
            </p>
        </section>
    }
}