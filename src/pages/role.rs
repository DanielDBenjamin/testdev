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
        <div class="student-mobile-container">
            {/* Header with logo and tagline */}
            <div class="student-header-section">
                <div class="student-logo-container">
                    <img src="/logo.png" srcset="/logo.png 1x" alt="Clock It" class="student-brand-logo-img" width="160" height="60" />
                </div>
                <p class="student-tagline">"Track your time, manage your life"</p>
            </div>

            {/* Role selection card */}
            <div class="student-role-card">
                <h2 class="student-role-title">"Choose your role:"</h2>
                <div class="student-role-buttons">
                    <button
                        class="student-role-button"
                        on:click=handle_student_click
                    >
                        "Student"
                    </button>
                    <button
                        class="student-role-button"
                        on:click=handle_lecturer_click
                    >
                        "Lecturer"
                    </button>
                    <button
                        class="student-role-button"
                        on:click=handle_tutor_click
                    >
                        "Tutor"
                    </button>
                </div>
            </div>
        </div>
    }
}
