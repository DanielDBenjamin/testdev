use crate::types::UserProfile;
use leptos::prelude::*;

// Simple global state using RwSignal
pub static CURRENT_USER: std::sync::OnceLock<RwSignal<Option<UserProfile>>> =
    std::sync::OnceLock::new();

pub fn init_user_context() {
    let signal = RwSignal::new(None);
    CURRENT_USER.set(signal).unwrap_or(());

    // Load user from storage after a small delay to ensure DOM is ready
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::task::spawn_local;
        spawn_local(async {
            // Small delay to ensure everything is initialized
            gloo_timers::future::TimeoutFuture::new(50).await;

            if let Some(stored_user) = load_user_from_storage() {
                web_sys::console::log_1(
                    &format!(
                        "Loading user from storage: {} {}",
                        stored_user.name, stored_user.surname
                    )
                    .into(),
                );
                if let Some(signal) = CURRENT_USER.get() {
                    signal.set(Some(stored_user));
                }
            }
        });
    }
}

pub fn get_user_role() -> Option<String> {
    if let Some(signal) = CURRENT_USER.get() {
        signal.get().map(|user| user.role.clone())
    } else {
        None
    }
}

pub fn is_student() -> bool {
    get_user_role()
        .map(|role| role == "student")
        .unwrap_or(false)
}

pub fn is_lecturer_or_tutor() -> bool {
    get_user_role()
        .map(|role| role == "lecturer" || role == "tutor")
        .unwrap_or(false)
}

pub fn set_current_user(user: UserProfile) {
    web_sys::console::log_1(&format!("Setting user: {} {}", user.name, user.surname).into());

    if let Some(signal) = CURRENT_USER.get() {
        signal.set(Some(user.clone()));

        // Save to localStorage (client-side only)
        #[cfg(target_arch = "wasm32")]
        {
            save_user_to_storage(&user);
            web_sys::console::log_1(&"User saved to localStorage".into());
        }
    }
}

pub fn get_current_user() -> ReadSignal<Option<UserProfile>> {
    CURRENT_USER.get_or_init(|| RwSignal::new(None)).read_only()
}

pub fn clear_current_user() {
    web_sys::console::log_1(&"Clearing current user".into());

    if let Some(signal) = CURRENT_USER.get() {
        signal.set(None);

        // Remove from localStorage (client-side only)
        #[cfg(target_arch = "wasm32")]
        {
            remove_user_from_storage();
            web_sys::console::log_1(&"User removed from localStorage".into());
        }
    }
}

pub fn get_user_name() -> String {
    if let Some(signal) = CURRENT_USER.get() {
        match signal.get() {
            Some(user) => format!("{} {}", user.name, user.surname),
            None => "User".to_string(),
        }
    } else {
        "User".to_string()
    }
}

// localStorage helper functions (client-side only)
#[cfg(target_arch = "wasm32")]
fn save_user_to_storage(user: &UserProfile) {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(serialized) = serde_json::to_string(user) {
                match storage.set_item("clock_it_user", &serialized) {
                    Ok(_) => web_sys::console::log_1(&"Successfully saved to localStorage".into()),
                    Err(e) => web_sys::console::log_1(
                        &format!("Error saving to localStorage: {:?}", e).into(),
                    ),
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn load_user_from_storage() -> Option<UserProfile> {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(stored)) = storage.get_item("clock_it_user") {
                web_sys::console::log_1(&format!("Found stored data: {}", stored).into());
                if let Ok(user) = serde_json::from_str::<UserProfile>(&stored) {
                    return Some(user);
                } else {
                    web_sys::console::log_1(&"Error parsing stored user data".into());
                }
            } else {
                web_sys::console::log_1(&"No stored user data found".into());
            }
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
fn remove_user_from_storage() {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("clock_it_user");
        }
    }
}
