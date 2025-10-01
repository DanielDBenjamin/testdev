use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn StudentProfilePage() -> impl IntoView {
    let navigate = use_navigate();

    let go_back = move |_| {
        navigate("/student/home", Default::default());
    };

    view! {
        <div class="profile-container">
            {/* Header */}
            <header class="profile-header">
                <button class="back-btn" on:click=go_back>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M19 12H5M12 19l-7-7 7-7"/>
                    </svg>
                </button>
                <h1 class="profile-title">"Profile"</h1>
                <button class="settings-btn">
                    <svg 
                        width="24" 
                        height="24" 
                        viewBox="0 0 24 24" 
                        fill="none" 
                        stroke="currentColor" 
                        stroke-width="1.5"
                    >
                        <path 
                            stroke-linecap="round" 
                            stroke-linejoin="round" 
                            d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" 
                        />
                        <path 
                            stroke-linecap="round" 
                            stroke-linejoin="round" 
                            d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" 
                        />
                    </svg>
                </button>
            </header>

            {/* Scrollable Content */}
            <div class="profile-content">
                {/* Profile Avatar Section */}
                <div class="profile-avatar-section">
                    <div class="profile-avatar-container">
                        <img src="https://mockmind-api.uifaces.co/content/human/125.jpg" alt="Profile Avatar" class="profile-avatar-img" />
                        <button class="avatar-edit-btn">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="white" stroke="currentColor" stroke-width="2">
                                <path d="M12 5v14m-7-7h14"></path>
                            </svg>
                        </button>
                    </div>
                    <h2 class="profile-name">"Sarah Johnson"</h2>
                    <p class="profile-subtitle">"Computer Science Student"</p>
                </div>

                {/* Personal Information */}
                <section class="profile-section">
                    <h3 class="section-title">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                            <circle cx="12" cy="7" r="4"></circle>
                        </svg>
                        "Personal Information"
                    </h3>

                    <div class="info-card">
                        <div class="info-icon info-icon-blue">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="4" width="18" height="16" rx="2"></rect>
                                <path d="M7 8h10M7 12h6"></path>
                            </svg>
                        </div>
                        <div class="info-text">
                            <div class="info-label">"Student ID"</div>
                            <div class="info-value">"STU-2024-001234"</div>
                        </div>
                        <button class="info-action-btn">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                            </svg>
                        </button>
                    </div>

                    <div class="info-card">
                        <div class="info-icon info-icon-teal">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                <polyline points="22,6 12,13 2,6"></polyline>
                            </svg>
                        </div>
                        <div class="info-text">
                            <div class="info-label">"Email Address"</div>
                            <div class="info-value">"sarah.johnson@university.edu"</div>
                        </div>
                        <button class="info-action-btn">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                                <polyline points="15 3 21 3 21 9"></polyline>
                                <line x1="10" y1="14" x2="21" y2="3"></line>
                            </svg>
                        </button>
                    </div>

                    <div class="info-card">
                        <div class="info-icon info-icon-cyan">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 10v6M2 10l10-5 10 5-10 5z"></path>
                                <path d="M6 12v5c3 3 9 3 12 0v-5"></path>
                            </svg>
                        </div>
                        <div class="info-text">
                            <div class="info-label">"University"</div>
                            <div class="info-value">"Tech University"</div>
                        </div>
                    </div>
                </section>

                {/* Account Settings */}
                <section class="profile-section">
                    <h3 class="section-title">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="3"></circle>
                            <path d="M12 1v6m0 6v6"></path>
                        </svg>
                        "Account Settings"
                    </h3>

                    <button class="settings-item">
                        <div class="settings-icon settings-icon-red">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 1v6"></path>
                                <path d="M12 17v6"></path>
                                <circle cx="12" cy="12" r="3"></circle>
                            </svg>
                        </div>
                        <div class="settings-text">
                            <div class="settings-label">"Reset Password"</div>
                            <div class="settings-desc">"Change your account password"</div>
                        </div>
                        <svg class="settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>

                    <button class="settings-item">
                        <div class="settings-icon settings-icon-blue">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                                <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                            </svg>
                        </div>
                        <div class="settings-text">
                            <div class="settings-label">"Notification Preferences"</div>
                            <div class="settings-desc">"Manage your notification settings"</div>
                        </div>
                        <svg class="settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>
                </section>

                {/* Quick Actions */}
                <section class="profile-section">
                    <h3 class="section-title">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
                        </svg>
                        "Quick Actions"
                    </h3>

                    <div class="quick-actions-grid">
                        <button class="quick-action-btn quick-action-purple">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                                <polyline points="7 10 12 15 17 10"></polyline>
                                <line x1="12" y1="15" x2="12" y2="3"></line>
                            </svg>
                            <span class="quick-action-label">"Download ID"</span>
                        </button>

                        <button class="quick-action-btn quick-action-green">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 2L11 13"></path>
                                <path d="M22 2l-7 20-4-9-9-4 20-7z"></path>
                            </svg>
                            <span class="quick-action-label">"Share Profile"</span>
                        </button>

                        <button class="quick-action-btn quick-action-orange">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                                <polyline points="14 2 14 8 20 8"></polyline>
                            </svg>
                            <span class="quick-action-label">"Import Timetable"</span>
                        </button>

                        <button class="quick-action-btn quick-action-gray">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M12 16v-4"></path>
                                <path d="M12 8h.01"></path>
                            </svg>
                            <span class="quick-action-label">"Help"</span>
                        </button>
                    </div>
                </section>

                {/* Other Options */}
                <section class="profile-section">
                    <button class="settings-item">
                        <div class="settings-icon settings-icon-gray">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
                            </svg>
                        </div>
                        <div class="settings-text">
                            <div class="settings-label">"Privacy Settings"</div>
                        </div>
                        <svg class="settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>

                    <button class="settings-item">
                        <div class="settings-icon settings-icon-gray">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
                                <line x1="12" y1="17" x2="12.01" y2="17"></line>
                            </svg>
                        </div>
                        <div class="settings-text">
                            <div class="settings-label">"Support"</div>
                        </div>
                        <svg class="settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>

                    <button class="settings-item settings-item-danger">
                        <div class="settings-icon settings-icon-red">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
                                <polyline points="16 17 21 12 16 7"></polyline>
                                <line x1="21" y1="12" x2="9" y2="12"></line>
                            </svg>
                        </div>
                        <div class="settings-text">
                            <div class="settings-label">"Sign Out"</div>
                        </div>
                        <svg class="settings-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                    </button>
                </section>
            </div>
        </div>
    }
}
