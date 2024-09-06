use crate::types::factura::{Factura, Impuesto};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub fn process_conceptos(reader: &mut Reader<&[u8]>, factura: &mut Factura) -> Result<(), String> {
    let mut buf = Vec::new();
    println!("Procesando atributos del concepto");
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"cfdi:Concepto" => {
                println!("Tag concepto autoclosing encontrado");
                for attr in e.attributes().filter_map(Result::ok) {
                    match attr.key.as_ref() {
                        b"ClaveProdServ" => {
                            factura.set_prod_serv(attr.unescape_value().unwrap().into_owned());
                            factura.set_es_gasolina(attr.unescape_value().unwrap().into_owned());
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"cfdi:Concepto" => {
                println!("Tag concepto encontrado");
                for attr in e.attributes().filter_map(Result::ok) {
                    match attr.key.as_ref() {
                        b"ClaveProdServ" => {
                            factura.set_prod_serv(attr.unescape_value().unwrap().into_owned());

                            factura.set_es_gasolina(attr.unescape_value().unwrap().into_owned());
                        }
                        _ => {}
                    }
                }
                println!("Procesando hijos concepto");
                process_concepto(reader, factura)?;
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"cfdi:Conceptos" => {
                println!("Tag de cierre de conceptos encontrado");
                break;
            }
            Err(e) => return Err(format!("Error al procesar conceptos: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    println!("Atributos del concepto procesados correctamente");

    println!("Clave prod serv final: {}", factura.clave_producto_servicio);

    Ok(())
}

fn process_concepto(reader: &mut Reader<&[u8]>, factura: &mut Factura) -> Result<(), String> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e))
                if e.name().as_ref() == b"cfdi:Impuestos" =>
            {
                println!("Tag impuestos encontrado");
                process_impuestos(reader, factura)?;
            }

            Ok(Event::End(ref e)) if e.name().as_ref() == b"cfdi:Concepto" => break,
            Err(e) => return Err(format!("Error al procesar concepto: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}

fn process_impuestos(reader: &mut Reader<&[u8]>, factura: &mut Factura) -> Result<(), String> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"cfdi:Retenciones" => process_retenciones(reader, factura)?,
                b"cfdi:Traslados" => process_traslados(reader, factura)?,
                _ => {}
            },
            Ok(Event::End(ref e)) if e.name().as_ref() == b"cfdi:Impuestos" => break,
            Err(e) => return Err(format!("Error al procesar impuestos: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}

fn process_retenciones(reader: &mut Reader<&[u8]>, factura: &mut Factura) -> Result<(), String> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e))
                if e.name().as_ref() == b"cfdi:Retencion" =>
            {
                let mut impuesto = Impuesto::new();
                for attr in e.attributes().filter_map(Result::ok) {
                    match attr.key.as_ref() {
                        b"Importe" => {
                            impuesto.importe = attr.unescape_value().unwrap().parse().unwrap_or(0.0)
                        }
                        b"TasaOCuota" => {
                            impuesto.tasa_o_cuota = attr.unescape_value().unwrap().parse().unwrap()
                        }
                        _ => {}
                    }
                }
                factura.set_impuesto(impuesto.tasa_o_cuota, impuesto.importe);
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"cfdi:Retenciones" => break,
            Err(e) => return Err(format!("Error al procesar retenciones: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}

fn process_traslados(reader: &mut Reader<&[u8]>, factura: &mut Factura) -> Result<(), String> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e))
                if e.name().as_ref() == b"cfdi:Traslado" =>
            {
                let mut impuesto = Impuesto::new();
                for attr in e.attributes().filter_map(Result::ok) {
                    match attr.key.as_ref() {
                        b"Importe" => {
                            impuesto.importe = attr.unescape_value().unwrap().parse().unwrap_or(0.0)
                        }
                        b"TasaOCuota" => {
                            impuesto.tasa_o_cuota =
                                attr.unescape_value().unwrap().parse().unwrap_or(0.0)
                        }
                        _ => {}
                    }
                }
                factura.set_impuesto(impuesto.tasa_o_cuota, impuesto.importe);
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"cfdi:Traslados" => break,
            Err(e) => return Err(format!("Error al procesar traslados: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}
