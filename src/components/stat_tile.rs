use leptos::prelude::*;

#[component]
pub fn StatTile<V>(value: V, label: &'static str) -> impl IntoView
where
    V: Fn() -> String + 'static + Send,
{
    view! {
        <div class="stat-tile">
            <div class="stat-value">{move || value()}</div>
            <div class="stat-label">{label}</div>
        </div>
    }
}
