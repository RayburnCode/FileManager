use crate::{components::Search, Route};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-white shadow-sm border-b",
            div { class: "max-w-6xl mx-auto px-6 py-4",
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center space-x-8",
                        Link {
                            to: Route::Home {},
                            class: "text-xl font-bold text-gray-900 hover:text-blue-600 transition-colors",
                            "🗂️ File Manager"
                        }
                        div { class: "flex space-x-6",
                            Link {
                                to: Route::Home {},
                                class: "text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md transition-colors",
                                "Home"
                            }
                            Link {
                                to: Route::FileManager {},
                                class: "text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md transition-colors",
                                "Browse Files"
                            }
                            Search {
                                value: "".to_string(),
                                on_change: move |query| {
                                    println!("Search query: {}", query);
                                },
                                on_submit: move |query| {
                                    println!("Search submitted: {}", query);
                                },
                            }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
