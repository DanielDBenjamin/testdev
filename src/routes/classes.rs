use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use crate::routes::class_functions::get_module_classes_fn;
use crate::database::classes::Class;

#[component]
pub fn ClassesPage() -> impl IntoView {
    let query = use_query_map();
    
    let module_code = Signal::derive(move || {
        query.with(|q| q.get("module").unwrap_or_default())
    });

    // Load classes for the module
    let classes_resource = Resource::new(
        move || module_code.get(),
        |code| async move {
            if code.is_empty() {
                return None;
            }
            match get_module_classes_fn(code.clone()).await {
                Ok(response) if response.success => Some(response.classes),
                _ => None,
            }
        },
    );

    // Calculate stats
    let total_classes = Signal::derive(move || {
        classes_resource.get()
            .and_then(|c| c.as_ref().map(|classes| classes.len()))
            .unwrap_or(0)
    });

    let completed_classes = Signal::derive(move || {
        classes_resource.get()
            .and_then(|c| c.as_ref().map(|classes| {
                classes.iter().filter(|c| c.status == "completed").count()
            }))
            .unwrap_or(0)
    });

    let upcoming_classes = Signal::derive(move || {
        classes_resource.get()
            .and_then(|c| c.as_ref().map(|classes| {
                classes.iter().filter(|c| c.status == "upcoming").count()
            }))
            .unwrap_or(0)
    });

    view! {
        <section class="classes-page">
            <div class="classes-header">
                <div class="header-content">
                    <A href="/home" attr:class="back-link">"← Back"</A>
                    <div class="header-text">
                        <h1 class="page-title">"Data Structures & Algorithms"</h1>
                        <p class="page-subtitle">{move || format!("{} • 2025", module_code.get())}</p>
                    </div>
                </div>
                <div class="header-actions">
                    <button class="btn btn-outline">"⭳ Export"</button>
                    <A href=move || format!("/classes/new?module={}", module_code.get()) attr:class="btn btn-primary">"+ Add Class"</A>
                </div>
            </div>

            <div class="stats-row">
                <div class="stat-tile">
                    <div class="stat-value">{move || total_classes.get().to_string()}</div>
                    <div class="stat-label">"Total Classes"</div>
                </div>
                <div class="stat-tile">
                    <div class="stat-value" style="color:#10b981;">{move || completed_classes.get().to_string()}</div>
                    <div class="stat-label">"Completed"</div>
                </div>
                <div class="stat-tile">
                    <div class="stat-value" style="color:#2563eb;">{move || upcoming_classes.get().to_string()}</div>
                    <div class="stat-label">"Upcoming"</div>
                </div>
                <div class="stat-tile">
                    <div class="stat-value">"156"</div>
                    <div class="stat-label">"Enrolled Students"</div>
                </div>
            </div>

            <Suspense fallback=move || view! { <div class="loading">"Loading classes..."</div> }>
                {move || {
                    classes_resource.get().map(|classes_opt| {
                        match classes_opt {
                            Some(classes) if !classes.is_empty() => {
                                view! {
                                    <div class="classes-section">
                                        <div class="section-header">
                                            <h3 class="heading">"Classes Schedule"</h3>
                                            <div class="search-controls">
                                                <input class="input search-input" placeholder="Search classes..." />
                                                <button class="btn btn-outline">"All Status"</button>
                                            </div>
                                        </div>

                                        <div class="classes-table-wrapper">
                                            <table class="classes-table">
                                                <thead>
                                                    <tr>
                                                        <th>"Class Title"</th>
                                                        <th>"Date"</th>
                                                        <th>"Time"</th>
                                                        <th>"Venue"</th>
                                                        <th>"Status"</th>
                                                        <th>"Actions"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {classes.into_iter().map(|class| {
                                                        view! { <ClassRow class=class/> }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                            Some(_) => {
                                view! {
                                    <div class="empty-state">
                                        <p>"No classes yet. Create your first class to get started!"</p>
                                        <A href=move || format!("/classes/new?module={}", module_code.get()) attr:class="btn btn-primary">"+ Add Class"</A>
                                    </div>
                                }.into_any()
                            }
                            None => {
                                view! {
                                    <div class="empty-state">
                                        <p>"Module not found or no classes available."</p>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn ClassRow(class: Class) -> impl IntoView {
    let badge_class = match class.status.as_str() {
        "completed" => "status-badge completed",
        "in_progress" => "status-badge in-progress",
        _ => "status-badge upcoming",
    };

    let status_text = match class.status.as_str() {
        "completed" => "Completed",
        "in_progress" => "In Progress",
        "upcoming" => "Upcoming",
        _ => "Unknown",
    };

    let class_id = class.class_id;
    let status_in_progress = class.status.clone() == "in_progress";
    let status_upcoming = class.status.clone() == "upcoming";
    view! {
        <tr>
            <td>
                <div class="class-cell">
                    <div class="class-title">{class.title.clone()}</div>
                    <div class="class-subtitle">"Week 1 • Lecture 1"</div>
                </div>
            </td>
            <td>
                <div class="date-cell">
                    <div>{class.date.clone()}</div>
                </div>
            </td>
            <td>
                <div class="time-cell">
                    <div>{class.time.clone()}</div>
                    <div class="duration">"90 minutes"</div>
                </div>
            </td>
            <td>
                <div class="venue-cell">
                    <div>{class.venue.clone().unwrap_or_else(|| "TBA".to_string())}</div>
                    <div class="building">"Building A"</div>
                </div>
            </td>
            <td>
                <div class="status-cell">
                    <span class=badge_class>{status_text}</span>
                    <Show when=move || status_in_progress >
                        <button class="end-btn">"End"</button>
                    </Show>
                    <Show when=move || status_upcoming >
                        <button class="start-btn">"Start"</button>
                    </Show>
                </div>
            </td>
            <td>
                <div class="actions-cell">
                    <A href=format!("/classes/edit?id={}", class_id) attr:class="btn-icon edit">
                        <span>"✏"</span>
                        "Edit"
                    </A>
                    <button class="btn-icon remove">"🗑 Remove"</button>
                </div>
            </td>
        </tr>
    }
}