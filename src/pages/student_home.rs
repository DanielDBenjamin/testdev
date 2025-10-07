use crate::components::QrScanner;
use crate::routes::class_functions::record_session_attendance_fn;
use crate::routes::student_functions::{get_student_schedule, StudentScheduleItem};
use crate::user_context::get_current_user;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use urlencoding::encode;

fn current_date_iso() -> String {
    chrono::Local::now()
        .naive_local()
        .format("%Y-%m-%d")
        .to_string()
}

fn format_pretty_date(date_iso: &str) -> String {
    chrono::NaiveDate::parse_from_str(date_iso, "%Y-%m-%d")
        .map(|date| date.format("%A, %d %b").to_string())
        .unwrap_or_else(|_| chrono::Local::now().format("%A, %d %b").to_string())
}

fn format_short_date(date_iso: &str) -> String {
    chrono::NaiveDate::parse_from_str(date_iso, "%Y-%m-%d")
        .map(|date| date.format("%d %b").to_string())
        .unwrap_or_else(|_| date_iso.to_string())
}

#[component]
pub fn StudentHomePage() -> impl IntoView {
    let navigate = use_navigate();
    let (show_scanner, set_show_scanner) = signal(false);
    let (_scanned_data, set_scanned_data) = signal(None::<String>);
    let feedback = RwSignal::new(None::<(bool, String)>);
    let current_user = get_current_user();
    
    // Auto-dismiss feedback after 4 seconds
    let set_feedback_with_timeout = {
        let feedback = feedback.clone();
        move |msg: Option<(bool, String)>| {
            feedback.set(msg.clone());
            if msg.is_some() {
                let feedback_clear = feedback.clone();
                spawn_local(async move {
                    #[cfg(not(feature = "ssr"))]
                    {
                        use gloo_timers::future::TimeoutFuture;
                        TimeoutFuture::new(4000).await;
                        feedback_clear.set(None);
                    }
                });
            }
        }
    };

    let selected_date = RwSignal::new(current_date_iso());
    let schedule_feedback = RwSignal::new(None::<String>);

    let user_full_name = Signal::derive(move || {
        current_user
            .get()
            .map(|u| format!("{} {}", u.name, u.surname))
            .unwrap_or_else(|| "Student".to_string())
    });

    let user_first_name = Signal::derive(move || {
        current_user
            .get()
            .map(|u| u.name.clone())
            .unwrap_or_else(|| "Student".to_string())
    });

    let avatar_url = Signal::derive(move || {
        current_user.get().map(|u| {
            let full_name = format!("{} {}", u.name, u.surname);
            let encoded = encode(&full_name);
            format!(
                "https://ui-avatars.com/api/?name={}&background=14b8a6&color=ffffff&format=svg",
                encoded
            )
        })
    });

    let date_label = Signal::derive(move || format_pretty_date(&selected_date.get()));

    let subtitle_text = Signal::derive(move || format!("Welcome back, {}!", user_first_name.get()));

    let schedule_resource = Resource::new(
        move || {
            current_user
                .get()
                .map(|user| (user.email_address.clone(), selected_date.get()))
        },
        {
            let schedule_feedback = schedule_feedback.clone();
            move |params: Option<(String, String)>| async move {
                match params {
                    Some((email, date)) => {
                        match get_student_schedule(email, Some(date.clone())).await {
                            Ok(response) => {
                                if (!response.success || response.classes.is_empty())
                                    && !response.message.is_empty()
                                {
                                    schedule_feedback.set(Some(response.message.clone()));
                                } else {
                                    schedule_feedback.set(None);
                                }
                                Some(response.classes)
                            }
                            Err(err) => {
                                schedule_feedback.set(Some(err.to_string()));
                                None
                            }
                        }
                    }
                    None => {
                        schedule_feedback
                            .set(Some("Please sign in to view your schedule.".to_string()));
                        None
                    }
                }
            }
        },
    );

    let handle_scan = {
        let set_scanned_data = set_scanned_data.clone();
        let set_show_scanner = set_show_scanner.clone();
        let set_feedback_with_timeout = set_feedback_with_timeout.clone();
        let current_user = current_user.clone();
        Callback::new(move |data: String| {
            set_scanned_data.set(Some(data.clone()));
            set_show_scanner.set(false);
            if let Some(user) = current_user.get() {
                let email = user.email_address.clone();
                let set_feedback_with_timeout = set_feedback_with_timeout.clone();
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
                                        set_feedback_with_timeout(Some((resp.success, resp.message)));
                                    }
                                    Err(e) => {
                                        set_feedback_with_timeout(Some((false, e.to_string())));
                                    }
                                }
                            }
                            Err(err) => {
                                set_feedback_with_timeout(Some((false, err)));
                            }
                        }
                    }

                    #[cfg(feature = "ssr")]
                    {
                        set_feedback_with_timeout(Some((
                            false,
                            "Location capture requires a browser.".to_string(),
                        )));
                    }
                });
            } else {
                set_feedback_with_timeout(Some((
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
                    <button class="student-profile-picture" on:click=go_to_profile>
                        <img
                            alt=move || user_full_name.get()
                            prop:src=move || {
                                avatar_url
                                    .get()
                                    .unwrap_or_else(|| "/logo.png".to_string())
                            }
                        />
                    </button>
                </div>
            </header>

            <div class="student-home-content">
                {/* Date and title section */}
                <section class="student-date-section">
                    <h2 class="student-date-title">{date_label}</h2>
                    <p class="student-date-subtitle">{subtitle_text}</p>
                </section>

                {/* Module cards */}
                <Suspense fallback=move || view! { <div class="student-modules-list"><div class="student-module-card loading">"Loading your schedule…"</div></div> }>
                    {move || {
                        schedule_resource
                            .get()
                            .map(|maybe_classes| {
                                let current_date = selected_date.get();
                                match maybe_classes {
                                    Some(classes) => {
                                        if classes.is_empty() {
                                            let message = schedule_feedback
                                                .get()
                                                .unwrap_or_else(|| {
                                                    "No upcoming classes found.".to_string()
                                                });
                                            view! {
                                                <div class="student-modules-list">
                                                    <div class="student-module-empty">{message}</div>
                                                </div>
                                            }
                                            .into_any()
                                        } else {
                                            view! {
                                                <div class="student-modules-list">
                                                    {classes
                                                        .into_iter()
                                                        .enumerate()
                                                        .map(|(index, class)| {
                                                            schedule_card(class, index, current_date.clone())
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </div>
                                            }
                                            .into_any()
                                        }
                                    }
                                    None => {
                                        let message = schedule_feedback
                                            .get()
                                            .unwrap_or_else(|| "No schedule data available.".to_string());
                                        view! {
                                            <div class="student-modules-list">
                                                <div class="student-module-empty">{message}</div>
                                            </div>
                                        }
                                        .into_any()
                                    }
                                }
                            })
                            .unwrap_or_else(|| {
                                view! {
                                    <div class="student-modules-list">
                                        <div class="student-module-empty">"Sign in to view your schedule."</div>
                                    </div>
                                }
                                .into_any()
                            })
                    }}
                </Suspense>
            </div>

            {/* Attendance Feedback Popup */}
            <Show when=move || feedback.get().is_some()>
                {move || feedback.get().map(|(success, message)| {
                    let popup_class = if success { 
                        "student-attendance-popup student-attendance-popup-success" 
                    } else { 
                        "student-attendance-popup student-attendance-popup-error" 
                    };
                    view! {
                        <div class="student-attendance-overlay">
                            <div class=popup_class>
                                <div class="student-attendance-icon">
                                    {if success {
                                        view! {
                                            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path d="M9 12l2 2 4-4"/>
                                                <circle cx="12" cy="12" r="10"/>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <circle cx="12" cy="12" r="10"/>
                                                <line x1="15" y1="9" x2="9" y2="15"/>
                                                <line x1="9" y1="9" x2="15" y2="15"/>
                                            </svg>
                                        }.into_any()
                                    }}
                                </div>
                                <div class="student-attendance-title">
                                    {if success { "Success!" } else { "Error" }}
                                </div>
                                <div class="student-attendance-message">
                                    {message}
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }).unwrap_or_else(|| view! { <></> }.into_any())}
            </Show>

            {/* Bottom Navigation */}
            <nav class="student-bottom-nav">
                <button class="student-nav-item student-nav-item-active">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                        <polyline points="9 22 9 12 15 12 15 22"></polyline>
                    </svg>
                    <span class="student-nav-label">"Home"</span>
                </button>

                <button class="student-nav-item student-nav-item-scan" on:click=open_scanner data-testid="scan-button">
                    <div class="student-scan-button">
                        <img src="/i.png" alt="Scan QR" width="46" height="32" data-testid="qr-icon"/>
                    </div>
                    <span class="student-nav-label">"Scan QR"</span>
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

fn schedule_card(
    class: StudentScheduleItem,
    index: usize,
    current_date_iso: String,
) -> impl IntoView {
    let StudentScheduleItem {
        class_id: _,
        module_code,
        module_title,
        class_title,
        venue,
        date,
        time,
        status: _,
    } = class;

    let color_class = match index % 4 {
        0 => "purple",
        1 => "red",
        2 => "yellow",
        _ => "teal",
    };

    let icon_text = module_code
        .chars()
        .find(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "•".to_string());

    let venue_text = venue.unwrap_or_else(|| "Location TBA".to_string());
    let details = if class_title.trim().is_empty() {
        venue_text.clone()
    } else if venue_text.trim().is_empty() {
        class_title.clone()
    } else {
        format!("{} · {}", class_title, venue_text)
    };

    let display_line = if date == current_date_iso {
        details
    } else {
        format!("{} · {}", format_short_date(&date), details)
    };

    view! {
        <button class="student-module-card">
            <div class="student-module-time">{time}</div>
            <div class={format!("student-module-icon student-module-icon-{}", color_class)}>
                {icon_text}
            </div>
            <div class="student-module-info">
                <div class="student-module-name">{module_title}</div>
                <div class="student-module-location">{display_line}</div>
            </div>
        </button>
    }
}
