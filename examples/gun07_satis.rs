fn kurus_to_tl(kurus: u32) -> (u32, u32) {
    let tam_kisim = kurus / 100; // Bölmenin tam kısmı.
    let kuruslu_kisim = kurus % 100; // Bölme sonucu kalan.
    (tam_kisim, kuruslu_kisim)
}
fn tl_formatla(kurus: u32) -> String {
    let (tam_kisim, kuruslu_kisim) = kurus_to_tl(kurus);
    format!("{}.{:02}", tam_kisim, kuruslu_kisim)
}
fn ozet(satislar: &[u32]) -> (u32, u32, u32) {
    // içinde u32 tipini barındıran bir array ödünç alınıyor. Sahipliği hala kendisinde !!
    let mut toplam = 0;
    let mut en_dusuk = satislar[0]; // 1250 TL
    let mut en_yuksek = satislar[0]; // 1250 TL
    for &satis in satislar {
        if en_dusuk > satis {
            en_dusuk = satis
        }
        if en_yuksek < satis {
            en_yuksek = satis
        }
        toplam += satis;
    }
    (toplam, en_dusuk, en_yuksek)
}
fn yazdir(baslik: &str, satislar: &[u32]) {
    println!("{}", baslik);
    let (toplam, en_dusuk, en_yuksek) = ozet(satislar);
    println!(
        "Toplam: {} TL | En Düşük: {} TL | En Yüksek: {} TL",
        tl_formatla(toplam),
        tl_formatla(en_dusuk),
        tl_formatla(en_yuksek),
    );
}
fn main() {
    let haftalik: [u32; 7] = [125000, 98050, 143075, 110000, 167525, 210000, 189950]; // [T; Length]
    yazdir("Haftalık Genel Durum", &haftalik);
    println!();
    yazdir("Hafta İçi Genel Durum", &haftalik[..5]);
    println!();
    yazdir("Hafta Sonu Genel Durum", &haftalik[5..]);
    println!();
    println!("3.günün satışı: {:?}", haftalik.get(2));
    println!("11.günün satışı: {:?}", haftalik.get(10));
    // println!("11.günün satışı: {:?}", haftalik[10]); // Kod bu satırdan dolayı derlenmez, çünkü array'in boyutu compile-time'da bilindiğinden RUST köşeli parantez içindeki değer, array'in length'ini aşıyorsa, kodu derlemez.
}

/*
Array vs Slice
Slice; verinin referansını alan, veriye bakış atan bir pencere, boyutu runtime'da belli oluyor.
Array, boyutu sabit-compile time'da belli, verinin sahibi.
*/
