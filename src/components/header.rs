use dioxus::prelude::*;

use crate::components::icons;

const ICON_COLORS: [&str; 7] = [
    "#3574f0", "#5fad65", "#f2c55c", "#db5c5c", "#e08855", "#955ae0", "#24a394",
];

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    #[props(!optional)]
    filename: Option<&'static str>,
    search_value: &'static str,
    on_search: EventHandler<String>,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let filename = props.filename.unwrap_or("Untitled.struct");

    // Choose a color for icon background based on filename
    let color_index =
        filename.chars().map(|c| c as usize).sum::<usize>() % ICON_COLORS.len();
    let icon_color = ICON_COLORS[color_index];

    // Get first letter of filename for icon
    let first_letter = filename
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    rsx! {
        header { class: "header-container",
            div { class: "header-left",
                div { class: "file-control",
                    div {
                        class: "file-icon",
                        style: "background-color: {icon_color};",
                        "{first_letter}"
                    }

                    span { class: "filename", "{filename}" }

                    button { class: "chevron-button",
                        icons::ChevronDown { size: 16 }
                    }
                }

                div { class: "separator" }

                div { class: "search-container",
                    span { class: "search-icon",
                        icons::Search { size: 16 }
                    }

                    input {
                        class: "search-input",
                        placeholder: "Search",
                        value: "{props.search_value}",
                        oninput: move |evt| {
                            props.on_search.call(evt.value().clone());
                        },
                    }
                }
            }

            div { class: "header-right",
                button { class: "share-button",
                    span { class: "share-icon",
                        icons::Share { size: 14 }
                        "Share"
                    }

                    span { class: "link-icon",
                        icons::Link { size: 16, stroke: 1.5 }
                    }
                }

                button { class: "user-button",
                    icons::User { size: 24, stroke: 1. }
                }
            }
        }
    }
}
