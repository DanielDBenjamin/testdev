use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use crate::routes::class_functions::get_module_classes_fn;
use crate::database::classes::Class;

#[component]
pub fn ClassesPage() -> impl IntoView {
    let query = use_query_map();
    
    let module_code = Signal::derive(move || {
        query.read().get("module").unwrap_or_default()
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

    view! {
        <section class="classes-page">
            <div class="page-header" style="display:flex;align-items:center;gap:8px;">
                <A href="/home" attr:class="link">"← Back"</A>
                <div>
                    <h1 class="page-title">{move || module_code.get()}</h1>
                    <p class="page-subtitle">"2025"</p>
                </div>
                <div style="margin-left:auto; display:flex; gap:8px;">
                    <button class="btn btn-outline">"⭳ Export"</button>
                    <A href=move || format!("/classes/new?module={}", module_code.get()) attr:class="btn btn-primary">"+ Add Class"</A>
                </div>
            </div>

            <Suspense fallback=move || view! { <div class="loading">"Loading classes..."</div> }>
                {move || {
                    classes_resource.get().map(|classes_opt| {
                        match classes_opt {
                            Some(classes) if !classes.is_empty() => {
                                view! {
                                    <div class="card" style="padding:0; margin-top:16px;">
                                        <div style="display:flex; align-items:center; justify-content:space-between; padding:12px 14px; border-bottom:1px solid var(--sidebar-border);">
                                            <h3 class="heading" style="margin:0;">"Classes Schedule"</h3>
                                            <div style="display:flex; gap:8px;">
                                                <input class="input" placeholder="Search classes..." style="width:240px;" />
                                                <button class="btn btn-outline">"All Status"</button>
                                            </div>
                                        </div>

                                        <div style="overflow:auto;">
                                            <table class="table">
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
                                }.into_view()
                            }
                            Some(_) => {
                                view! {
                                    <div class="empty-state">
                                        <p>"No classes yet. Create your first class to get started!"</p>                                
                                    </div>
                                }.into_view()
                            }
                            None => {
                                view! {
                                    <div class="empty-state">
                                        <p>"Module not found or no classes available."</p>
                                    </div>
                                }.into_view()
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
        "completed" => "badge badge-green",
        "in_progress" => "badge badge-amber",
        _ => "badge badge-blue",
    };

    let status_display = match class.status.as_str() {
        "completed" => "Completed",
        "in_progress" => "In Progress",
        "upcoming" => "Upcoming",
        _ => &class.status,
    };

    view! {
        <tr>
            <td>
                <div class="t-title">{class.title.clone()}</div>
            </td>
            <td>{class.date.clone()}</td>
            <td>{class.time.clone()}</td>
            <td>{class.venue.unwrap_or_else(|| "TBA".to_string())}</td>
            <td><span class=badge_class>{status_display}</span></td>
            <td style="white-space:nowrap;">
                <button class="btn btn-outline btn-small">"Edit"</button>
                <button class="btn btn-outline btn-small" style="margin-left:6px; color:#ef4444; border-color:#fecaca;">"Remove"</button>
            </td>
        </tr>
    }
}