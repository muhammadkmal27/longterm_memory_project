# 📋 Senarai Induk Ujian Ciri (Features & Unit Tests Checklist)

- [x] `[x]` **Ujian Argumen CLI & Input**: Memastikan sokongan URL tunggal (`-u`), domain (`-d`), senarai domain HackerOne (`-l`), dan mod `--dalfox` berfungsi dengan tepat.
- [x] `[x]` **Ujian Model & Sumber Data**: Memastikan struktur `DiscoveredParam`, `ParamSource`, dan `TargetEndpoint` boleh dimanipulasi dan diselaraskan.
- [x] `[x]` **Ujian Ekstraksi Parameter HTML**: Memastikan parser HTML berjaya mengekstrak parameter daripada `<input name="...">`, `<input type="hidden">`, `<form>`, dan tag `<a>`.
- [x] `[x]` **Ujian Ekstraksi Regex JavaScript**: Memastikan modul JS miner berjaya mengekstrak parameter daripada panggilan API JavaScript (`URLSearchParams`, `axios`, `fetch`, `params.set`).
- [x] `[x]` **Ujian Ekstraksi Inline JavaScript**: Memastikan skrip inline `<script>...</script>` berjaya diekstrak dan diproses sebagai sumber `InlineJs`.
- [x] `[x]` **Ujian Pengekstrakan Arkib Pasif (Wayback/OTX)**: Memastikan query parameters daripada respons arkib URL diekstrak dan dibersihkan daripada fail media/duplikasi.
- [x] `[x]` **Ujian Pengekstrakan URLScan.io**: Memastikan ekstraksi URL dan parameter daripada API URLScan.io beroperasi dengan lancar.
- [x] `[x]` **Ujian Pengekstrakan Common Crawl**: Memastikan query parameters daripada indeks Common Crawl diekstrak dengan tepat.
- [x] `[x]` **Ujian Penuai Robots.txt**: Memastikan laluan `Disallow:` dan `Allow:` diekstrak bersama query parameter berkaitan.
- [x] `[x]` **Ujian Penuai Sitemap.xml**: Memastikan tag `<loc>` dalam fail XML sitemap diekstrak untuk penemuan endpoint berparameter.
- [x] `[x]` **Ujian Penapis Fail Statik Spider**: Memastikan perayap mengabaikan aset statik/media (.png, .css, .jpg, dll.) semasa merayap.
- [x] `[x]` **Ujian Pemuat Kamus Parameter**: Memastikan kamus terbina 1,000+ perkataan bug bounty dimuatkan dan fungsi membaca fail tersuai (`-w`) disahkan.
- [x] `[x]` **Ujian Pengesan Pantulan Canary (XSS Reflection)**: Memastikan canary unik dikesan apabila terpantul di dalam HTML respons atau header.
- [x] `[x]` **Ujian Logik Dichotomy Heuristik**: Memastikan algoritma carian binari (dichotomy) membahagi kelompok parameter tanpa perulangan tanpa henti (*infinite loop*).
- [x] `[x]` **Ujian Pengagregatan Pintar (Smart Query Aggregator)**: Memastikan URL dengan endpoint yang sama digabungkan menjadi satu URL yang mengandungi semua parameter unik dengan token `FUZZ`.
- [x] `[x]` **Ujian Format Output Dalfox**: Memastikan output baris tunggal adalah URL yang sah dan siap diproses oleh `dalfox pipe`.
