# Pelan Naik Taraf Ekstrem: Abu_Hanifah_Parameters v2.0 (The Ultimate Bug Bounty Parameter Engine)

Alat parameter sedia ada sebelum ini hanya menemui 2 parameter kerana ia hanya melakukan **satu permintaan GET (Depth = 0)** pada halaman utama sasaran tanpa merayap (*crawling*), mengabaikan skrip inline JS, tidak menyemak `robots.txt`/`sitemap.xml`, hanya mempunyai 54 perkataan kamus heuristik, serta sumber arkib pasif yang terhad.

Pelan ini menggariskan naik taraf menyeluruh **Abu_Hanifah_Parameters v2.0** bagi melangkaui keupayaan **ParamSpider, Katana, Arjun, dan Waybackurls** secara serentak dalam satu binari Rust berprestasi tinggi.

---

## Analisis Jurang (Gap Analysis): Mengapa Hanya 2 Parameter Dijumpai?

| Ciri / Dimensi | Abu_Hanifah_Parameters v1.0 | Katana / ParamSpider / Arjun | Sasaran Naik Taraf Abu_Hanifah_Parameters v2.0 |
| :--- | :--- | :--- | :--- |
| **Penjelajahan Halaman (Crawling)** | Hanya 1 URL (Depth = 0), tiada susulan pautan dalaman | Katana merayap rekursif (Depth 2-3) menjejaki semua `<a href>` | **Recursive Spider Engine** (Depth 1–5, Breadth-First Queue, skop domain tegar) |
| **Peta Laman & Fail Konfigurasi** | Tiada semakan | Katana mengutip `robots.txt` & `sitemap.xml` | **Robots & Sitemap Harvester** automatik mengumpul semua endpoints tersembunyi |
| **Analisis JavaScript** | Terhad kepada fail `.js` luaran (maksimum 20 fail) | Meneliti `<script>` inline dan regex endpoint | Ekstraksi **Inline `<script>` + Regex Endpoint & Query String Penuh** dari fail JS |
| **Sumber Arkib Pasif (OSINT)** | Wayback (HTTP) + AlienVault OTX sahaja | Waybackurls mengutip pelbagai arkib termasuk Common Crawl | Ditambah **URLScan.io API** & **Common Crawl Index API**, serta Wayback HTTPS moden |
| **Enjin Heuristik (Arjun Mode)** | 54 patah perkataan, semakan delta saiz badan sahaja | Arjun ada ribuan kamus, semakan refleksi, kod status, header | **1,000+ Curated Top Bug Bounty Params**, sokongan fail `-w/--wordlist`, dan **Canary Reflection Detection** (XSS Prime Hunter) |

---

## User Review Required

> [!IMPORTANT]
> **Had Lalai Perayap (*Crawler Defaults*)**:
> Secara lalai untuk mengelakkan beban berlebihan atau lambat pada sasaran besar:
> - `--depth`: Nilai lalai adalah `2` (boleh diselaraskan antara `1` hingga `5`).
> - `--max-pages`: Nilai lalai adalah `50` halaman setiap domain.
> Pengguna boleh mengubah nilai ini melalui argumen CLI (cth: `--depth 3 --max-pages 200`).

> [!TIP]
> **XSS Canary Reflection Engine (Keistimewaan Baharu)**:
> Seperti Arjun, enjin ini akan menyuntik nilai ujian canary (cth: `ahfuzz_928`) ke dalam parameter kelompok. Jika nilai tersebut terpantul dalam badan HTML/JSON atau header respons, parameter tersebut ditandakan sebagai `Reflection (High XSS Risk)` secara automatik, menjadikannya sasaran utama untuk `| dalfox pipe`.

---

## Open Questions

> [!NOTE]
> 1. Adakah akhi mahu enjin heuristik aktif menghantar permintaan `POST` (JSON/Form) secara automatik selain daripada `GET`, atau fokus kepada `GET` terlebih dahulu bagi mempercepatkan imbasan? *(Kami cadangkan GET secara lalai dengan flag `--method both` jika mahu POST).*
> 2. Adakah akhi mempunyai senarai kamus perkataan parameter tersendiri (contoh: senarai daripada SecLists atau Assetnote) yang mahu dijadikan rujukan, atau memadai dengan kamus terbina 1,000+ parameter yang kami susun daripada kompilasi HackerOne & Arjun?

