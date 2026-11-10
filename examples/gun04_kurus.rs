fn main(){
    // 1. Bölüm 
    println!("{}", 0.1 + 0.2); // Bilgisayarlar sayıları binary formatta sakladığı için hesaplamalarda çok ufak farklılıklar çıkabilir. Bu durum da bakiye bilgisi gibi hassas bilgilerde ufak da olsa yanlışlığa sebep olabilir.
    let kurus: [u32;3] = [1250, 4500, 35000];
    let ust_limit = 1_000_000_u32; // Daha okunabilir gözükmesi için
    let toplam: u32 = kurus.iter().sum();
    let indirimli_toplam = toplam * 85 / 100;
    let tam_kisim = indirimli_toplam / 100; // Kaç tane 100 var?
    let ondalikli_kisim = indirimli_toplam % 100; // 100 e bölümünden kalan kaç
    println!("{}. {:02} TL", tam_kisim, ondalikli_kisim);
}