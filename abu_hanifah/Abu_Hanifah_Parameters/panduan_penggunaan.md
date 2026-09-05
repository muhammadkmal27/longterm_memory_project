# 📖 Panduan Penggunaan Abu_Hanifah_Parameters v2.0

Perkakas pencari parameter web berprestasi tinggi (*The Ultimate High-Performance Bug Bounty Parameter Discovery Engine*) yang dibina menggunakan bahasa pengaturcaraan **Rust**. 

Menggabungkan dan melangkaui kehebatan **Katana, ParamSpider, Arjun, dan Waybackurls** ke dalam satu binari tunggal yang pantas dan siap disalurkan ke **Dalfox** untuk pembuktian kerentanan XSS secara automatik.

---

## ⚡ Ciri-Ciri Utama v2.0 (Superpowers)

1. **Enjin Perayap Rekursif (BFS Spider - Katana Style)**:
   - Merayap pautan dalaman secara rekursif mengikut kedalaman (`--depth 1-5`) dan had halaman (`--max-pages`).
   - Penapisan skop domain yang tegar (*Strict In-Scope*) bagi menghalang kebocoran ke domain luaran.
   - Mengabaikan fail media/statik secara automatik (.png, .jpg, .css, .woff, dsb.).

2. **Penuai Konfigurasi (Robots.txt & Sitemap.xml Harvester)**:
   - Mengekstrak laluan dan parameter tersembunyi daripada `/robots.txt` (`Disallow:` & `Allow:`).
   - Memproses XML sitemap (`/sitemap.xml`, `/sitemap_index.xml`) untuk mengumpul semua URL berparameter.

3. **Supercharged Passive OSINT (ParamSpider & Waybackurls Style)**:
   - **Wayback Machine (Archive.org)**: Menapis rekod arkib masa lalu secara selamat (HTTPS) tanpa pemotongan query parameter berlebihan.
   - **AlienVault OTX**: Menyedut rekod URL ancaman global.
   - **URLScan.io API**: Menapis hasil sejarah imbasan domain secara pasif.
   - **Common Crawl Index**: Mengutip jutaan URL arkib web global.

4. **Pengorek JavaScript Klien & Skrip Inline**:
   - Ekstraksi skrip inline (`<script>...</script>`) dan fail bundle `.js` luaran.
   - Regex pintar mengesan `URLSearchParams`, `axios`, `fetch`, `router.query`, `params.set`, `params.append`, dan objek JSON.

5. **XSS Canary Reflection Hunter (Arjun Style)**:
   - Menyuntik nilai ujian canary unik ke dalam kelompok parameter secara serentak.
   - Mengesan pantulan nilai secara terus dalam respons HTML atau header HTTP (High XSS Risk).

6. **Enjin Heuristik Parameter Tersembunyi (Dichotomy Fuzzing)**:
   - Kamus terbina 1,000+ parameter berimpak tinggi (*Top Bug Bounty Parameters*).
   - Sokongan membaca kamus tersuai pengguna melalui `-w / --wordlist <FAIL>`.
   - Sokongan kaedah permintaan fleksibel melalui `--method <get|post|both>`.

7. **Pengagregat Pintar & Integrasi Dalfox**:
   - Menggabungkan parameter unik mengikut endpoint menjadi URL sedia-Dalfox dengan penanda `FUZZ`.
   - Mod `--split` untuk menghasilkan URL berasingan bagi setiap parameter.

---

## 💻 Panduan Khas Penggunaan di Windows PowerShell

Pengguna Windows PowerShell boleh menjalankan binari kompilasi Rust secara terus dengan prestasi maksimum.

### 0. Navigasi ke Direktori Projek
Buka PowerShell dan masuk ke direktori binari projek ini terlebih dahulu:
```powershell
cd "C:\Users\mypc\Desktop\Longterm Memory Project\abu_hanifah\Abu_Hanifah_Parameters"
```

