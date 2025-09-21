use leptos::prelude::*;
use crate::components::{Header, ModuleCard, Calendar, ClassList, StatTile};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="home">
            <Header title="Welcome back, Dr. Gerber" subtitle="Manage your modules and schedule your classes"/>

            <div class="dashboard-grid">
                <div>
                    <h3 class="heading">"Your Modules"</h3>
                    <div class="modules-grid">
                        <ModuleCard code="CS112" name="Introduction to Programming" desc="Basics of programming in Rust" students=120/>
                        <ModuleCard code="CS301" name="Data Structures & Algorithms" desc="Complexity, trees, graphs" students=156/>
                        <ModuleCard code="CS305" name="Computer Networks" desc="Layers, protocols, security" students=67/>
                        <ModuleCard code="CS410" name="Artificial Intelligence" desc="Search, optimization, ML" students=43/>
                    </div>

                    <div class="stats-row" style="margin-top:16px;">
                        <StatTile value="315" label="Total Students"/>
                        <StatTile value="12" label="Classes This Week"/>
                        <StatTile value="24h" label="Teaching Hours"/>
                    </div>
                </div>

                <aside class="schedule-panel">
                    <h3 class="heading">"Schedule"</h3>
                    <Calendar/>
                    <h3 class="heading">"Today's Classes"</h3>
                    <ClassList/>
                </aside>
            </div>
        </section>
    }
}
