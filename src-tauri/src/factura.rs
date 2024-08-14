#[derive(Debug)]

pub struct Impuesto {
    pub importe: f64,
    pub tasa_o_cuota: f64,
}

impl Impuesto {
    pub fn new() -> Self {
        Impuesto {
            importe: 0.0,
            tasa_o_cuota: 0.0,
        }
    }
}

pub struct Iva {
    pub iva_16: f64,
    pub iva_ret_4: f64,
    pub iva_ret_10: f64,
}

pub struct Isr {
    pub isr_ret_1: f64,
    pub isr_ret_10: f64,
}

pub struct Factura {
    pub folio_fiscal: String,
    pub rfc_emisor: String,
    pub nombre_emisor: String,
    pub rfc_receptor: String,
    pub nombre_receptor: String,
    pub fecha_emision: String,
    pub subtotal: f64,
    pub ieps: f64,
    pub isr: Isr,
    pub iva: Iva,
    pub ish: f64,
    pub total: f64,
    pub metodo_pago: String,
    pub efecto_comprobante: String,
    pub tipo_factura: String,
    pub descuento: f64,
    pub moneda: String,
    pub es_gasolina: bool,
    pub forma_pago: String,
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
            ieps: 0.0,
            isr: Isr {
                isr_ret_1: 0.0,
                isr_ret_10: 0.0,
            },
            iva: Iva {
                iva_16: 0.0,
                iva_ret_4: 0.0,
                iva_ret_10: 0.0,
            },
            ish: 0.0,
            total: 0.0,
            metodo_pago: String::new(),
            efecto_comprobante: String::new(),
            tipo_factura: String::new(),
            descuento: 0.0,
            moneda: String::new(),
            es_gasolina: false,
            forma_pago: String::new(),
        }
    }
    pub fn set_efecto_comprobante(&mut self, efecto: &str) {
        self.efecto_comprobante = match efecto {
            "Ingreso" | "I" => "Ingreso".to_string(),
            "Egreso" | "E" => "Egreso".to_string(),
            "Pago" | "P" => "Pago".to_string(),
            "Nomina" | "N" => "Nomina".to_string(),
            _ => "Desconocido".to_string(),
        };
    }
    pub fn set_forma_pago(&mut self, codigo: String) {
        self.forma_pago = match codigo.as_str() {
            "01" => "Efectivo".to_string(),
            "02" => "Cheque nominativo".to_string(),
            "03" => "Transferencia electrónica de fondos".to_string(),
            "04" => "Tarjeta de crédito".to_string(),
            "05" => "Monedero electrónico".to_string(),
            "06" => "Dinero electrónico".to_string(),
            "08" => "Vales de despensa".to_string(),
            "12" => "Dación en pago".to_string(),
            "13" => "Pago por subrogación".to_string(),
            "14" => "Pago por consignación".to_string(),
            "15" => "Condonación".to_string(),
            "17" => "Compensación".to_string(),
            "23" => "Novación".to_string(),
            "24" => "Confusión".to_string(),
            "25" => "Remisión de deuda".to_string(),
            "26" => "Prescripción o caducidad".to_string(),
            "27" => "A satisfacción del acreedor".to_string(),
            "28" => "Tarjeta de débito".to_string(),
            "29" => "Tarjeta de servicios".to_string(),
            "30" => "Aplicación de anticipos".to_string(),
            "31" => "Intermediario pagos".to_string(),
            "99" => "Por definir".to_string(),
            "" => "Desconocido".to_string(),
            _ => "Desconocido".to_string(),
        };
    }

    pub fn set_es_gasolina(&mut self, clave_producto: String) {
        self.es_gasolina = clave_producto.contains("151015");
    }
    pub fn set_ieps(&mut self, subtotal: f64, iva: f64) {
        self.ieps = subtotal - (iva / 0.16);
    }
    pub fn set_impuesto(&mut self, porcentaje: f64, importe: f64) {
        if porcentaje == 0.160000 {
            self.iva.iva_16 += importe;
        } else if porcentaje == 0.040000 {
            self.iva.iva_ret_4 += importe * -1.0;
        } else if porcentaje == 0.106667 {
            self.iva.iva_ret_10 += importe * -1.0;
        } else if porcentaje == 0.012500 {
            self.isr.isr_ret_1 += importe * -1.0;
        } else if porcentaje == 0.100000 {
            self.isr.isr_ret_10 += importe * -1.0;
        }
    }
    pub fn print_factura(&mut self) {
        println!("<------------------ Factura ------------------>");
        println!("Folio fiscal: {}", self.folio_fiscal);
        println!("RFC Emisor: {}", self.rfc_emisor);
        println!("Nombre Emisor: {}", self.nombre_emisor);
        println!("RFC Receptor: {}", self.rfc_receptor);
        println!("Nombre Receptor: {}", self.nombre_receptor);
        println!("Fecha de emision: {}", self.fecha_emision);
        println!("Subtotal: {}", self.subtotal);
        println!("IEPS: {}", self.ieps);
        println!("ISR Retenido 1%: {}", self.isr.isr_ret_1);
        println!("ISR Retenido 10%: {}", self.isr.isr_ret_10);
        println!("IVA 16%: {}", self.iva.iva_16);
        println!("IVA Retenido 4%: {}", self.iva.iva_ret_4);
        println!("IVA Retenido 10.6667%: {}", self.iva.iva_ret_10);
        println!("ISH 2%: {}", self.ish);
        println!("Total: {}", self.total);
        println!("Metodo de pago: {}", self.metodo_pago);
        println!("Efecto comprobante: {}", self.efecto_comprobante);
        println!("Tipo de factura: {}", self.tipo_factura);
        println!("Descuento: {}", self.descuento);
        println!("Moneda: {}", self.moneda);
        println!("Es gasolina: {}", self.es_gasolina);
        println!("Forma de pago: {}", self.forma_pago);
        println!("-----------------------------------------------")
    }
}
