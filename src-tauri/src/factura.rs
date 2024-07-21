#[derive(Debug)]

pub struct Factura {
    pub folio_fiscal: String,
    pub rfc_emisor: String,
    pub nombre_emisor: String,
    pub rfc_receptor: String,
    pub nombre_receptor: String,
    pub fecha_emision: String,
    pub subtotal: f64,
    pub iva: f64,
    pub total: f64,
    pub efecto_comprobante: String,
    pub tipo_factura: String,
}

impl Factura {
    pub fn new() -> Self {
        Factura {
            folio_fiscal: String::new(),
            rfc_emisor: String::new(),
            nombre_emisor: String::new(),
            rfc_receptor: String::new(),
            nombre_receptor: String::new(),
            fecha_emision: String::new(),
            subtotal: 0.0,
            iva: 0.0,
            total: 0.0,
            efecto_comprobante: String::new(),
            tipo_factura: String::new(),
        }
    }
    pub fn set_efecto_comprobante(&mut self, efecto: &str) {
        self.efecto_comprobante = match efecto {
            "Ingreso" | "I" => "Ingreso".to_string(),
            "Egreso" | "E" => "Egreso".to_string(),
            "Pago" | "P" => "Pago".to_string(),
            _ => "Desconocido".to_string(),
        };
    }
}
