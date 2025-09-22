use leptos::prelude::*;
use leptos::ev::{MouseEvent, KeyboardEvent};
use leptos_router::hooks::use_navigate;

#[component]
pub fn ModuleCard(
    code: &'static str,
    name: &'static str,
    desc: &'static str,
    students: u32,
    icon: &'static str,
    variant: &'static str,
    href: &'static str,
) -> impl IntoView {
    let icon_classes = format!("module-icon {}", variant);
    let navigate = use_navigate();
    let dest = href;
    let go_card = { 
        let navigate = navigate.clone();
        move |_| {
            navigate(dest, Default::default());
        }
    };
    let go_card_key = {let navigate = navigate.clone(); move |e: KeyboardEvent| {
        let k = e.key();
        if k == "Enter" || k == " " { // spacebar or Enter
            navigate(dest, Default::default());
        }
        }
    };
    let go_new_class = move |e: MouseEvent| {
        e.stop_propagation();
        e.prevent_default();
        navigate("/classes/new", Default::default());
    };
    view! {
        <div class="module-card-link" role="link" tabindex="0" on:click=go_card on:keydown=go_card_key>
            <div class="card module-card">
                <div class=icon_classes aria-hidden="true">{icon}</div>
                <div class="module-body">
                    <div class="module-code">{code}</div>
                    <div class="module-name">{name}</div>
                    <p class="module-desc">{desc}</p>
                    <div class="module-meta">
                        <span class="meta-left">
                            <span aria-hidden="true">"👥"</span>
                            <span class="muted">{students} " students"</span>
                        </span>
                        <button class="btn btn-primary btn-small" on:click=go_new_class>"+ Add Class"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
