use dioxus::prelude::*;

// Icon background color options in RGB format
const ICON_COLORS: [&str; 7] = [
    "#3574f0", // Blue
    "#5fad65", // Green
    "#f2c55c", // Yellow
    "#db5c5c", // Red
    "#e08855", // Orange
    "#955ae0", // Purple
    "#24a394", // Teal
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
        div {
            class: "header-container",
            style: "display: flex; align-items: center; width: 100%; height: 44px; padding: 6px; background-color: #1e1e1e; border-bottom: 1px solid #333;",

            // File control section
            div {
                class: "file-control",
                style: "display: flex; align-items: center; padding-left: 6px; gap: 6px;",

                // File icon with first letter
                div {
                    class: "file-icon",
                    style: "display: flex; align-items: center; justify-content: center; width: 20px; height: 20px; background-color: {icon_color}; border-radius: 4px; color: white; font-weight: 600; font-size: 12px;",
                    "{first_letter}"
                }

                // Filename
                span {
                    class: "filename",
                    style: "font-size: 12px; font-weight: 500;",
                    "{filename}"
                }

                // Chevron button
                button {
                    class: "chevron-button",
                    style: "background: none; border: none; color: #999; padding: 2px; cursor: pointer;",
                    "▼"
                }
            }

            // Separator
            div {
                class: "separator",
                style: "height: 20px; width: 1px; background-color: #333; margin: 0 8px;"
            }

            // Search container
            div {
                class: "search-container",
                style: "display: flex; align-items: center; background-color: #2e2e2e; border-radius: 4px; padding-left: 10px; max-width: 250px; height: 36px; flex-grow: 1;",

                // Search icon
                span {
                    class: "search-icon",
                    style: "color: #999;",
                    "🔍"
                }

                // Search input
                input {
                    class: "search-input",
                    style: "background: transparent; border: none; color: #999; padding: 6px; font-size: 16px; width: 100%; outline: none;",
                    placeholder: "Search",
                    value: "{props.search_value}",
                    oninput: move |evt| {
                        props.on_search.call(evt.value().clone());
                    }
                }
            }
        }
    }
}
