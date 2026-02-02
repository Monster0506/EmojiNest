use crate::components::{
    CategoryFilter, EmojiGrid, RecentlyCopied, SearchBar, Toast, ToastContainer,
};
use crate::data::{Emoji, fetch_emojis};
use leptos::prelude::*;

fn copy_to_clipboard(text: &str) {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();
    let _ = clipboard.write_text(text);
}

fn load_recent_from_storage() -> Vec<Emoji> {
    let window = web_sys::window().unwrap();
    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(json)) = storage.get_item("recently_copied") {
            return serde_json::from_str(&json).unwrap_or_default();
        }
    }
    vec![]
}

fn save_recent_to_storage(emojis: &[Emoji]) {
    let window = web_sys::window().unwrap();
    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(json) = serde_json::to_string(emojis) {
            let _ = storage.set_item("recently_copied", &json);
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (all_emojis, set_all_emojis) = signal(Vec::<Emoji>::new());
    let (is_loading, set_is_loading) = signal(true);
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal("smileys-emotion".to_string());
    let (recently_copied, set_recently_copied) = signal(load_recent_from_storage());
    let (toasts, set_toasts) = signal(Vec::<Toast>::new());
    let (toast_id, set_toast_id) = signal(0u32);

    // Fetch emojis on mount
    Effect::new(move || {
        wasm_bindgen_futures::spawn_local(async move {
            let emojis = fetch_emojis().await;
            set_all_emojis.set(emojis);
            set_is_loading.set(false);

            gloo_timers::future::TimeoutFuture::new(500).await;
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(loading) = document.get_element_by_id("loading-screen") {
                        let _ = loading.remove();
                    }
                }
            }
        });
    });

    // Keyboard shortcuts
    Effect::new(move || {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::prelude::*;

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let closure =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
                // Don't trigger if typing in an input
                if let Some(target) = event.target() {
                    if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                        let tag = element.tag_name().to_lowercase();
                        if tag == "input" || tag == "textarea" {
                            return;
                        }
                    }
                }

                let key = event.key();
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();

                match key.as_str() {
                    "s" => {
                        event.prevent_default();
                        if let Some(input) = document.get_element_by_id("search-input") {
                            if let Some(input) = input.dyn_ref::<web_sys::HtmlInputElement>() {
                                let _ = input.focus();
                            }
                        }
                    }
                    "r" => {
                        if let Some(recent) = document.get_element_by_id("recently-copied") {
                            recent.scroll_into_view();
                        }
                    }
                    _ => {}
                }
            });

        let _ =
            document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();
    });

    let filtered_emojis = Memo::new(move |_| {
        let all = all_emojis.get();
        let query = search_query.get().to_lowercase();
        let category = selected_category.get();
        all.iter()
            .filter(|e| {
                let matches_category = category == "all" || e.group == category;
                let matches_search = query.is_empty()
                    || e.annotation.to_lowercase().contains(&query)
                    || e.tags.to_lowercase().contains(&query);
                matches_category && matches_search
            })
            .take(200)
            .cloned()
            .collect::<Vec<_>>()
    });

    let (display_emojis, set_display_emojis) = signal(Vec::<Emoji>::new());
    Effect::new(move || {
        set_display_emojis.set(filtered_emojis.get());
    });

    let handle_emoji_click = move |emoji: Emoji| {
        copy_to_clipboard(&emoji.emoji);

        let mut recent = recently_copied.get();
        recent.retain(|e| e.hexcode != emoji.hexcode);
        recent.insert(0, emoji.clone());
        recent.truncate(25);
        save_recent_to_storage(&recent);
        set_recently_copied.set(recent);

        // Show toast
        let id = toast_id.get();
        set_toast_id.set(id + 1);
        let new_toast = Toast {
            id,
            emoji: emoji.emoji.clone(),
            message: "Copied!".to_string(),
            exiting: false,
        };
        set_toasts.update(|t| t.push(new_toast));

        // Auto-dismiss after 2 seconds
        let set_toasts_clone = set_toasts;
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(1700).await;
            set_toasts_clone.update(|t| {
                if let Some(toast) = t.iter_mut().find(|x| x.id == id) {
                    toast.exiting = true;
                }
            });
            gloo_timers::future::TimeoutFuture::new(300).await;
            set_toasts_clone.update(|t| t.retain(|x| x.id != id));
        });
    };

    let on_recent_click = handle_emoji_click.clone();

    view! {
        <section class="hero is-medium">
            <div class="hero-body">
                <div class="container">
                    <h1 class="title is-1 has-text-centered app-title">
                        "🪺 EmojiNest"
                    </h1>
                    <p class="subtitle has-text-centered has-text-white-ter">
                        "Find the perfect emoji instantly"
                    </p>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="container">
                <div class="box main-card">
                    <SearchBar search_query=search_query set_search_query=set_search_query />
                    <CategoryFilter
                        selected_category=selected_category
                        set_selected_category=set_selected_category
                    />

                    <Show
                        when=move || !is_loading.get()
                        fallback=|| view! {
                            <div class="has-text-centered py-6">
                                <p class="is-size-4">"🔄 Loading emojis..."</p>
                            </div>
                        }
                    >
                        <EmojiGrid
                            emojis=display_emojis
                            on_emoji_click=handle_emoji_click.clone()
                        />
                    </Show>

                    <RecentlyCopied emojis=recently_copied on_click=on_recent_click />
                </div>
            </div>
        </section>

        <ToastContainer toasts=toasts />
    }
}
