use crate::rfc_clientes::RFC_CLIENTES;
use crate::types::factura::Factura;
use crate::utils::processors::{comprobante, conceptos, emisor, receptor, timbre_fiscal};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::path::{Path, PathBuf};

pub fn process_folder(paths: Vec<PathBuf>) -> Result<Vec<Factura>, String> {
    let mut facturas: Vec<Factura> = Vec::new();
    let mut processed_files = 0;
    let mut total_amount = 0.0;

    for path in paths.iter() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("xml") {
                match process_file(&file_path) {
                    Ok(factura) => {
                        processed_files += 1;
                        total_amount += factura.total;
                        facturas.push(factura);
                    }
                    Err(e) => println!("Error processing file {:?}: {}", file_path, e),
                }
            }
        }
    }

    facturas.sort_by(|a, b| a.fecha_emision.cmp(&b.fecha_emision));
    println!(
        "Facturas procesadas correctamente, {} archivos, {} total",
        processed_files, total_amount
    );
    Ok(facturas)
}

fn process_file(file_path: &Path) -> Result<Factura, String> {
    let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let mut reader = Reader::from_str(&file_content);
    reader.config_mut().trim_text(true);

    let mut factura = Factura::new();

    loop {
        match reader.read_event() {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => match e.name().as_ref() {
                b"cfdi:Comprobante" => comprobante::process_comprobante(&mut factura, &e),
                b"cfdi:Emisor" => emisor::process_emisor(&mut factura, &e),
                b"cfdi:Receptor" => receptor::process_receptor(&mut factura, &e),
                b"cfdi:Conceptos" => conceptos::process_conceptos(&mut reader, &mut factura)?,
                b"tfd:TimbreFiscalDigital" => {
                    timbre_fiscal::process_timbre_fiscal(&mut factura, &e)
                }
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Error al parsear XML: {}", e)),
            _ => (),
        }

        if factura.es_gasolina {
            factura.set_ieps(factura.subtotal, factura.iva.iva_16)
        }

        factura.set_prod_serv(factura.clave_producto_servicio.clone());

        factura.tipo_factura = if RFC_CLIENTES.contains(&factura.rfc_emisor.as_str()) {
            "Ingreso".to_string()
        } else {
            "Egreso".to_string()
        };
    }
    factura.print_factura();

    Ok(factura)
}
