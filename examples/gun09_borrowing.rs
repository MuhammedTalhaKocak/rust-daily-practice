fn listele(liste: &[String]) {
    for urun in liste {
        println!("{}", urun);
    }
}

// değiştirilebilir bir u32 tip dataları tutan stoklar listesi al.
// hangi elemanı değiştireceksin, indeks = usize olmalı.
// Stok ne kadar azalacak? kullanıcı miktar vermeli, çıkarma işlemi yapılabilmesi için
// kullanıcının vereceği miktar u32 olmalı.
// u32 negatif sayı tutamadığı için eğer güncel stok - miktar < 0 ise saturating_sub fonksiyonu programı panikletmek
// yerine sonucu u32'in alabileceği en düşük değere eşitler.
fn stok_azalt(stoklar: &mut [u32], indeks: usize, miktar: u32) {
    stoklar[indeks] = stoklar[indeks].saturating_sub(miktar);
}
fn main() {
    let urunler = vec![
        String::from("Kalem"),
        String::from("Defter"),
        String::from("Çanta"),
    ];
    let mut stoklar: Vec<u32> = vec![10, 2, 25];
    listele(&urunler); // veriyi ödünç verdik, sahiplik bizde.
    println!("{:#?}", urunler); // bu satırdaki kod çalışıyorsa, sahipliğin bizde olduğu kanıtlanmıştır.
    println!("Önce: {:?}", stoklar);
    stok_azalt(&mut stoklar, 1, 5);
    println!("Sonra: {:?}", stoklar);

    let r1 = &stoklar;
    println!("{:#?}", r1); // misafir ödünç alma işlemini tamamladı.
    let _r2 = &mut stoklar; // şimdi bir başka misafir ödünç alacak ama bu misafirin değişim yapma hakkı da var.
                           // println!("{:#?}", r1); >> Bu satırı çalıştırmayın, kod derlenmez.
                           // Hata kodu: E0502
                           // Hata sebebi, RUST aynı anda sayısız misafir'e izin verirken, aynı anda hem misafir'e hem de değişiklik yapacak ustaya izin vermez.
                           // RUST der ki, önce usta değişiklik yapsın ardından sen misafirliğe gel.
                           // Ya da önce misafirler evden gitsin, sonra usta tadilata başlasın.

    // Clone ile & Farkı
    // Clone orijinal verinin kopyasını oluşturur, ancak büyük miktardaki verileri kopyalamak bilgisayarı yorar.
    // & ise verinin sahipliğini almadan, veriyi okumaya, yazdırmaya hatta &mut ifadesi ile üzerinde değişiklik yapmaya yarar.
    // örneğin & sadece, kitabı ödünç almak demektir.
    // &mut ise, ödünç aldığın kitabın üzerinde istediğin değişiklikleri yapabilirsin demektir. Orijinal kitap değişmiş olur.
}
