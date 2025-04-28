use dioxus::prelude::*;

#[component]
pub fn Toolbar() -> Element {
    rsx! {
        div { class: "toolbar",

            // Home button
            button { class: "toolbar-button",
                span { "🏠" }
            }

            // Plus button
            button { class: "toolbar-button",
                span { "➕" }
            }

            // Class button
            button { class: "toolbar-button",
                span { "📋" }
            }

            // Settings button
            button { class: "toolbar-button",
                span { "⚙️" }
            }
        }
    }
}
