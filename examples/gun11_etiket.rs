fn kategori(kod: &str) -> &str {
    &kod[..3]
}
fn ulke(kod: &str) -> &str {
    &kod[kod.len() - 2..]
}
fn temizle(ham: &str) -> String {
    ham.trim().to_uppercase()
}
fn etiket(ad: &str, kod: &str) -> String {
    format!(
        "{} | Kategori: {} | Ülke: {}",
        ad.to_uppercase(),
        kategori(kod),
        ulke(kod)
    )
}
fn yurt_disi_say(kodlar: &[String]) -> usize {
    let mut yurtdisi_urun_sayisi = 0;
    for kod in kodlar {
        let ulke_kodu = ulke(kod);
        if ulke_kodu != "TR" {
            yurtdisi_urun_sayisi += 1;
        }
    }
    yurtdisi_urun_sayisi
}
fn surum_ekle(kodlar: &mut [String]) {
    // verinin sahipliği bizde, sadece veriyi üzerinde değişiklik yapabilme yetkisiyle ödünç veriyoruz.
    for kod in kodlar {
        kod.push_str("-V2");
    }
}
fn main() {
    let ham_kodlar = [
        "  elk-1042-tr ",
        "kts-2001-de  ",
        " aks-0300-tr",
        "kts-0042-fr ",
    ];
    let adlar = ["kalem", "defter", "çanta", "cetvel"];
    let mut kodlar = Vec::new();
    for ham_kod in ham_kodlar {
        kodlar.push(temizle(ham_kod))
    }
    for (ad, kod) in adlar.iter().zip(kodlar.iter()) {
        println!("{}", etiket(ad, kod));
    }
    let yurt_disi_urun_sayisi = yurt_disi_say(&kodlar);
    println!("Yurt dışı ürün sayısı: {}", yurt_disi_urun_sayisi);
    surum_ekle(&mut kodlar);
    println!("{:?}", kodlar);
}
