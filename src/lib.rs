use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;
use std::{error::Error, os::windows::fs::MetadataExt};

#[derive(Debug)] // Derivar automáticamente el trait Debug
pub struct Config {
    pub file_paths: Vec<String>,
    pub show_hidden: bool,
    pub sort: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut show_hidden = false;
        let mut sort = false;
        let mut paths = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-a" => show_hidden = true,
                "--sort" => sort = true,
                _ => paths.push(arg),
            }
        }

        Ok(Config {
            file_paths: paths, // Usar vec![] para crear un vector de Strings
            show_hidden: show_hidden,
            sort: sort,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);
    // Imprimir encabezados de la tabla
    println!(
        "{:<30} {:<10} {:<15} {:<20} {:<10}",
        "Nombre", "Tipo", "Tamaño", "Modificado", "Permisos"
    );
    println!("{:-<85}", ""); // Línea divisoria
    for path in config.file_paths {
        if let Err(err) = list_directory(&path, config.show_hidden, config.sort) {
            println!("Error al listar el directorio '{}': {}", path, err)
        }
    }
    Ok(())
}

pub fn list_directory(path: &str, show_hidden: bool, sort: bool) -> Result<(), Box<dyn Error>> {
    let path: &Path = Path::new(path);
    let mut arr_files = Vec::new();

    let entries = fs::read_dir(path)?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                arr_files.push((file_name, entry));
            }
            Err(e) => {
                println!("Error al leer una entrada {e}");
            }
        }
    }

    if sort {
        arr_files.sort_by(|a, b| a.0.cmp(&b.0));
    }

    for (_, entry) in arr_files {
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

        println!(
            "{:<30} {:<10} {:<15} {:<20} {:<10}",
            file_name_str, // Nombre del archivo
            if metadata.is_dir() {
                "Directorio"
            } else {
                "Archivo"
            }, // Tipo de archivo
            human_readable_size(metadata.len()), // Tamaño del archivo
            modified.format("%Y-%m-%d %H:%M:%S"), // Fecha de modificación
            readonly       // Permisos de solo lectura
        );
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
