use leptos::prelude::*;
use crate::routes::stats_functions::*;
use crate::user_context::get_current_user;

#[component]
pub fn Statistics() -> impl IntoView {
    let current_user = get_current_user();
    
    // Reactive filter signals
    let (selected_module, set_selected_module) = signal(None::<String>);
    let (selected_class, set_selected_class) = signal(None::<i64>);

    // Fetch data using server functions with filters
    let overall_stats = Resource::new(
        move || (current_user.get().map(|u| u.email_address), selected_module.get(), selected_class.get()),
        |(email, module, class)| async move {
            match email {
                Some(email) => get_overall_stats(email, module, class).await,
                None => Err(ServerFnError::new("Not logged in".to_string()))
            }
        }
    );

    // Timeframe filter: Weekly or Monthly (default Monthly as in mockup)
    let (timeframe, set_timeframe) = signal("Monthly".to_string());

    let weekly_trends = Resource::new(
        move || (current_user.get().map(|u| u.email_address), selected_module.get(), timeframe.get()),
        |(email, module, tf)| async move {
            match email {
                Some(email) => get_weekly_trends(email, module, Some(tf)).await,
                None => Err(ServerFnError::new("Not logged in".to_string()))
            }
        }
    );

    let missed_modules = Resource::new(
        move || (current_user.get().map(|u| u.email_address), selected_module.get()),
        |(email, module)| async move {
            match email {
                Some(email) => get_most_missed_modules(email, module).await,
                None => Err(ServerFnError::new("Not logged in".to_string()))
            }
        }
    );
    
    let module_options = Resource::new(
        move || current_user.get().map(|u| u.email_address),
        |email| async move {
            match email {
                Some(email) => get_module_options(email).await,
                None => Err(ServerFnError::new("Not logged in".to_string()))
            }
        }
    );
    
    let class_options = Resource::new(
        move || (current_user.get().map(|u| u.email_address), selected_module.get()),
        |(_email, module)| async move {
            get_class_options(module).await
        }
    );

    // Export removed for now per request — focus on stats + charts only

    view! {
        <div class="statistics-container">
            <div class="stats-header">
                <div class="stats-title">
                    <h1>{move || {
                        if let Some(code) = selected_module.get() {
                            if let Some(Ok(mods)) = module_options.get() {
                                if let Some(m) = mods.into_iter().find(|m| m.module_code == code) {
                                    return format!("Attendance Statistics - {}", m.module_title);
                                }
                            }
                            "Attendance Statistics - Selected Module".to_string()
                        } else {
                            "Attendance Statistics - All Modules".to_string()
                        }
                    }}</h1>
                    <p class="stats-subtitle">"Monitor student attendance"</p>
                </div>
                
                <div class="stats-controls">
                    <select 
                        class="stats-filter"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            if value.is_empty() || value == "all" {
                                set_selected_module.set(None);
                                set_selected_class.set(None);
                            } else {
                                set_selected_module.set(Some(value));
                                set_selected_class.set(None);
                            }
                        }
                    >
                        <option value="all">"All Modules"</option>
                        <Suspense fallback=|| view! { <option>"Loading..."</option> }>
                            {move || module_options.get().map(|opts| {
                                match opts {
                                    Ok(modules) => modules.into_iter().map(|m| {
                                        view! {
                                            <option value={m.module_code.clone()}>
                                                {m.module_title}
                                            </option>
                                        }
                                    }).collect_view(),
                                    Err(_) => vec![view! { 
                                        <option value={String::new()}>
                                            {"Error loading modules".to_string()}
                                        </option> 
                                    }]
                                }
                            })}
                        </Suspense>
                    </select>
                    
                    <select 
                        class="stats-filter"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            if value.is_empty() { set_timeframe.set("Monthly".to_string()); }
                            else { set_timeframe.set(value); }
                        }
                    >
                        <option value="Monthly">"Monthly"</option>
                        <option value="Weekly">"Weekly"</option>
                    </select>

                    <select 
                        class="stats-filter"
                        disabled=move || selected_module.get().is_none()
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            if value.is_empty() || value == "all" {
                                set_selected_class.set(None);
                            } else {
                                if let Ok(id) = value.parse::<i64>() {
                                    set_selected_class.set(Some(id));
                                }
                            }
                        }
                    >
                        <option value="all">"All Classes"</option>
                        <Suspense fallback=|| view! { <option>"Loading..."</option> }>
                            {move || class_options.get().map(|opts| {
                                match opts {
                                    Ok(classes) if !classes.is_empty() => classes.into_iter().map(|c| {
                                        view! {
                                            <option value={c.class_id.to_string()}>
                                                {c.title}
                                            </option>
                                        }
                                    }).collect_view(),
                                    Ok(_) => vec![view! { 
                                        <option value={String::new()}>
                                            {"No classes in this module".to_string()}
                                        </option> 
                                    }],
                                    Err(_) => vec![view! { 
                                        <option value={String::new()}>
                                            {"Error loading classes".to_string()}
                                        </option> 
                                    }]
                                }
                            })}
                        </Suspense>
                    </select>
                    
                    // Export removed for now
                </div>
            </div>

            <div class="stats-grid">
                <Suspense fallback=move || view! { 
                    <div class="stat-card">
                        <div class="stat-content">
                            <div class="stat-value">"Loading..."</div>
                        </div>
                    </div>
                }>
                    {move || overall_stats.get().map(|stats| {
                        match stats {
                            Ok(data) => view! {
                                <div class="stat-card">
                                    <div class="stat-content">
                                        <div class="stat-header">
                                            <span class="stat-label">"Overall Attendance"</span>
                                            <span class="stat-icon"><img src="/attendance.svg" alt="attendance"/></span>
                                        </div>
                                        <div class="stat-value">{format!("{:.1}%", data.attendance_rate)}</div>
                                        <div class="stat-meta">{move || {
                                            if let Some(Ok(tr)) = weekly_trends.get() {
                                                if tr.len() >= 2 {
                                                    let last = tr[tr.len()-1].attendance_rate;
                                                    let prev = tr[tr.len()-2].attendance_rate;
                                                    let diff = last - prev;
                                                    if diff >= 0.0 {
                                                        format!("↑ +{:.1}% from last {}", diff, if timeframe.get()=="Monthly" {"month"} else {"week"})
                                                    } else {
                                                        format!("↓ {:.1}% from last {}", diff.abs(), if timeframe.get()=="Monthly" {"month"} else {"week"})
                                                    }
                                                } else { String::new() }
                                            } else { String::new() }
                                        }}</div>
                                    </div>
                                </div>
                                
                                <div class="stat-card">
                                    <div class="stat-content">
                                        <div class="stat-header">
                                            <span class="stat-label">"Total Students"</span>
                                            <span class="stat-icon"><img src="/Totalstudent.svg" alt="total students"/></span>
                                        </div>
                                        <div class="stat-value">{data.total_students}</div>
                                        <div class="stat-meta">
                                            {move || {
                                                if let Some(Ok(mods)) = module_options.get() {
                                                    format!("Across {} modules", mods.len())
                                                } else { String::new() }
                                            }}
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="stat-card">
                                    <div class="stat-content">
                                        <div class="stat-header">
                                            <span class="stat-label">"Avg Class Size"</span>
                                            <span class="stat-icon"><img src="/avg class size.svg" alt="avg class size"/></span>
                                        </div>
                                        <div class="stat-value">{format!("{:.0}", data.avg_class_size)}</div>
                                        <div class="stat-meta">"Students per class"</div>
                                    </div>
                                </div>
                                
                                <div class="stat-card warning">
                                    <div class="stat-content">
                                        <div class="stat-header">
                                            <span class="stat-label">"Absent Today"</span>
                                            <span class="stat-icon"><img src="/absent today.svg" alt="absent today"/></span>
                                        </div>
                                        <div class="stat-value">{data.absent_today}</div>
                                        <div class="stat-meta">
                                            {move || if data.total_students > 0 {
                                                let pct = (data.absent_today as f64) * 100.0 / (data.total_students as f64);
                                                format!("{:.1}% of total students", pct)
                                            } else { String::new() }}
                                        </div>
                                    </div>
                                </div>
                            }.into_any(),
                            Err(_) => view! {
                                <div class="stat-card">
                                    <div class="stat-content">
                                        <div class="stat-value">"Error loading stats"</div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </div>
            
            <div class="charts-section">
                <div class="chart-card">
                    <div class="chart-header">
                        <h3>{move || if timeframe.get() == "Monthly" { "Attendance Trend (Last 8 Months)".to_string() } else { "Attendance Trend (Last 8 Weeks)".to_string() }}</h3>
                        <div class="chart-legend">
                            <span class="legend-item">
                                <span class="legend-dot attendance"></span>
                                "Attendance Rate"
                            </span>
                        </div>
                    </div>
                    <div class="chart-container">
                        <Suspense fallback=|| view! { <div class="chart-placeholder">"Loading trends..."</div> }>
                            {move || weekly_trends.get().map(|trends| {
                                match trends {
                                    Ok(data) if !data.is_empty() => {
                                        let max_points = data.len();
                                        let width = 700.0;
                                        let height = 300.0;
                                        let padding = 50.0;

                                        let chart_width = width - 2.0 * padding;
                                        let chart_height = height - 2.0 * padding;
                                        let x_step = chart_width / (max_points as f64 - 1.0).max(1.0);

                                        // Dynamic Y scaling for more informative variation
                                        let min_rate = data.iter().map(|t| t.attendance_rate).fold(100.0, |a, b| a.min(b));
                                        let max_rate = data.iter().map(|t| t.attendance_rate).fold(0.0, |a, b| a.max(b));
                                        let mut y_min = (min_rate - 5.0).floor().max(0.0);
                                        let mut y_max = (max_rate + 5.0).ceil().min(100.0);
                                        if (y_max - y_min) < 10.0 { y_max = (y_min + 10.0).min(100.0); }

                                        let to_y = |rate: f64| padding + chart_height * (1.0 - ((rate - y_min) / (y_max - y_min)).max(0.0).min(1.0));

                                        let points: Vec<(f64, f64)> = data.iter().enumerate().map(|(i, t)| {
                                            let x = padding + (i as f64 * x_step);
                                            let y = to_y(t.attendance_rate);
                                            (x, y)
                                        }).collect();

                                        let polyline_points = points.iter()
                                            .map(|(x, y)| format!("{},{}", x, y))
                                            .collect::<Vec<_>>()
                                            .join(" ");

                                        // Area fill under the line
                                        let area_points = format!(
                                            "{} {},{} {},{}",
                                            polyline_points,
                                            padding + chart_width,
                                            padding + chart_height,
                                            padding,
                                            padding + chart_height
                                        );
                                        
                                        view! {
                                            <svg class="trend-chart" viewBox={format!("0 0 {} {}", width, height)} 
                                                 preserveAspectRatio="xMidYMid meet"
                                                 xmlns="http://www.w3.org/2000/svg"
                                                 style="width: 100%; height: auto; max-height: 300px; background: white; border-radius: 8px;">
                                                
                                                // Y-axis
                                                <line x1={padding.to_string()} y1={padding.to_string()} 
                                                      x2={padding.to_string()} y2={(height - padding).to_string()} 
                                                      stroke="#d1d5db" stroke-width="2"/>
                                                      
                                                // X-axis
                                                <line x1={padding.to_string()} y1={(height - padding).to_string()} 
                                                      x2={(width - padding).to_string()} y2={(height - padding).to_string()} 
                                                      stroke="#d1d5db" stroke-width="2"/>
                                                
                                                // Grid lines (dynamic scale)
                                                {(0..=4).map(|i| {
                                                    let v = y_min + (i as f64) * (y_max - y_min) / 4.0;
                                                    let y = to_y(v);
                                                    let label = format!("{:.0}%", v);
                                                    view! { <g>
                                                        <line x1={padding.to_string()} y1={y.to_string()} 
                                                              x2={(width - padding).to_string()} y2={y.to_string()} 
                                                              stroke="#f3f4f6" stroke-width="1"/>
                                                        <text x={(padding - 10.0).to_string()} y={y.to_string()} 
                                                              text-anchor="end" alignment-baseline="middle"
                                                              font-size="12" fill="#6b7280">{label}</text>
                                                    </g> }
                                                }).collect_view()}

                                                // Average line
                                                {let avg: f64 = data.iter().map(|t| t.attendance_rate).sum::<f64>() / (data.len() as f64);
                                                 let y_avg = to_y(avg);
                                                 view! { <line x1={padding.to_string()} y1={y_avg.to_string()}
                                                             x2={(width - padding).to_string()} y2={y_avg.to_string()}
                                                             stroke="#a7f3d0" stroke-dasharray="4,4" stroke-width="2"/> }.into_view()
                                                }

                                                // Area fill
                                                <defs>
                                                    <linearGradient id="area" x1="0" y1="0" x2="0" y2="1">
                                                        <stop offset="0%" stop-color="#14b8a6" stop-opacity="0.25"/>
                                                        <stop offset="100%" stop-color="#14b8a6" stop-opacity="0"/>
                                                    </linearGradient>
                                                </defs>
                                                <polygon points={area_points} fill="url(#area)" opacity="0.6"/>
                                                
                                                // Line path
                                                <polyline points={polyline_points} 
                                                          fill="none" 
                                                          stroke="#14b8a6" 
                                                          stroke-width="3"
                                                          stroke-linecap="round"
                                                          stroke-linejoin="round"/>
                                                
                                                // Data points and labels
                                                {points.iter().enumerate().map(|(i, (x, y))| {
                                                    let rate = data[i].attendance_rate;
                                                    let x_label = {
                                                        let raw = data[i].week.clone();
                                                        if raw.contains('-') {
                                                            // Expect YYYY-MM -> Mon 'YY
                                                            let parts: Vec<&str> = raw.split('-').collect();
                                                            if parts.len() == 2 {
                                                                let month = match parts[1] {"01"=>"Jan","02"=>"Feb","03"=>"Mar","04"=>"Apr","05"=>"May","06"=>"Jun","07"=>"Jul","08"=>"Aug","09"=>"Sep","10"=>"Oct","11"=>"Nov","12"=>"Dec", _=>""};
                                                                let yr = &parts[0][2..];
                                                                format!("{} '{}'", month, yr)
                                                            } else { raw }
                                                        } else { raw }
                                                    };
                                                    view! {
                                                        <g>
                                                            <circle cx={x.to_string()} cy={y.to_string()} 
                                                                    r="6" fill="#14b8a6" stroke="white" stroke-width="2"/>
                                                            <text x={x.to_string()} y={(height - padding + 20.0).to_string()} 
                                                                  text-anchor="middle" font-size="11" fill="#6b7280">
                                                                {x_label}
                                                            </text>
                                                            <text x={x.to_string()} y={(y - 15.0).to_string()} 
                                                                  text-anchor="middle" font-size="12" font-weight="600" fill="#1f2937">
                                                                {format!("{:.0}%", rate)}
                                                            </text>
                                                        </g>
                                                    }
                                                }).collect_view()}
                                            </svg>
                                        }.into_any()
                                    },
                                    Ok(_) => view! { 
                                        <div class="chart-placeholder">"No trend data available. Add more attendance records across different weeks."</div> 
                                    }.into_any(),
                                    Err(_) => view! { 
                                        <div class="chart-placeholder">"Error loading trends"</div> 
                                    }.into_any()
                                }
                            })}
                        </Suspense>
                    </div>
                </div>
                
                <div class="chart-card">
                    <div class="chart-header">
                        <h3>"Most Missed Modules"</h3>
                        <div class="chart-legend">
                            <span class="legend-item">
                                <span class="legend-dot absence"></span>
                                "Absence Rate"
                            </span>
                        </div>
                    </div>
                    <div class="absence-chart">
                        <Suspense fallback=|| view! { 
                            <div class="chart-placeholder">"Loading modules..."</div> 
                        }>
                            {move || missed_modules.get().map(|modules| {
                                match modules {
                                    Ok(data) if !data.is_empty() => view! {
                                        <div class="absence-list">
                                            {data.into_iter().map(|m| {
                                                let width = format!("{}%", m.absence_rate.min(100.0));
                                                view! {
                                                    <div class="absence-bar">
                                                        <div class="module-info">
                                                            <span class="module-name">{m.module_title}</span>
                                                            <span class="absence-value">{format!("{:.1}%", m.absence_rate)}</span>
                                                        </div>
                                                        <div class="bar-container">
                                                            <div class="bar-background">
                                                                <div class="bar-fill" style:width={width}></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any(),
                                    Ok(_) => view! { 
                                        <div class="chart-placeholder">
                                            "No absence data available"
                                        </div> 
                                    }.into_any(),
                                    Err(e) => view! { 
                                        <div class="chart-placeholder chart-error">
                                            {format!("Error loading module data: {}", e)}
                                        </div> 
                                    }.into_any()
                                }
                            })}
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}
