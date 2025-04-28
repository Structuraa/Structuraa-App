use dioxus::prelude::*;

#[component]
pub fn Toolbar() -> Element {
    rsx! {
        div {
            class: "toolbar",
            style: "width: 50px; height: 100%; display: flex; flex-direction: column; align-items: center; padding: 10px 0; background-color: #1e1e1e;",

            // Home button
            button {
                class: "toolbar-button",
                style: "width: 44px; height: 44px; margin-bottom: 10px; background: none; border: none; color: #fff; cursor: pointer; display: flex; justify-content: center; align-items: center;",
                // Icon would go here - using placeholder
                span {
                    style: "font-size: 24px;",
                    "🏠"
                }
            }

            // Plus button
            button {
                class: "toolbar-button",
                style: "width: 44px; height: 44px; margin-bottom: 10px; background: none; border: none; color: #fff; cursor: pointer; display: flex; justify-content: center; align-items: center;",
                span {
                    style: "font-size: 24px;",
                    "➕"
                }
            }

            // Class button
            button {
                class: "toolbar-button",
                style: "width: 44px; height: 44px; margin-bottom: 10px; background: none; border: none; color: #fff; cursor: pointer; display: flex; justify-content: center; align-items: center;",
                span {
                    style: "font-size: 24px;",
                    "📋"
                }
            }

            // Settings button
            button {
                class: "toolbar-button",
                style: "width: 44px; height: 44px; margin-bottom: 10px; background: none; border: none; color: #fff; cursor: pointer; display: flex; justify-content: center; align-items: center;",
                span {
                    style: "font-size: 24px;",
                    "⚙️"
                }
            }
        }
    }
}
