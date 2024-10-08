#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod rfc_clientes;
mod types;
mod utils;

use crate::utils::{file, xml};
use tauri::Manager;

#[tauri::command]
async fn open_file(app: tauri::AppHandle, ruta: String) -> Result<String, String> {
    println!("Intentando abrir archivo: {}", ruta);
    file::abrir_archivo(app, ruta).await
}

#[tauri::command]
async fn main_xml(folder_xml_path: Vec<String>, doc_dir_path: String) -> Result<String, String> {
    for path in folder_xml_path.iter() {
        println!("Procesando XMLs en: {}", path);
    }
    xml::process_xml_folder(folder_xml_path, doc_dir_path).await
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let window = _app.get_window("main").unwrap();
                window.center().unwrap();
            }
            println!("Tauri inicializado");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![main_xml, open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
