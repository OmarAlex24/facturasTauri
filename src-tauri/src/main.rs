#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod excel_generator;
mod factura;
mod rfc_clientes;
mod xml_processor;

use factura::Factura;
use serde_json::json;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tauri::api::path::{resolve_path, BaseDirectory};
use tauri::Manager;

#[tauri::command]
async fn abrir_archivo(app: tauri::AppHandle, ruta: String) -> Result<String, String> {
    println!("Intentando abrir archivo: {}", ruta);

    // Resolver la ruta del archivo
    let path = resolve_path(
        &app.config(),
        &app.package_info(),
        &app.env(),
        &ruta,
        Some(BaseDirectory::Document),
    )
    .map_err(|e| format!("Error al resolver la ruta: {}", e))?;

    // Verificar si el archivo existe
    if !path.exists() {
        return Err(format!("El archivo no existe: {}", path.display()));
    }

    // Intentar abrir el archivo usando el comando adecuado según el sistema operativo
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", "", path.to_str().unwrap()])
            .output()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path.to_str().unwrap()).output()
    } else {
        Command::new("xdg-open")
            .arg(path.to_str().unwrap())
            .output()
    };

    match result {
        Ok(_) => {
            println!("Archivo abierto exitosamente: {}", path.display());
            Ok(format!("Archivo abierto: {}", path.display()))
        }
        Err(e) => {
            let error_msg = format!("Error al abrir el archivo {}: {}", path.display(), e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
fn dummy_command() -> String {
    "Tauri is initialized".into()
}

#[tauri::command]
async fn process_xml_folder(
    folder_xml_path: String,
    app_data_dir_path: String,
) -> Result<String, String> {
    let path = Path::new(&folder_xml_path);
    if !path.is_dir() {
        return Err("La ruta proporcionada no es un directorio".into());
    }

    let facturas: Vec<Factura> = xml_processor::process_folder(&path)?;

    // Crear el archivo Excel
    let excel_file_name = "facturas.xlsx";
    let excel_path = PathBuf::from(&app_data_dir_path).join(excel_file_name);

    println!("Excel path: {}", excel_path.to_str().unwrap());

    excel_generator::create_excel(&excel_path, &facturas).map_err(|e| e.to_string())?;

    Ok(json!({
        "excel_path": excel_path.to_str().unwrap()
    })
    .to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.center().unwrap();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            dummy_command,
            process_xml_folder,
            abrir_archivo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
