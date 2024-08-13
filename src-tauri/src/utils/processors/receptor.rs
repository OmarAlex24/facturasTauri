use crate::factura::Factura;

pub fn process_receptor(factura: &mut Factura, e: &quick_xml::events::BytesStart) {
    for attr in e.attributes().filter_map(Result::ok) {
        match attr.key.as_ref() {
            b"Rfc" => factura.rfc_receptor = attr.unescape_value().unwrap().into_owned(),
            b"Nombre" => factura.nombre_receptor = attr.unescape_value().unwrap().into_owned(),
            _ => {}
        }
    }
}
