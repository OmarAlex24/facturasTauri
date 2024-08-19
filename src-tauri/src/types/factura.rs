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
    pub uso_cfdi_receptor: String,
    pub clave_producto_servicio: String,
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
            uso_cfdi_receptor: String::new(),
            clave_producto_servicio: String::new(),
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

    pub fn set_uso_cfdi(&mut self, codigo: String) {
        self.uso_cfdi_receptor = match codigo.as_str() {
            "G01" => "Adquisición de mercancías".to_string(),
            "G02" => "Devoluciones, descuentos o bonificaciones".to_string(),
            "G03" => "Gastos en general".to_string(),
            "I01" => "Construcciones".to_string(),
            "I02" => "Mobiliario y equipo de oficina por inversiones".to_string(),
            "I03" => "Equipo de transporte".to_string(),
            "I04" => "Equipo de cómputo y accesorios".to_string(),
            "I05" => "Dados, troqueles, moldes, matrices y herramental".to_string(),
            "I06" => "Comunicaciones telefónicas".to_string(),
            "I07" => "Comunicaciones satelitales".to_string(),
            "I08" => "Otra maquinaria y equipo".to_string(),
            "D01" => "Honorarios médicos, dentales y gastos hospitalarios".to_string(),
            "D02" => "Gastos médicos por incapacidad o discapacidad".to_string(),
            "D03" => "Gastos funerales".to_string(),
            "D04" => "Donativos".to_string(),
            "D05" => "Intereses reales efectivamente pagados por créditos hipotecarios (casa habitación)".to_string(),
            "D06" => "Aportaciones voluntarias al SAR".to_string(),
            "D07" => "Primas por seguros de gastos médicos".to_string(),
            "D08" => "Gastos de transportación escolar obligatoria".to_string(),
            "D09" => "Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones".to_string(),
            "D10" => "Pagos por servicios educativos (colegiaturas)".to_string(),
            "CP01" => "Pagos".to_string(),
            "CN01" => "Nómina".to_string(),
            "S01" => "Sin Efectos Fiscales".to_string(),
            "" => "Desconocido".to_string(),
            _ => "Desconocido".to_string(),
        };
    }
    pub fn set_prod_serv(&mut self, codigo: String) {
        if codigo.len() < 3 {
            self.clave_producto_servicio = "Desconocido".to_string();
            return;
        }
        self.clave_producto_servicio = match &codigo[0..2] {
            "10" | "11" | "12" | "13"  => "Materias primas y químicos".to_string(),
            "14" => "Materiales y productos de papel".to_string(),
            "15" => "Combustibles".to_string(),
            "20" | "21" | "23" | "24" | "26" | "27" => "Herramientas y equipos industriales".to_string(),
            "30" | "31" | "32" | "39" => "Suministros y componentes".to_string(),
            "22" | "25" | "40" => "Suministros y equipos de construcción, edificaciones y transportes".to_string(),
            "41" | "42" | "51" => "Productos farmacéuticos, y suministros y equipos de ensayo, de laboratorio y médicos".to_string(),
            "47" | "48" | "50" => "Suministros y equipos de servicios, limpieza y comida".to_string(),
            "43" | "44" | "45" | "55" => "Suministros y equipos tecnológicos, de comunicaciones y de negocios".to_string(),
            "46" => "Suministros y equipos de defensa y seguridad".to_string(),
            "49" | "52" | "53" | "54" | "56" | "60" => "Suministros y equipos de consumo, domésticos y personales".to_string(),
            "64" | "70" | "71" | "72" | "73" | "76" | "77" | "78" | "80" | "81" | "82" | "83" | "84" | "85" | "86" | "90" | "91" | "92" | "93" | "94" => "Servicios".to_string(),
            "95" => "Bienes Inmuebles".to_string(),
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
