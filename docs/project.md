# Topographic Execution Substrate (TES)

## 1. Amaç ve Kapsam

Bu doküman, davranış merkezli yazılım modellerine alternatif olarak geliştirilen, **topografik ve alan-temelli bir işlem katmanının** kavramsal ve teknik spesifikasyonunu tanımlar. TES bir runtime, programlama dili veya agent framework değildir. Amaç; varoluşların **nerede, ne kadar ve ne süreyle var olabileceğini** belirleyen edilgen bir uzay sunmaktır. Sistem semantik üretmez, karar vermez ve davranış icra etmez; yalnızca yaşanabilirlik koşullarını uygular.

Bu çalışma akademik ve deneysel niteliktedir. Ürünleşme, genel amaçlı kullanım veya evrensel hesaplama iddiası yoktur.

## 2. Ontolojik Aksiyom

Temel varsayım **act → space** şeklinde ifade edilir. Ontolojik olarak eylem uzayı doğurur; ancak mühendislikte bu ilişki tersinden temsil edilir. TES, eylemin kendisini değil, eylemin izin verilen izdüşümünü tanımlar. Bu, ontolojik sadakatin operatif uyarlamasıdır.

> **Not:** `act` yalnızca bu ontolojik aksiyomda kullanılır ve TES'in kapsam dışı başlangıç koşulunu ifade eder.

TES'in sunduğu uzay, üç boyutlu fiziksel bir koordinat sistemi değildir. Yüksek boyutlu, vektörel ve topografik bir durum alanıdır. Bu alan semantik taşımaz; yalnızca metrik ve geometrik özelliklere sahiptir.

## 3. Sistem Modeli

TES, mevcut dijital altyapı üzerinde **parazitik** bir katman olarak yaşar. Alt katmandaki CPU, thread, function call, scheduler ve benzeri mekanizmalar sistemin ontolojisinin parçası değildir; taşıyıcı gürültü olarak kabul edilir. TES bu mekanizmaları adreslemez ve kontrol etmez.

Sistem üç ana kavram etrafında şekillenir:

* Uzay (Topographic Space): Yaşanabilirlik koşullarının tanımlandığı vektörel alan.
* Taşıyıcılar (Shapes): Önceden allocate edilmiş, sınırlı hafızaya sahip, uzay içinde var olabilen yapılar.
* İzler (Traces): Temporal varoluşun uzayda bıraktığı **skaler yoğunluk alanı**. Trace bir data structure değil, topografik **deformasyondur**.

## 4. Uzay Tanımı

Uzay, kullanıcı tarafından tanımlanan **abstract model** üzerinden inşa edilir. Kullanıcı davranış, komut veya kontrol akışı tanımlamaz; yalnızca topografik parametreleri işaretler. Sistem bu parametrelerden hareketle ideal yaşama koşullarını oluşturur.

> Abstract model, uzayın topografik parametrelerini tanımlayan **dışsal bir yapılandırmadır**. DSL, config veya type-level tanım olabilir; TES bunu yorumlar ama doğrulamaz.

Uzayın temel nitelikleri şunlardır:

* Vektörel boyutlar: Anlamsal değil, metrik eksenlerdir.
* Ağırlık (weight): Yerel yoğunluk ve direnç ölçüsü.
* Bozunum (decay): Gözlemsel projeksiyon fonksiyonu — uzayı mutate etmez.
* Yaşam döngüsü (lifecycle): Varlıkların temporal bağının sürdüğü dönem.
* Geçirgenlik ve faz davranışı: Trace birikimini etkileyen rejim katsayıları.

Uzay atemporaldir; **uzayın metriklerine ilişkin tüm değişim, yalnızca temporal gözlem boyunca gözlemlenir**. Uzay kendinde değişmez; değişim, zamansal projeksiyonda görünür.

## 5. Temporal Model

Zaman, olay sırası veya clock-tick olarak ele alınmaz. TES'te zaman **tükenebilir bir kaynak**tır. Temporal kontrol, bozunum ve yaşam döngüsü ile doğrudan ilişkilidir. Zaman ilerlemez; harcanır.

**Temporal gözlem başlangıç koşulu:**

> `temporal_observation(s) ⟺ payload(s) ≠ ∅ ∧ ∃s': related(s, s')`

Temporal gözlem, yalnızca bir shape **payload taşıyor** ve **ilişkisel olarak var** olduğunda başlar. Initialize anında (space ve shape alanları allocate edildiğinde) temporal gözlem yoktur — çünkü henüz payload, ilişki ve maruz kalma yoktur.

Bu yaklaşım, sistemin doğal olarak sönümlenmesini sağlar. Uzun süre yaşayan ama iz üretmeyen yapılar anlamsızlaşır ve ortadan kalkar.

> **Stigmergy (İşaretle Koordinasyon):** TES, biyolojik stigmergy prensibini uygular. Bellek shape'te değil, **ortamda** (uzayın trace yoğunluğunda) saklanır. Karınca feromon bırakan diğer karıncayı bilmez; sadece "burada yoğunluk var" der. TES'te shape'ler yeterince aptal, uzay yeterince duyarlı olduğunda, "akıl" shape'in kodunda değil, **uzayın topografisinde** ortaya çıkar.

## 6. Taşıyıcılar (Shapes)

Taşıyıcılar, uzay içinde var olabilen, sınırlı ve önceden allocate edilmiş hafıza bloklarıdır. Her taşıyıcının:

