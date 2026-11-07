use std::fmt;

#[derive(Debug)]
struct Siparis{
    id:u32,
    adet:u32,
    tutar:f64,
}
impl fmt::Display for Siparis { 
    // Siparis struct'runa Display traitini uygulayalım.
    // &self nesnenin kendisini değil referansını vererek, nesnenin sahipliğini kendimizde koruyoruz.
    // formatter nesnesini değiştirilebilir ve referans olarak vermek istediğimizden &mut formatter şeklinde kullanıyoruz.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "Sipariş #{} | {} adet | {:.2} TL", 
            self.id, self.adet, self.tutar
        )
    }

}
fn main(){
    let siparis1 = Siparis{
        id:1, 
        adet:5, 
        tutar: 10.0,
    };
    let siparis2 = Siparis{
        id:1042, 
        adet:3, 
        tutar: 477.50,
    };
    println!("{:#?}", siparis1); // displaysız, daha okunabilir debug.
    println!("{:?}", siparis1); // displaysız, debug.
    println!("{}", siparis1); // displaylı
    println!("{}", siparis2); // displaylı
}
// :? ile :#? arasındaki fark
// :? Struct'ı düz bir satırda yazar.
// :#? Struct'ı yukarıda oluştuğu görüntüde yani daha okunabilir biçimde yazdırır.
/*
Debug ve Display arasındaki fark

Debug → geliştirici için. Amacı struct’ın içinde ne olduğunu görmek: alan adları, ham değerler (tutar: 10.0). Hata ayıklarken kullanırsın.
Display → son kullanıcı için. Amacı ekranda düzgün ve anlamlı bir çıktı göstermek (477.50 TL). Kullanıcı Siparis { id: ... } görmek istemez.
*/