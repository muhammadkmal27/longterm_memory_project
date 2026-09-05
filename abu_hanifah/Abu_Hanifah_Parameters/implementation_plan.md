# Pelan Pembangunan: Abu_Hanifah_Parameters (Enjin Pencari Parameter Berprestasi Tinggi Berasaskan Rust)

Alat ini bertujuan untuk menyelesaikan masalah kritikal pemburu pepijat (*bug hunters*) di HackerOne: mencungkil semua parameter (termasuk *hidden parameters*) daripada domain/subdomain sasaran, membersih dan menggabungkan parameter secara pintar, serta memformatkannya secara langsung untuk disalurkan ke dalam **Dalfox** (`| dalfox pipe`).

---

## User Review Required

> [!IMPORTANT]
> **Format Integrasi Dalfox**:
> Secara lalai, Dalfox menerima input melalui standard input (`cat urls.txt | dalfox pipe`) atau fail flag `dalfox file urls.txt`.
> `Abu_Hanifah_Parameters` akan menyokong mod `--dalfox` / `--pipe` yang hanya mencetak senarai URL bersih dengan penanda `FUZZ` (contoh: `https://target.com/page?param1=FUZZ&param2=FUZZ`) tanpa banner atau log bising, supaya boleh terus di-pipe ke Dalfox.

> [!TIP]
> **Ujian Sasaran Sebenar Pengguna**:
> Pengguna telah menyatakan sudah mempunyai laman web sasaran yang mengandungi kerentanan XSS. Sebaik sahaja enjin siap dibina, kita akan menjalankan ujian langsung ke atas URL tersebut untuk mengesahkan bahawa parameter XSS tersebut berjaya ditemui dan dibaca oleh Dalfox.

---

## Open Questions

> [!NOTE]
> 1. Adakah akhi mahu berkongsi URL sasaran sekarang untuk kami masukkan ke dalam senarai semak pengesahan, atau akhi mahu menjalankan arahan CLI sendiri setelah binary siap di-*build*?
> 2. Adakah perkakas ini perlu menyokong input fail senarai domain pukal (contoh: `domains.txt` dari `subfinder`) selain domain tunggal? *(Kami akan sertakan sokongan flag `-l / --list` secara lalai).*

---

## Proposed Changes

Projek ini akan dibina di bawah folder baharu:
`c:\Users\mypc\Desktop\Longterm Memory Project\abu_hanifah\Abu_Hanifah_Parameters`

### 1. Konfigurasi Cargo & Dependencies

#### [NEW] [Cargo.toml](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/Cargo.toml)
Dependencies yang akan digunakan:
- `clap`: Pengurusan argumen CLI (`--domain`, `--url`, `--list`, `--dalfox`, `--silent`, `--concurrency`, `--deep-js`, `--bruteforce`)
- `tokio`: Rangka kerja asynchronous runtime untuk kelajuan maksimum
- `reqwest`: Klien HTTP async berprestasi tinggi dengan connection pooling dan custom User-Agent
- `regex`: Pengekstrakan parameter regex berkelajuan tinggi dari fail JavaScript & HTML
- `scraper`: Parsing elemen HTML (borang, input tersembunyi `type="hidden"`, pautan)
- `url`: Parsing dan manipulasi query string yang selamat
- `serde`, `serde_json`: Parsing respons API (Wayback, OTX, JSON endpoints) dan eksport data
- `colored`: Format paparan terminal yang kemas dan estetik

---

### 2. Teras Enjin (Core Engine Modules)

Setiap fail dipastikan mematuhi prinsip **Single Responsibility Principle (SRP)** dan tidak melebihi **250 baris kod**.

#### [NEW] [src/main.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/main.rs)
- Titik masuk (entrypoint) program CLI.
- Menguruskan aliran kerja antara Passive Collector, Client-side JS Miner, Heuristic Brute-force, dan Aggregator Normalizer.

#### [NEW] [src/models.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/models.rs)
- Struktur data: `DiscoveredParam`, `TargetEndpoint`, `ParamSource` (Passive, JS, Form, Heuristic), `CliArgs`.

#### [NEW] [src/passive/wayback.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/passive/wayback.rs)
- Mengutip rekod sejarah URL dan parameter dari Archive.org Wayback CDX API.

#### [NEW] [src/passive/otx.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/passive/otx.rs)
- Mengutip URL dari AlienVault Open Threat Exchange (OTX) API.

#### [NEW] [src/crawler/html_parser.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/crawler/html_parser.rs)
- Mengekstrak parameter daripada tag `<form>`, `<input name="...">`, `<input type="hidden">`, `<a href="...">`, dan skrip inline.

#### [NEW] [src/crawler/js_miner.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/crawler/js_miner.rs)
- Memuat turun skrip JavaScript yang dipautkan (`.js` chunks/bundles).
- Mengekstrak nama parameter menggunakan regex pintar (`URLSearchParams.get`, `params: { ... }`, `axios`, `fetch`, `window.location.search`).

#### [NEW] [src/active/heuristic_miner.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/active/heuristic_miner.rs)
- Enjin *Hidden Parameter Miner* menggunakan kaedah **Dichotomy (Binary Search)**.
- Menghantar kelompok parameter kamus (Top 150+ parameter HackerOne seperti `debug`, `redirect`, `admin`, `test`, `callback`, `preview`).
- Mengesan anomali: perbezaan saiz respons (*length delta*), status kod, atau refleksi teks.

#### [NEW] [src/aggregator/normalizer.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/aggregator/normalizer.rs)
- **Smart Query Aggregator**: Mengumpulkan parameter unik bagi setiap endpoint unik (mengelakkan ribuan URL bertindih).
- Menjana URL berformat Dalfox dengan placeholder `FUZZ` (cth: `https://target.com/search?q=FUZZ&category=FUZZ&sort=FUZZ`).

#### [NEW] [src/reporter/cli.rs](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/src/reporter/cli.rs)
- Menghasilkan paparan terminal interaktif, mod `--dalfox` (piping terus tanpa teks tambahan), dan eksport fail.

---

### 3. Dokumentasi Piawaian Abu Hanifah

#### [NEW] [roadmap.md](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/roadmap.md)
- Senarai tugasan berfasa bagi memantau kemajuan pembangunan.

#### [NEW] [features.md](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/Abu_Hanifah_Parameters/features.md)
- Senarai semak unit test bagi setiap fungsi dan modul yang dibina.

---

## Verification Plan

### Automated Tests
1. **Unit Tests (Cargo Test)**:
   - `cargo test --bin abu_hanifah_parameters`:
     - Ujian `html_parser`: Memastikan input `<input name="search" type="hidden">` diekstrak dengan tepat.
     - Ujian `js_miner`: Memastikan regex mengekstrak parameter daripada potongan kod JavaScript (cth: `params.get('debug')`).
     - Ujian `normalizer`: Memastikan URL bertindih digabungkan menjadi satu URL lengkap dengan token `FUZZ`.
     - Ujian `heuristic_miner`: Memastikan logik pembahagian kelompok parameter (dichotomy) berfungsi tanpa *infinite loop*.

### Manual Verification
1. Uji arahan CLI terhadap domain sasaran:
   ```bash
   cargo run -- -u "https://target.com" --dalfox
   ```
2. Sahkan bahawa output memaparkan URL sedia-Dalfox:
   ```text
   https://target.com/vulnerable-endpoint?target_param=FUZZ
   ```
3. Uji piping terus ke Dalfox (jika dipasang):
   ```bash
   cargo run -- -u "https://target.com" --dalfox | dalfox pipe
   ```
