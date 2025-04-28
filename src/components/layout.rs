use dioxus::prelude::*;

use super::editor::Editor;
use super::header::Header;
use super::sidebar::Sidebar;
use super::toolbar::Toolbar;

#[component]
pub fn Layout() -> Element {
    rsx! {
        Header {
            filename: Some("Untitled.struct"),
            search_value: "Search",
            on_search: |value| {
                println!("Search value: {}", value);
            },
        }
        main {
            Toolbar {}
            Sidebar {
                structure_expanded: true,
                properties_expanded: true,
                on_toggle_structure: |_| {
                    println!("Toggle structure");
                },
                on_toggle_properties: |_| {
                    println!("Toggle properties");
                },
            }
            Editor {}
        }
        footer {
            class: "footer",
            p {
                "Structuraa"
            }
        }
    }
}
