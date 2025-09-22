use leptos::prelude::*;

#[component]
pub fn Header(title: &'static str, subtitle: &'static str) -> impl IntoView {
    view! {
        <header class="page-header">
            <h1 class="page-title">{title}</h1>
            <p class="page-subtitle">{subtitle}</p>
        </header>
    }
}

