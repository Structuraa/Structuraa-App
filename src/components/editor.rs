use dioxus::prelude::*;

#[component]
pub fn Editor() -> Element {
    rsx! {
        div {
            class: "editor",
            style: "width: 100%; height: 100%; background-color: rgb(31, 31, 31); color: #fff;",
            " " // Empty space as placeholder content
        }
    }
}
