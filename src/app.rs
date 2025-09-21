use crate::components::NavBar;
use crate::routes::About;
use crate::routes::HomePage;  
use crate::routes::Statistics;
use crate::routes::Timetable;
use crate::routes::Login;
use crate::routes::Register;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
    hooks::use_location,
};
use std::thread::Scope;
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/clock-it.css"/>
        <Title text="Clock-It"/>
        <Router>
            <AppShell/>
        </Router>
    }
}

#[component]
fn AppShell() -> impl IntoView {
    let location = use_location();
    let show_sidebar = Signal::derive(move || {
        let path = location.pathname.get();
        !(path == "/login" || path == "/register")
    });
    view! {
        <div class="app-shell">
            <Show when=move || show_sidebar.get()>
                <NavBar/>
            </Show>
            <main class="content">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("home") view=HomePage/>
                    <Route path=StaticSegment("timetable") view=Timetable/>
                    <Route path=StaticSegment("statistics") view=Statistics/>
                    <Route path=StaticSegment("about") view=About/>
                    <Route path=StaticSegment("") view=Login/>
                    <Route path=StaticSegment("register") view=Register/>
                </Routes>
            </main>
            <footer class="footer">
                <small>"Built with Leptos"</small>
            </footer>
        </div>
    }
}
