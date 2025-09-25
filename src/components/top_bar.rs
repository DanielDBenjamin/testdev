use leptos::prelude::*;
use leptos_router::components::A;


#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <header class="topbar" role="banner">
            <div class="topbar-left">
                <A href="/home" attr:class="brand" attr:aria-label="Go to home">
                    <img src="/logo.png" srcset="/logo.png 1x" alt="Clock It" class="brand-logo-img" width="160" height="60" />
                </A>
            </div>
            <div class="topbar-right">
                <A
                    href="/lecturer/profile"
                    attr:class="user-chip"
                    attr:aria-label="Open profile settings"
                >
                    <span class="avatar" aria-hidden="true">"👩🏻‍🏫"</span>
                    <span class="name">"Jane Gerber"</span>
                </A>
            </div>
        </header>
    }
}
