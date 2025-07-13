use dioxus::prelude::*;

#[derive(Clone, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: Option<u64>,
}

#[component]
pub fn AddFile(
    /// Whether to allow multiple file selection
    #[props(default = false)]
    multiple: bool,
    /// File type filter (e.g., ".jpg,.png,.gif" or "image/*")
    #[props(default = None)]
    accept: Option<String>,
    /// Event handler for when files are selected
    on_change: EventHandler<Vec<FileInfo>>,
) -> Element {
    let mut selected_files = use_signal(Vec::<FileInfo>::new);
    
    rsx! {
        div { class: "w-full space-y-4",
            // File input section
            div { class: "border-2 border-dashed border-gray-300 rounded-lg p-6 text-center hover:border-gray-400 transition-colors",
                label { class: "cursor-pointer", r#for: "file_input",
                    div { class: "space-y-2",
                        div { class: "text-gray-600", "📁 Click to select files" }
                        div { class: "text-sm text-gray-500",
                            if multiple {
                                "You can select multiple files"
                            } else {
                                "Select a single file"
                            }
                        }
                    }
                }
                input {
                    class: "hidden",
                    id: "file_input",
                    r#type: "file",
                    multiple,
                    accept: accept.as_deref().unwrap_or(""),
                    onchange: move |evt| {
                        if let Some(file_engine) = evt.files() {
                            let files = file_engine.files();
                            let file_infos: Vec<FileInfo> = files
                                .iter()
                                .map(|file_name| {
                                    FileInfo {
                                        name: file_name.clone(),
                                        size: None,
                                    }
                                })
                                .collect();
                            selected_files.set(file_infos.clone());
                            on_change.call(file_infos);
                        }
                    },
                }
            }
            // Selected files display
            if !selected_files().is_empty() {
                div { class: "bg-gray-50 rounded-lg p-4",
                    h4 { class: "text-sm font-medium text-gray-900 mb-2", "Selected files:" }
                    div { class: "space-y-2",
                        for file in selected_files() {
                            div { class: "flex items-center justify-between bg-white p-2 rounded border",
                                div { class: "flex items-center space-x-2",
                                    span { class: "text-sm", "📄" }
                                    span { class: "text-sm text-gray-700", "{file.name}" }
                                }
                                button {
                                    class: "text-red-500 hover:text-red-700 text-sm",
                                    onclick: move |_| {
                                        let mut files = selected_files();
                                        files.retain(|f| f.name != file.name);
                                        selected_files.set(files.clone());
                                        on_change.call(files);
                                    },
                                    "✕"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}