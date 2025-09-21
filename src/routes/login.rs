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

    let login_action = Action::new(|(e, p): &(String, String)| {
        let e = e.clone();
        let p = p.clone();
        async move { login(e, p).await.ok() }
    });

    view! {
        <div class="auth-layout">
            <div class="auth-card">
                <h1 class="page-title">"Sign in"</h1>
                <p class="page-subtitle">"Access your Clock-It dashboard"</p>

                <div class="form" style="margin-top:12px;">
                    <label class="label">"Email"</label>
                    <input class="input" type="email" placeholder="name@example.com" bind:value=email />

                    <label class="label">"Password"</label>
                    <input class="input" type="password" placeholder="••••••••" bind:value=password />

                    <button
                        class="btn btn-primary"
                        on:click=move |_| { login_action.dispatch((email.get(), password.get())); }
                        disabled=move || login_action.pending().get()
                    >
                        {move || if login_action.pending().get() { "Signing in...".into_view() } else { "Sign in".into_view() }}
                    </button>

                    <p class="muted" style="margin-top:8px;">
                        "No account? "
                        <A href="/register" attr:class="link">"Create one"</A>
                    </p>

                    <Show when=move || login_action.value().get().flatten().unwrap_or(false)>
                        <p class="success">"Signed in successfully (stub)."</p>
                    </Show>
                </div>
            </div>
        </div>
    }
}

