use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_query_map};
use crate::routes::class_functions::{get_class_fn, update_class_fn, rewrite_recurring_series_fn};

#[component]
pub fn EditClass() -> impl IntoView {
    let navigate = use_navigate();
    let query = use_query_map();
    
    let class_id = Signal::derive(move || {
        query.with(|q| q.get("id").and_then(|id| id.parse::<i64>().ok()).unwrap_or(0))
    });
    
    let title = RwSignal::new(String::new());
    let venue = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let recurring = RwSignal::new("No repeat".to_string());
    let recurrence_count = RwSignal::new("8".to_string());
    let original_recurring: RwSignal<Option<String>> = RwSignal::new(None);
    let date = RwSignal::new(String::new());
    let hour = RwSignal::new("10".to_string());
    let minute = RwSignal::new("00".to_string());
    let message = RwSignal::new(String::new());
    let success = RwSignal::new(false);
    let class_title_display = RwSignal::new(String::new());
    let original_title = RwSignal::new(String::new());
    let module_code = RwSignal::new(String::new());
    
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
        let venue_val = if venue.get().trim().is_empty() { None } else { Some(venue.get()) };
        let desc_val = if desc.get().trim().is_empty() { None } else { Some(desc.get()) };
        let recurring_val = if recurring.get() == "No repeat" { None } else { Some(recurring.get()) };
        let current_class_id = class_id.get();
        let current_title = title.get();
        let current_date = date.get();
        let nav = navigate.clone();
        let mod_code = module_code.get();
        let original_rec = original_recurring.get();
        let orig_title = original_title.get();
        let count_val = if recurring.get() != "No repeat" {
            recurrence_count.get().parse::<i32>().ok()
        } else { None };

        spawn_local(async move {
            // If recurrence pattern changed, rewrite the series; otherwise update just this class
            let changed_recurrence = recurring_val != original_rec;
            let should_rewwrite = changed_recurrence || (recurring_val.is_some() && count_val.is_some());
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
                    recurring_val.clone(),
                    count_val,
                ).await
            } else {
                update_class_fn(
                    current_class_id,
                    current_title,
                    desc_val,
                    current_date,
                    time_str,
                    venue_val,
                    recurring_val,
                ).await
            };

            match resp {
                Ok(response) => {
                    message.set(response.message);
                    success.set(response.success);
                    if response.success {
                        set_timeout(
                            move || {
                                nav(&format!("/classes?module={}", mod_code), Default::default());
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
                <A href=move || format!("/classes?module={}", module_code.get()) attr:class="link">"←"</A>
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
                                    </aside>
                                </div>

                                <Show when=move || !message.get().is_empty()>
                                    <p class=move || if success.get() { "success center" } else { "error center" } style="margin-top:16px;">
                                        {message}
                                    </p>
                                </Show>

                                <div class="actions-row">
                                    <button class="btn btn-accent" on:click=on_submit>"✓ Save Class"</button>
                                    <button class="btn btn-primary">"⚡ Start Session"</button>
                                    <A href=move || format!("/classes?module={}", module_code.get()) attr:class="btn btn-outline">"✕ Cancel"</A>
                                </div>
                            </div>
                        }.into_any()
                    })
                }}
            </Suspense>
        </section>
    }
}
