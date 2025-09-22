use leptos::prelude::*;
use leptos::server_fn::error::ServerFnError;
use leptos_router::components::A;

#[server(Register, "/api")]
async fn register(name: String, email: String, password: String) -> Result<bool, ServerFnError> {
    if name.trim().is_empty() || email.trim().is_empty() || password.trim().is_empty() {
        return Err(ServerFnError::new("All fields are required"));
    }
    // TODO: create user in DB, hash password, handle uniqueness, etc.
    Ok(true)
}

#[component]
pub fn Register() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let surname = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm = RwSignal::new(String::new());
    let client_error = RwSignal::new(String::new());
    let role = RwSignal::new("Lecturer");

    let register_action = Action::new(|(n, e, p): &(String, String, String)| {
        let (n, e, p) = (n.clone(), e.clone(), p.clone());
        async move { register(n, e, p).await.ok() }
    });

    let on_submit = move |_| {
        client_error.set(String::new());
        if password.get() != confirm.get() {
            client_error.set("Passwords do not match".into());
            return;
        }
        register_action.dispatch((name.get(), email.get(), password.get()));
    };

    view! {
        <div class="auth-layout">
            <div class="auth-card">
                <div class="auth-header">
                    <div class="brand-logo">"clock it"</div>
                    <p class="tagline">"Track your time, manage your life"</p>
                </div>
                <div class="segmented">
                    <button
                        class=move || if role.get() == "Lecturer" { "seg-btn active" } else { "seg-btn" }
                        on:click=move |_| role.set("Lecturer")
                    >"Lecturer"</button>
                    <button
                        class=move || if role.get() == "Tutor" { "seg-btn active" } else { "seg-btn" }
                        on:click=move |_| role.set("Tutor")
                    >"Tutor"</button>
                    <button
                        class=move || if role.get() == "Student" { "seg-btn active" } else { "seg-btn" }
                        on:click=move |_| role.set("Student")
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
                        {move || if register_action.pending().get() { "Creating...".into_view() } else { "Create Account".into_view() }}
                    </button>

                    <p class="small muted center" style="margin-top:8px;">
                        "By creating an account, you agree to our "
                        <A href="#" attr:class="link accent">"Terms of Service"</A>
                        " and "
                        <A href="#" attr:class="link accent">"Privacy Policy"</A>
                    </p>

                    <Show when=move || !client_error.get().is_empty()>
                        <p class="error center">{client_error}</p>
                    </Show>

                    <Show when=move || register_action.value().get().flatten().unwrap_or(false)>
                        <p class="success center">"Account created (stub). You can now sign in."</p>
                    </Show>

                    <p class="muted center" style="margin-top:8px;">
                        "Already have an account? "
                        <A href="/" attr:class="link accent">"Sign in"</A>
                    </p>
                </div>
            </div>
        </div>
    }
}
