use crate::components::QrScanner;
use crate::routes::class_functions::record_session_attendance_fn;
use crate::user_context::get_current_user;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

#[component]
pub fn StudentHomePage() -> impl IntoView {
    let navigate = use_navigate();
    let (show_scanner, set_show_scanner) = signal(false);
    let (_scanned_data, set_scanned_data) = signal(None::<String>);
    let feedback = RwSignal::new(None::<(bool, String)>);
    let current_user = get_current_user();

    // Mock data for modules - replace with real data later
    let modules = vec![
        ("08:00", "MATH 112", "Merensky 101", "purple", "📱"),
        ("10:00", "PHYS 114", "VDS 212", "red", "⚛️"),
        ("13:00", "CHEM 201", "OrgChem 222", "yellow", "⚗️"),
    ];

    let handle_scan = {
        let set_scanned_data = set_scanned_data.clone();
        let set_show_scanner = set_show_scanner.clone();
        let feedback = feedback.clone();
        let current_user = current_user.clone();
        Callback::new(move |data: String| {
            set_scanned_data.set(Some(data.clone()));
            set_show_scanner.set(false);
            if let Some(user) = current_user.get() {
                let email = user.email_address.clone();
                let feedback = feedback.clone();
                let payload = data.clone();
                spawn_local(async move {
                    #[cfg(not(feature = "ssr"))]
                    {
                        match crate::utils::geolocation::get_current_location().await {
                            Ok(location) => {
                                match record_session_attendance_fn(
                                    payload.clone(),
                                    email.clone(),
                                    Some(location.latitude),
                                    Some(location.longitude),
                                    location.accuracy,
                                )
                                .await
                                {
                                    Ok(resp) => {
                                        feedback.set(Some((resp.success, resp.message)));
                                    }
                                    Err(e) => {
                                        feedback.set(Some((false, e.to_string())));
                                    }
                                }
                            }
                            Err(err) => {
                                feedback.set(Some((false, err)));
                            }
                        }
                    }

                    #[cfg(feature = "ssr")]
                    {
                        feedback.set(Some((
                            false,
                            "Location capture requires a browser.".to_string(),
                        )));
                    }
                });
            } else {
                feedback.set(Some((
                    false,
                    "Please log in as a student to record attendance.".to_string(),
                )));
            }
        })
    };

    let handle_close_scanner = Callback::new(move |_| {
        set_show_scanner.set(false);
    });

    let open_scanner = move |_| {
        set_show_scanner.set(true);
    };

    let navigate_clone = navigate.clone();
    let go_to_profile = move |_| {
        navigate_clone("/student/profile", Default::default());
    };

    let go_to_statistics = move |_| {
        navigate("/student/statistics", Default::default());
    };

    view! {
        <div class="student-home-container">
            {/* Header */}
            <header class="student-home-header">
                <div class="student-header-logo">
                    <img src="/logo.png" srcset="/logo.png 1x" alt="Clock It" class="student-brand-logo-img" width="160" height="60" />
                </div>
                <div class="student-header-actions">
                    <button class="student-notification-btn">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                            <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                        </svg>
                    </button>
                    <button class="student-profile-picture" on:click=go_to_profile>
                        <img src="https://mockmind-api.uifaces.co/content/human/80.jpg" alt="Profile Avatar" />
                    </button>
                </div>
            </header>

            {/* Date and title section */}
            <section class="student-date-section">
                <h2 class="student-date-title">"Monday, 24 Aug"</h2>
                <p class="student-date-subtitle">"Select a module to manage attendance"</p>
            </section>

            {/* Module cards */}
            <div class="student-modules-list">
                {modules.into_iter().map(|(time, module_name, location, color, icon)| {
                    view! {
                        <button class="student-module-card">
                            <div class="student-module-time">{time}</div>
                            <div class={format!("student-module-icon student-module-icon-{}", color)}>
                                {icon}
                            </div>
                            <div class="student-module-info">
                                <div class="student-module-name">{module_name}</div>
                                <div class="student-module-location">{location}</div>
                            </div>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            {move || feedback.get().map(|(ok, msg)| {
                let class_name = if ok { "student-feedback success" } else { "student-feedback error" };
                view! { <div class=class_name>{msg}</div> }.into_any()
            }).unwrap_or_else(|| view! { <></> }.into_any())}

            {/* Bottom Navigation */}
            <nav class="student-bottom-nav">
                <button class="student-nav-item student-nav-item-active">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                        <polyline points="9 22 9 12 15 12 15 22"></polyline>
                    </svg>
                    <span class="student-nav-label">"Home"</span>
                </button>

                <button class="student-nav-item student-nav-item-scan" on:click=open_scanner>
                    <div class="student-scan-button">
                        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="3" y="3" width="7" height="7"></rect>
                            <rect x="14" y="3" width="7" height="7"></rect>
                            <rect x="14" y="14" width="7" height="7"></rect>
                            <rect x="3" y="14" width="7" height="7"></rect>
                        </svg>
                    </div>
                    <span class="student-nav-label">"Scan"</span>
                </button>

                <button class="student-nav-item" on:click=go_to_statistics>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="20" x2="18" y2="10"></line>
                        <line x1="12" y1="20" x2="12" y2="4"></line>
                        <line x1="6" y1="20" x2="6" y2="14"></line>
                    </svg>
                    <span class="student-nav-label">"Stats"</span>
                </button>
            </nav>

            {/* QR Scanner Modal */}
            {move || if show_scanner.get() {
                view! {
                    <QrScanner
                        on_scan=handle_scan
                        on_close=handle_close_scanner
                    />
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}
