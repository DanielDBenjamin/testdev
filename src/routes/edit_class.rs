use crate::routes::class_functions::{
    delete_class_fn, get_class_fn, rewrite_recurring_series_fn, update_class_fn,
};
use crate::routes::helpers::build_return_path;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_query_map};

#[component]
pub fn EditClass() -> impl IntoView {
    let navigate = use_navigate();
    let query = use_query_map();
    let query_for_id = query.clone();
    let class_id = Signal::derive(move || {
        query_for_id.with(|q| {
            q.get("id")
                .and_then(|id| id.parse::<i64>().ok())
                .unwrap_or(0)
        })
    });
    let query_for_origin = query.clone();
    let origin =
        Signal::derive(move || query_for_origin.with(|q| q.get("origin").map(|s| s.to_string())));

    let title = RwSignal::new(String::new());
    let venue = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let recurring = RwSignal::new("No repeat".to_string());
    let recurrence_count = RwSignal::new("8".to_string());
    let original_recurring: RwSignal<Option<String>> = RwSignal::new(None);
    let date = RwSignal::new(String::new());
    let hour = RwSignal::new("10".to_string());
    let minute = RwSignal::new("00".to_string());
    let duration = RwSignal::new("90".to_string());
    let message = RwSignal::new(String::new());
    let success = RwSignal::new(false);
    let class_title_display = RwSignal::new(String::new());
    let original_title = RwSignal::new(String::new());
    let module_code = RwSignal::new(String::new());

    let module_code_for_return = module_code.clone();
    let origin_for_return = origin.clone();
    let return_path = Signal::derive(move || {
        let origin_val = origin_for_return.get();
        let module_val = module_code_for_return.get();
        build_return_path(origin_val, &module_val)
    });

    // Load class data
    let class_resource = Resource::new(
        move || class_id.get(),
        |id| async move {
            if id == 0 {
                return None;
            }
            match get_class_fn(id).await {
                Ok(response) if response.success => response.class,
                _ => None,
            }
        },
    );

    // Populate form when class loads
    Effect::new(move |_| {
        if let Some(Some(class)) = class_resource.get() {
            title.set(class.title.clone());
            class_title_display.set(class.title.clone());
            original_title.set(class.title.clone());
            module_code.set(class.module_code.clone());
            venue.set(class.venue.unwrap_or_default());
            desc.set(class.description.unwrap_or_default());
            let rec = class.recurring.clone();
            recurring.set(rec.clone().unwrap_or_else(|| "No repeat".to_string()));
            original_recurring.set(rec);
            date.set(class.date.clone());

            // Parse time
            let parts: Vec<&str> = class.time.split(':').collect();
            if parts.len() >= 2 {
                hour.set(parts[0].to_string());
                minute.set(parts[1].to_string());
            }

            duration.set(class.duration_minutes.max(15).to_string());
        }
    });

    let delete_modal_visible = RwSignal::new(false);

    // Delete class action
    let delete_action = Action::new(move |id: &i64| {
        let id = *id;
        async move { delete_class_fn(id).await }
    });

    Effect::new({
        let nav = navigate.clone();
        let return_path = return_path.clone();
        let message = message.clone();
        let success = success.clone();
        let delete_modal_visible = delete_modal_visible.clone();
        move |_| {
            if let Some(result) = delete_action.value().get() {
                match result {
                    Ok(response) => {
                        message.set(response.message.clone());
                        success.set(response.success);
                        if response.success {
                            delete_modal_visible.set(false);
                            let dest = return_path.get();
                            nav(&dest, Default::default());
                        }
                    }
                    Err(e) => {
                        message.set(format!("Error: {}", e));
                        success.set(false);
                    }
                }
            }
        }
    });

    let on_submit = move |_| {
        message.set(String::new());
        success.set(false);

        if title.get().trim().is_empty() {
            message.set("Please enter a class title".to_string());
            return;
        }

        if date.get().trim().is_empty() {
            message.set("Please select a date".to_string());
            return;
        }

        let time_str = format!("{}:{}", hour.get(), minute.get());
        let venue_val = if venue.get().trim().is_empty() {
            None
        } else {
            Some(venue.get())
        };
        let desc_val = if desc.get().trim().is_empty() {
            None
        } else {
            Some(desc.get())
        };
        let recurring_val = if recurring.get() == "No repeat" {
            None
        } else {
            Some(recurring.get())
        };
        let current_class_id = class_id.get();
        let current_title = title.get();
        let current_date = date.get();
        let nav = navigate.clone();
        let mod_code = module_code.get();
        let original_rec = original_recurring.get();
        let orig_title = original_title.get();
        let count_val = if recurring.get() != "No repeat" {
            recurrence_count.get().parse::<i32>().ok()
        } else {
            None
        };
        let return_to = return_path.get();
        let duration_minutes = duration.get().parse::<i32>().unwrap_or(90).max(15);

        spawn_local(async move {
            // If recurrence pattern changed, rewrite the series; otherwise update just this class
            let changed_recurrence = recurring_val != original_rec;
            let should_rewwrite =
                changed_recurrence || (recurring_val.is_some() && count_val.is_some());
            let resp = if should_rewwrite {
                rewrite_recurring_series_fn(
                    current_class_id,
                    mod_code.clone(),
                    orig_title,
                    original_rec,
                    current_title,
                    desc_val.clone(),
                    venue_val.clone(),
                    current_date.clone(),
                    time_str.clone(),
                    duration_minutes,
                    recurring_val.clone(),
                    count_val,
                )
                .await
            } else {
                update_class_fn(
                    current_class_id,
                    current_title,
                    desc_val,
                    current_date,
                    time_str,
                    duration_minutes,
                    venue_val,
                    recurring_val,
                )
                .await
            };

            match resp {
                Ok(response) => {
                    message.set(response.message);
                    success.set(response.success);
                    if response.success {
                        set_timeout(
                            move || {
                                let dest = return_to.clone();
                                nav(&dest, Default::default());
                            },
                            std::time::Duration::from_millis(1000),
                        );
                    }
                }
                Err(e) => {
                    message.set(format!("Error: {}", e));
                    success.set(false);
                }
            }
        });
    };

    view! {
        <section class="edit-class">
            <div class="header-content">
                <A href=move || return_path.get() attr:class="link">"←"</A>
                <div class="header-text">
                    <h1 class="page-title">"Edit Class: " {move || class_title_display.get()}</h1>
                </div>
            </div>

            <Suspense fallback=move || view! { <div class="loading">"Loading class..."</div> }>
                {move || {
                    let on_submit = on_submit.clone();
                    class_resource.get().map(move |_| {
                        view! {
                            <div class="form-card">
                                <div class="form-grid">
                                    <div class="form-col">
                                        <label class="label">"Title " <span class="required">"*"</span></label>
                                        <input class="input" placeholder="Hash Tables & Collision" bind:value=title />

                                        <label class="label" style="margin-top:16px;">"Venue"</label>
                                        <input class="input" placeholder="Room C301" bind:value=venue />

                                        <label class="label" style="margin-top:16px;">"Description"</label>
                                        <textarea class="textarea" placeholder="Enter a class description" bind:value=desc></textarea>

                                    <label class="label" style="margin-top:16px;">"Recurring"</label>
                                    <select class="input" bind:value=recurring>
                                        <option selected>"No repeat"</option>
                                        <option>"Daily"</option>
                                        <option>"Weekly"</option>
                                        <option>"Monthly"</option>
                                    </select>

                                    // Show recurrence count input if recurring is enabled
                                    <Show when=move || recurring.get() != "No repeat">
                                        <div style="margin-top:10px;">
                                            <label class="label">"Number of Occurrences"</label>
                                            <div style="display:flex; align-items:center; gap:12px;">
                                                <input
                                                    type="number"
                                                    class="input"
                                                    min="2"
                                                    max="52"
                                                    bind:value=recurrence_count
                                                    style="max-width:120px;"
                                                />
                                                <span class="muted" style="font-size:13px;">
                                                    {move || match recurring.get().as_str() {
                                                        "Daily" => "days",
                                                        "Weekly" => "weeks",
                                                        "Monthly" => "months",
                                                        _ => "instances"
                                                    }}
                                                </span>
                                            </div>
                                            <p class="muted" style="font-size:12px; margin-top:6px;">
                                                {move || {
                                                    let count = recurrence_count.get().parse::<i32>().unwrap_or(8);
                                                    let pattern = recurring.get();
                                                    format!("This will create {} {} class instances", count,
                                                        match pattern.as_str() {
                                                            "Daily" => "daily",
                                                            "Weekly" => "weekly",
                                                            "Monthly" => "monthly",
                                                            _ => ""
                                                        }
                                                    )
                                                }}
                                            </p>
                                        </div>
                                    </Show>
                                </div>


                                    <aside class="form-side">
                                        <label class="label">"Date"</label>
                                        <input class="input" type="date" bind:value=date />

                                        <label class="label" style="margin-top:16px;">"Enter time"</label>
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

                                        <label class="label" style="margin-top:16px;">"Duration"</label>
                                        <select class="input" bind:value=duration>
                                            <option value="45">"45 minutes"</option>
                                            <option value="60">"1 hour"</option>
                                            <option value="75">"1 hour 15 min"</option>
                                            <option value="90">"1 hour 30 min"</option>
                                            <option value="105">"1 hour 45 min"</option>
                                            <option value="120">"2 hours"</option>
                                        </select>
                                    </aside>
                                </div>

                                <Show when=move || !message.get().is_empty()>
                                    <p class=move || if success.get() { "success center" } else { "error center" } style="margin-top:16px;">
                                        {message}
                                    </p>
                                </Show>

                                <div class="actions-row">
                                    <button class="btn btn-accent" on:click=on_submit>"✓ Save Class"</button>
                                    <button
                                        class="btn btn-danger"
                                        disabled=move || delete_action.pending().get()
                                        on:click=move |_| delete_modal_visible.set(true)
                                    >"Delete Class"</button>
                                    <A href=move || return_path.get() attr:class="btn btn-outline">"✕ Cancel"</A>
                                </div>
                                <Show when=move || delete_modal_visible.get()>
                                    <div class="modal-overlay" on:click=move |_| delete_modal_visible.set(false)>
                                        <div class="modal-content" on:click=|e| e.stop_propagation()>
                                            <h2 class="modal-title">"Delete Class"</h2>
                                            <p class="modal-text">{format!("Are you sure you want to delete \"{}\"?", class_title_display.get())}</p>
                                            <div class="modal-actions">
                                                <button class="btn btn-outline" on:click=move |_| delete_modal_visible.set(false)>"Cancel"</button>
                                                <button
                                                    class="btn btn-danger"
                                                    disabled=move || delete_action.pending().get()
                                                    on:click=move |_| {
                                                        delete_action.dispatch(class_id.get());
                                                    }
                                                >{move || if delete_action.pending().get() { "Deleting..." } else { "Delete" }}</button>
                                            </div>
                                        </div>
                                    </div>
                                </Show>
                            </div>
                        }.into_any()
                    })
                }}
            </Suspense>
        </section>
    }
}
