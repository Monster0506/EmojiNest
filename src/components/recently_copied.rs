use crate::data::Emoji;
use leptos::prelude::*;

#[component]
pub fn RecentlyCopied(
    emojis: ReadSignal<Vec<Emoji>>,
    on_click: impl Fn(Emoji) + Send + Sync + Clone + 'static,
) -> impl IntoView {
    let on_click = std::sync::Arc::new(on_click);

    view! {
        <Show when=move || !emojis.get().is_empty()>
            {
                let on_click = on_click.clone();
                view! {
                    <div class="box mt-4">
                        <p class="is-size-7 has-text-grey mb-2">
                            <strong>"Recently Copied"</strong>
                        </p>
                        <div class="recently-copied">
                            <For
                                each=move || emojis.get()
                                key=|e| e.hexcode.clone()
                                children={
                                    let on_click = on_click.clone();
                                    move |emoji| {
                                        let on_click = on_click.clone();
                                        let emoji_char = emoji.emoji.clone();
                                        let emoji_for_click = emoji.clone();
                                        let title = emoji.annotation.clone();
                                        view! {
                                            <span
                                                class="emoji-item"
                                                title=title
                                                on:click=move |_| on_click(emoji_for_click.clone())
                                            >
                                                {emoji_char}
                                            </span>
                                        }
                                    }
                                }
                            />
                        </div>
                    </div>
                }
            }
        </Show>
    }
}
