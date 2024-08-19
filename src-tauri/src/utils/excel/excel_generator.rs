use std::path::Path;
use xlsxwriter::Workbook;
use xlsxwriter::Worksheet;

use crate::types::factura::Factura;

pub fn create_excel(path: &Path, facturas: &[Factura]) -> Result<(), xlsxwriter::XlsxError> {
    let workbook = Workbook::new(path.to_str().unwrap())?;
    let mut sheet_ingreso = workbook.add_worksheet(Some("Ingreso"))?;
    let mut sheet_egreso: Worksheet = workbook.add_worksheet(Some("Egreso"))?;
    let mut sheet_complementos: Worksheet = workbook.add_worksheet(Some("Complementos"))?;

    let headers_ingreso = [
        "Folio Fiscal",
        "Fecha Emisión",
        "Uso CFDI",
        "Clave ProdServ",
        "RFC Receptor",
        "Nombre o Razón Social Receptor",
        "Subtotal",
        "IVA 16%",
        "IVA RET 4%",
        "IVA RET 10.6667%",
        "ISR RET 10%",
        "ISR RET 1.25%",
        "IEPS",
        "ISH 2%",
        "Total",
        "Efecto Comprobante",
        "Metodo de pago",
        "Descuento",
        "Forma de pago",
        "Moneda",
    ];

    let headers_egreso = [
        "Folio Fiscal",
        "Fecha Emisión",
        "Uso CFDI",
        "Clave ProdServ",
        "RFC Emisor",
        "Nombre o Razón Social Emisor",
        "Subtotal",
        "IVA 16%",
        "IVA RET 4%",
        "IVA RET 10.6667%",
        "ISR RET 10%",
        "ISR RET 1.25%",
        "IEPS",
        "ISH 2%",
        "Total",
        "Efecto Comprobante",
        "Metodo de pago",
        "Descuento",
        "Forma de pago",
        "Moneda",
    ];

    let headers_complementos = [
        "Folio Fiscal", //idDocumento
        "RFC Receptor",
        "Nombre o Razón Social Receptor",
        "Fecha de Pago",
        "Monto Pagado",
        "Metodo de pago",
    ];

    for (col, header) in headers_ingreso.iter().enumerate() {
        sheet_ingreso.write_string(0, col as u16, header, None)?;
    }
    for (col, header) in headers_egreso.iter().enumerate() {
        sheet_egreso.write_string(0, col as u16, header, None)?;
    }
    for (col, header) in headers_complementos.iter().enumerate() {
        sheet_complementos.write_string(0, col as u16, header, None)?;
    }

    let mut row_ingreso = 1;
    let mut row_egreso = 1;
    let mut row_complemento = 1;

    for factura in facturas.iter() {
        if &factura.tipo_factura == "Ingreso" {
            sheet_ingreso.write_string(row_ingreso, 0, &factura.folio_fiscal, None)?;
            sheet_ingreso.write_string(row_ingreso, 1, &factura.fecha_emision, None)?;
            sheet_ingreso.write_string(row_ingreso, 2, &factura.uso_cfdi_receptor, None)?;
            sheet_ingreso.write_string(row_ingreso, 3, &factura.clave_producto_servicio, None)?;
            sheet_ingreso.write_string(row_ingreso, 4, &factura.rfc_receptor, None)?;
            sheet_ingreso.write_string(row_ingreso, 5, &factura.nombre_receptor, None)?;
            sheet_ingreso.write_number(row_ingreso, 6, factura.subtotal, None)?;
            sheet_ingreso.write_number(row_ingreso, 7, factura.iva.iva_16, None)?;
            sheet_ingreso.write_number(row_ingreso, 8, factura.iva.iva_ret_4, None)?;
            sheet_ingreso.write_number(row_ingreso, 9, factura.iva.iva_ret_10, None)?;
            sheet_ingreso.write_number(row_ingreso, 10, factura.isr.isr_ret_10, None)?;
            sheet_ingreso.write_number(row_ingreso, 11, factura.isr.isr_ret_1, None)?;
            sheet_ingreso.write_number(row_ingreso, 12, factura.ieps, None)?;
            sheet_ingreso.write_number(row_ingreso, 13, factura.ish, None)?;
            sheet_ingreso.write_number(row_ingreso, 14, factura.total, None)?;
            sheet_ingreso.write_string(row_ingreso, 15, &factura.efecto_comprobante, None)?;
            sheet_ingreso.write_string(row_ingreso, 16, &factura.metodo_pago, None)?;
            sheet_ingreso.write_number(row_ingreso, 17, factura.descuento, None)?;
            sheet_ingreso.write_string(row_ingreso, 18, &factura.forma_pago, None)?;
            sheet_ingreso.write_string(row_ingreso, 19, &factura.moneda, None)?;
            row_ingreso += 1;
        } else if &factura.tipo_factura == "Egreso" {
            sheet_egreso.write_string(row_egreso, 0, &factura.folio_fiscal, None)?;
            sheet_egreso.write_string(row_egreso, 1, &factura.fecha_emision, None)?;
            sheet_egreso.write_string(row_egreso, 2, &factura.uso_cfdi_receptor, None)?;
            sheet_egreso.write_string(row_egreso, 3, &factura.clave_producto_servicio, None)?;
            sheet_egreso.write_string(row_egreso, 4, &factura.rfc_emisor, None)?;
            sheet_egreso.write_string(row_egreso, 5, &factura.nombre_emisor, None)?;
            sheet_egreso.write_number(row_egreso, 6, factura.subtotal, None)?;
            sheet_egreso.write_number(row_egreso, 7, factura.iva.iva_16, None)?;
            sheet_egreso.write_number(row_egreso, 8, factura.iva.iva_ret_4, None)?;
            sheet_egreso.write_number(row_egreso, 9, factura.iva.iva_ret_10, None)?;
            sheet_egreso.write_number(row_egreso, 10, factura.isr.isr_ret_10, None)?;
            sheet_egreso.write_number(row_egreso, 11, factura.isr.isr_ret_1, None)?;
            sheet_egreso.write_number(row_egreso, 12, factura.ieps, None)?;
            sheet_egreso.write_number(row_egreso, 13, factura.ish, None)?;
            sheet_egreso.write_number(row_egreso, 14, factura.total, None)?;
            sheet_egreso.write_string(row_egreso, 15, &factura.efecto_comprobante, None)?;
            sheet_egreso.write_string(row_egreso, 16, &factura.metodo_pago, None)?;
            sheet_egreso.write_number(row_egreso, 17, factura.descuento, None)?;
            sheet_egreso.write_string(row_egreso, 18, &factura.forma_pago, None)?;
            sheet_egreso.write_string(row_egreso, 19, &factura.moneda, None)?;
            row_egreso += 1;
        }
    }

    workbook.close()?;
    Ok(())
}
