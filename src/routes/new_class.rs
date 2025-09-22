use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NewClass() -> impl IntoView {
    let title = RwSignal::new(String::new());
    let venue = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let _recurring = RwSignal::new("No repeat".to_string());
    let hour = RwSignal::new("10".to_string());
    let minute = RwSignal::new("00".to_string());

    view! {
        <section class="new-class">
            <div class="page-header" style="display:flex;align-items:center;gap:8px;">
                <A href="/classes" attr:class="link">"← Back"</A>
                <h1 class="page-title">"New Class"</h1>
            </div>

            <div class="form-card">
                <div class="form-grid">
                    <div class="form-col">
                        <label class="label">"Title"</label>
                        <input class="input" placeholder="Linked List Fundamentals – Lecture 4" bind:value=title />

                        <label class="label" style="margin-top:10px;">"Venue"</label>
                        <input class="input" placeholder="Room A101" bind:value=venue />

                        <label class="label" style="margin-top:10px;">"Description"</label>
                        <textarea class="textarea" placeholder="Enter a class description" bind:value=desc></textarea>

                        <label class="label" style="margin-top:10px;">"Recurring"</label>
                        <select class="input">
                            <option selected>"No repeat"</option>
                            <option>"Daily"</option>
                            <option>"Weekly"</option>
                            <option>"Monthly"</option>
                        </select>
                    </div>

                    <aside class="form-side">
                        <div class="calendar mini">
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
                                {move || (1..=31).map(|d| view! { <div class="day">{d}</div> }).collect_view()}
                            </div>
                        </div>
                        <div class="time-picker">
                            <div class="time-box">
                                <input type="number" min="0" max="23" bind:value=hour />
                                <div class="t-label">"Hour"</div>
                            </div>
                            <div class="colon">":"</div>
                            <div class="time-box">
                                <input type="number" min="0" max="59" bind:value=minute />
                                <div class="t-label">"Minute"</div>
                            </div>
                        </div>
                    </aside>
                </div>

                <div class="actions-row">
                    <button class="btn btn-accent">"✓ Save Class"</button>
                    <button class="btn btn-primary">"▦ Start Session"</button>
                    <A href="/classes" attr:class="btn btn-outline">"× Cancel"</A>
                </div>
            </div>
        </section>
    }
}
