use std::fmt;

#[derive(Debug)]
struct Urun {
    ad: String,
    adet: u32,
    birim_kurus: u32,
}
impl fmt::Display for Urun  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (tam_kisim, kuruslu_kisim) = kurus_to_tl(self.birim_kurus);
        let fiyat = format!("{}.{:02}", tam_kisim, kuruslu_kisim);
        write!(f, "{:<13} {:>4} {:>9}", self.ad, self.adet, fiyat)
    }
}
fn kurus_to_tl(kurus: u32) -> (u32, u32) {
    let tam_kisim = kurus / 100;
    let kuruslu_kisim = kurus % 100;
    (tam_kisim, kuruslu_kisim)
}
fn indirim(toplam: u32, oran: u32) -> (u32, u32) {
    let toplam_uygulanan_indirim = (toplam * oran) / 100; // önce indirim tutarını hesapla
    let odenecek_tutar = toplam - toplam_uygulanan_indirim; // ardından toplam ücretten indirim tutarını çıkar.
    (odenecek_tutar, toplam_uygulanan_indirim)
    // yarım kuruş müşteri lehine yuvarlanır
}
fn tl_formatla(kurus: u32) -> String {
    let (tam_kisim, kuruslu_kisim) = kurus_to_tl(kurus);
    format!("{}.{:02}", tam_kisim, kuruslu_kisim)
}
fn main() {
    let urun1 = Urun {
        ad: String::from("Kalem"),
        adet: 3,
        birim_kurus: 1250,
    };
    let urun2 = Urun {
        ad: String::from("Defter"),
        adet: 2,
        birim_kurus: 4500,
    };
    let urun3 = Urun {
        ad: "Çanta".to_string(),
        adet: 1,
        birim_kurus: 35000,
    };
    let urunler = [urun1, urun2, urun3];
    println!("{:<13} {:>4} {:>9}", "Ürün", "Adet", "Fiyat");
    for urun in &urunler{
        println!("{}", urun);
    }
    println!("{}", "-".repeat(28));
    let toplam: u32 = urunler.iter().map(|u| u.birim_kurus * u.adet).sum();
    let ust_limit = 1_000_000_u32; // Üst Limit = 1_000_000 kuruş = 10.000 TL
    if toplam > ust_limit {
        eprintln!("Toplam sepet tutarı ÜST limitten büyük olamaz!");
        return
    }
    let (odenecek_tutar, toplam_uygulanan_indirim) = indirim(toplam, 15);
    let formatli_ara_toplam = tl_formatla(toplam);
    let formatli_indirim = tl_formatla(toplam_uygulanan_indirim);
    let formatli_odenecek_ucret = tl_formatla(odenecek_tutar);
    println!("{:<15} {:>12}", "Ara Toplam", formatli_ara_toplam);
    println!("{:<15} {:>12}", "İndirim (%15)", formatli_indirim);
    println!("{:<15} {:>12}", "ÖDENECEK", formatli_odenecek_ucret);
}