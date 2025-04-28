use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    #[props(default = true)]
    structure_expanded: bool,
    #[props(default = false)]
    properties_expanded: bool,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let mut structure_expanded = use_signal(|| props.structure_expanded);
    let mut properties_expanded = use_signal(|| props.properties_expanded);

    let structure_icon = if structure_expanded() { "▼" } else { "▶" };
    let properties_icon = if properties_expanded() { "▼" } else { "▶" };

    let mut on_toggle_structure = move |_| {
        structure_expanded.set(!structure_expanded());
        tracing::info!("Toggle structure");
    };
    let mut on_toggle_properties = move |_| {
        properties_expanded.set(!properties_expanded());
        tracing::info!("Toggle properties");
    };

    rsx! {
        div { class: "sidebar",

            // Structure section
            div { class: "sidebar-section",

                // Structure header
                button {
                    class: "sidebar-header",
                    onclick: move |_| on_toggle_structure(()),

                    span { class: "sidebar-icon", "{structure_icon}" }
                    span { class: "sidebar-title", "Structure" }
                }

                // Structure content
                div {
                    class: "sidebar-content",
                    display: if props.structure_expanded { "block" } else { "none" },
                    "Structure content would go here"
                }
            }

            // Properties section
            div { class: "sidebar-section",

                // Properties header
                button {
                    class: "sidebar-header",
                    onclick: move |_| on_toggle_properties(()),

                    span { class: "sidebar-icon", "{properties_icon}" }
                    span { class: "sidebar-title", "Properties" }
                }

                // Properties content
                div {
                    class: "sidebar-content",
                    display: if props.properties_expanded { "block" } else { "none" },
                    "Properties content would go here"
                }
            }
        }
    }
}
