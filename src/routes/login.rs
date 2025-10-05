use crate::routes::auth_functions::{login_user, reset_password_fn};
use crate::types::{LoginData, ResetPasswordData};
use crate::user_context::set_current_user;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Login() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let _role = RwSignal::new("Lecturer".to_string());
    let message = RwSignal::new(String::new());
    let success = RwSignal::new(false);

    let show_reset = RwSignal::new(false);
    let reset_email = RwSignal::new(String::new());
    let reset_new_password = RwSignal::new(String::new());
    let reset_confirm_password = RwSignal::new(String::new());
    let reset_message = RwSignal::new(String::new());
    let reset_success = RwSignal::new(false);

    let navigate = use_navigate();

    let login_action = Action::new(|data: &LoginData| {
        let data = data.clone();
        async move { login_user(data).await }
    });

    let reset_action = Action::new(|data: &ResetPasswordData| {
        let data = data.clone();
        async move { reset_password_fn(data).await }
    });

    let on_submit = move |_| {
        message.set(String::new());
        success.set(false);

        let data = LoginData {
            email: email.get(),
            password: password.get(),
        };

        login_action.dispatch(data);
    };

    let on_reset_submit = move |_| {
        reset_message.set(String::new());
        reset_success.set(false);

        let data = ResetPasswordData {
            email: reset_email.get(),
            new_password: reset_new_password.get(),
            confirm_password: reset_confirm_password.get(),
        };

        reset_action.dispatch(data);
    };

    // Handle login response
    Effect::new(move |_| {
        if let Some(result) = login_action.value().get() {
            match result {
                Ok(auth_response) => {
                    message.set(auth_response.message);
                    success.set(auth_response.success);

                    if auth_response.success {
                        // Store the logged-in user
                        if let Some(user) = auth_response.user {
                            // Redirect based on role
                            let redirect_path = match user.role.as_str() {
                                "student" => "/student/home",
                                "lecturer" | "tutor" => "/home",
                                _ => "/home", // Default fallback
                            };

                            set_current_user(user);
                            navigate(redirect_path, Default::default());
                        }
                    }
                }
                Err(e) => {
                    message.set(format!("Error: {}", e));
                    success.set(false);
                }
            }
        }
    });

    Effect::new(move |_| {
        if let Some(result) = reset_action.value().get() {
            match result {
                Ok(response) => {
                    reset_message.set(response.message.clone());
                    reset_success.set(response.success);
                }
                Err(e) => {
                    reset_message.set(format!("Error: {}", e));
                    reset_success.set(false);
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
                        on:click=on_submit
                        disabled=move || login_action.pending().get()
                    >
                        {move || if login_action.pending().get() {
                            "Signing in...".into_view()
                        } else {
                            "Sign In".into_view()
                        }}
                    </button>

                    <p class="center" style="margin:10px 0 0;">
                        <button class="link-button accent" on:click=move |ev: leptos::ev::MouseEvent| {
                            ev.prevent_default();
                            let next = !show_reset.get();
                            show_reset.set(next);
                            if next { reset_email.set(email.get()); }
                            reset_message.set(String::new());
                            reset_success.set(false);
                        }>"Forgot password?"</button>
                    </p>
                    <Show when=move || show_reset.get()>
                        <div class="reset-inline">
                            <label class="label">"Email"</label>
                            <input class="input" type="email" placeholder="jane.gerber@university.edu" bind:value=reset_email />

                            <label class="label">"New Password"</label>
                            <input class="input" type="password" placeholder="••••••••" bind:value=reset_new_password />

                            <label class="label">"Confirm Password"</label>
                            <input class="input" type="password" placeholder="••••••••" bind:value=reset_confirm_password />

                            <button class="btn btn-outline btn-block" on:click=on_reset_submit disabled=move || reset_action.pending().get()>
                                {move || if reset_action.pending().get() { "Updating..." } else { "Reset Password" }}
                            </button>

                            <Show when=move || !reset_message.get().is_empty()>
                                <p class=move || if reset_success.get() { "success center" } else { "error center" }>
                                    {reset_message}
                                </p>
                            </Show>
                        </div>
                    </Show>

                    <p class="muted center" style="margin:6px 0 0;">
                        "Don't have an account? "
                        <A href="/register" attr:class="text-link accent">"Create account"</A>
                    </p>

                    // Show messages
                    <Show when=move || !message.get().is_empty()>
                        <p class=move || if success.get() { "success center" } else { "error center" }>
                            {message}
                        </p>
                    </Show>
                </div>
            </div>
        </div>
    }
}
