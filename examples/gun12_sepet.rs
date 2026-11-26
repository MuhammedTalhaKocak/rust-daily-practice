// Urun Struct'ı ad (String), fiyat(u32) ve adet(u32)'den oluşsun.
struct Urun {
    ad: String,
    fiyat: u32,
    adet: u32,
}
struct Sepet {
    urunler: Vec<Urun>,
}
impl Sepet {
    // Associated function, bir struct'a ait yeni nesneler oluşturmayı sağlar.
    // method ise Struct üzerinden yazılan fonksiyonlardır.
    fn new() -> Self {
        Sepet {
            urunler: Vec::new(),
        }
    }
    // Sepet'in sahipliğini alma, urun'un sahipliğini al ve ürünü sepete ekle.
    fn ekle(&mut self, urun: Urun) {
        self.urunler.push(urun)
    }
    fn toplam(&self) -> u32 {
        let mut sepet_toplam_tutari = 0;
        for urun in &self.urunler {
            sepet_toplam_tutari += urun.adet * urun.fiyat
        }
        sepet_toplam_tutari
    }
    fn cesit_sayisi(&self) -> usize {
        self.urunler.len()
    }
}
fn main() {
    let mut benim_sepetim = Sepet::new();
    benim_sepetim.ekle(Urun {
        ad: "Kalem".to_string(),
        fiyat: 12,
        adet: 3,
    });
    benim_sepetim.ekle(Urun {
        ad: "Defter".to_string(),
        fiyat: 45,
        adet: 2,
    });
    benim_sepetim.ekle(Urun {
        ad: "Çanta".to_string(),
        fiyat: 350,
        adet: 1,
    });
    println!(
        "Toplam: {} TL | Çeşit: {}",
        benim_sepetim.toplam(),
        benim_sepetim.cesit_sayisi()
    );
}
