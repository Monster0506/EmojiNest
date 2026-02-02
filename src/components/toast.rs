use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: u32,
    pub emoji: String,
    pub message: String,
    pub exiting: bool,
}

#[component]
pub fn ToastContainer(toasts: ReadSignal<Vec<Toast>>) -> impl IntoView {
    view! {
        <div class="toast-container">
            <For
                each=move || toasts.get()
                key=|t| t.id
                children=move |toast| {
                    let class = if toast.exiting {
                        "toast exiting"
                    } else {
                        "toast"
                    };
                    view! {
                        <div class=class>
                            <span class="toast-emoji">{toast.emoji}</span>
                            <span>{toast.message}</span>
                        </div>
                    }
                }
            />
        </div>
    }
}
