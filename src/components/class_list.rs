use leptos::prelude::*;

#[component]
pub fn ClassList() -> impl IntoView {
    view! {
        <ul class="class-list">
            <li class="class-item">
                <span class="dot dot-purple" aria-hidden="true"></span>
                <div class="class-info">
                    <div class="class-title">"CS112 Lecture"</div>
                    <div class="class-sub">"10:00 AM – Room 201"</div>
                </div>
            </li>
            <li class="class-item">
                <span class="dot dot-blue" aria-hidden="true"></span>
                <div class="class-info">
                    <div class="class-title">"CS201 Lab"</div>
                    <div class="class-sub">"2:00 PM – Lab 15"</div>
                </div>
            </li>
        </ul>
    }
}

