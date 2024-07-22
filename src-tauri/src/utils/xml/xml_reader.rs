use serde_json::json;
use std::path::{Path, PathBuf};

use crate::factura::Factura;
use crate::utils::excel;
use crate::utils::xml;

pub async fn process_xml_folder(
    folder_xml_path: String,
    doc_dir_path: String,
) -> Result<String, String> {
    println!("Directorio de datos de la aplicación: {}", doc_dir_path);

    let path = Path::new(&folder_xml_path);
    if !path.is_dir() {
        return Err("La ruta proporcionada no es un directorio".into());
    }

    println!("Éxito, ruta correcta");
    let facturas: Vec<Factura> = xml::process_folder(&path)?;

    let excel_file_name = "facturas.xlsx";
    let excel_path = PathBuf::from(&doc_dir_path).join(excel_file_name);

    println!("Excel path: {}", excel_path.to_str().unwrap());

    excel::create_excel(&excel_path, &facturas).map_err(|e| e.to_string())?;

    Ok(json!({
        "excel_path": excel_path.to_str().unwrap()
    })
    .to_string())
}
