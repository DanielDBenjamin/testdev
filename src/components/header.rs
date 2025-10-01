use leptos::prelude::*;

#[component]
pub fn Header<F>(title: F, subtitle: String) -> impl IntoView 
where
    F: Fn() -> String + 'static + Send,
{
    view! {
        <header class="page-header">
            <h1 class="page-title">{title}</h1>
            <p class="page-subtitle">{subtitle}</p>
        </header>
    }
}