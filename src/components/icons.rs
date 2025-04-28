use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    #[props(default = 24)]
    pub size: u32,
    #[props(default = "currentColor")]
    pub color: &'static str,
    #[props(default = 2.)]
    pub stroke: f32,
    #[props]
    children: Option<Element>,
}

#[component]
pub fn ChevronDown(props: IconProps) -> Element {
    render_icon(IconProps {
        size: props.size,
        color: props.color,
        stroke: props.stroke,
        children: Some(rsx! {
            path { d: "m6 9 6 6 6-6" }
        }),
    })
}

#[component]
pub fn Search(props: IconProps) -> Element {
    render_icon(IconProps {
        size: props.size,
        color: props.color,
        stroke: props.stroke,
        children: Some(rsx! {
            circle { cx: "11", cy: "11", r: "8" }
            path { d: "m21 21-4.3-4.3" }
        }),
    })
}

#[component]
pub fn User(props: IconProps) -> Element {
    render_icon(IconProps {
        size: props.size,
        color: props.color,
        stroke: props.stroke,
        children: Some(rsx! {
            path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        }),
    })
}

#[component]
pub fn Share(props: IconProps) -> Element {
    render_icon(IconProps {
        size: props.size,
        color: props.color,
        stroke: props.stroke,
        children: Some(rsx! {
            path { d: "M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" }
            polyline { points: "16 6 12 2 8 6" }
            line {
                x1: "12",
                x2: "12",
                y1: "2",
                y2: "15",
            }
        }),
    })
}

#[component]
pub fn Link(props: IconProps) -> Element {
    render_icon(IconProps {
        size: props.size,
        color: props.color,
        stroke: props.stroke,
        children: Some(rsx! {
            path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
            path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
        }),
    })
}

fn render_icon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: props.size,
            height: props.size,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: props.color,
            stroke_width: props.stroke,
            stroke_linecap: "round",
            stroke_linejoin: "round",

            {props.children.unwrap_or_else(|| rsx! {})}
        }
    }
}
