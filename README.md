# Topographic Execution Substrate (TES)

> **"Behaviour-centric → Topology-centric"**

TES, dijital sistemler için yeni bir zihin değil; **yeni bir coğrafya** önerir.

## Nedir?

TES bir runtime, programlama dili veya agent framework **değildir**.

Amaç: Varlıkların **nerede, ne kadar ve ne süreyle var olabileceğini** belirleyen edilgen bir uzay sunmaktır.

## Kavramsal Temel

| Kavram | Açıklama |
|--------|----------|
| **Space** | Atemporal, edilgen topografik alan |
| **Shape** | Sınırlı hafızalı taşıyıcı |
| **Trace** | Varoluşun yan ürünü (side-effect) — skaler yoğunluk alanı |
| **Isotope** | Servis türlerini ayırt eden RGB renk imzası |

## Özgün Katkılar

- **Identity-Free Coordination** — literatürdeki tek model
- **Source Amnesia** — kimin iz bıraktığı bilinmez (tasarım gereği)
- **No Rollback** — decay ile sönümlenme, geri alma yok
- **Linda + Reaction-Diffusion** — koordinasyon modeli hibrit

## Hızlı Başlangıç

```rust
use tes::{Substrate, IsotopeGrid, ServiceColor};

// Basit substrat oluştur
let mut substrate = Substrate::new(100, 100, 5, 1000);

// Shape spawn et
substrate.spawn(50, 50, 100, 10);

// Simülasyon çalıştır
substrate.run(100);

// Spektroskopik analiz için
let grid = IsotopeGrid::new(100, 100, 5, 1000, 500);
let auth = ServiceColor::from_name("AuthService");
grid.contribute(50, 50, 99, auth);
```

## Dokümanlar

| Doküman | Açıklama |
|---------|----------|
| [project.md](docs/project.md) | Ana spesifikasyon |
| [formalization.md](docs/formalization.md) | Matematiksel formalizasyon |
| [comparative_analysis.md](docs/comparative_analysis.md) | Literatür karşılaştırması |
| [gui.md](docs/gui.md) | Spektroskopik arayüz tasarımı |

## Literatürdeki Pozisyon

```
TES = Linda ∩ Reaction-Diffusion − Rules
    = Generative Communication + Continuous Decay − Explicit Retrieval
```

## Test

```bash
cargo test
```

## Lisans

MIT

---

*Bu çalışma akademik ve deneysel niteliktedir. Ürünleşme, genel amaçlı kullanım veya evrensel hesaplama iddiası yoktur.*
