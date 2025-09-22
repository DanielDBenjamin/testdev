use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone)]
struct Student { id: &'static str, name: &'static str, email: &'static str }

#[component]
pub fn NewModule() -> impl IntoView {
    let title = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let students = RwSignal::new(vec![
        Student { id: "STU001", name: "John Smith", email: "john.smith@university.edu" },
        Student { id: "STU002", name: "Sarah Johnson", email: "sarah.johnson@university.edu" },
    ]);

    view! {
        <section class="new-module">
            <div class="page-header" style="display:flex;align-items:center;gap:8px;">
                <A href="/home" attr:class="link">"← Back"</A>
                <h1 class="page-title">"New Module"</h1>
                <p class="page-subtitle" style="margin-left:8px;">"Create a new module and manage student enrollment"</p>
            </div>

            <div class="form-card">
                <h3 class="heading">"Module Information"</h3>
                <label class="label" style="margin-top:6px;">"Module Title "<span style="color:#ef4444;">"*"</span></label>
                <input class="input" placeholder="e.g., CS112 – Introduction to Programming" bind:value=title />

                <label class="label" style="margin-top:10px;">"Description"</label>
                <textarea class="textarea" placeholder="Enter module description..." bind:value=desc></textarea>

                <div class="divider"></div>

                <div class="heading" style="display:flex; align-items:center; justify-content:space-between;">
                    <span>"Student Management"</span>
                    <div style="display:flex; gap:8px;">
                        <button class="btn btn-outline">"⭳ Import Class List"</button>
                        <button class="btn btn-accent">"+ Add Student"</button>
                    </div>
                </div>

                <div class="card" style="padding:0; margin-top:10px;">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>"Student ID"</th>
                                <th>"Name"</th>
                                <th>"Email"</th>
                                <th>"Action"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || students.get().into_iter().map(|s| view! {
                                <tr>
                                    <td>{s.id}</td>
                                    <td>{s.name}</td>
                                    <td>{s.email}</td>
                                    <td><button class="btn btn-outline btn-small" style="color:#ef4444; border-color:#fecaca;">"🗑 Remove"</button></td>
                                </tr>
                            }).collect_view()}
                        </tbody>
                    </table>
                </div>

                <div class="actions-row">
                    <button class="btn btn-accent">"Save Module"</button>
                    <A href="/home" attr:class="btn btn-outline">"Cancel"</A>
                </div>
            </div>
        </section>
    }
}

