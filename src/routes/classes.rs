use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use crate::routes::class_functions::{get_module_classes_fn, delete_class_fn, update_class_status_fn};
use crate::routes::module_functions::get_module_fn;
use crate::database::classes::Class;
use leptos::web_sys::window;

#[component]
pub fn ClassesPage() -> impl IntoView {
    let query = use_query_map();
    
    let module_code = Signal::derive(move || {
        query.with(|q| q.get("module").unwrap_or_default())
    });

    // Load module details
    let module_resource = Resource::new(
        move || module_code.get(),
        |code| async move {
            if code.is_empty() {
                return None;
            }
            match get_module_fn(code.clone()).await {
                Ok(response) if response.success => response.module,
                _ => None,
            }
        },
    );

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
                    <A href="/home" attr:class="link">"←"</A>
                    <div class="header-text">
                        <Suspense fallback=move || view! { <h1 class="page-title">"Loading..."</h1> }>
                            {move || module_resource.get().map(|module_opt| {
                                match module_opt {
                                    Some(module) => view! {
                                        <h1 class="page-title">{module.module_title.clone()}</h1>
                                        <p class="page-subtitle">{format!("{} • 2025", module.module_code)}</p>
                                    }.into_any(),
                                    None => view! {
                                        <h1 class="page-title">"Module Not Found"</h1>
                                        <p class="page-subtitle">{move || format!("{} • 2025", module_code.get())}</p>
                                    }.into_any()
                                }
                            })}
                        </Suspense>
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
    let class_title = class.title.clone();
    let current_status = RwSignal::new(class.status.clone());
    let show_delete_modal = RwSignal::new(false);
    
    let status_in_progress = Signal::derive(move || current_status.get() == "in_progress");
    let status_upcoming = Signal::derive(move || current_status.get() == "upcoming");
    
    // Delete action
    let delete_action = Action::new(move |id: &i64| {
        let id = *id;
        async move {
            delete_class_fn(id).await
        }
    });
    
    // Status update action
    let status_action = Action::new(move |(id, status): &(i64, String)| {
        let id = *id;
        let status = status.clone();
        async move {
            update_class_status_fn(id, status).await
        }
    });
    
    // Handle delete response
    Effect::new(move |_| {
        if let Some(result) = delete_action.value().get() {
            match result {
                Ok(response) if response.success => {
                    // Reload page on success
                    _ = window().unwrap().location().reload();
                }
                Ok(response) => {
                    leptos::logging::log!("Delete failed: {}", response.message);
                }
                Err(e) => {
                    leptos::logging::log!("Delete error: {}", e);
                }
            }
        }
    });
    
    // Handle status update response
    Effect::new(move |_| {
        if let Some(result) = status_action.value().get() {
            match result {
                Ok(response) if response.success => {
                    if let Some(updated_class) = response.class {
                        current_status.set(updated_class.status);
                    }
                }
                Ok(response) => {
                    leptos::logging::log!("Status update failed: {}", response.message);
                }
                Err(e) => {
                    leptos::logging::log!("Status update error: {}", e);
                }
            }
        }
    });
    
    let on_delete_click = move |_| {
        show_delete_modal.set(true);
    };
    
    let on_confirm_delete = move |_| {
        delete_action.dispatch(class_id);
        show_delete_modal.set(false);
    };
    
    let on_cancel_delete = move |_| {
        show_delete_modal.set(false);
    };
    
    let on_start = move |_| {
        status_action.dispatch((class_id, "in_progress".to_string()));
    };
    
    let on_end = move |_| {
        status_action.dispatch((class_id, "completed".to_string()));
    };
    
    view! {
        <>
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
                        <Show when=move || status_in_progress.get()>
                            <button class="end-btn" on:click=on_end>"End"</button>
                        </Show>
                        <Show when=move || status_upcoming.get()>
                            <button class="start-btn" on:click=on_start>"Start"</button>
                        </Show>
                    </div>
                </td>
                <td>
                    <div class="actions-cell">
                        <A href=format!("/classes/edit?id={}", class_id) attr:class="btn-icon edit">
                            <span>"✏"</span>
                            "Edit"
                        </A>
                        <button 
                            class="btn-icon remove" 
                            on:click=on_delete_click
                            disabled=move || delete_action.pending().get()
                        >
                            {move || if delete_action.pending().get() { 
                                "⏳".to_string() 
                            } else { 
                                "🗑 Remove".to_string() 
                            }}
                        </button>
                    </div>
                </td>
            </tr>
            
            <Show when=move || show_delete_modal.get()>
                <div class="modal-overlay" on:click=move |_| show_delete_modal.set(false)>
                    <div class="modal-content" on:click=|e| e.stop_propagation()>
                        <h2 class="modal-title">"Delete Class"</h2>
                        <p class="modal-text">
                            "Are you sure you want to delete "
                            <strong>{class_title.clone()}</strong>
                            " Class?"
                        </p>
                        <div class="modal-actions">
                            <button class="btn btn-outline" on:click=on_cancel_delete>"Cancel"</button>
                            <button class="btn btn-danger" on:click=on_confirm_delete>"Delete"</button>
                        </div>
                    </div>
                </div>
            </Show>
        </>
    }
}