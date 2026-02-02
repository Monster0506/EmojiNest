use crate::components::EmojiCard;
use crate::data::Emoji;
use leptos::prelude::*;

#[component]
pub fn EmojiGrid(
    emojis: ReadSignal<Vec<Emoji>>,
    on_emoji_click: impl Fn(Emoji) + Send + Sync + Clone + 'static,
) -> impl IntoView {
    view! {
        <div class="emoji-grid">
            <For
                each=move || emojis.get()
                key=|e| e.hexcode.clone()
                children=move |emoji| {
                    let callback = on_emoji_click.clone();
                    view! {
                        <EmojiCard emoji=emoji on_click=callback />
                    }
                }
            />
        </div>
    }
}
