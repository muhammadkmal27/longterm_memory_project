# 🗺️ Roadmap Projek: Abu_Hanifah_Parameters (Rust)

_Enjin Pencari Parameter Web Berkelajuan Tinggi Khusus untuk Bug Bounty HackerOne & Integrasi Dalfox._

## Fasa 1: Perancangan & Senibina Teras ✅
- [x] Pelan pelaksanaan dipersetujui oleh pengguna.
- [x] Inisialisasi repositori Rust `Abu_Hanifah_Parameters`.
- [x] Pembentukan fail perancangan `roadmap.md` dan `features.md`.

## Fasa 2: Persediaan Teras & Pengurusan Argumen CLI ✅
- [x] Konfigurasi `Cargo.toml` dengan dependencies async berprestasi tinggi (`tokio`, `reqwest`, `clap`, `regex`, `scraper`, `url`, `serde`).
- [x] Pengurusan CLI (`src/cli.rs`) menyokong domain tunggal (`-u / -d`), senarai domain HackerOne (`-l / --list`), mod `--dalfox`, mod `--silent`, dan had *threads/concurrency*.
- [x] Model data teras (`src/models.rs`).

## Fasa 3: Enjin Pasif OSINT (Passive Parameter Mining) ✅
- [x] Modul penuaian rekod Wayback Machine CDX (`src/passive/wayback.rs`).
- [x] Modul penuaian rekod AlienVault OTX (`src/passive/otx.rs`).
- [x] Pengekstrakan dan normalisasi query parameter daripada URL arkib.

## Fasa 4: Enjin Pengikis Klien (Client-Side JS & HTML Miner) ✅
- [x] Modul parser HTML (`src/crawler/html_parser.rs`) untuk `<form action>`, `<input name="...">`, dan `<input type="hidden">`.
- [x] Modul pengorek JavaScript (`src/crawler/js_miner.rs`) menggunakan regex berprestasi tinggi untuk mengekstrak parameter dari `URLSearchParams`, `axios`, `fetch`, dan objek JSON.

## Fasa 5: Enjin Heuristik Parameter Tersembunyi (Hidden Parameter Miner) ✅
- [x] Kamus parameter berimpak tinggi (*Top Bug Bounty Parameters*).
- [x] Algoritma Dichotomy (Binary Search) menghantar kelompok parameter dan mengesan anomali perubahan respons (status, saiz badan, header).

## Fasa 6: Pengagregat Pintar & Normalizer Mesra Dalfox (Dalfox Ready) ✅
- [x] Modul deduplikasi dan penggabungan parameter pintar (`src/aggregator/normalizer.rs`).
- [x] Penjanaan format output `FUZZ` khusus untuk `| dalfox pipe` tanpa gangguan banner (`--dalfox` / `--pipe`).
- [x] Modul paparan terminal interaktif & berwarna (`src/reporter/cli.rs`).

## Fasa 7: Ujian Automasi & Penentuduksahan (Testing & Quality Assurance) ✅
- [x] Pelaksanaan kesemua unit test di dalam `features.md` (100% Lulus).
- [x] Kompilasi binary akhir `abu_hanifah_parameters.exe` (Selesai dibina dalam mod release).
- [x] Ujian pengesahan langsung terhadap sasaran laman web pengguna.

## Fasa 8: Naik Taraf Ekstrem v2.0 (The Ultimate Bug Bounty Engine) ✅
- [x] **Enjin Perayap Rekursif (BFS Spider Engine)**: Merayap kedalaman 1-5 (`--depth`) dan kawalan had halaman (`--max-pages`) dengan penapisan skop domain tegar.
- [x] **Penuai Konfigurasi (Robots & Sitemap Harvester)**: Ekstraksi automatik endpoint berparameter daripada `/robots.txt`, `/sitemap.xml`, dan `/sitemap_index.xml`.
- [x] **Ekstraksi Skrip Inline & Peluasan JS Regex**: Mengekstrak skrip inline `<script>...</script>` dan memperluas corak pengesanan parameter objek, API calls (`params.set`, `req.query`, dsb.).
- [x] **Sumber Arkib Pasif Diperluas**: Integrasi API URLScan.io dan Common Crawl Index untuk OSINT menyeluruh.
- [x] **Kamus Parameter 1,000+ & Pemuat Tersuai**: Kamus terbina diperluas kepada 1,000+ top bug bounty params, berserta sokongan `-w / --wordlist <PATH>`.
- [x] **Enjin Pantulan Canary XSS (Canary Reflection Hunter)**: Mengesan parameter yang memantulkan nilai canary secara langsung dalam HTML/header respons untuk sasaran kritikal XSS Dalfox.
- [x] **Sokongan Pelbagai Kaedah HTTP**: Menambah flag `--method <get|post|both>` untuk fleksibiliti imbasan aktif.
- [x] **100% Ujian Lulus & Binari Release Siap**: Kesemua 18 unit test disahkan lulus dan binari `target/release/abu_hanifah_parameters.exe` dijana tanpa sebarang amaran.

