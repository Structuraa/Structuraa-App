use dioxus::prelude::*;

#[component]
pub fn ActionBar() -> Element {
    rsx! {
        div { class: "action-bar" }
    }
}
