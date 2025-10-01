use leptos::prelude::*;
use leptos::ev::{MouseEvent, KeyboardEvent};
use leptos_router::hooks::use_navigate;

#[component]
pub fn ModuleCard(
    code: String,
    name: String,
    desc: String,
    students: i32,
    class_count: i32,
    module_code: String,
) -> impl IntoView {
    let navigate = use_navigate();
    
    // Generate icon and color based on module code hash
    let hash = module_code.chars().map(|c| c as u32).sum::<u32>();
    let (icon, bg_color, icon_color) = match hash % 4 {
        0 => ("</>" , "bg-purple-100", "text-purple-600"),
        1 => ("🗄️", "bg-blue-100", "text-blue-600"),
        2 => ("🧩", "bg-orange-100", "text-orange-600"),
        _ => ("🍃", "bg-green-100", "text-green-600"),
    };
    
    let href = format!("/classes?module={}", module_code);
    
    let go_card = {
        let href = href.clone();
        let navigate = navigate.clone();
        move |_: MouseEvent| {
            navigate(&href, Default::default());
        }
    };
    
    let go_card_key = {
        let href = href.clone();
        let navigate = navigate.clone();
        move |e: KeyboardEvent| {
            let k = e.key();
            if k == "Enter" || k == " " {
                e.prevent_default();
                navigate(&href, Default::default());
            }
        }
    };
    
    let go_new_class = {
        let module_code = module_code.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
            navigate(&format!("/classes/new?module={}", module_code), Default::default());
        }
    };

    view! {
        <div 
            class="group cursor-pointer bg-white border border-gray-200 rounded-2xl p-4 transition-all duration-200 hover:shadow-lg hover:border-blue-300 focus-within:ring-2 focus-within:ring-blue-500 focus-within:ring-offset-2"
            role="button"
            tabindex="0"
            on:click=go_card
            on:keydown=go_card_key
        >
            <div class="flex gap-4 h-full">
                // Icon
                <div class={format!("w-16 h-16 rounded-xl {} {} flex items-center justify-center text-2xl flex-shrink-0", bg_color, icon_color)}>
                    {icon}
                </div>
                
                // Content
                <div class="flex-1 min-w-0 flex flex-col">
                    // Module code and title
                    <div class="mb-2">
                        <div class="font-extrabold text-sm text-gray-900">{code}</div>
                        <div class="font-bold text-base text-gray-900 leading-tight">{name}</div>
                    </div>
                    
                    // Description with line clamp
                    <p class="text-sm text-gray-600 mb-3 line-clamp-3 flex-1">
                        {if desc.is_empty() { 
                            "No description available".to_string() 
                        } else { 
                            desc 
                        }}
                    </p>
                    
                    // Footer with stats and button
                    <div class="flex items-center justify-between mt-auto pt-2">
                        <div class="flex items-center gap-1 text-sm text-gray-500">
                            <span>"👥"</span>
                            <span>{students} " students"</span>
                        </div>
                        <button 
                            class="px-3 py-1.5 bg-blue-600 text-white text-sm font-bold rounded-lg hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                            on:click=go_new_class
                        >
                            "+ Add Class"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}