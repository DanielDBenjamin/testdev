use leptos::prelude::*;
use crate::database::classes::Class;

#[component]
pub fn ClassList(
    #[prop(into)] classes: Signal<Vec<Class>>,
) -> impl IntoView {
    view! {
        <div class="class-list-container">
            <Show
                when=move || !classes.get().is_empty()
                fallback=|| view! {
                    <p class="no-classes">"No classes scheduled for this day"</p>
                }
            >
                <ul class="class-list">
                    {move || classes.get().into_iter().map(|class| {
                        let color = match class.module_code.chars().next() {
                            Some('C') if class.module_code.contains("S1") => "dot-purple",
                            Some('C') if class.module_code.contains("S3") => "dot-blue",
                            _ => "dot-green",
                        };
                        
                        view! {
                            <li class="class-item">
                                <span class=format!("dot {}", color) aria-hidden="true"></span>
                                <div class="class-info">
                                    <div class="class-title">{class.title.clone()}</div>
                                    <div class="class-sub">
                                        {class.time.clone()} 
                                        {class.venue.as_ref().map(|v| format!(" – {}", v)).unwrap_or_default()}
                                    </div>
                                </div>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </Show>
        </div>
    }
}

