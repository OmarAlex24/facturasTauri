use crate::factura::Factura;
use crate::rfc_clientes::RFC_CLIENTES;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::path::Path;

pub fn process_folder(path: &Path) -> Result<Vec<Factura>, String> {
    let mut facturas: Vec<Factura> = Vec::new();
    let mut processed_files = 0;
    let mut total_amount = 0.0;

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();
        if file_path.extension().and_then(|s| s.to_str()) == Some("xml") {
            match process_xml_file(&file_path) {
                Ok(factura) => {
                    processed_files += 1;
                    total_amount += factura.total;
                    facturas.push(factura);
                }
                Err(e) => println!("Error processing file {:?}: {}", file_path, e),
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

fn process_xml_file(file_path: &Path) -> Result<Factura, String> {
    let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let mut reader = Reader::from_str(&file_content);
    reader.config_mut().trim_text(true);

    let mut factura = Factura::new();

    loop {
        println!("Recoleccion de datos iniciada");
        match reader.read_event() {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => match e.name().as_ref() {
                b"cfdi:Comprobante" => {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"Total" => {
                                    factura.total =
                                        attr.unescape_value().unwrap().parse().unwrap_or(0.0)
                                }
                                b"Fecha" => {
                                    factura.fecha_emision =
                                        attr.unescape_value().unwrap().into_owned()
                                }
                                b"TipoDeComprobante" => {
                                    factura.set_efecto_comprobante(
                                        &attr.unescape_value().unwrap().into_owned(),
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
                b"cfdi:Emisor" => {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"Rfc" => {
                                    factura.rfc_emisor = attr.unescape_value().unwrap().into_owned()
                                }
                                b"Nombre" => {
                                    factura.nombre_emisor =
                                        attr.unescape_value().unwrap().into_owned()
                                }
                                _ => {}
                            }
                        }
                    }
                }
                b"cfdi:Receptor" => {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"Rfc" => {
                                    factura.rfc_receptor =
                                        attr.unescape_value().unwrap().into_owned()
                                }
                                b"Nombre" => {
                                    factura.nombre_receptor =
                                        attr.unescape_value().unwrap().into_owned()
                                }
                                _ => {}
                            }
                        }
                    }
                }
                b"tfd:TimbreFiscalDigital" => {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            if attr.key.as_ref() == b"UUID" {
                                factura.folio_fiscal = attr.unescape_value().unwrap().into_owned();
                            }
                        }
                    }
                }
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Error al parsear XML: {}", e)),
            _ => (),
        }

        factura.tipo_factura = if RFC_CLIENTES.contains(&factura.rfc_emisor.as_str()) {
            "Ingreso".to_string()
        } else {
            "Egreso".to_string()
        };
    }

    Ok(factura)
}
