use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use crate::routes::auth_functions::login_user;
use crate::types::LoginData;
use crate::user_context::set_current_user;

#[component]
pub fn Login() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let _role = RwSignal::new("Lecturer".to_string());
    let message = RwSignal::new(String::new());
    let success = RwSignal::new(false);

    let navigate = use_navigate();

    let login_action = Action::new(|data: &LoginData| {
        let data = data.clone();
        async move { login_user(data).await }
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
                        <A href="#" attr:class="text-link accent">"Forgot password?"</A>
                    </p>
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
