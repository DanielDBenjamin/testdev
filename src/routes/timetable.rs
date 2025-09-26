use leptos::prelude::*;
use crate::components::{Calendar, ClassList, Header};

#[component]
pub fn Timetable() -> impl IntoView {
    view! {
        <section class="timetable">
            <Header title=move || "Schedule".to_string() subtitle="Review your weekly classes".to_string()/>
            <div class="timetable-grid">
                <div class="calendar-panel">
                    <Calendar/>
                </div>
                <aside class="today-panel">
                    <h3 class="heading">"Today's Classes"</h3>
                    <ClassList/>
                </aside>
            </div>
        </section>
    }
}
