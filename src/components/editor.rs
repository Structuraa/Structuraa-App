use dioxus::prelude::*;

use crate::components::actionbar::ActionBar;

#[component]
pub fn Editor() -> Element {
    rsx! {
        div { class: "editor", ActionBar {} }
    }
}
