use leptos::prelude::*;
use crate::routes::stats_functions::*;

#[component]
pub fn Statistics() -> impl IntoView {
    // Reactive filter signals
    let (selected_module, set_selected_module) = signal(None::<i64>);
    let (selected_class, set_selected_class) = signal(None::<i64>);

    // Fetch data using server functions with filters
    let overall_stats = Resource::new(
        move || (selected_module.get(), selected_class.get()),
        |(module, class)| async move { 
            get_overall_stats(module, class).await 
        }
    );

    let weekly_trends = Resource::new(
        move || selected_module.get(),
        |module| async move { 
            get_weekly_trends(module).await 
        }
    );

    let missed_modules = Resource::new(
        move || selected_module.get(),
        |module| async move { 
            get_most_missed_modules(module).await 
        }
    );
    let module_options = Resource::new(|| (), |_| async { get_module_options().await });
    
    let class_options = Resource::new(
        move || selected_module.get(),
        |module| async move { 
            get_class_options(module).await 
        }
    );

    view! {
        <div class="statistics-container">
            <div class="stats-header">
                <div class="stats-title">
                    <h1>"Attendance Statistics"</h1>
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
                                if let Ok(code) = value.parse::<i64>() {
                                    set_selected_module.set(Some(code));
                                    set_selected_class.set(None);
                                }
                            }
                        }
                    >
                        <option value="all">"All Modules"</option>
                        <Suspense fallback=|| view! { <option>"Loading..."</option> }>
                            {move || module_options.get().map(|opts| {
                                match opts {
                                    Ok(modules) => modules.into_iter().map(|m| {
                                        view! {
                                            <option value={m.module_code.to_string()}>
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
                    
                    <button class="export-btn">"↓ Export Report"</button>
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
                                            {format!("Across {} classes", data.total_classes)}
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
                        <h3>"Attendance Trend (Last 8 Weeks)"</h3>
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
                                        
                                        let points: Vec<(f64, f64)> = data.iter().enumerate().map(|(i, t)| {
                                            let x = padding + (i as f64 * x_step);
                                            let y = padding + chart_height - ((t.attendance_rate / 100.0) * chart_height);
                                            (x, y)
                                        }).collect();
                                        
                                        let polyline_points = points.iter()
                                            .map(|(x, y)| format!("{},{}", x, y))
                                            .collect::<Vec<_>>()
                                            .join(" ");
                                        
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
                                                
                                                // Grid lines
                                                {(0..=4).map(|i| {
                                                    let y = padding + (i as f64 * chart_height / 4.0);
                                                    let percent = 100 - (i * 25);
                                                    view! {
                                                        <g>
                                                            <line x1={padding.to_string()} y1={y.to_string()} 
                                                                  x2={(width - padding).to_string()} y2={y.to_string()} 
                                                                  stroke="#f3f4f6" stroke-width="1"/>
                                                            <text x={(padding - 10.0).to_string()} y={y.to_string()} 
                                                                  text-anchor="end" alignment-baseline="middle"
                                                                  font-size="12" fill="#6b7280">
                                                                {format!("{}%", percent)}
                                                            </text>
                                                        </g>
                                                    }
                                                }).collect_view()}
                                                
                                                // Line path
                                                <polyline points={polyline_points} 
                                                          fill="none" 
                                                          stroke="#3b82f6" 
                                                          stroke-width="3"
                                                          stroke-linecap="round"
                                                          stroke-linejoin="round"/>
                                                
                                                // Data points and labels
                                                {points.iter().enumerate().map(|(i, (x, y))| {
                                                    let rate = data[i].attendance_rate;
                                                    view! {
                                                        <g>
                                                            <circle cx={x.to_string()} cy={y.to_string()} 
                                                                    r="6" fill="#3b82f6" stroke="white" stroke-width="2"/>
                                                            <text x={x.to_string()} y={(height - padding + 20.0).to_string()} 
                                                                  text-anchor="middle" font-size="11" fill="#6b7280">
                                                                {data[i].week.clone()}
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