### 1. Konfigurasi Awal Encoding UTF-8 (Sangat Disyorkan)
Sebelum melakukan piping ke Dalfox, jalankan arahan ini di konsol PowerShell untuk mengelakkan isu decoding aksara URL:
```powershell
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
```

### 2. Menetapkan Alias PowerShell (Boleh Diakses Di Mana-mana)
Supaya tidak perlu menaip laluan binari yang panjang, daftarkan alias pantas:
```powershell
Set-Alias -Name ahparam -Value "C:\Users\mypc\Desktop\Longterm Memory Project\abu_hanifah\Abu_Hanifah_Parameters\target\release\abu_hanifah_parameters.exe"
```
Selepas ini, anda hanya perlu menaip `ahparam` di PowerShell!

### 2.5 Pemasangan Dalfox di Windows PowerShell
Jika arahan `dalfox` belum dikenali dalam sistem anda, pasang Dalfox menggunakan Go (kaedah rasmi):
```powershell
go install github.com/hahwul/dalfox/v2@latest
```
*(Binari `dalfox.exe` akan dijana di `C:\Users\mypc\go\bin` yang sudah sedia ada dalam pembolehubah persekitaran PATH).*

Jika sistem tiada Go, anda juga boleh memuat turun fail binari `dalfox_x.x.x_windows_amd64.zip` terus daripada [Releases Rasmi Dalfox di GitHub](https://github.com/hahwul/dalfox/releases), ekstrak dan letakkan `dalfox.exe` ke dalam folder yang sama.

### 3. Piping Terus ke Dalfox di PowerShell
```powershell
.\target\release\abu_hanifah_parameters.exe -u "https://target.com" --dalfox | dalfox pipe
```
*(Atau dengan alias: `ahparam -u "https://target.com" --dalfox | dalfox pipe`)*

### 4. Mengimbas Berbilang Subdomain Menggunakan Gelung PowerShell
Gunakan `Get-Content` untuk membaca senarai domain HackerOne dan menyalurkannya terus ke Dalfox:
```powershell
Get-Content hackerone_targets.txt | ForEach-Object {
    Write-Host "[*] Mengimbas target: $_" -ForegroundColor Cyan
    .\target\release\abu_hanifah_parameters.exe -d $_ --dalfox
} | dalfox pipe
```

### 5. Menyimpan Output Bersih ke Fail Teks Menggunakan Out-File
```powershell
.\target\release\abu_hanifah_parameters.exe -l targets.txt --dalfox | Out-File -Encoding utf8 dalfox_queue.txt
```

### 6. Menghantar Header Tersuai di PowerShell
Gunakan tanda petik dua bagi mengelakkan ralat parser PowerShell:
```powershell
.\target\release\abu_hanifah_parameters.exe -u "https://target.com/admin" -H "Cookie: session=abc123xyz" -H "Authorization: Bearer mytoken" --dalfox | dalfox pipe
```

### 7. Melihat Hasil Parameter Sahaja (Tanpa Perlu Dalfox)
Jika anda hanya mahu mengumpul atau menyemak parameter yang ditemui tanpa sebarang serangan XSS:

- **Mod Paparan Terminal Berwarna (Lalai)**:
  ```powershell
  .\target\release\abu_hanifah_parameters.exe -u "https://target.com"
  # atau untuk domain menyeluruh:
  .\target\release\abu_hanifah_parameters.exe -d "target.com"
  ```
  *(Memaparkan log penemuan setiap enjin, jumlah endpoint, dan senarai parameter terperinci)*

- **Format JSON Terperinci**:
  ```powershell
  .\target\release\abu_hanifah_parameters.exe -u "https://target.com" --json
  ```

- **Hanya Ekstrak Nama Parameter Sahaja Menggunakan PowerShell**:
  ```powershell
  (.\target\release\abu_hanifah_parameters.exe -u "https://target.com" --json | ConvertFrom-Json).params | Sort-Object -Unique
  ```

- **Simpan Terus ke Fail Teks / JSON Tanpa Dalfox**:
  ```powershell
  # Simpan senarai URL dan parameter ke fail teks:
  .\target\release\abu_hanifah_parameters.exe -u "https://target.com" -o "hasil_parameter.txt"

  # Simpan format JSON:
  .\target\release\abu_hanifah_parameters.exe -u "https://target.com" --json -o "hasil_parameter.json"
  ```

---

## 🚀 Arahan & Contoh Penggunaan CLI Am

### 1. Imbasan Lengkap Sedia-Dalfox (Piping Terus)
```bash
abu_hanifah_parameters -u "https://target.com" --dalfox | dalfox pipe
```

### 2. Imbasan Rekursif Dalam (Spider Mode)
Menjelajah sehingga kedalaman 3 dengan had 100 halaman:
```bash
abu_hanifah_parameters -u "https://target.com" --depth 3 --max-pages 100 --dalfox | dalfox pipe
```

### 3. Menggunakan Kamus Parameter Tersuai (Custom Wordlist)
```bash
abu_hanifah_parameters -u "https://target.com/api" -w "my_params.txt" --method both
```

### 4. Imbasan Senarai Subdomain Skop HackerOne
```bash
abu_hanifah_parameters -l "hackerone_scope.txt" --dalfox > targets_dalfox.txt
dalfox file targets_dalfox.txt
```

### 5. Mod Pantas (Hanya Arkib Pasif & Laman Utama Tanpa Spider)
```bash
abu_hanifah_parameters -d "target.com" --no-spider --no-heuristic
```

### 6. Menyimpan Hasil ke Fail JSON Berstruktur
```bash
abu_hanifah_parameters -d "target.com" --json -o "hasil_scan.json"
```

---

## 🛠️ Ringkasan Flag Perintah (CLI Flags)

| Flag | Keterangan | Lalai (Default) |
| :--- | :--- | :--- |
| `-u, --url` | URL sasaran tunggal | Tiada |
| `-d, --domain` | Domain atau subdomain sasaran | Tiada |
| `-l, --list` | Fail senarai domain/subdomain sasaran | Tiada |
| `--dalfox` | Mod khas Dalfox: mengeluarkan URL bersih dengan `FUZZ` | `false` |
| `--split` | Keluarkan 1 URL berasingan bagi setiap parameter | `false` |
| `-s, --silent` | Mod senyap (hanya cetak URL penemuan) | `false` |
| `--json` | Output dalam format JSON berstruktur | `false` |
| `-o, --output` | Simpan hasil ke fail teks/JSON | Tiada |
| `-c, --concurrency` | Bilangan sambungan serentak | `20` |
| `--timeout` | Had masa tamat sambungan (saat) | `10` |
| `--depth` | Kedalaman perayapan rekursif Spider (1-5) | `2` |
| `--max-pages` | Had maksimum halaman dirayap setiap domain | `50` |
| `-w, --wordlist` | Fail kamus parameter tersuai | Kamus terbina 1,000+ |
| `--method` | Kaedah HTTP untuk semakan aktif: `get`, `post`, `both` | `get` |
| `--no-spider` | Lumpuhkan perayap rekursif (hanya halaman asas) | `false` |
| `--no-robots` | Lumpuhkan penuai robots.txt & sitemap.xml | `false` |
| `--no-urlscan` | Lumpuhkan carian pasif URLScan.io | `false` |
| `--no-commoncrawl` | Lumpuhkan carian pasif Common Crawl | `false` |
| `--no-passive` | Lumpuhkan semua carian arkib pasif (Wayback/OTX) | `false` |
| `--no-js` | Lumpuhkan pengorekan fail JavaScript | `false` |
| `--no-heuristic` | Lumpuhkan carian heuristik & pantulan aktif | `false` |
