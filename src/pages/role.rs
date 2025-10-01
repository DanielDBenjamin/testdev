use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn RolePage() -> impl IntoView {
    let navigate = use_navigate();

    let handle_student_click = move |_| {
        leptos::logging::log!("Selected role: Student");
        navigate("/student/login", Default::default());
    };

    let handle_lecturer_click = move |_| {
        leptos::logging::log!("Selected role: Lecturer");
        leptos::logging::log!("Lecturer login not implemented yet");
    };

    let handle_tutor_click = move |_| {
        leptos::logging::log!("Selected role: Tutor");
        leptos::logging::log!("Tutor login not implemented yet");
    };

    view! {
        <div class="mobile-container">
            {/* Header with logo and tagline */}
            <div class="header-section">
                <div class="logo-container">
                    <img src="/logo.png" srcset="/logo.png 1x" alt="Clock It" class="brand-logo-img" width="160" height="60" />
                </div>
                <p class="tagline">"Track your time, manage your life"</p>
            </div>

            {/* Role selection card */}
            <div class="role-card">
                <h2 class="role-title">"Choose your role:"</h2>
                <div class="role-buttons">
                    <button
                        class="role-button"
                        on:click=handle_student_click
                    >
                        "Student"
                    </button>
                    <button
                        class="role-button"
                        on:click=handle_lecturer_click
                    >
                        "Lecturer"
                    </button>
                    <button
                        class="role-button"
                        on:click=handle_tutor_click
                    >
                        "Tutor"
                    </button>
                </div>
            </div>
        </div>
    }
}