use leptos::prelude::*;
use crate::components::{Header, ModuleCard, Calendar, ClassList, StatTile};
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="home">
            <Header title="Welcome back, Dr. Gerber" subtitle="Manage your modules and schedule your classes"/>

            <div class="dashboard-grid">
                <div>
                    <div class="add-module-row">
                        <h3 class="heading">"Your Modules"</h3>
                        <A href="/modules/new" attr:class="btn btn-outline btn-small">"+ Add Module"</A>
                    </div>
                    <div class="modules-grid">
                        <ModuleCard code="CS112" name="Introduction to Programming" desc="Basics of programming in Rust" students=120 icon="</>" variant="mod-purp" href="/classes"/>
                        <ModuleCard code="CS301" name="Data Structures & Algorithms" desc="Complexity, trees, graphs" students=156 icon="🗄️" variant="mod-blue" href="/classes"/>
                        <ModuleCard code="CS305" name="Computer Networks" desc="Layers, protocols, security" students=67 icon="🧩" variant="mod-orange" href="/classes"/>
                        <ModuleCard code="CS410" name="Artificial Intelligence" desc="Search, optimization, ML" students=43 icon="🍃" variant="mod-green" href="/classes"/>
                    </div>

                    <div class="stats-row" style="margin-top:16px;">
                        <StatTile value="315" label="Total Students"/>
                        <StatTile value="12" label="Classes This Week"/>
                        <StatTile value="24h" label="Teaching Hours"/>
                    </div>
                </div>

                <aside class="schedule-panel">
                    <div class="heading">
                        <span>"Schedule"</span>
                        <button class="kebab" aria-label="Menu">"⋯"</button>
                    </div>
                    <Calendar/>
                    <h3 class="heading">"Today's Classes"</h3>
                    <ClassList/>
                </aside>
            </div>
        </section>
    }
}
