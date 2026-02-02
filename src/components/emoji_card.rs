use crate::data::Emoji;
use leptos::prelude::*;

#[component]
pub fn EmojiCard(
    emoji: Emoji,
    on_click: impl Fn(Emoji) + Send + Sync + Clone + 'static,
) -> impl IntoView {
    let emoji_char = emoji.emoji.clone();
    let name = emoji.annotation.clone();
    let name_for_title = name.clone();
    let emoji_for_click = emoji.clone();

    view! {
        <div
            class="emoji-card"
            on:click=move |_| on_click(emoji_for_click.clone())
            data-tooltip=name_for_title
        >
            <span class="emoji">{emoji_char}</span>
            <span class="emoji-name">{name}</span>
        </div>
    }
}
