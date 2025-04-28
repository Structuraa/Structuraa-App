use dioxus::prelude::*;

mod components;

use components::layout::Layout;

fn main() {
    dioxus::launch(App)
}

#[component]
fn App() -> Element {
    rsx! {
          document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styles.scss")
        }
        Layout {}
    }
}
