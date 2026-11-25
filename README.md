# rust-daily-practice

Rust'ı her gün küçük adımlarla, kod yazarak öğrendiğim çalışma reposu. Konular FreeCodeCamp Rust kursunun müfredatını takip ediyor; her görev elle yazılıyor, kontrol ediliyor ve commit'leniyor.

---

## Günlük akış (30–45 dk)

1. **Rustlings** (15–20 dk): O günün bölümü. Önce hatayı oku, `h` (ipucu) son çare.
2. **Görev** (20–30 dk): `examples/gunXX_isim.rs` dosyasına yaz.
3. **Kontrol:** `cargo fmt` → `cargo clippy --example gunXX_isim` → `cargo run --example gunXX_isim`
4. **Review:** Kodu Claude'a gönder, geri bildirimi uygula.
5. **Commit + push:**
   - İlk hal: `day XX: <konu> - <ne yapıldı>`
   - Düzeltme: `day XX: fix <ne düzeltildi>`

## Kurallar

- Plan **30 gün boyunca değişmez**. "Ne yapmalıyım?" diye sorulmaz, sadece "Gün X bitti" denir.
- Değerlendirme yalnızca **blok sonlarında** yapılır.
- Takılınca önce **15 dk tek başına uğraş**, sonra yardım iste.
- Yazdığım her satırı açıklayabilmeliyim. Açıklayamıyorsam o satır benim değil.
- Henüz görmediğim konulara atlamıyorum (jump-ahead yok).
- Rust için tek sohbet, tek plan: bu README.

---

## Tamamlananlar

| Gün | Konu | Görev | Durum |
|---|---|---|---|
| 1 | Formatted print | Fiş yazdırıcı | ✅ |
| 2 | Debug ve Display | Sipariş yazdırma | ✅ |
| 3 | Display (liste), `?` | Sepet yazdırma | ✅ |
| 4 | Literals ve operators | Kuruş hesabı | ✅ |
| 5 | Tuples | Tuple ile indirim | ✅ |
| 5b | Overflow | `checked_` aritmetiği | ✅ |
| 6 | **Mini proje 0** | Kasa fişi | ✅ |

---

## 30 Günlük Plan

### Blok 1 — Slices, Ownership, Borrowing, Strings

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 7 | Arrays ve slices | `vecs` | Haftalık satış özeti | ✅ |
| 8 | Ownership, move, clone | `move_semantics` 1–3 | Ürün listesinin sahipliğini fonksiyonlar arasında taşıma | ✅ |
| 9 | Borrowing (`&`, `&mut`) | `move_semantics` 4–5 | Stok güncelleme (mutable borrow) | ⬜ |
| 10 | `String` ve `&str`, string metotları, slicing | `strings` | Ürün kodu ayrıştırıcı | ⬜ |
| 11 | **Mini proje 1** | `structs` 1 | Ürün etiketi metin işleyici | ⬜ |

### Blok 2 — Struct, impl, Enum, match, Option

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 12 | Struct, method, `impl` | `structs` 2–3 | Sepet struct'ı ve metotları | ⬜ |
| 13 | Enum ve `match` | `enums` | Sipariş durumu (state) | ⬜ |
| 14 | `Option` | `options` | Ürün arama (bulunamazsa `None`) | ⬜ |
| 15 | **Mini proje 2** | `modules` | Sipariş durum takip sistemi | ⬜ |

### Blok 3 — Flow control, döngüler, ileri pattern matching

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 16 | `loop`/`while`/`for`, label, değerli `break` | `hashmaps` 1–2 | Stok bitene kadar satış simülasyonu | ⬜ |
| 17 | Guard, `@` binding, iç içe destructuring | `hashmaps` 3 + `quiz2` | Kampanya kuralı eşleştirici | ⬜ |

### Blok 4 — Error handling

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 18 | `Result`, parse, `match` ile hata | `error_handling` 1–3 | Kullanıcı girdisinden fiyat okuma | ⬜ |
| 19 | `?` operatörü, error propagation, `main`'de Result | `error_handling` 4–6 | Çok adımlı sipariş doğrulama | ⬜ |
| 20 | Custom error enum, error'lar için `From` | `conversions` 1–2 | Kendi hata tipin | ⬜ |
| 21 | **Mini proje 3** | `conversions` 3–5 | CSV satırı parser (bozuk satırı atla ve logla) | ⬜ |

### Blok 5 — Collections

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 22 | Vec metotları (`retain`, `sort`, `dedup`…) | `generics` | Sepet temizleme ve sıralama | ⬜ |
| 23 | HashMap, entry API | `traits` 1–2 | Kategori bazlı satış sayımı | ⬜ |
| 24 | **Mini proje 4** | `traits` 3–5 | Event type sayacı (view/cart/purchase) | ⬜ |

### Blok 6 — Traits ve Generics

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 25 | Trait tanımlama, default method | `quiz3` | Fiyatlandırılabilir ürünler | ⬜ |
| 26 | Generics, trait bounds | `lifetimes` | Generic en yüksek/en düşük fonksiyonu | ⬜ |
| 27 | Trait objects, `Box<dyn Trait>` | `tests` | Farklı ürün tiplerini tek listede tutma | ⬜ |
| 28 | **Mini proje 5** | `iterators` 1–2 | İndirim kuralları motoru | ⬜ |

### Blok 7 — Iterators ve Closures

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 29 | `iter`/`into_iter`/`iter_mut`, adapters (`map`, `filter`) | `iterators` 3–4 | Satış verisini filtreleme | ⬜ |
| 30 | Consuming adapters (`sum`, `fold`, `collect`), zincirleme | `iterators` 5 | Rapor özetleri | ⬜ |
| 31 | Closures, `move`, Fn trait'leri, parametre olarak closure | `smart_pointers` 1–2 | Dinamik filtre kuralları | ⬜ |
| 32 | **Mini proje 6**: custom iterator | `smart_pointers` 3–4 | Zaman penceresi iterator'ı | ⬜ |

### Blok 8 — Lifetimes

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 33 | Lifetime annotation, elision | `threads` 1–2 | Referans döndüren fonksiyonlar | ⬜ |
| 34 | `'static`, referans tutan struct | `threads` 3 | Log satırını kopyalamadan parçalama | ⬜ |

### Blok 9 — Modules ve Crates

| Gün | Konu | Rustlings | Görev | Durum |
|---|---|---|---|---|
| 35 | Modules, `pub`, dosyalara bölme | `macros` + `clippy` | Kasa fişini modüllere bölme | ⬜ |
| 36 | **Final proje** | Kalan egzersizler | Çok dosyalı mini sipariş uygulaması | ⬜ |

---

## Blok sonu değerlendirmeleri

Her bloğun son gününde: neyi rahat yazıyorum, nerede takılıyorum, tekrar gereken konu var mı? Notlar buraya eklenir.

## 30 gün sonra

Temel tamamlanınca clickstream anomaly projesine dönüş (concurrency/tokio dahil) değerlendirilecek.
