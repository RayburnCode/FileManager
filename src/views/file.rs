use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;


#[component]
fn FileManager() -> Element {
    let mut current_path = use_signal(|| PathBuf::from("."));
    let mut dir_contents = use_resource(move || async move {
        read_dir_contents(&current_path())
    });

    rsx! {
        div {
            h1 { "Rust File Manager" }
            // Path navigation
            div {
                button {
                    onclick: move |_| {
                        if let Some(parent) = current_path().parent() {
                            current_path.set(parent.to_path_buf());
                        }
                    },
                    "↑ Up"
                }
                input {
                    value: "{current_path().display()}",
                    onchange: move |e| {
                        let path = PathBuf::from(e.value());
                        if path.exists() {
                            current_path.set(path);
                        }
                    },
                }
            }
            // File listing
            match &*dir_contents.read_unchecked.clone() {
                Some(Ok(contents)) => rsx! {
                    table {
                        for entry in contents {
                            tr {
                                td {
                                    if entry.is_dir {
                                        "📁"
                                    } else {
                                        "📄"
                                    }
                                }
                                td {
                                    button {
                                        onclick: move |_| {
                                            if entry.is_dir {
                                                current_path.set(entry.path.clone());
                                            }
                                        },
                                        "{entry.name}"
                                    }
                                }
                                td {
                                    if !entry.is_dir {
                                        "{entry.size} bytes"
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(err)) => rsx! {
                    div { "Error reading directory: {err}" }
                },
                None => rsx! {
                    div { "Loading..." }
                },
            }
        }
    }
}

struct DirEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
}

fn read_dir_contents(path: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut entries = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        
        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: path.clone(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
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