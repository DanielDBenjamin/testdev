use leptos::prelude::*;
use leptos::server_fn::error::ServerFnError;
use leptos_router::components::A;

#[server(Login, "/api")]
async fn login(email: String, password: String) -> Result<bool, ServerFnError> {
    // Placeholder auth check. Replace with real DB/session logic.
    if email.trim().is_empty() || password.trim().is_empty() {
        return Err(ServerFnError::new("Email and password are required"));
    }
    Ok(true)
}

#[component]
pub fn Login() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let role = RwSignal::new("Lecturer");

    let login_action = Action::new(|(e, p): &(String, String)| {
        let e = e.clone();
        let p = p.clone();
        async move { login(e, p).await.ok() }
    });

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
                    <label class="label">"Email"</label>
                    <div class="input-group">
                        <input class="input" type="email" placeholder="jane.gerber@university.edu" bind:value=email />
                        <span class="input-icon" aria-hidden="true">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 4h16v16H4z" opacity="0"></path><path d="M4 8l8 6 8-6"/><rect x="4" y="4" width="16" height="16" rx="2"/></svg>
                        </span>
                    </div>

                    <label class="label">"Password"</label>
                    <div class="input-group">
                        <input class="input" type="password" placeholder="••••••••" bind:value=password />
                        <span class="input-icon" aria-hidden="true">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7-11-7-11-7z"/><circle cx="12" cy="12" r="3"/></svg>
                        </span>
                    </div>

                    <button
                        class="btn btn-accent btn-block"
                        on:click=move |_| { login_action.dispatch((email.get(), password.get())); }
                        disabled=move || login_action.pending().get()
                    >
                        {move || if login_action.pending().get() { "Signing in...".into_view() } else { "Sign In".into_view() }}
                    </button>

                    <p class="center" style="margin:10px 0 0;">
                        <A href="#" attr:class="link accent">"Forgot password?"</A>
                    </p>
                    <p class="muted center" style="margin:6px 0 0;">
                        "Don't have an account? "
                        <A href="/register" attr:class="link accent">"Create account"</A>
                    </p>

                    <Show when=move || login_action.value().get().flatten().unwrap_or(false)>
                        <p class="success center">"Signed in successfully (stub)."</p>
                    </Show>
                </div>
            </div>
        </div>
    }
}
