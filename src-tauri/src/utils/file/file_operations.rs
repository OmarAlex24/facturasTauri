use std::process::Command;
use tauri::api::path::{resolve_path, BaseDirectory};
use tauri::Manager;

pub async fn abrir_archivo(app: tauri::AppHandle, ruta: String) -> Result<String, String> {
    println!("Intentando abrir archivo: {}", ruta);

    let path = resolve_path(
        &app.config(),
        &app.package_info(),
        &app.env(),
        &ruta,
        Some(BaseDirectory::Document),
    )
    .map_err(|e| format!("Error al resolver la ruta: {}", e))?;

    if !path.exists() {
        return Err(format!("El archivo no existe: {}", path.display()));
    }

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
