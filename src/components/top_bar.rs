use leptos::prelude::*;

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <header class="topbar" role="banner">
            <div class="topbar-left">
                <div class="brand">"clock it"</div>
            </div>
            <div class="topbar-right">
                <div class="user-chip">
                    <span class="avatar" aria-hidden="true">"👩🏻‍🏫"</span>
                    <span class="name">"Jane Gerber"</span>
                </div>
            </div>
        </header>
    }
}

