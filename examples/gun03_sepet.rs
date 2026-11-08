use std::fmt;

struct Sepet(Vec<f64>);

impl fmt::Display for Sepet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fiyatlar = &self.0; // zaten referans olarak aldığımız nesnenin bir parçasını daha referans olarak alıyoruz.
                      // Benzetme kütüphaneden alınan ödünç bir kitabın sayfaları fotoğraf olarak çekilebilir ancak yırtılıp sahipliği alınamaz.
        write!(f, "[")?;
        for (index, v) in fiyatlar.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{index}: {v:.2}")?;
        }
        write!(f, "]")

    }
}
fn main() {
    let sepet1 = Sepet(vec![12.50, 45.00, 350.00]);
    println!("{}", sepet1);
}