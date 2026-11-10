fn kurus_to_tl(k:u32) -> (u32, u32) {
    let tl = k / 100; // 100 kuruş = 1 tl, elimizdeki x kuruş içinde kaç tl olduğunu hesaplıyoruz.
    let kurus = k % 100; // 100 kuruş = 1 tl, elimizdeki x kuruş içinde kaç tl olduğunu hesapladıktan sonra geriye kaç kuruş kaldığına bakıyoruz.
    (tl, kurus) // Sonuç olarak (u32, u32) tuple döndürüyoruz.
}
fn indirim(toplam:u32, oran:u32) -> (u32, u32) {
    let odenecek = toplam * (100 - oran) / 100;
    let indirim_tutari =  toplam - odenecek;
    (odenecek, indirim_tutari)
}
fn main(){
    let toplam_ucret = 40750;
    let oran = 15;
    let (odenecek, indirim_tutari) = indirim(toplam_ucret, oran); // destructuring ile tuple'ın içinde dönen sayıları ayrı ayrı değişkenlere atıyoruz.
    let (odenecek_tl, odenecek_kurus) = kurus_to_tl(odenecek);
    let (indirim_tl, indirim_kurus) = kurus_to_tl(indirim_tutari);
    // {:02} -> Kurusu her zaman 2 basamaklı yazar, eksikse sola 0 koyar.
    println!("İndirim: {}.{:02} TL | Ödenecek: {}.{:02} TL", indirim_tl, indirim_kurus, odenecek_tl, odenecek_kurus);
    println!("{:?}", (5u32));
    println!("{:?}", (5u32,));
    // RUST (5u32) ifadesini u32 veri tipinde 5 sayısı olarak kabul edip normal biçimde yazdırabilirken
    // (5u32,) ifadesini tek elemanlı bir tuple olarak kabul ettiği için normal biçimde yazdırmaya izin vermiyor.
    // bu tuple için {:?} veya {:#?} uygulanarak yazdırma işlemi yapılabilir.
}