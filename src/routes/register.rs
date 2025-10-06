use leptos::prelude::*;
use leptos_router::components::A;

// Import the server functions and types
use crate::routes::auth_functions::register_user;
use crate::types::RegisterData;

#[component]
pub fn Register() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let surname = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm = RwSignal::new(String::new());
    let role = RwSignal::new("Lecturer".to_string());
    let message = RwSignal::new(String::new());
    let success = RwSignal::new(false);

    let register_action = Action::new(|data: &RegisterData| {
        let data = data.clone();
        async move { register_user(data).await }
    });

    let on_submit = move |_| {
        message.set(String::new());
        success.set(false);

        let data = RegisterData {
            name: name.get(),
            surname: surname.get(),
            email: email.get(),
            password: password.get(),
            confirm_password: confirm.get(),
            role: role.get().to_lowercase(),
        };

        register_action.dispatch(data);
    };

    // Handle response
    Effect::new(move |_| {
        if let Some(result) = register_action.value().get() {
            match result {
                Ok(auth_response) => {
                    message.set(auth_response.message);
                    success.set(auth_response.success);

                    if auth_response.success {
                        // Clear form on success
                        name.set(String::new());
                        surname.set(String::new());
                        email.set(String::new());
                        password.set(String::new());
                        confirm.set(String::new());
                    }
                }
                Err(e) => {
                    message.set(format!("Error: {}", e));
                    success.set(false);
                }
            }
        }
    });

    view! {
        <div class="auth-layout">
            <div class="auth-card">
                <div class="auth-header">
                    <div class="logo-container">
                        <img src="/logo.png" srcset="/logo.png 1x" alt="Clock It" class="brand-logo-img" width="160" height="60" />
                    </div>
                    <p class="tagline">"Track your time, manage your life"</p>
                </div>
                <div class="segmented">
                    <button
                        class=move || if role.get() == "Lecturer" { "seg-btn active" } else { "seg-btn" }
                        on:click=move |_| role.set("Lecturer".to_string())
                    >"Lecturer"</button>
                    <button
                        class=move || if role.get() == "Tutor" { "seg-btn active" } else { "seg-btn" }
                        on:click=move |_| role.set("Tutor".to_string())
                    >"Tutor"</button>
                    <button
                        class=move || if role.get() == "Student" { "seg-btn active" } else { "seg-btn" }
                        on:click=move |_| role.set("Student".to_string())
                    >"Student"</button>
                </div>

                <div class="form">
                    <div class="row-2">
                        <div>
                            <label class="label">"Name"</label>
                            <input class="input" type="text" placeholder="Name" bind:value=name />
                        </div>
                        <div>
                            <label class="label">"Surname"</label>
                            <input class="input" type="text" placeholder="Surname" bind:value=surname />
                        </div>
                    </div>

                    <label class="label">"Email"</label>
                    <div class="input-group">
                        <input class="input" type="email" placeholder="Enter your email" bind:value=email />
                        <span class="input-icon" aria-hidden="true">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 8l8 6 8-6"/><rect x="4" y="4" width="16" height="16" rx="2"/></svg>
                        </span>
                    </div>

                    <label class="label">"Password"</label>
                    <div class="input-group">
                        <input class="input" type="password" placeholder="Enter your password" bind:value=password />
                        <span class="input-icon" aria-hidden="true">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7-11-7-11-7z"/><circle cx="12" cy="12" r="3"/></svg>
                        </span>
                    </div>

                    <label class="label">"Confirm password"</label>
                    <input class="input" type="password" placeholder="Re-enter your password" bind:value=confirm />

                    <button
                        class="btn btn-accent btn-block"
                        on:click=on_submit
                        disabled=move || register_action.pending().get()
                    >
                        {move || if register_action.pending().get() {
                            "Creating Account...".into_view()
                        } else {
                            "Create Account".into_view()
                        }}
                    </button>

                    <p class="small muted center" style="margin-top:8px;">
                        "By creating an account, you agree to our "
                        <A href="#" attr:class="text-link accent">"Terms of Service"</A>
                        " and "
                        <A href="#" attr:class="text-link accent">"Privacy Policy"</A>
                    </p>

                    // Show messages
                    <Show when=move || !message.get().is_empty()>
                        <p class=move || if success.get() { "success center" } else { "error center" }>
                            {message}
                        </p>
                    </Show>

                    <p class="muted center" style="margin-top:8px;">
                        "Already have an account? "
                        <A href="/" attr:class="text-link accent">"Sign in"</A>
                    </p>
                </div>
            </div>
        </div>
    }
}
