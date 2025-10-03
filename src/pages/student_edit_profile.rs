use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn StudentEditProfilePage() -> impl IntoView {
    let navigate = use_navigate();

    let navigate_back = navigate.clone();
    let go_back = move |_| {
        navigate_back("/student/profile", Default::default());
    };

    let navigate_confirm = navigate.clone();
    let handle_confirm = move |_| {
        // Here you would typically save the form data
        // For now, we'll just navigate back to the profile page
        navigate_confirm("/student/profile", Default::default());
    };

    view! {
        <div class="student-edit-profile-container">
            {/* Header */}
            <header class="student-edit-profile-header">
                <button class="student-back-btn" on:click=go_back>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M19 12H5M12 19l-7-7 7-7"/>
                    </svg>
                </button>
                <h1 class="student-edit-profile-title">"Profile"</h1>
                <div class="student-header-spacer"></div>
            </header>

            {/* Scrollable Content */}
            <div class="student-edit-profile-content">
                {/* Profile Avatar Section */}
                <div class="student-edit-profile-avatar-section">
                    <div class="student-edit-profile-avatar-container">
                        <img src="https://mockmind-api.uifaces.co/content/human/125.jpg" alt="Profile Avatar" class="student-edit-profile-avatar-img" />
                        <button class="student-avatar-edit-btn">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="white">
                                <path d="M12 4c4.41 0 8 3.59 8 8s-3.59 8-8 8-8-3.59-8-8 3.59-8 8-8m0-2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 4h-2v4H7v2h4v4h2v-4h4v-2h-4V6z" fill="currentColor"/>
                            </svg>
                        </button>
                    </div>
                    <h2 class="student-edit-profile-name">"Sarah Johnson"</h2>
                    <p class="student-edit-profile-subtitle">"Computer Science Student"</p>
                </div>

                {/* Personal Information Form */}
                <section class="student-edit-profile-section">
                    <h3 class="student-edit-section-title">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="#14b8a6" stroke="currentColor" stroke-width="2">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                            <circle cx="12" cy="7" r="4"></circle>
                        </svg>
                        "Personal Information"
                    </h3>

                    {/* Student ID Input */}
                    <div class="student-edit-input-group">
                        <div class="student-edit-input-icon student-edit-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="4" width="18" height="16" rx="2"></rect>
                                <path d="M7 8h10M7 12h6"></path>
                            </svg>
                        </div>
                        <input
                            type="text"
                            class="student-edit-input"
                            value="STU-2024-001234"
                            placeholder="Student ID"
                            readonly
                        />
                    </div>

                    {/* Email Input */}
                    <div class="student-edit-input-group">
                        <div class="student-edit-input-icon student-edit-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                <polyline points="22,6 12,13 2,6"></polyline>
                            </svg>
                        </div>
                        <input
                            type="email"
                            class="student-edit-input"
                            value="sarah.johnson@university.edu"
                            placeholder="Email Address"
                        />
                    </div>

                    {/* Password Input */}
                    <div class="student-edit-input-group">
                        <div class="student-edit-input-icon student-edit-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
                                <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
                            </svg>
                        </div>
                        <input
                            type="password"
                            class="student-edit-input"
                            value="**************"
                            placeholder="Password"
                        />
                        <button class="student-edit-input-action">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                                <circle cx="12" cy="12" r="3"></circle>
                            </svg>
                        </button>
                    </div>

                    {/* University Input */}
                    <div class="student-edit-input-group">
                        <div class="student-edit-input-icon student-edit-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 10v6M2 10l10-5 10 5-10 5z"></path>
                                <path d="M6 12v5c3 3 9 3 12 0v-5"></path>
                            </svg>
                        </div>
                        <input
                            type="text"
                            class="student-edit-input"
                            value="Tech University"
                            placeholder="University"
                        />
                    </div>

                    {/* First Name Input */}
                    <div class="student-edit-input-group">
                        <div class="student-edit-input-icon student-edit-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                                <circle cx="12" cy="7" r="4"></circle>
                            </svg>
                        </div>
                        <input
                            type="text"
                            class="student-edit-input"
                            value="Sarah"
                            placeholder="First Name"
                        />
                    </div>

                    {/* Last Name Input */}
                    <div class="student-edit-input-group">
                        <div class="student-edit-input-icon student-edit-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                                <circle cx="12" cy="7" r="4"></circle>
                            </svg>
                        </div>
                        <input
                            type="text"
                            class="student-edit-input"
                            value="Johnson"
                            placeholder="Last Name"
                        />
                    </div>
                </section>

                {/* Confirm Button */}
                <button class="student-confirm-btn" on:click=handle_confirm>
                    "Confirm"
                </button>
            </div>
        </div>
    }
}
