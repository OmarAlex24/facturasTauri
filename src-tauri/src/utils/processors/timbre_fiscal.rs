use crate::types::factura::Factura;

pub fn process_timbre_fiscal(factura: &mut Factura, e: &quick_xml::events::BytesStart) {
    for attr in e.attributes().filter_map(Result::ok) {
        if attr.key.as_ref() == b"UUID" {
            factura.folio_fiscal = attr.unescape_value().unwrap().into_owned();
            println!("{:?}", factura.folio_fiscal);
            break;
        }
    }
    println!("Atributos del timbre procesados correctamente");
}
