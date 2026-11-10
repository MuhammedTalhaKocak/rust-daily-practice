fn guvenli_indirim(toplam: u32, oran: u32) -> Option<u32> {
    let cikarma_sonucu = 100u32.checked_sub(oran)?;
    let carpim = toplam.checked_mul(cikarma_sonucu)?;
    let sonuc = carpim / 100;
    Some(sonuc)
}
fn main() {
    let a: u32 = 100;
    println!("{:?}", a.checked_sub(30)); // Some(70)  → sorun yok, sonuç 70
    println!("{:?}", a.checked_sub(120)); // None      → taşma oldu
    println!("{:?}", guvenli_indirim(40750, 15));
    println!("{:?}", guvenli_indirim(40750, 120));
}
