use leptos::prelude::*;
use crate::components::{Calendar, ClassList, Header};

#[component]
pub fn Timetable() -> impl IntoView {
    view! {
        <section class="timetable">
            <Header title="Schedule" subtitle="Review your weekly classes"/>
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

