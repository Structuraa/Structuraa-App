use dioxus::prelude::*;

#[component]
pub fn Topbar() -> Element {
    rsx! {
        div { class: "topbar" }
    }
}
