use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use crate::routes::module_functions::create_module_fn;
use crate::user_context::get_current_user;

#[derive(Clone)]
struct Student { 
    id: String, 
    name: String, 
    email: String 
}

#[component]
pub fn NewModule() -> impl IntoView {
    let current_user = get_current_user();
    let navigate = use_navigate();
    
    let module_code = RwSignal::new(String::new());
    let title = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let success = RwSignal::new(false);
    
    let students = RwSignal::new(vec![
        Student { 
            id: "STU001".to_string(), 
            name: "John Smith".to_string(), 
            email: "john.smith@university.edu".to_string() 
        },
        Student { 
            id: "STU002".to_string(), 
            name: "Sarah Johnson".to_string(), 
            email: "sarah.johnson@university.edu".to_string() 
        },
    ]);

    let create_action = Action::new(move |(code, title_val, desc_val, email): &(String, String, Option<String>, String)| {
        let code = code.clone();
        let title_val = title_val.clone();
        let desc_val = desc_val.clone();
        let email = email.clone();
        async move {
            create_module_fn(code, title_val, desc_val, email).await
        }
    });

    let on_submit = move |_| {
        message.set(String::new());
        success.set(false);
        
        // Validate module code
        let code = module_code.get().trim().to_string();
        if code.is_empty() {
            message.set("Please enter a module code".to_string());
            success.set(false);
            return;
        }
        
        // Validate title
        if title.get().trim().is_empty() {
            message.set("Please enter a module title".to_string());
            success.set(false);
            return;
        }
        
        // Get current user email
        let email = match current_user.get() {
            Some(user) => {
                web_sys::console::log_1(&format!("Creating module for email: {}", user.email_address).into());
                user.email_address
            }
            None => {
                message.set("You must be logged in to create a module".to_string());
                success.set(false);
                return;
            }
        };
        
        let desc_val = if desc.get().trim().is_empty() {
            None
        } else {
            Some(desc.get())
        };
        
        create_action.dispatch((code, title.get(), desc_val, email));
    };

    // Handle create response
    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if let Some(result) = create_action.value().get() {
                match result {
                    Ok(response) => {
                        message.set(response.message.clone());
                        success.set(response.success);
                        
                        if response.success {
                            // Clear form and navigate to home after a brief delay
                            module_code.set(String::new());
                            title.set(String::new());
                            desc.set(String::new());
                            
                            // Navigate to home page after 1 second
                            let nav = navigate.clone();
                            set_timeout(
                                move || {
                                    nav("/home", Default::default());
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
        <section class="new-module">
            <div class="page-header" style="display:flex;align-items:center;gap:8px;">
                <A href="/home" attr:class="link">"← Back"</A>
                <h1 class="page-title">"New Module"</h1>
                <p class="page-subtitle" style="margin-left:8px;">"Create a new module and manage student enrollment"</p>
            </div>

            <div class="form-card">
                <h3 class="heading">"Module Information"</h3>
                
                <label class="label" style="margin-top:6px;">"Module Code "<span style="color:#ef4444;">"*"</span></label>
                <input 
                    class="input" 
                    type="text" 
                    placeholder="e.g., CS112 or MATH201" 
                    bind:value=module_code 
                />
                
                <label class="label" style="margin-top:10px;">"Module Title "<span style="color:#ef4444;">"*"</span></label>
                <input 
                    class="input" 
                    placeholder="e.g., Introduction to Programming" 
                    bind:value=title 
                />

                <label class="label" style="margin-top:10px;">"Description"</label>
                <textarea 
                    class="textarea" 
                    placeholder="Enter module description..." 
                    bind:value=desc
                ></textarea>

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
                                    <td>
                                        <button 
                                            class="btn btn-outline btn-small" 
                                            style="color:#ef4444; border-color:#fecaca;"
                                        >"🗑 Remove"</button>
                                    </td>
                                </tr>
                            }).collect_view()}
                        </tbody>
                    </table>
                </div>

                // Show messages
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
                            "Creating Module...".into_view() 
                        } else { 
                            "Save Module".into_view() 
                        }}
                    </button>
                    <A href="/home" attr:class="btn btn-outline">"Cancel"</A>
                </div>
            </div>
        </section>
    }
}