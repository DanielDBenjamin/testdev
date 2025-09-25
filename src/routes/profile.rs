use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <section class="profile-page">
            <header class="page-header">
                <div class="page-title-row">
                    <A href="/home" attr:class="link">"← Back"</A>
                    <h1 class="page-title">"Profile & Account Settings"</h1>
                </div>
                <p class="page-subtitle">"Manage your personal information and account preferences"</p>
            </header>

            <section class="profile-card" aria-labelledby="profile-summary">
                <div class="profile-avatar">
                    <img src="https://i.pravatar.cc/160?img=47" alt="Portrait of Dr. Jane Gerber"/>
                    <button class="avatar-edit" type="button" aria-label="Update profile picture">
                        <svg width="16" height="16" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M2 14.5V18h3.5l9.9-9.9-3.5-3.5L2 14.5z"/><path d="M12.4 4l3.6 3.6"/></svg>
                    </button>
                </div>
                <div class="profile-summary">
                    <h2 id="profile-summary" class="profile-name">"Dr. Jane Gerber"</h2>
                    <p class="profile-role">"Lecturer"</p>
                    <p class="profile-department">"Computer Science Department"</p>
                </div>
            </section>

            <section class="profile-section" aria-labelledby="personal-information">
                <div class="profile-section-header">
                    <span class="profile-section-icon" aria-hidden="true">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="7" r="4"/><path d="M5.5 21a6.5 6.5 0 0 1 13 0"/></svg>
                    </span>
                    <h2 id="personal-information" class="profile-section-title">"Personal Information"</h2>
                </div>
                <div class="profile-form">
                    <div class="profile-field">
                        <label class="profile-label" for="profile-full-name">"Full Name"</label>
                        <input id="profile-full-name" class="input" type="text" attr:value="Dr. Jane Gerber" autocomplete="name"/>
                    </div>
                    <div class="profile-field">
                        <label class="profile-label" for="profile-email">"Email Address"</label>
                        <input id="profile-email" class="input" type="email" attr:value="jane.gerber@university.edu" autocomplete="email"/>
                    </div>
                    <div class="profile-field">
                        <label class="profile-label" for="profile-university">"University"</label>
                        <input id="profile-university" class="input" type="text" attr:value="Stellenbosch University" autocomplete="organization"/>
                    </div>
                    <div class="profile-field">
                        <label class="profile-label" for="profile-contact">"Contact Number"</label>
                        <input id="profile-contact" class="input" type="tel" attr:value="+27 (555) 123-4567" autocomplete="tel"/>
                    </div>
                </div>
            </section>

            <section class="profile-section" aria-labelledby="account-settings">
                <div class="profile-section-header">
                    <span class="profile-section-icon profile-section-icon-gear" aria-hidden="true">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.09A1.65 1.65 0 0 0 9 4.09V4a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51h.09a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.09a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
                    </span>
                    <h2 id="account-settings" class="profile-section-title">"Account Settings"</h2>
                </div>
                <div class="profile-reset-row">
                    <div>
                        <h3 class="profile-reset-title">"Reset Password"</h3>
                        <p class="profile-reset-subtitle">"Change your account password"</p>
                    </div>
                    <button class="btn profile-reset-btn" type="button">"Reset"</button>
                </div>
            </section>

            <div class="profile-actions">
                <button class="btn profile-save" type="button">"Save Changes"</button>
                <button class="btn profile-cancel" type="button">"Cancel"</button>
            </div>
        </section>
    }
}

