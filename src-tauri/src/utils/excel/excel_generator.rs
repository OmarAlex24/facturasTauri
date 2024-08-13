use crate::factura::Factura;
use std::path::Path;
use xlsxwriter::Workbook;
use xlsxwriter::Worksheet;

pub fn create_excel(path: &Path, facturas: &[Factura]) -> Result<(), xlsxwriter::XlsxError> {
    let workbook = Workbook::new(path.to_str().unwrap())?;
    let mut sheet_ingreso = workbook.add_worksheet(Some("Ingreso"))?;
    let mut sheet_egreso: Worksheet = workbook.add_worksheet(Some("Egreso"))?;

    let headers_ingreso = [
        "Folio Fiscal",
        "RFC Receptor",
        "Nombre o Razón Social Receptor",
        "Fecha Emisión",
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
        "Moneda",
        "Descuento",
    ];

    let headers_egreso = [
        "Folio Fiscal",
        "RFC Emisor",
        "Nombre o Razón Social Emisor",
        "Fecha Emisión",
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
        "Moneda",
        "Descuento",
    ];

    for (col, header) in headers_ingreso.iter().enumerate() {
        sheet_ingreso.write_string(0, col as u16, header, None)?;
    }
    for (col, header) in headers_egreso.iter().enumerate() {
        sheet_egreso.write_string(0, col as u16, header, None)?;
    }

    let mut row_ingreso = 1;
    let mut row_egreso = 1;

    for factura in facturas.iter() {
        if &factura.tipo_factura == "Ingreso" {
            sheet_ingreso.write_string(row_ingreso, 0, &factura.folio_fiscal, None)?;
            sheet_ingreso.write_string(row_ingreso, 1, &factura.rfc_receptor, None)?;
            sheet_ingreso.write_string(row_ingreso, 2, &factura.nombre_receptor, None)?;
            sheet_ingreso.write_string(row_ingreso, 3, &factura.fecha_emision, None)?;
            sheet_ingreso.write_number(row_ingreso, 4, factura.subtotal, None)?;
            sheet_ingreso.write_number(row_ingreso, 5, factura.iva.iva_16, None)?;
            sheet_ingreso.write_number(row_ingreso, 6, factura.iva.iva_ret_4, None)?;
            sheet_ingreso.write_number(row_ingreso, 7, factura.iva.iva_ret_10, None)?;
            sheet_ingreso.write_number(row_ingreso, 8, factura.isr.isr_ret_10, None)?;
            sheet_ingreso.write_number(row_ingreso, 9, factura.isr.isr_ret_1, None)?;
            sheet_ingreso.write_number(row_ingreso, 10, factura.ieps, None)?;
            sheet_ingreso.write_number(row_ingreso, 11, factura.ish, None)?;
            sheet_ingreso.write_number(row_ingreso, 12, factura.total, None)?;
            sheet_ingreso.write_string(row_ingreso, 13, &factura.efecto_comprobante, None)?;
            sheet_ingreso.write_string(row_ingreso, 14, &factura.metodo_pago, None)?;
            sheet_ingreso.write_string(row_ingreso, 15, &factura.moneda, None)?;
            sheet_ingreso.write_number(row_ingreso, 16, factura.descuento, None)?;
            row_ingreso += 1;
        } else if &factura.tipo_factura == "Egreso" {
            sheet_egreso.write_string(row_egreso, 0, &factura.folio_fiscal, None)?;
            sheet_egreso.write_string(row_egreso, 1, &factura.rfc_emisor, None)?;
            sheet_egreso.write_string(row_egreso, 2, &factura.nombre_emisor, None)?;
            sheet_egreso.write_string(row_egreso, 3, &factura.fecha_emision, None)?;
            sheet_egreso.write_number(row_egreso, 4, factura.subtotal, None)?;
            sheet_egreso.write_number(row_egreso, 5, factura.iva.iva_16, None)?;
            sheet_egreso.write_number(row_egreso, 6, factura.iva.iva_ret_4, None)?;
            sheet_egreso.write_number(row_egreso, 7, factura.iva.iva_ret_10, None)?;
            sheet_egreso.write_number(row_egreso, 8, factura.isr.isr_ret_10, None)?;
            sheet_egreso.write_number(row_egreso, 9, factura.isr.isr_ret_1, None)?;
            sheet_egreso.write_number(row_egreso, 10, factura.ieps, None)?;
            sheet_egreso.write_number(row_egreso, 11, factura.ish, None)?;
            sheet_egreso.write_number(row_egreso, 12, factura.total, None)?;
            sheet_egreso.write_string(row_egreso, 13, &factura.efecto_comprobante, None)?;
            sheet_egreso.write_string(row_egreso, 14, &factura.metodo_pago, None)?;
            sheet_egreso.write_string(row_egreso, 15, &factura.moneda, None)?;
            sheet_egreso.write_number(row_egreso, 16, factura.descuento, None)?;
            row_egreso += 1;
        }
    }

    workbook.close()?;
    Ok(())
}
