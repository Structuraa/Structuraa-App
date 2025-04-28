use dioxus::prelude::*;

use super::editor::Editor;
use super::footer::Footer;
use super::header::Header;
use super::sidebar::Sidebar;
use super::toolbar::Toolbar;
use super::topbar::Topbar;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "app-container",
            Header {
                filename: Some("ClassDiagram.struct"),
                search_value: "",
                on_search: |value| {
                    println!("Search value: {}", value);
                },
            }
            main { class: "main-content",
                Toolbar {}
                Sidebar { structure_expanded: true, properties_expanded: false }
                div { class: "editor-container",
                    Topbar {}
                    Editor {}
                }
            }
            Footer {}
        }
    }
}
