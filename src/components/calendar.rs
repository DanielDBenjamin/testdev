use leptos::prelude::*;

#[component]
pub fn Calendar() -> impl IntoView {
    let days: Vec<i32> = (1..=31).collect();

    view! {
        <div class="calendar">
            <div class="calendar-header">
                <button class="cal-nav" aria-label="Previous Month">"‹"</button>
                <div class="month">"January 2024"</div>
                <button class="cal-nav" aria-label="Next Month">"›"</button>
            </div>
            <div class="calendar-grid">
                <div class="dow">"S"</div>
                <div class="dow">"M"</div>
                <div class="dow">"T"</div>
                <div class="dow">"W"</div>
                <div class="dow">"T"</div>
                <div class="dow">"F"</div>
                <div class="dow">"S"</div>
                {move || days.iter().map(|d| view! { <div class="day">{d.to_string()}</div> }).collect_view()}
            </div>
        </div>
    }
}

