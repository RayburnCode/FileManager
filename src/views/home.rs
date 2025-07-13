use crate::Route;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100",
            div { class: "max-w-4xl mx-auto px-6 py-16",
                div { class: "text-center",
                    h1 { class: "text-4xl font-bold text-gray-900 mb-4", 
                        "🗂️ Rust File Manager" 
                    }
                    p { class: "text-xl text-gray-600 mb-8", 
                        "A modern, fast, and intuitive file manager built with Rust and Dioxus" 
                    }
                    
                    div { class: "grid md:grid-cols-2 gap-8 mt-12",
                        div { class: "bg-white rounded-lg shadow-lg p-6",
                            div { class: "text-3xl mb-4", "⚡" }
                            h3 { class: "text-xl font-semibold mb-2", "Lightning Fast" }
                            p { class: "text-gray-600", "Built with Rust for maximum performance and reliability" }
                        }
                        div { class: "bg-white rounded-lg shadow-lg p-6",
                            div { class: "text-3xl mb-4", "🎨" }
                            h3 { class: "text-xl font-semibold mb-2", "Modern UI" }
                            p { class: "text-gray-600", "Clean and intuitive interface with Tailwind CSS" }
                        }
                        div { class: "bg-white rounded-lg shadow-lg p-6",
                            div { class: "text-3xl mb-4", "🔒" }
                            h3 { class: "text-xl font-semibold mb-2", "Secure" }
                            p { class: "text-gray-600", "Memory-safe file operations powered by Rust" }
                        }
                        div { class: "bg-white rounded-lg shadow-lg p-6",
                            div { class: "text-3xl mb-4", "🚀" }
                            h3 { class: "text-xl font-semibold mb-2", "Cross-Platform" }
                            p { class: "text-gray-600", "Works seamlessly across different operating systems" }
                        }
                    }
                    
                    div { class: "mt-12",
                        Link {
                            to: Route::FileManager {},
                            class: "inline-flex items-center px-8 py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 transition-colors shadow-lg",
                            "Start Browsing Files →"
                        }
                    }
                }
            }
        }
    }
}
