fn main(){
    let urunler = vec![
        ("Kalem", 3, 12.5),
        ("Defter", 2, 45.00),
        ("Çanta", 1, 350.00),
    ];
    // :<13 toplam 13 karakter sola yaslı
    // :<4 toplam 4 karakter sola yaslı
    // :>9 toplam 9 karakter sağa yaslı
    let mut toplam = 0.0;
    println!("{:<13} {:<4} {:>9}", "Ürün", "Adet", "Fiyat"); // : > sağa yaslı, < sola yaslı.
    for (urun, adet, fiyat) in &urunler{
        println!("{:<13} {:>4} {:>9.2}", urun, adet, fiyat);
        toplam += *adet as f64 * *fiyat;

    }
    // Ürün sola yaslı toplam 13 karakter
    println!("{}", "-".repeat(28));
    println!("{:<13} {toplam:>14.2}", "TOPLAM");
    // let sayi = 34.90595995959;
    // println!("{:08.3}", sayi); // toplam 8 karakter, virgülden sonra 3 rakam. eksik varsa 0 ile doldur.
    // Beklenen çıktı: 0034.906
}