use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ClassesPage() -> impl IntoView {
    // Static placeholder: backend will provide selected module details.
    let title = "Data Structures & Algorithms";
    let code = "CS301";

    view! {
        <section class="classes-page">
            <div class="page-header" style="display:flex;align-items:center;gap:8px;">
                <A href="/home" attr:class="link">"← Back"</A>
                <div>
                    <h1 class="page-title">{title}</h1>
                    <p class="page-subtitle">{format!("{} • 2025", code)}</p>
                </div>
                <div style="margin-left:auto; display:flex; gap:8px;">
                    <button class="btn btn-outline">"⭳ Export"</button>
                    <A href="/classes/new" attr:class="btn btn-primary">"+ Add Class"</A>
                </div>
            </div>

            <div class="stats-row" style="margin: 12px 0 16px;">
                <div class="stat-tile"><div class="stat-value">"24"</div><div class="stat-label">"Total Classes"</div></div>
                <div class="stat-tile"><div class="stat-value" style="color:#10b981;">"18"</div><div class="stat-label">"Completed"</div></div>
                <div class="stat-tile"><div class="stat-value" style="color:#2563eb;">"6"</div><div class="stat-label">"Upcoming"</div></div>
                <div class="stat-tile"><div class="stat-value">"156"</div><div class="stat-label">"Enrolled Students"</div></div>
            </div>

            <div class="card" style="padding:0;">
                <div style="display:flex; align-items:center; justify-content:space-between; padding:12px 14px; border-bottom:1px solid var(--sidebar-border);">
                    <h3 class="heading" style="margin:0;">"Classes Schedule"</h3>
                    <div style="display:flex; gap:8px;">
                        <input class="input" placeholder="Search classes..." style="width:240px;" />
                        <button class="btn btn-outline">"All Status"</button>
                    </div>
                </div>

                <div style="overflow:auto;">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>"Class Title"</th>
                                <th>"Date"</th>
                                <th>"Time"</th>
                                <th>"Venue"</th>
                                <th>"Status"</th>
                                <th>"Actions"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {|| (0..5).map(|i| view! { <ScheduleRow idx=i/> }).collect_view()}
                        </tbody>
                    </table>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ScheduleRow(idx: i32) -> impl IntoView {
    let (title, date, time, venue, status) = match idx {
        0 => ("Introduction to Arrays", "Mon, Jan 15\n2024", "09:00 - 10:30\n90 minutes", "Room A101\nBuilding A", "Completed"),
        1 => ("Linked Lists Fundamentals", "Wed, Jan 17\n2024", "09:00 - 10:30\n90 minutes", "Room A101\nBuilding A", "In Progress"),
        2 => ("Linked Lists Fundamentals", "Thurs, Jan 18\n2024", "09:00 - 10:30\n90 minutes", "Room A101\nBuilding A", "Upcoming"),
        3 => ("Linked Lists Fundamentals", "Fri, Jan 19\n2024", "14:00 - 15:30\n90 minutes", "Room B205\nBuilding B", "Upcoming"),
        _ => ("Hash Tables & Collision", "Wed, Jan 24\n2024", "14:00 - 15:30\n90 minutes", "Room C301\nBuilding C", "Upcoming"),
    };

    let badge_class = match status {
        "Completed" => "badge badge-green",
        "In Progress" => "badge badge-amber",
        _ => "badge badge-blue",
    };

    view! {
        <tr>
            <td><div class="t-title">{title}<div class="t-sub">"Week • Lecture"</div></div></td>
            <td>{date.replace('\n', "\n")}</td>
            <td>{time.replace('\n', "\n")}</td>
            <td>{venue.replace('\n', "\n")}</td>
            <td><span class=badge_class>{status}</span></td>
            <td style="white-space:nowrap;">
                <button attr:class="btn btn-outline btn-small">"Edit"</button>
                <button attr:class="btn btn-outline btn-small" style="margin-left:6px; color:#ef4444; border-color:#fecaca;">"Remove"</button>
            </td>
        </tr>
    }
}
