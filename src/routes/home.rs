use leptos::prelude::*;
use crate::components::{Header, ModuleCard, Calendar, ClassList, StatTile};
use crate::user_context::get_current_user;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    let current_user = get_current_user();

    // Use leptos::logging::log! which is SSR-safe (prints to server log / console.log in browser)
    leptos::logging::log!("HomePage mounted");

    // Optional reactive debug without web_sys
    Effect::new(move |_| {
        leptos::logging::log!("HomePage - current user changed: {:?}", current_user.get());
    });

    // Define greeting for BOTH targets; only browser branch touches web_sys.
    let greeting = move || -> String {
        let user = current_user.get();

        // browser-only extra logging (optional)
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsValue;
            web_sys::console::log_1(&JsValue::from_str(&format!(
                "Generating greeting for user: {:?}", user
            )));
        }

        match user {
            Some(user) => {
                let title = match user.role.as_str() {
                    "lecturer" => "Dr.",
                    "tutor" => "Mr./Ms.",
                    _ => "",
                };
                format!("Welcome back, {} {} {}", title, user.name, user.surname)
            }
            None => "Welcome back".to_string(),
        }
    };

    view! {
        <section class="home">
            // If Header.title expects a signal/closure, passing `greeting` is fine.
            // If it expects a String, use `title=greeting()`.
            <Header
                title=greeting
                subtitle="Manage your modules and schedule your classes".to_string()
            />

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
