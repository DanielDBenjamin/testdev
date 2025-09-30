use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_query_map};
use crate::routes::class_functions::create_class_fn;

#[component]
pub fn NewClass() -> impl IntoView {
    let navigate = use_navigate();
    let query = use_query_map();
    
    let title = RwSignal::new(String::new());
    let venue = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let recurring = RwSignal::new("No repeat".to_string());
    let date = RwSignal::new(String::new());
    let hour = RwSignal::new("10".to_string());
    let minute = RwSignal::new("00".to_string());
    let message = RwSignal::new(String::new());
    let success = RwSignal::new(false);
    
    // Get module code from URL query params
    let module_code = move || {
        query.with(|q| q.get("module").unwrap_or_default())
    };
    
    let create_action = Action::new(move |(module, title_val, venue_val, desc_val, recurring_val, date_val, time_val): &(String, String, Option<String>, Option<String>, Option<String>, String, String)| {
        let module = module.clone();
        let title_val = title_val.clone();
        let venue_val = venue_val.clone();
        let desc_val = desc_val.clone();
        let recurring_val = recurring_val.clone();
        let date_val = date_val.clone();
        let time_val = time_val.clone();
        async move {
            create_class_fn(module, title_val, venue_val, desc_val, recurring_val, date_val, time_val).await
        }
    });

    let on_submit = move |_| {
        message.set(String::new());
        success.set(false);
        
        let current_module = module_code(); // Call the closure
        
        if current_module.is_empty() {
            message.set("No module selected. Please go back and select a module.".to_string());
            return;
        }
        
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
        
        create_action.dispatch((
            current_module, // Use the captured value
            title.get(),
            venue_val,
            desc_val,
            recurring_val,
            date.get(),
            time_str,
        ));
    };

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if let Some(result) = create_action.value().get() {
                match result {
                    Ok(response) => {
                        message.set(response.message.clone());
                        success.set(response.success);
                        
                        if response.success {
                            let nav = navigate.clone();
                            let mod_code = module_code(); // Call the closure
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
            }
        }
    });

    view! {
        <section class="new-class">
            <div class="page-header" style="display:flex;align-items:center;gap:8px;">
                <A href=move || format!("/classes?module={}", module_code()) attr:class="link">"←"</A>
                <h1 class="page-title">"New Class"</h1>
                <Show when=move || !module_code().is_empty()>
                    <p class="page-subtitle" style="margin-left:8px;">
                        {move || format!("for {}", module_code())}
                    </p>
                </Show>
            </div>

            <div class="form-card">
                <div class="form-grid">
                    <div class="form-col">
                        <label class="label">"Title"</label>
                        <input class="input" placeholder="Linked List Fundamentals – Lecture 4" bind:value=title />

                        <label class="label" style="margin-top:10px;">"Venue"</label>
                        <input class="input" placeholder="Room A101" bind:value=venue />

                        <label class="label" style="margin-top:10px;">"Description"</label>
                        <textarea class="textarea" placeholder="Enter a class description" bind:value=desc></textarea>

                        <label class="label" style="margin-top:10px;">"Recurring"</label>
                        <select class="input" bind:value=recurring>
                            <option selected>"No repeat"</option>
                            <option>"Daily"</option>
                            <option>"Weekly"</option>
                            <option>"Monthly"</option>
                        </select>
                    </div>

                    <aside class="form-side">
                        <label class="label">"Date"</label>
                        <input class="input" type="date" bind:value=date />
                        
                        <label class="label" style="margin-top:10px;">"Time"</label>
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
                    <p class=move || if success.get() { "success center" } else { "error center" } style="margin-top:12px;">
                        {message}
                    </p>
                </Show>

                <div class="actions-row">
                    <button 
                        class="btn btn-accent" 
                        on:click=on_submit
                        disabled=move || create_action.pending().get()
                    >
                        {move || if create_action.pending().get() { 
                            "Creating Class...".into_view() 
                        } else { 
                            "✓ Save Class".into_view() 
                        }}
                    </button>
                    <A href=move || format!("/classes?module={}", module_code()) attr:class="btn btn-outline">"× Cancel"</A>
                </div>
            </div>
        </section>
    }
}