---

## Proposed Changes

Struktur modul sedia ada akan diperluas dengan mematuhi **Single Responsibility Principle (SRP)** dan setiap fail kekal **di bawah 250 baris kod**:

```text
c:\Users\mypc\Desktop\Longterm Memory Project\abu_hanifah\Abu_Hanifah_Parameters\
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── models.rs
│   ├── scanner.rs
│   ├── crawler/
│   │   ├── mod.rs
│   │   ├── html_parser.rs        [MODIFY] Sokong skrip inline, atribut form action
│   │   ├── js_miner.rs           [MODIFY] Regex endpoint & query string diperluas
│   │   ├── spider.rs             [NEW] Enjin rekursif (Katana-style)
│   │   └── robots_sitemap.rs     [NEW] Penuai robots.txt & sitemap.xml
│   ├── passive/
│   │   ├── mod.rs
│   │   ├── wayback.rs            [MODIFY] Guna HTTPS & tiada urlkey collapse berlebihan
│   │   ├── otx.rs
│   │   ├── urlscan.rs            [NEW] Enjin penuai URLScan.io API
│   │   └── commoncrawl.rs        [NEW] Enjin penuai Common Crawl index
│   ├── active/
│   │   ├── mod.rs
│   │   ├── wordlist.rs           [MODIFY] Kamus 1,000+ parameter & pembaca fail tersuai
│   │   ├── reflection.rs         [NEW] Pengesan pantulan canary (XSS Hunter)
│   │   └── heuristic.rs          [MODIFY] Integrasi refleksi & dichotomy berbilang faktor
│   ├── aggregator/
│   │   ├── mod.rs
│   │   └── normalizer.rs         [MODIFY] Sokongan tag punca refleksi & keutamaan XSS
│   └── reporter/
│       ├── mod.rs
│       └── cli.rs                [MODIFY] Paparan visual sumber diperkaya
```

---

### 1. Komponen Perayap Web (Crawler & Spider - Katana Style)

#### [MODIFY] [src/crawler/html_parser.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/crawler/html_parser.rs)
- Tambah ekstraksi kandungan skrip inline (`<script>...</script>`) untuk disalurkan terus ke `JsMiner`.
- Ekstraksi pautan internal `<a href>` untuk disalurkan ke antrian (*queue*) Spider.
- Pastikan atribut `<form action="...">` diselesaikan mengikut URL sebenar borang dan bukan sekadar URL halaman asas.

#### [NEW] [src/crawler/robots_sitemap.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/crawler/robots_sitemap.rs)
- Mengutip dan menganalisis `/robots.txt` sasaran (mengekstrak laluan `Disallow:` dan `Allow:` yang mengandungi parameter atau endpoint menarik).
- Mengutip `/sitemap.xml` dan `/sitemap_index.xml` untuk mengutip URL berparameter secara automatik.

#### [NEW] [src/crawler/spider.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/crawler/spider.rs)
- Menguruskan aliran kerja *Breadth-First Search* (BFS) merentasi halaman dalam skop domain yang sama (*Strict In-Scope*).
- Menghormati konfigurasi had kedalaman (`--depth`) dan had jumlah halaman (`--max-pages`).

#### [MODIFY] [src/crawler/js_miner.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/crawler/js_miner.rs)
- Memperluas corak regex untuk menangkap:
  - Panggilan API penuh: `fetch('/api/v1/user?id=' + val)`, `$.ajax({ url: '/search', data: { q: '...' } })`.
  - Pembolehubah objek parameter: `{ params: { filter: ..., sort: ... } }`.
  - Ekstraksi rentetan laluan berparameter dalam bundle webpack/vite/next.js.

---

### 2. Komponen Arkib Pasif (Supercharged Passive OSINT - ParamSpider & Waybackurls Style)

