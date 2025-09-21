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
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm = RwSignal::new(String::new());
    let client_error = RwSignal::new(String::new());

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
                <h1 class="page-title">"Create account"</h1>
                <p class="page-subtitle">"Sign up to start using Clock-It"</p>

                <div class="form" style="margin-top:12px;">
                    <label class="label">"Full name"</label>
                    <input class="input" type="text" placeholder="Jane Doe" bind:value=name />

                    <label class="label">"Email"</label>
                    <input class="input" type="email" placeholder="name@example.com" bind:value=email />

                    <label class="label">"Password"</label>
                    <input class="input" type="password" placeholder="••••••••" bind:value=password />

                    <label class="label">"Confirm password"</label>
                    <input class="input" type="password" placeholder="••••••••" bind:value=confirm />

                    <button
                        class="btn btn-primary"
                        on:click=on_submit
                        disabled=move || register_action.pending().get()
                    >
                        {move || if register_action.pending().get() { "Creating...".into_view() } else { "Create account".into_view() }}
                    </button>

                    <Show when=move || !client_error.get().is_empty()>
                        <p class="error">{client_error}</p>
                    </Show>

                    <Show when=move || register_action.value().get().flatten().unwrap_or(false)>
                        <p class="success">"Account created (stub). You can now sign in."</p>
                    </Show>

                    <p class="muted" style="margin-top:8px;">
                        "Already have an account? "
                        <A href="/login" attr:class="link">"Sign in"</A>
                    </p>
                </div>
            </div>
        </div>
    }
}

