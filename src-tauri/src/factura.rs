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
    pub metodo_pago: String,
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
            metodo_pago: String::new(),
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
    pub fn set_metodo_pago(&mut self, metodo_pago: &str) {
        self.metodo_pago = match metodo_pago {
            "PUE" => "Pago una exhibicion".to_string(),
            "PPD" | "Pago en parcialidades" => "Egreso".to_string(),
            _ => "Desconocido".to_string(),
        };
    }
    pub fn set_iva(&mut self, total: f64, subtotal: f64) {
        self.iva = total - subtotal;
    }
}
