use leptos::prelude::*;
use crate::components::StatTile;

#[component]
pub fn Statistics() -> impl IntoView {
    view! {
        <section class="statistics">
            <h2 class="heading">"Statistics"</h2>
            <div class="stats-row">
                <StatTile value=move || "315".to_string() label="Total Students"/>
                <StatTile value=move || "12".to_string() label="Classes This Week"/>
                <StatTile value=move || "24h".to_string() label="Teaching Hours"/>
            </div>
        </section>
    }
}

