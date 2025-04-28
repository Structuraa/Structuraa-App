use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    #[props(default = false)]
    structure_expanded: bool,
    #[props(default = false)]
    properties_expanded: bool,
    #[props(default = Callback::new(|()| {
        println!("Toggle structure");
    }))]
    on_toggle_structure: Callback<()>,
    #[props(default = Callback::new(|()| {
        println!("Toggle properties");
    }))]
    on_toggle_properties: Callback<()>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let structure_icon = if props.structure_expanded {
        "▼"
    } else {
        "▶"
    };
    let properties_icon = if props.properties_expanded {
        "▼"
    } else {
        "▶"
    };

    rsx! {
        div {
            class: "sidebar",
            style: "width: 256px; height: 100%; background-color: #1e1e1e; color: #fff;",

            // Structure section
            div {
                class: "sidebar-section",

                // Structure header
                button {
                    class: "sidebar-header",
                    style: "display: flex; align-items: center; width: 100%; padding: 8px; background: none; border: none; color: #fff; text-align: left; cursor: pointer;",
                    onclick: move |_| props.on_toggle_structure.call(()),

                    span {
                        class: "sidebar-icon",
                        style: "margin-right: 10px; font-size: 14px;",
                        "{structure_icon}"
                    }
                    span {
                        class: "sidebar-title",
                        style: "font-size: 14px;",
                        "Structure"
                    }
                }

                // Structure content
                {
                    if props.structure_expanded {
                        rsx! {
                            div {
                                class: "sidebar-content",
                                style: "padding: 8px; height: 400px;",
                                "Structure content would go here"
                            }
                        }
                    } else {
                        rsx! { "" }
                    }
                }
            }

            // Properties section
            div {
                class: "sidebar-section",

                // Properties header
                button {
                    class: "sidebar-header",
                    style: "display: flex; align-items: center; width: 100%; padding: 8px; background: none; border: none; color: #fff; text-align: left; cursor: pointer;",
                    onclick: move |_| props.on_toggle_properties.call(()),

                    span {
                        class: "sidebar-icon",
                        style: "margin-right: 10px; font-size: 14px;",
                        "{properties_icon}"
                    }
                    span {
                        class: "sidebar-title",
                        style: "font-size: 14px;",
                        "Properties"
                    }
                }

                // Properties content
                {
                    if props.properties_expanded {
                        rsx! {
                            div {
                                class: "sidebar-content",
                                style: "padding: 8px;",
                                "Properties content would go here"
                            }
                        }
                    } else {
                        rsx! { "" }
                    }
                }
            }
        }
    }
}
