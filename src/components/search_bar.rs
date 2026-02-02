use leptos::prelude::*;

#[component]
pub fn SearchBar(
    search_query: ReadSignal<String>,
    set_search_query: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="search-container mb-4">
            <input
                class="input is-large search-input"
                type="text"
                placeholder="Search emojis..."
                prop:value=search_query
                on:input=move |ev| {
                    set_search_query.set(event_target_value(&ev));
                }
            />
        </div>
    }
}
