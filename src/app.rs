use leptos::prelude::*;
use leptos_router::StaticSegment;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_location,
};
use crate::routes::{Error, HomePage, Login, Statistics, Timetable, About, Register, ClassesPage, NewClass, NewModule};
use crate::components::{NavBar, TopBar};

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
        path.starts_with("/home")
            || path.starts_with("/timetable")
            || path.starts_with("/statistics")
            || path.starts_with("/about")
            || path.starts_with("/classes")
            || path.starts_with("/modules")
    });
    let show_footer = Signal::derive(move || show_sidebar.get());
    let shell_class = Signal::derive(move || {
        if show_sidebar.get() { "app-shell".to_string() } else { "app-shell no-sidebar".to_string() }
    });
    view! {
    <div class=move || shell_class.get()>
            <Show when=move || show_sidebar.get()>
                <TopBar/>
                <NavBar/>
            </Show>
            <main class="content">
                <Routes fallback=|| view! { <Error/> }>
                    <Route path=StaticSegment("") view=Login/>
                    <Route path=StaticSegment("register") view=Register/>
                    <Route path=StaticSegment("home") view=HomePage/>
                    <Route path=StaticSegment("classes") view=ClassesPage/>
                    <Route path=(StaticSegment("classes"), StaticSegment("new")) view=NewClass/>
                    <Route path=(StaticSegment("modules"), StaticSegment("new")) view=NewModule/>
                    <Route path=StaticSegment("timetable") view=Timetable/>
                    <Route path=StaticSegment("statistics") view=Statistics/>
                    <Route path=StaticSegment("about") view=About/>
                </Routes>
            </main>
            <Show when=move || show_footer.get()>
                <footer class="footer">
                    <small>"Built with Leptos"</small>
                </footer>
            </Show>
        </div>
    }
}
