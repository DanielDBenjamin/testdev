use leptos::prelude::*;
use crate::components::QrScanner;
use leptos_router::hooks::use_navigate;

#[component]
pub fn StudentHomePage() -> impl IntoView {
    let navigate = use_navigate();
    let (show_scanner, set_show_scanner) = signal(false);
    let (_scanned_data, set_scanned_data) = signal(None::<String>);

    // Mock data for modules - replace with real data later
    let modules = vec![
        ("08:00", "MATH 112", "Merensky 101", "purple", "📱"),
        ("10:00", "PHYS 114", "VDS 212", "red", "⚛️"),
        ("13:00", "CHEM 201", "OrgChem 222", "yellow", "⚗️"),
    ];

    let handle_scan = Callback::new(move |data: String| {
        set_scanned_data.set(Some(data.clone()));
        set_show_scanner.set(false);
        // TODO: Process the scanned QR code data here
        leptos::logging::log!("Scanned QR code: {}", data);
    });

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
        <div class="home-container">
            {/* Header */}
            <header class="home-header">
                <div class="header-logo">
                    <img src="/logo.png" srcset="/logo.png 1x" alt="Clock It" class="brand-logo-img" width="160" height="60" />
                </div>
                <div class="header-actions">
                    <button class="notification-btn">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                            <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                        </svg>
                    </button>
                    <button class="profile-picture" on:click=go_to_profile>
                        <img src="https://mockmind-api.uifaces.co/content/human/80.jpg" alt="Profile Avatar" />
                    </button>
                </div>
            </header>

            {/* Date and title section */}
            <section class="date-section">
                <h2 class="date-title">"Monday, 24 Aug"</h2>
                <p class="date-subtitle">"Select a module to manage attendance"</p>
            </section>

            {/* Module cards */}
            <div class="modules-list">
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

            {/* Bottom Navigation */}
            <nav class="bottom-nav">
                <button class="nav-item nav-item-active">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                        <polyline points="9 22 9 12 15 12 15 22"></polyline>
                    </svg>
                    <span class="nav-label">"Home"</span>
                </button>
                
                <button class="nav-item nav-item-scan" on:click=open_scanner>
                    <div class="scan-button">
                        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="3" y="3" width="7" height="7"></rect>
                            <rect x="14" y="3" width="7" height="7"></rect>
                            <rect x="14" y="14" width="7" height="7"></rect>
                            <rect x="3" y="14" width="7" height="7"></rect>
                        </svg>
                    </div>
                    <span class="nav-label">"Scan"</span>
                </button>
                
                <button class="nav-item" on:click=go_to_statistics>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="20" x2="18" y2="10"></line>
                        <line x1="12" y1="20" x2="12" y2="4"></line>
                        <line x1="6" y1="20" x2="6" y2="14"></line>
                    </svg>
                    <span class="nav-label">"Stats"</span>
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