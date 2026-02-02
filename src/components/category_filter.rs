use crate::data::CATEGORIES;
use leptos::prelude::*;

#[component]
pub fn CategoryFilter(
    selected_category: ReadSignal<String>,
    set_selected_category: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="category-buttons mb-4">
            {CATEGORIES.iter().map(|(key, label)| {
                let key = key.to_string();
                let key_clone = key.clone();
                let label = *label;
                view! {
                    <button
                        class=move || {
                            let base = "button category-btn is-small";
                            if selected_category.get() == key {
                                format!("{} is-active", base)
                            } else {
                                base.to_string()
                            }
                        }
                        on:click=move |_| set_selected_category.set(key_clone.clone())
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
