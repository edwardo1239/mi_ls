use std::{error::Error, os::windows::fs::MetadataExt};
use std::path::Path;
use std::fs;
use chrono::{DateTime, Local};

#[derive(Debug)] // Derivar automáticamente el trait Debug
pub struct Config {
    pub file_paths: Vec<String>,
    pub show_hidden: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut show_hidden = false;
        let mut paths = Vec::new();

        for arg in args {
            if arg == "-a" {
                show_hidden = true
            } else {
                paths.push(arg)
            }
        }

        Ok(Config { 
            file_paths: paths, // Usar vec![] para crear un vector de Strings
            show_hidden: show_hidden 
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("{:?}", config);
    for path in config.file_paths {
        if let Err(err) = list_directory(&path, config.show_hidden) {
            println!("Error al listar el directorio '{}': {}", path, err)
        }
    }
    Ok(())
}

pub fn list_directory(path: &str, show_hidden: bool) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);

    let entries = fs::read_dir(path)?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                let metadata = entry.metadata()?;

                // Comprobar si el archivo es oculto en Windows
                let is_hidden = metadata.file_attributes() & 0x2 != 0; // 0x2 es el valor para `FILE_ATTRIBUTE_HIDDEN`

                if !show_hidden && is_hidden {
                    continue; // Saltar archivos ocultos si no se usa -a
                }

                let modified: DateTime<Local> = metadata.modified()?.into();

                let readonly = if metadata.permissions().readonly() {
                    "R"
                } else {
                    "-"
                };

                println!("{} {} {} {} {}",
                    readonly,
                    human_readable_size(metadata.len()),
                    modified.format("%Y-%m-%d %H:%M:%S"),
                    if metadata.is_dir() { "D" } else { "F" },
                    file_name_str
                );
            }
            Err(e) => {
                println!("Error al leer una entrada {e}");
            }
        }
    }

    Ok(())
}

pub fn human_readable_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size >= TB {
        format!("{:.2} TB", size as f64 / TB as f64)
    } else if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}