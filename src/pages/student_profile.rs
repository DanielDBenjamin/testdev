use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::user_context::{clear_current_user, get_current_user};

#[component]
pub fn StudentProfilePage() -> impl IntoView {
    let navigate = use_navigate();
    let current_user = get_current_user();

    let navigate_back = navigate.clone();
    let go_back = move |_| {
        navigate_back("/student/home", Default::default());
    };
    let value = navigate.clone();
    let go_to_edit = move |_| {
        value("/student/profile/edit", Default::default());
    };

    // Handle sign out
    let navigate_logout = navigate.clone();
    let handle_sign_out = move |_| {
        // Clear the current user from context and localStorage
        clear_current_user();
        // Redirect to login page
        navigate_logout("/", Default::default());
    };

    // Get user info from context
    let user_name = move || {
        current_user.get()
            .map(|u| format!("{} {}", u.name, u.surname))
            .unwrap_or_else(|| "Student".to_string())
    };

    let user_email = move || {
        current_user.get()
            .map(|u| u.email_address.clone())
            .unwrap_or_else(|| "student@university.edu".to_string())
    };

    let user_id = move || {
        current_user.get()
            .map(|u| format!("STU-{:06}", u.user_id))
            .unwrap_or_else(|| "STU-000000".to_string())
    };

    view! {
        <div class="student-profile-container">
            {/* Header */}
            <header class="student-profile-header">
                <button class="student-back-btn" on:click=go_back>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M19 12H5M12 19l-7-7 7-7"/>
                    </svg>
                </button>
                <h1 class="student-profile-title">"Profile"</h1>
                <button class="student-settings-btn" on:click=go_to_edit>
                    <svg
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                    </svg>
                </button>
            </header>

            {/* Scrollable Content */}
            <div class="student-profile-content">
                {/* Profile Avatar Section */}
                <div class="student-profile-avatar-section">
                    <div class="student-profile-avatar-container">
                        <img src="https://mockmind-api.uifaces.co/content/human/125.jpg" alt="Profile Avatar" class="student-profile-avatar-img" />
                        <button class="student-avatar-edit-btn">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="white" stroke="currentColor" stroke-width="2">
                                <path d="M12 5v14m-7-7h14"></path>
                            </svg>
                        </button>
                    </div>
                    <h2 class="student-profile-name">{user_name}</h2>
                    <p class="student-profile-subtitle">"Computer Science Student"</p>
                </div>

                {/* Personal Information */}
                <section class="student-profile-section">
                    <h3 class="student-section-title">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                            <circle cx="12" cy="7" r="4"></circle>
                        </svg>
                        "Personal Information"
                    </h3>

                    <div class="student-info-card">
                        <div class="student-info-icon student-info-icon-blue">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="4" width="18" height="16" rx="2"></rect>
                                <path d="M7 8h10M7 12h6"></path>
                            </svg>
                        </div>
                        <div class="student-info-text">
                            <div class="student-info-label">"Student ID"</div>
                            <div class="student-info-value">{user_id}</div>
                        </div>
                        <button class="student-info-action-btn">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                            </svg>
                        </button>
                    </div>

                    <div class="student-info-card">
                        <div class="student-info-icon student-info-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                <polyline points="22,6 12,13 2,6"></polyline>
                            </svg>
                        </div>
                        <div class="student-info-text">
                            <div class="student-info-label">"Email Address"</div>
                            <div class="student-info-value">{user_email}</div>
                        </div>
                        <button class="student-info-action-btn">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                                <polyline points="15 3 21 3 21 9"></polyline>
                                <line x1="10" y1="14" x2="21" y2="3"></line>
                            </svg>
                        </button>
                    </div>

                    <div class="student-info-card">
                        <div class="student-info-icon student-info-icon-cyan">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 10v6M2 10l10-5 10 5-10 5z"></path>
                                <path d="M6 12v5c3 3 9 3 12 0v-5"></path>
                            </svg>
                        </div>
                        <div class="student-info-text">
                            <div class="student-info-label">"University"</div>
                            <div class="student-info-value">"Tech University"</div>
                        </div>
                    </div>
                </section>

                {/* Account Settings */}
                <section class="student-profile-section">
                    <h3 class="student-section-title">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="3"></circle>
                            <path d="M12 1v6m0 6v6"></path>
                        </svg>
                        "Account Settings"
                    </h3>

                    <button class="student-settings-item">
                        <div class="student-settings-icon student-settings-icon-red">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 1v6"></path>
                                <path d="M12 17v6"></path>
                                <circle cx="12" cy="12" r="3"></circle>
                            </svg>
                        </div>
                        <div class="student-settings-text">
                            <div class="student-settings-label">"Reset Password"</div>
                            <div class="student-settings-desc">"Change your account password"</div>
                        </div>
                        <svg class="student-settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>

                    <button class="student-settings-item">
                        <div class="student-settings-icon student-settings-icon-blue">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                                <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                            </svg>
                        </div>
                        <div class="student-settings-text">
                            <div class="student-settings-label">"Notification Preferences"</div>
                            <div class="student-settings-desc">"Manage your notification settings"</div>
                        </div>
                        <svg class="student-settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>
                </section>

                {/* Quick Actions */}
                <section class="student-profile-section">
                    <h3 class="student-section-title">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
                        </svg>
                        "Quick Actions"
                    </h3>

                    <div class="student-quick-actions-grid">
                        <button class="student-quick-action-btn student-quick-action-purple">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                                <polyline points="7 10 12 15 17 10"></polyline>
                                <line x1="12" y1="15" x2="12" y2="3"></line>
                            </svg>
                            <span class="student-quick-action-label">"Download ID"</span>
                        </button>

                        <button class="student-quick-action-btn student-quick-action-green">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 2L11 13"></path>
                                <path d="M22 2l-7 20-4-9-9-4 20-7z"></path>
                            </svg>
                            <span class="student-quick-action-label">"Share Profile"</span>
                        </button>

                        <button class="student-quick-action-btn student-quick-action-orange">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                                <polyline points="14 2 14 8 20 8"></polyline>
                            </svg>
                            <span class="student-quick-action-label">"Import Timetable"</span>
                        </button>

                        <button class="student-quick-action-btn student-quick-action-gray">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M12 16v-4"></path>
                                <path d="M12 8h.01"></path>
                            </svg>
                            <span class="student-quick-action-label">"Help"</span>
                        </button>
                    </div>
                </section>

                {/* Other Options */}
                <section class="student-profile-section">
                    <button class="student-settings-item">
                        <div class="student-settings-icon student-settings-icon-gray">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
                            </svg>
                        </div>
                        <div class="student-settings-text">
                            <div class="student-settings-label">"Privacy Settings"</div>
                        </div>
                        <svg class="student-settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>

                    <button class="student-settings-item">
                        <div class="student-settings-icon student-settings-icon-gray">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
                                <line x1="12" y1="17" x2="12.01" y2="17"></line>
                            </svg>
                        </div>
                        <div class="student-settings-text">
                            <div class="student-settings-label">"Support"</div>
                        </div>
                        <svg class="student-settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>

                    <button class="student-settings-item student-settings-item-danger" on:click=handle_sign_out>
                        <div class="student-settings-icon student-settings-icon-red">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
                                <polyline points="16 17 21 12 16 7"></polyline>
                                <line x1="21" y1="12" x2="9" y2="12"></line>
                            </svg>
                        </div>
                        <div class="student-settings-text">
                            <div class="student-settings-label">"Sign Out"</div>
                        </div>
                        <svg class="student-settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>
                </section>
            </div>
        </div>
    }
}