* Sabit bir hafıza bütçesi,
* Tanımlı bir yaşam süresi,
* Duyarlılık katsayısı (sensitivity) vardır — uzayın shape'e etkisini belirler.

> **Not:** Duyarlılık tek yönlüdür: uzay → shape. Shape uzayı etkilemez.

Taşıyıcılar bilgi taşımaz; bilgiye ev sahipliği yapar. Taşınan bilgi ortadan kalktığında, taşıyıcı da yok olur. Bu ilişki Rust'ta ownership ve lifetime semantiğiyle birebir örtüşür.

Garbage collection yoktur; yalnızca **var olamama** vardır. Ölüm deterministik hesaplanmaz, ancak kaçınılmazlığı deterministiktir.

## 7. Axiom ve Relation İşaretleme

Axiom ve relation kavramları kural veya policy olarak çalışmaz. Bunlar uzay üzerinde **işaretleyici** (marker) niteliğindedir. İhlal durumunda hata, exception veya fallback üretilmez. İhlal edilen yapı yalnızca yaşayamaz hale gelir.

Bu fark, sistemin istemeden policy engine veya rule-based bir yapıya evrilmesini engeller.

## 8. Komutların Türetilmesi

Komutlar birincil varlıklar değildir. Uzay inşa edildikten sonra, yalnızca bu uzayda yaşayabilen komut izdüşümleri ortaya çıkabilir. Komut, uzayın izin verdiği ölçüde vardır. Bu yaklaşım, davranışı değil, davranışın geometrisini merkez alır.

## 9. Uygulama Dili ve Altyapı

Referans implementasyon dili Rust'tır. Bunun nedeni performans değil, hafıza ve yaşam döngüsünün zorunlu olarak açık edilmesidir. Allocation, ownership ve drop semantiği, modelin ontolojik karşılıklarıdır.

Uzay, memory-mapped yapılar, vektörel indeksler ve gerektiğinde GPU buffer'ları üzerinden temsil edilebilir. TES ayrı bir process veya engine olarak değil, sessiz bir arka plan katmanı olarak konumlanır.

## 10. Temporal Clarification

TES'te **uzaysal hareket veya karşılıklı etkileşim yoktur**. Uzay edilgendir ve atemporal kabul edilir; shape uzayı etkilemez. Görülen tüm "etki", yalnızca **zamansal izdüşüm** üzerinde ortaya çıkar.

Bir shape'in varlığı şu şekilde ayrıştırılır:

* **Uzaysal boyut:** Shape, topografik uzayda yalnızca *yaşanabilirlik koşullarına maruz kalan* bir varlık olarak bulunur. Uzayda hareket, geri besleme veya etkileşim yoktur.
* **Zamansal boyut:** Shape, tükenebilir bir temporal kaynak boyunca var olur. Bu süreç boyunca **trace desenleri** oluşur.

Bu nedenle:

* "Yok oluş" (death), uzayın silinmesi değil, **temporal bağın kopmasıdır**.
* Decay, uzayı mutate etmez; decay **gözlemsel projeksiyon fonksiyonudur**. ω immutable kabul edilir.
* TES'te presence, yalnızca temporal düzlemde görülebilen bir varoluş farkıdır.

## 11. Faz Rejimleri

Faz geçişleri, shape'in durumu değil, uzayın o noktadaki **trace davranış rejimidir**:

| Rejim | Tanım |
|-------|-------|
| **Solid** | Düşük geçirgenlik, trace birikimi hızla saturate olur |
| **Liquid** | Orta geçirgenlik, trace dağılır |
| **Gas** | Yüksek geçirgenlik, trace anlamlı desen oluşturmaz |

## 12. Sınırlar ve Bilinçli Kısıtlar

TES:

* Genel amaçlı değildir.
* Turing-tam olmayı hedeflemez.
* Debug edilebilirlik iddiası yoktur.
* Determinizm garanti etmez.
* **Rollback/Replay desteklemez** — Trace side-effect olarak işlendiğinden, hangi shape'in hangi izi bıraktığı bilinmez (Source Amnesia). Bu tasarım gereğidir: geri alma değil, yalnızca decay ile sönümlenme.

Bu kısıtlar kusur değil, tasarımın kendisidir.

## 13. Değer Önerisi

TES'in değeri davranışı akıllandırmasında değil, davranışın **nerede boğulacağını erken göstermesinde** yatar. Özellikle agentic, yüksek eşzamanlı ve adversarial sistemlerde, semantik karar katmanlarından önce gelen bir bastırma mekanizması sunar.

**Somut örnek:** Bir prompt injection saldırısı düşünün. TES prompt'u anlamaz ve semantik analiz yapmaz. Ancak prompt'un oluşturduğu yoğunluk, belirli uzay bölgelerini doygunlaştırır. Sonuç: bazı davranış yolları topografik olarak **yaşayamaz** hale gelir. Bu rate limiting değildir — talep sayısını saymaz, semantiği bilmez, yalnızca topografik boğulma yaratır.

## 14. Konumlandırma

Bu çalışma bir ürün değildir. Akademik ve deneysel bir ontolojik zemin araştırmasıdır. Open-source olarak paylaşılabilir; merak edenler inceleyebilir, tartışabilir ve sınırlarını test edebilir.

TES, dijital sistemler için yeni bir zihin değil; **yeni bir coğrafya** önerir.
