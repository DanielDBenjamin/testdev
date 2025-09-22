use leptos::prelude::*;
use crate::components::StatTile;

#[component]
pub fn Statistics() -> impl IntoView {
    view! {
        <section class="statistics">
            <h2 class="heading">"Statistics"</h2>
            <div class="stats-row">
                <StatTile value="315" label="Total Students"/>
                <StatTile value="12" label="Classes This Week"/>
                <StatTile value="24h" label="Teaching Hours"/>
            </div>
        </section>
    }
}

