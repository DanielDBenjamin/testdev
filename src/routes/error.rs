use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Error() -> impl IntoView {
    view! {
        <section class="error-page" role="alert" aria-labelledby="e-title">
            <div class="error-wrap">
                <div class="error-hero" aria-hidden="true">
                    <span class="e-digit">"4"</span>
                    <span class="e-clock" title="Clock">
                        <svg viewBox="0 0 64 64" width="112" height="112" xmlns="http://www.w3.org/2000/svg">
                            <circle cx="32" cy="32" r="28" fill="none" stroke="#0f172a" stroke-width="4"/>
                            <circle cx="32" cy="18" r="2" fill="#0f172a"/>
                            <circle cx="32" cy="46" r="2" fill="#0f172a"/>
                            <circle cx="18" cy="32" r="2" fill="#0f172a"/>
                            <circle cx="46" cy="32" r="2" fill="#0f172a"/>
                            <path d="M32 32 L32 22" stroke="#0f172a" stroke-width="4" stroke-linecap="round"/>
                            <path d="M32 32 L42 32" stroke="#0f172a" stroke-width="4" stroke-linecap="round"/>
                        </svg>
                    </span>
                    <span class="e-digit">"4"</span>
                </div>

                <h1 id="e-title" class="error-title">"clocked out"</h1>
                <p class="error-sub">"The page you’re looking for doesn’t exist..."</p>

                <A href="/home" attr:class="error-btn">"Return Home"</A>
            </div>
        </section>
    }
}
