use leptos::prelude::*;

#[component]
pub fn ModuleCard(code: &'static str, name: &'static str, desc: &'static str, students: u32) -> impl IntoView {
    view! {
        <div class="card module-card">
            <div class="module-icon" aria-hidden="true">"💠"</div>
            <div class="module-body">
                <div class="module-title">{code} " — " {name}</div>
                <p class="module-desc">{desc}</p>
                <div class="module-meta">
                    <span class="students">{students} " students"</span>
                    <button class="btn btn-primary">"+ Add Class"</button>
                </div>
            </div>
        </div>
    }
}

