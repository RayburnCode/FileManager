use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;

use crate::components::AddFile;

#[component]
pub fn FileManager() -> Element {
    let mut current_path = use_signal(|| PathBuf::from("."));
    
    let mut dir_contents = use_resource(move || {
        let path = current_path();
        async move {
            read_dir_contents(&path)
        }
    });

    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            div { class: "max-w-6xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900 mb-2", "File Manager" }
                    p { class: "text-gray-600", "Browse and navigate your files and directories" }
                }

                // Navigation controls
                div { class: "bg-white rounded-lg shadow-sm border p-4 mb-6",
                    div { class: "flex items-center gap-4",
                        button {
                            class: "flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: current_path().parent().is_none(),
                            onclick: move |_| {
                                if let Some(parent) = current_path().parent() {
                                    current_path.set(parent.to_path_buf());
                                }
                            },
                            "↑ Up"
                        }
                        div { class: "flex-1",
                            label { class: "block text-sm font-medium text-gray-700 mb-1",
                                "Current Path:"
                            }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                                r#type: "text",
                                value: "{current_path().display()}",
                                onchange: move |e| {
                                    let path = PathBuf::from(e.value());
                                    if path.exists() && path.is_dir() {
                                        current_path.set(path);
                                    }
                                },
                            }
                        }
                    }
                }
                // File listing
                div { class: "bg-white rounded-lg shadow-sm border overflow-hidden",
                    if let Some(dir_result) = dir_contents.read().as_ref() {
                        match dir_result {
                            Ok(entries) => rsx! {
                                if entries.is_empty() {
                                    div { class: "p-8 text-center text-gray-500", "This directory is empty" }
                                } else {
                                    table { class: "w-full",
                                        thead { class: "bg-gray-50 border-b",
                                            tr {
                                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider w-12",
                                                    "Type"
                                                }
                                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                                    "Name"
                                                }
                                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider w-32",
                                                    "Size"
                                                }
                                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider w-48",
                                                    "Modified"
                                                }
                                            }
                                        }
                                        tbody { class: "bg-white divide-y divide-gray-200",
                                            for entry in entries {
                                                tr {
                                                    key: "{entry.path.display()}",
                                                    class: "hover:bg-gray-50 transition-colors",
                                                    td { class: "px-6 py-4 whitespace-nowrap text-2xl",
                                                        if entry.is_dir {
                                                            "📁"
                                                        } else {
                                                            "📄"
                                                        }
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap",
                                                        if entry.is_dir {
                                                            button {
                                                                class: "text-blue-600 hover:text-blue-800 font-medium transition-colors cursor-pointer",
                                                                onclick: {
                                                                    let entry_path = entry.path.clone();
                                                                    move |_| {
                                                                        current_path.set(entry_path.clone());
                                                                    }
                                                                },
                                                                "{entry.name}"
                                                            }
                                                        } else {
                                                            span { class: "text-gray-900", "{entry.name}" }
                                                        }
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                                        if !entry.is_dir {
                                                            "{format_file_size(entry.size)}"
                                                        } else {
                                                            "-"
                                                        }
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                                        if let Some(modified) = &entry.modified {
                                                            "{modified}"
                                                        } else {
                                                            "-"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Err(err) => rsx! {
                                div { class: "p-6 text-center",
                                    div { class: "text-red-600 bg-red-50 border border-red-200 rounded-md p-4",
                                        h3 { class: "font-medium text-red-800 mb-2", "Error accessing directory" }
                                        p { class: "text-sm", "{err}" }
                                    }
                                }
                            },
                        }
                    } else {
                        div { class: "p-8 text-center",
                            div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto" }
                            p { class: "mt-2 text-gray-500", "Loading..." }
                        }
                    }
                }
            }
        }
    }
}

// ...existing code...

#[derive(Clone)]
struct DirEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
    modified: Option<String>,
}

fn read_dir_contents(path: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut entries = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        
        let modified = metadata.modified()
            .ok()
            .and_then(|time| {
                use std::time::{SystemTime, UNIX_EPOCH};
                let now = SystemTime::now();
                if let (Ok(file_time), Ok(current_time)) = (
                    time.duration_since(UNIX_EPOCH),
                    now.duration_since(UNIX_EPOCH)
                ) {
                    if current_time > file_time {
                        let diff_secs = current_time.as_secs() - file_time.as_secs();
                        let days = diff_secs / 86400;
                        let hours = (diff_secs % 86400) / 3600;
                        let minutes = (diff_secs % 3600) / 60;
                        
                        if days > 0 {
                            Some(format!("{} days ago", days))
                        } else if hours > 0 {
                            Some(format!("{} hours ago", hours))
                        } else if minutes > 0 {
                            Some(format!("{} minutes ago", minutes))
                        } else {
                            Some("Just now".to_string())
                        }
                    } else {
                        Some("Recently".to_string())
                    }
                } else {
                    None
                }
            });
        
        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: path.clone(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified,
        });
    }
    
    // Sort directories first, then by name
    entries.sort_by(|a, b| {
        if a.is_dir == b.is_dir {
            a.name.cmp(&b.name)
        } else if a.is_dir {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    
    Ok(entries)
}

fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}