#### [NEW] [src/passive/urlscan.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/passive/urlscan.rs)
- Menghantar permintaan ke `https://urlscan.io/api/v1/search/?q=domain:{}&size=100`.
- Mengekstrak semua URL sejarah imbasan laman tersebut dan menyaring parameter.

#### [NEW] [src/passive/commoncrawl.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/passive/commoncrawl.rs)
- Menghantar pertanyaan ke indeks terkini Common Crawl (`index.commoncrawl.org/CC-MAIN-...`) untuk domain sasaran.

#### [MODIFY] [src/passive/wayback.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/passive/wayback.rs)
- Menggunakan `https://` yang selamat dan stabil untuk Archive.org.
- Mengoptimumkan parameter carian CDX supaya tidak memotong kepelbagaian query parameter.

---

### 3. Komponen Heuristik & Pantulan Aktif (Arjun Style)

#### [MODIFY] [src/active/wordlist.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/active/wordlist.rs)
- Membesarkan kamus parameter bawaan daripada 54 perkataan kepada **1,000+ perkataan bernilai tinggi** (XSS candidates, SSRF, SQLi, IDOR, LFI, Admin, Debug, OAuth, API tokens).
- Menyediakan fungsi membaca kamus luaran pengguna melalui argumen `-w / --wordlist <FAIL>`.

#### [NEW] [src/active/reflection.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/active/reflection.rs)
- Enjin khusus penyuntikan canary dinamik (cth: `ahsec<RANDOM>`).
- Menyemak sama ada canary terpantul dalam HTML, tag atribut, blok skrip, atau header respons HTTP.

#### [MODIFY] [src/active/heuristic.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/active/heuristic.rs)
- Menggabungkan semakan anomali saiz badan, status kod, dan pengesanan refleksi canary.

---

### 4. Pengagregatan, Model & CLI Updates

#### [MODIFY] [src/models.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/models.rs)
- Menambah nilai enum `ParamSource`: `UrlScan`, `CommonCrawl`, `RobotsTxt`, `Sitemap`, `InlineJs`, `Reflection`.
- Menambah medan pilihan pada `ScanOptions`: `depth`, `max_pages`, `wordlist_path`, `enable_urlscan`, `enable_commoncrawl`.

#### [MODIFY] [src/cli.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/cli.rs)
- Menambah flag CLI baharu:
  - `--depth <N>`: Kedalaman perayapan (lalai: 2).
  - `--max-pages <N>`: Had maksimum halaman dirayap (lalai: 50).
  - `-w, --wordlist <PATH>`: Fail kamus perkataan tersuai.
  - `--no-spider`: Matikan perayapan rekursif jika mahu mod pantas.
  - `--no-robots`: Matikan semakan robots & sitemap.

#### [MODIFY] [src/scanner.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/scanner.rs) & [src/main.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/main.rs)
- Menghubungkan semua saluran paip baharu secara teratur, selamat, dan pantas dengan pemprosesan asynchronous Tokio.

---

## Verification Plan

### Automated Tests
1. Jalankan unit test komprehensif bagi setiap modul baharu:
   ```bash
   cargo test
   ```
   - Ujian `spider`: Memastikan penapisan skop domain dalaman dan kawalan kedalaman BFS berfungsi.
   - Ujian `robots_sitemap`: Memastikan ekstraksi URL dan parameter daripada fail robots.txt dan XML sitemap berfungsi.
   - Ujian `urlscan`: Memastikan parsing respons JSON URLScan berjalan tanpa panik.
   - Ujian `wordlist`: Memastikan kamus 1,000+ parameter dimuatkan dan fungsi pemuat fail tersuai disahkan.
   - Ujian `reflection`: Memastikan pengesanan pantulan canary dalam teks HTML berfungsi dengan tepat.

### Manual Verification
1. Uji kompilasi binary release:
   ```bash
   cargo build --release
   ```
2. Jalankan binary ke atas sasaran ujian web dengan mod `--dalfox`:
   ```bash
   target\release\abu_hanifah_parameters.exe -u "https://httpbin.org" --dalfox
   ```
3. Sahkan bilangan parameter yang ditemui jauh melangkaui hasil sebelum ini dan terformat sedia-Dalfox secara sempurna.
