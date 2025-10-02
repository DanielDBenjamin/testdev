use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn StudentStatisticsPage() -> impl IntoView {
    let navigate = use_navigate();

    let navigate_clone1 = navigate.clone();
    let navigate_clone2 = navigate.clone();
    let go_to_home = move |_| {
        navigate_clone1("/student/home", Default::default());
    };

    let go_to_home_nav = move |_| {
        navigate_clone2("/student/home", Default::default());
    };

    let go_to_profile = move |_| {
        navigate("/student/profile", Default::default());
    };

    // Mock data for weekly attendance
    let weekly_data = vec![
        ("Mon", 100.0),
        ("Tue", 95.0),
        ("Wed", 100.0),
        ("Thu", 100.0),
        ("Fri", 85.0),
    ];

    // Mock data for module breakdown
    let module_data = vec![
        ("Math", 100.0, "#10B981"),
        ("Physics", 85.0, "#F59E0B"),
        ("Chemistry", 70.0, "#EF4444"),
        ("Biology", 90.0, "#8B5CF6"),
    ];

    // Mock data for recent activity
    let recent_activities = vec![
        ("Attended Chemistry Lab", "Today, 2:30 PM", true),
        ("Attended Mathematics", "Today, 10:00 AM", true),
        ("Missed Physics Lab", "Yesterday, 3:00 PM", false),
    ];

    view! {
        <div class="stats-container">
            {/* Header */}
            <header class="stats-header">
                <button class="back-button" on:click=go_to_home>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M19 12H5M12 19l-7-7 7-7"/>
                    </svg>
                </button>
                <div class="stats-header-title">
                    <h1>"Statistics"</h1>
                    <p>"Sarah Johnson"</p>
                </div>
                <div class="stats-header-actions">
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

            {/* Filter dropdowns */}
            <div class="stats-filters">
                <select class="filter-select">
                    <option>"This Week"</option>
                    <option>"This Month"</option>
                    <option>"This Semester"</option>
                </select>
                <select class="filter-select">
                    <option>"All Courses"</option>
                    <option>"Mathematics"</option>
                    <option>"Physics"</option>
                    <option>"Chemistry"</option>
                    <option>"Biology"</option>
                </select>
            </div>

            {/* Overall stats cards */}
            <div class="stats-cards">
                <div class="stat-card stat-card-overall">
                    <div class="stat-card-header">
                        <span class="stat-label">"Overall"</span>
                        <div class="stat-icon stat-icon-green">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
                            </svg>
                        </div>
                    </div>
                    <div class="stat-value">"94%"</div>
                    <div class="stat-change stat-change-positive">"+2.1% from last week"</div>
                </div>

                <div class="stat-card stat-card-week">
                    <div class="stat-card-header">
                        <span class="stat-label">"This Week"</span>
                        <div class="stat-icon stat-icon-blue">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                                <line x1="16" y1="2" x2="16" y2="6"></line>
                                <line x1="8" y1="2" x2="8" y2="6"></line>
                                <line x1="3" y1="10" x2="21" y2="10"></line>
                            </svg>
                        </div>
                    </div>
                    <div class="stat-value">"96%"</div>
                    <div class="stat-change">"4 out of 5 days"</div>
                </div>
            </div>

            {/* Weekly Attendance Chart */}
            <div class="chart-container">
                <div class="chart-header">
                    <h2>"Weekly Attendance"</h2>
                    <button class="chart-menu-btn">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="1"></circle>
                            <circle cx="12" cy="5" r="1"></circle>
                            <circle cx="12" cy="19" r="1"></circle>
                        </svg>
                    </button>
                </div>
                <div class="chart-content">
                    <div class="chart-y-axis">
                        <span>"100%"</span>
                        <span>"50%"</span>
                        <span>"0%"</span>
                    </div>
                    <div class="chart-plot">
                        <svg class="line-chart" viewBox="0 0 300 150" preserveAspectRatio="none">
                            <polyline
                                fill="none"
                                stroke="#3B82F6"
                                stroke-width="2"
                                points="0,0 75,7.5 150,0 225,0 300,22.5"
                            />
                            <circle cx="0" cy="0" r="4" fill="#3B82F6"/>
                            <circle cx="75" cy="7.5" r="4" fill="#3B82F6"/>
                            <circle cx="150" cy="0" r="4" fill="#3B82F6"/>
                            <circle cx="225" cy="0" r="4" fill="#3B82F6"/>
                            <circle cx="300" cy="22.5" r="4" fill="#3B82F6"/>
                        </svg>
                    </div>
                </div>
                <div class="chart-x-axis">
                    {weekly_data.into_iter().map(|(day, _)| {
                        view! {
                            <span>{day}</span>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            {/* Attendance Insights */}
            <div class="insights-section">
                <h2>"Attendance Insights"</h2>

                <div class="insight-card insight-card-danger">
                    <div class="insight-icon" style="background-color: #FEE2E2;">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="#EF4444">
                            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                            <line x1="16" y1="2" x2="16" y2="6" stroke="white" stroke-width="2"></line>
                            <line x1="8" y1="2" x2="8" y2="6" stroke="white" stroke-width="2"></line>
                            <line x1="3" y1="10" x2="21" y2="10" stroke="white" stroke-width="2"></line>
                        </svg>
                    </div>
                    <div class="insight-content">
                        <h3>"Most Missed Days"</h3>
                        <p>"You tend to miss classes on Fridays"</p>
                    </div>
                    <div class="insight-percentage">
                        <div class="progress-bar">
                            <div class="progress-fill progress-fill-red" style="width: 60%;"></div>
                        </div>
                        <span class="percentage-text">"60%"</span>
                    </div>
                </div>

                <div class="insight-card insight-card-warning">
                    <div class="insight-icon" style="background-color: #FEF3C7;">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="#F59E0B">
                            <rect x="4" y="4" width="16" height="16" rx="2" ry="2"></rect>
                        </svg>
                    </div>
                    <div class="insight-content">
                        <h3>"Most Missed Modules"</h3>
                        <p>"Physics Lab sessions need attention"</p>
                    </div>
                    <div class="insight-percentage">
                        <div class="progress-bar">
                            <div class="progress-fill progress-fill-orange" style="width: 45%;"></div>
                        </div>
                        <span class="percentage-text">"45%"</span>
                    </div>
                </div>

                <div class="insight-card insight-card-success">
                    <div class="insight-icon" style="background-color: #D1FAE5;">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="#10B981">
                            <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z"/>
                        </svg>
                    </div>
                    <div class="insight-content">
                        <h3>"Best Performance"</h3>
                        <p>"Perfect attendance in Mathematics"</p>
                    </div>
                    <div class="insight-percentage">
                        <div class="progress-bar">
                            <div class="progress-fill progress-fill-green" style="width: 100%;"></div>
                        </div>
                        <span class="percentage-text">"100%"</span>
                    </div>
                </div>
            </div>

            {/* Module Breakdown */}
            <div class="module-breakdown">
                <div class="module-breakdown-header">
                    <h2>"Module Breakdown"</h2>
                    <button class="view-all-btn">"View All"</button>
                </div>
                <div class="bar-chart">
                    <div class="bar-chart-y-axis">
                        <span>"100%"</span>
                        <span>"50%"</span>
                        <span>"0%"</span>
                    </div>
                    <div class="bar-chart-content">
                        {module_data.iter().map(|(name, percentage, color)| {
                            view! {
                                <div class="bar-column">
                                    <div class="bar-wrapper">
                                        <div class="bar" style={format!("height: {}%; background-color: {}", percentage, color)}></div>
                                    </div>
                                    <span class="bar-label">{*name}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>

            {/* Recent Activity */}
            <div class="recent-activity">
                <h2>"Recent Activity"</h2>
                <div class="activity-list">
                    {recent_activities.into_iter().map(|(title, time, attended)| {
                        let icon_class = if attended { "activity-icon-success" } else { "activity-icon-error" };
                        let icon_bg = if attended { "background-color: #D1FAE5;" } else { "background-color: #FEE2E2;" };
                        view! {
                            <div class="activity-item">
                                <div class={format!("activity-icon {}", icon_class)} style={icon_bg}>
                                    {if attended {
                                        view! {
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#10B981" stroke-width="2">
                                                <polyline points="20 6 9 17 4 12"></polyline>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#EF4444" stroke-width="2">
                                                <line x1="18" y1="6" x2="6" y2="18"></line>
                                                <line x1="6" y1="6" x2="18" y2="18"></line>
                                            </svg>
                                        }.into_any()
                                    }}
                                </div>
                                <div class="activity-content">
                                    <h4>{title}</h4>
                                    <p>{time}</p>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            {/* Bottom Navigation */}
            <nav class="bottom-nav">
                <button class="nav-item" on:click=go_to_home_nav>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                        <polyline points="9 22 9 12 15 12 15 22"></polyline>
                    </svg>
                    <span class="nav-label">"Home"</span>
                </button>

                <button class="nav-item nav-item-scan">
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

                <button class="nav-item nav-item-active">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="20" x2="18" y2="10"></line>
                        <line x1="12" y1="20" x2="12" y2="4"></line>
                        <line x1="6" y1="20" x2="6" y2="14"></line>
                    </svg>
                    <span class="nav-label">"Stats"</span>
                </button>
            </nav>
        </div>
    }
}
