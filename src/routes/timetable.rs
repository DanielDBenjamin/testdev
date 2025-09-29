use leptos::prelude::*;
use crate::components::{Calendar, ClassList, Header};
use crate::database::classes::Class;

#[component]
pub fn Timetable() -> impl IntoView {
    let empty_classes = Signal::derive(|| Vec::<Class>::new());
    let on_date_select = Callback::new(|_date: String| {});
    
    view! {
        <section class="timetable">
            <Header title=move || "Schedule".to_string() subtitle="Review your weekly classes".to_string()/>
            <div class="timetable-grid">
                <div class="calendar-panel">
                    <Calendar classes=empty_classes on_date_select=on_date_select/>
                </div>
                <aside class="today-panel">
                    <h3 class="heading">"Today's Classes"</h3>
                    <ClassList classes=empty_classes/>
                </aside>
            </div>
        </section>
    }
}
