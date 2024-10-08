use crate::types::factura::Factura;

pub fn process_emisor(factura: &mut Factura, e: &quick_xml::events::BytesStart) {
    for attr in e.attributes().filter_map(Result::ok) {
        match attr.key.as_ref() {
            b"Rfc" => factura.rfc_emisor = attr.unescape_value().unwrap().into_owned(),
            b"Nombre" => factura.nombre_emisor = attr.unescape_value().unwrap().into_owned(),
            _ => {}
        }
    }
    println!("Atributos del emisor procesados correctamente");
}
