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
        "Total",
        "Efecto Comprobante",
    ];

    let headers_egreso = [
        "Folio Fiscal",
        "RFC Emisor",
        "Nombre o Razón Social Emisor",
        "Fecha Emisión",
        "Total",
        "Efecto Comprobante",
    ];

    // Escribir datos de ingreso

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
            sheet_ingreso.write_number(row_ingreso, 4, factura.total, None)?;
            sheet_ingreso.write_string(row_ingreso, 5, &factura.efecto_comprobante, None)?;
            row_ingreso += 1;
        } else if &factura.tipo_factura == "Egreso" {
            sheet_egreso.write_string(row_egreso, 0, &factura.folio_fiscal, None)?;
            sheet_egreso.write_string(row_egreso, 1, &factura.rfc_emisor, None)?;
            sheet_egreso.write_string(row_egreso, 2, &factura.nombre_emisor, None)?;
            sheet_egreso.write_string(row_egreso, 3, &factura.fecha_emision, None)?;
            sheet_egreso.write_number(row_egreso, 4, factura.total, None)?;
            sheet_egreso.write_string(row_egreso, 5, &factura.efecto_comprobante, None)?;
            row_egreso += 1;
        }
    }

    workbook.close()?;
    Ok(())
}
