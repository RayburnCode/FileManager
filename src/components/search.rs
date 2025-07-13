use dioxus::prelude::*;

#[component]
pub fn Search(
    /// The current search query value
    #[props(default = "".to_string())]
    value: String,
    /// Callback triggered when the search input changes
    on_change: EventHandler<String>,
    /// Callback triggered when the form is submitted
    on_submit: EventHandler<String>,
) -> Element {
    rsx! {
        form {
            action: "#",
            method: "GET",
            onsubmit: {
                let value_clone = value.clone();
                move |evt: dioxus::events::FormEvent| {
                    evt.prevent_default();
                    on_submit.call(value_clone.clone());
                }
            },

            label { class: "sr-only", r#for: "topbar-search", "Search" }

            div { class: "relative mt-1 lg:w-96",
                div { class: "flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none",
                    svg {
                        class: "w-4 h-4 text-gray-500 dark:text-gray-400",
                        fill: "none",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                        }
                    }
                }

                input {
                    class: "bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full pl-9 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500",
                    id: "topbar-search",
                    name: "search",
                    placeholder: "Search",
                    r#type: "text",
                    value: "{value}",
                    oninput: move |evt| on_change.call(evt.value().clone()),
                }
            }
        }
    }
}