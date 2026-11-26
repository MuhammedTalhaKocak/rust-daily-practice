enum SiparisDurumu {
    Hazirlaniyor,
    Kargoda(String),
    TeslimEdildi,
    Iptal { sebep: String },
    IadeEdildi { sebep: String },
}
fn aciklama(d: &SiparisDurumu) -> String {
    match d {
        SiparisDurumu::Hazirlaniyor => String::from("Siparişiniz hazırlanıyor."),
        SiparisDurumu::Kargoda(firma) => format!("Siparişiniz {} ile kargoya verildi.", firma),
        SiparisDurumu::TeslimEdildi => String::from("Kargonuz teslim edildi."),
        SiparisDurumu::Iptal { sebep } => format!("Siparişiniz iptal edildi. Sebep: {}", sebep),
        SiparisDurumu::IadeEdildi { sebep } => {
            format!("Siparişinizi {} sebebinden dolayı iade ettiniz.", sebep)
        }
    }
}
fn iptal_edilebilir_mi(d: &SiparisDurumu) -> bool {
    match d {
        SiparisDurumu::Hazirlaniyor => true,
        _ => false,
    }
}
fn main() {
    let siparis_durumlari = [
        SiparisDurumu::Hazirlaniyor,
        SiparisDurumu::Kargoda("DHL".to_string()),
        SiparisDurumu::TeslimEdildi,
        SiparisDurumu::Iptal {
            sebep: "Evde kimsenin olmaması".to_string(),
        },
        SiparisDurumu::IadeEdildi {
            sebep: "Beden uyuşmazlığı".to_string(),
        },
    ];
    for durum in &siparis_durumlari {
        println!(
            "Sipariş durumu: {} Siparişin iptal edilebilme durumu: {}",
            aciklama(durum),
            iptal_edilebilir_mi(durum)
        );
    }
}

// Derleyici IadeEdildi durumunun match bloğu içinde ele alınmamasından dolayı hata verdi. Hata aşağıda
// error[E0004]: non-exhaustive patterns: `&SiparisDurumu::IadeEdildi { .. }` not covered
