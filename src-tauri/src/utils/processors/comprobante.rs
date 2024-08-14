use crate::factura::Factura;

pub fn process_comprobante(factura: &mut Factura, e: &quick_xml::events::BytesStart) {
    for attr in e.attributes().filter_map(Result::ok) {
        match attr.key.as_ref() {
            b"Moneda" => factura.moneda = attr.unescape_value().unwrap().into_owned(),
            b"Descuento" => {
                factura.descuento = attr.unescape_value().unwrap().parse().unwrap_or(0.0)
            }
            b"SubTotal" => factura.subtotal = attr.unescape_value().unwrap().parse().unwrap_or(0.0),
            b"Total" => factura.total = attr.unescape_value().unwrap().parse().unwrap_or(0.0),
            b"MetodoPago" => factura.metodo_pago = attr.unescape_value().unwrap().into_owned(),
            b"FormaPago" => factura.set_forma_pago(attr.unescape_value().unwrap().into_owned()),

            b"Fecha" => factura.fecha_emision = attr.unescape_value().unwrap().into_owned(),
            b"TipoDeComprobante" => {
                factura.set_efecto_comprobante(&attr.unescape_value().unwrap().into_owned())
            }
            _ => {}
        }
    }
    println!("Atributos del comprobante procesados correctamente");
}
