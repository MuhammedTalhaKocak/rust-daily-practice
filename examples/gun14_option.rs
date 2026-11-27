// Urun adında bir struct oluştur. 2 field'dan oluşsun.
// 1. Field = ad: String
// 2. Field = stok: u32
struct Urun {
    ad: String,
    stok: u32,
}
// stok_bul fonksiyonu ürün varsa stoğunu Some içinde, yoksa None döndürsün
fn stok_bul(urunler: &[Urun], ad: &str) -> Option<u32> {
    // liste üzerinde gez. Kullanıcının adını yazdığı ürün listedeki ürünlerden biriyle eşleşiyorsa
    // o ürünün stok'unu döndür.
    for urun in urunler {
        if urun.ad == ad {
            return Some(urun.stok);
        }
        // return anahtar kelimesi sayesinde değer döndürüldü ve o anda döngüden çıkıldı.
    }
    // döngü bitti, ilgili ürün bulunamadı >> None döndür.
    None
}
fn stok_yazdir(urunler: &[Urun], ad: &str) {
    match stok_bul(urunler, ad) {
        Some(stok) => println!("İlgili ürünün stok adedi: {}", stok),
        None => println!("İlgili ürün bulunamadı."),
    }
}
fn main() {
    let urunler = [
        Urun {
            ad: "Kalem".to_string(),
            stok: 10,
        },
        Urun {
            ad: "Defter".to_string(),
            stok: 0,
        },
        Urun {
            ad: "Çanta".to_string(),
            stok: 5,
        },
    ];
    stok_yazdir(&urunler, "Defter");
    stok_yazdir(&urunler, "Silgi");
    if let Some(stok) = stok_bul(&urunler, "Çanta") {
        println!("İlgili ürünün stok adedi: {}", stok);
    }
}
