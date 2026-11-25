fn listele(liste: Vec<String>) {
    // Bu fonksiyon bir parametre alacak ve bu parametre stringden oluşan vec olacak.
    for urun in liste {
        println!("{}", urun);
    }
}
fn urun_ekle(mut liste: Vec<String>, yeni_urun: String) -> Vec<String> {
    // değiştirilebilir listeye yeni ürün ekle, ve yeni listeyi döndür.
    liste.push(yeni_urun);
    liste
}
fn main() {
    let urunler = vec![
        String::from("Kalem"),
        String::from("Defter"),
        String::from("Çanta"),
    ];
    // listele(urunler);
    let urunler = urun_ekle(urunler, "Silgi".to_string());
    println!("{:#?}", urunler);
    // println!("{:#?}", urunler);
    // Derleyici hata kodu: E0382,
    // Derleyici hatası: error[E0382]: borrow of moved value: `urunler`
    // Hatanın sebebi; urunler değişkeninin sahipliğinin fonksiyona geçtikten sonra tekrardan yazdırılmaya çalışılması. Fonksiyon çağrısı bitince urunler'de RAM'den silinir ve bu değişkene bir daha ulaşılamaz.
    listele(urunler.clone()); // clone metodu verinin kopyasını fonksiyona verdiğinden asıl veri yaşamaya devam eder.
    // clone metodu heap'teki verinin tamamını yeniden kopyalıyor.
    // Her String için yeni bellek ayrılıyor ve metin baştan yazılıyor. 3 ürün için önemsiz, ama 1 milyon satırlık bir veride ciddi bir maliyet.
    println!("{:#?}", urunler);
    let a = 5;
    let b = a;
    println!("b = {}, a = {}", b, a);
    // RUST'ta tam sayılar (i8, i32, f64, usize ..) gibi ifadeler copy trait'ini uygulayabildiği için değişkene ait değer kopyalanır, değişkenin sahipliği yeni değişkene taşınmaz.
}
