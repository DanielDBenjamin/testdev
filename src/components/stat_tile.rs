use leptos::prelude::*;

#[component]
pub fn StatTile(value: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <div class="stat-tile">
            <div class="stat-value">{value}</div>
            <div class="stat-label">{label}</div>
        </div>
    }
}

