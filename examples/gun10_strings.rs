fn kategori(kod: &str) -> &str {
    &kod[..3]
}
fn ulke(kod: &str) -> &str {
    &kod[kod.len() - 2..]
}
fn main() {
    // 1. Adım: Baştaki/sondaki boşlukları temizleme ve büyük harfe çevirme
    let ham = String::from("  elk-1042-tr  ");
    let mut temiz_kod = ham.trim().to_uppercase();
    println!("Temizlenmiş kod: {}", temiz_kod); // ELK-1042-TR

    // 2. ve 3. Adım: Dilimleme (Slice) fonksiyonlarını kullanma
    println!("Kategori: {}", kategori(&temiz_kod)); // ELK
    println!("Ülke: {}", ulke(&temiz_kod)); // TR

    // 4. Adım: Temizlenmiş koda ekleme yapma (String büyütülebilir)
    temiz_kod.push_str("-V2");
    println!("Güncellenmiş kod: {}", temiz_kod); // ELK-1042-TR-V2

    let _s = "Çanta";
    // println!("{}", &s[..1]);

    // Alınan hata: end byte index 1 is not a char boundary; it is inside 'Ç' (bytes 0..2 of string)
    // RUST'ta stringler UTF-8 kodlaması kullanır. İngilizce ASCII karakterler 1 byte yer kaplarken,
    // Türkçe "Ç" harfi bellekte 2 byte (0. ve 1. indeksler) yer kaplar. &s[0..1] yazdığımızda 'Ç' harfini
    // tam ortasından ikiye bölmeye çalıştığımız için Rust geçersiz karakter oluşmasını önlemek adına programı durdurur.
}
