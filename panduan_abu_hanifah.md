# 📖 Panduan Lengkap Memori AI: Abu Hanifah

Panduan ini disediakan untuk membantu Tuan dan rakan Tuan memahami struktur, cara pemasangan, dan cara penggunaan sistem memori **Abu Hanifah** yang telah dinaik taraf dengan ciri-ciri canggih daripada `AI MemoryCore`.

---

## 1. Konsep & Struktur Memori

Memori Abu Hanifah menggunakan pendekatan **Satu Sumber Kebenaran (Single Source of Truth)**. Maklumat disimpan secara fizikal dalam bentuk fail `.md` (Markdown) yang dibaca secara langsung oleh AI ejen (Antigravity, Claude Code, atau VS Code GitHub Copilot).

### Struktur Direktori Fizikal

```
abu_hanifah/
├── master-memory.md               # Pusat kawalan & indeks memori utama
├── Abu_Hanifah_Safeguard/         # Alat pengimbas keselamatan (Rust Pentest Tool)
├── identity/
│   └── identity-core.md           # Persona & gaya kod Abu Hanifah (Security-First)
├── relationship/
│   └── user-preferences.md        # 33 Peraturan Keselamatan Global & bahasa
├── session/
│   ├── current-session.md         # RAM memori sementara (padam setiap sesi baru)
│   └── post-mortems.md            # Log pengajaran kegagalan (Post-Mortem)
├── library/
│   ├── security-standards.md      # Rujukan standard keselamatan sistem
│   └── post-mortem-core.md        # Panduan format penulisan post-mortem
├── skills/
│   ├── manage-project.md          # Protokol logik pengurusan projek LRU
│   └── post-mortem.md             # Protokol logik pengesan kegagalan
└── projects/
    ├── active/                    # Folder menyimpan fail projek aktif (Had 10)
    ├── archived/                  # Folder menyimpan fail projek lama yang diautomatik-arkib
    └── project-list.md            # Indeks portfolio projek aktif/arkib
```

---

## 2. Cara Pemasangan & Persediaan (3 Pilihan Platform)

Tuan boleh memilih cara memuatkan memori Abu Hanifah mengikut alat pembangunan (_development tool_) yang digunakan:

### 🔹 Pilihan A: Antigravity (Ejen IDE Bersepadu)

Platform ini memuatkan memori secara terus kerana ia dibina untuk membaca fail persekitaran tempatan (_local workspace_).

1.  **Persediaan**: Letakkan folder `abu_hanifah/` di dalam projek Tuan atau direktori desktop.
2.  **Pemuatan**: Antigravity akan membaca indeks teras memori melalui arahan sistem _System Prompt_ atau _Knowledge Item_ pada permulaan sesi secara fizikal. Tiada konfigurasi tambahan diperlukan daripada pihak Tuan.

---

### 🔹 Pilihan B: Claude Code (Terminal / CLI)

Claude Code menggunakan sistem _hook_ permulaan untuk membaca fail Markdown secara senyap semasa terminal dibuka.

1.  Pergi ke direktori tetapan Claude Code: `C:\Users\mypc\.claude\`.
2.  Cipta folder bernama `hooks/` di dalamnya jika belum ada.
3.  Cipta fail bernama `abu-hanifah-session-start.ps1` (untuk Windows PowerShell) di dalam `hooks/` dan masukkan kod berikut:
    ```powershell
    [Console]::In.ReadToEnd() | Out-Null
    Write-Output "AUTOLOAD: You are Abu Hanifah. Immediately read C:\Users\mypc\Desktop\Longterm Memory Project\abu_hanifah\master-memory.md and execute the Abu Hanifah greeting/restoration protocol described inside. Do NOT wait for the user to type the name - this is the automatic startup load. Greet the user once memory is restored."
    ```
4.  Cipta atau edit fail `C:\Users\mypc\.claude\settings.json` dan masukkan konfigurasi _hook_ ini:
    ```json
    {
      "hooks": {
        "SessionStart": [
          {
            "matcher": "startup|resume|clear|compact",
            "hooks": [
              {
                "type": "command",
                "command": "powershell -NoProfile -ExecutionPolicy Bypass -File \"C:\\Users\\mypc\\.claude\\hooks\\abu-hanifah-session-start.ps1\"",
                "timeout": 15,
                "async": true
              }
            ]
          }
        ]
      }
    }
    ```

---

### 🔹 Pilihan C: VS Code GitHub Copilot Chat (Termasuk DeepSeek Model)

GitHub Copilot mempunyai storan memori tersendiri (_virtual workspace storage_) yang diasingkan daripada fail desktop. Tuan boleh menyelaraskan (_sync_) memori Abu Hanifah ke dalam Copilot secara _read-only_.

1.  **Kefahaman Teknikal Storan Copilot**:
    Copilot menyimpan konfigurasi memori tersendiri secara maya di dalam folder:
    ```
    /memories/
    ├── abu-hanifah-persona.md      <-- Identiti & peraturan global Abu Hanifah
    └── session/
        └── memory-update-protocol.md
    ```
    Secara fizikal di komputer Windows Tuan, ia tersimpan di dalam folder storan VS Code:
    `C:\Users\mypc\AppData\Roaming\Code\User\workspaceStorage\<workspace-id>\GitHub.copilot-chat\...`
2.  **Langkah Menarik Memori (Sync)**:
    Setiap kali Tuan membuka sesi perbualan baharu di Copilot VS Code, arahkan Copilot seperti berikut:
    > _"Tolong tarik memori Abu Hanifah daripada folder C:\Users\mypc\Desktop\Longterm Memory Project\abu_hanifah\"_
3.  **Proses Kerja Copilot**:
    - Copilot akan membaca kesemua fail dalam folder fizikal Tuan.
    - Ia membandingkan kandungan baharu dengan fail maya `/memories/abu-hanifah-persona.md`.
    - Copilot akan mengemas kini storan memori chatnya dengan segala peraturan terbaharu.
    - Folder fizikal Tuan kekal bersih dan tidak diubah (_read-only_).

---

## 3. Cara Penggunaan Modul-Modul Utama Abu Hanifah

### 📂 Modul 1: LRU Project Management (Pengurusan Projek)

Sistem ini membantu Abu Hanifah menjejaki projek-projek yang sedang Tuan usahakan (sehingga 10 projek aktif). Jika Tuan memulakan projek ke-11, projek yang paling lama tidak disentuh akan dipindahkan ke folder `projects/archived/` secara automatik.

#### Arahan Penting (Taip Terus di Chat):

- `new project [NamaProjek]` — Membina fail projek baharu di kedudukan #1. Abu Hanifah akan bertanyakan penerangan ringkas dan teknologi yang digunakan untuk menjana fail projek.
- `load project [NamaProjek]` — Memuatkan projek sedia ada (aktif atau arkib). Projek tersebut akan melonjak ke kedudukan #1, dan jika ada projek di kedudukan #11, ia akan diautomatik-arkib.
- `save project` — Menyimpan status terkini kemajuan projek yang sedang aktif. Abu Hanifah akan membaca log komit git terkini untuk mengira jumlah masa yang telah Tuan luangkan secara automatik.
- `list projects` — Memaparkan senarai semua projek aktif (1-10) dan senarai projek yang telah diarkibkan.

#### Peraturan Emas:

1. Setiap kali projek dimuatkan (`load project`) atau disimpan (`save project`), fail indeks [project-list.md](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/projects/project-list.md) akan dikemaskini.
2. Panjang fail projek dihadkan kepada **1000 baris sahaja**. Jika melebihi, Abu Hanifah akan meringkaskan sesi lama secara automatik agar ingatan ejen kekal tajam tanpa melimpah.

---

### ⚡ Modul 2: Auto-Load Hook (Automasi Pemuatan)

Sistem ini adalah **automatik di terminal (Claude Code)**.

- **Cara Berfungsi**: Apabila terminal Claude Code dibuka, settings.json akan memanggil skrip PowerShell secara senyap di belakang tab. Skrip ini akan menyuntik arahan awal supaya Claude membaca fail `master-memory.md` Abu Hanifah tanpa perlu Tuan menaip nama ejen lagi.
- **Nota**: Ciri ini berjalan di terminal CLI local. Bagi platform IDE bersepadu seperti Antigravity, sistem ini menggunakan kaedah pembacaan direktori fizikal ejen (_Physical Workspace/Knowledge Item_) pada permulaan sesi perbualan.

---

### 🔥 Modul 3: Post-Mortem System (Log Pembelajaran Ralat)

Sistem ini berfungsi untuk merekodkan kesilapan pengekodan, ralat kritikal database, atau kegagalan logik supaya Abu Hanifah tidak akan mengulangi kesilapan yang sama di masa depan.

#### Cara Berfungsi:

1.  **Pengesanan Pasif (Automatik)**:
    Jika Tuan menulis atau menyebut ayat kegagalan seperti:
    - _"Ujian unit saya gagal"_
    - _"Database crash masa run migration"_
    - _"Terpaksa buat rollback production"_

    Abu Hanifah secara automatik akan mengesan isyarat ini dan bertanya:

    > _"That didn't go as planned. Worth a post-mortem?"_

2.  **Laporan Manual**:
    Tuan juga boleh mengarahkan secara manual dengan menaip:
    - `post-mortem` atau `log this failure`

3.  **Proses Pengisian Log**:
    Jika bersetuju untuk log, Abu Hanifah akan bertanya beberapa soalan ringkas berasaskan prinsip **5 Whys** (Kenapa ia berlaku secara mendalam) dan merekodkan:
    - **Severity**: Minor, Moderate, atau Major.
    - **What happened**: Apa yang rosak secara fakta.
    - **Why**: Punca akar umbi masalah.
    - **Lesson**: Pengajaran boleh guna (_reusable insight_).
    - **Prevention**: Langkah konkrit pencegahan (contoh: tambah unit test atau semak konfigurasi).

    Data ini akan disimpan ke dalam [post-mortems.md](file:///c:/Users/mypc/Desktop/Longterm%20Memory%20Project/abu_hanifah/session/post-mortems.md).

4.  **Rujukan Domain Masa Depan**:
    Setiap kali Tuan memulakan tugasan baharu yang berkaitan dengan domain ralat lama (contohnya, tugasan database/migration), Abu Hanifah akan membaca fail log ini secara senyap dan memberikan amaran:
    > _"⚠️ Reminder: [lesson] — see post-mortem [date]"_

---

### 🕵️ Modul 4: Auto-Detect Mockup UI & Roadmap Updater

Sistem pengimbas automatik ini berfungsi untuk mengesan komponen antaramuka (Frontend) yang sekadar paparan olok-olok (_mockup_) dan tidak mempunyai penyambungan API sebenar.

#### Cara Berfungsi:

1.  **Pengimbasan Automatik (AST)**:
    Apabila Tuan mengarahkan Abu Hanifah melaksanakan **Fasa 4** (Pelaksanaan Kod), Abu Hanifah akan menjalankan skrip Node.js (`detect-mockups.mjs`) di terminal. Skrip ini menggunakan _ts-morph_ untuk membaca _Abstract Syntax Tree (AST)_ bagi setiap komponen React/Next.js.
2.  **Kriteria Pengesanan**:
    - Fungsi `onClick` atau `onSubmit` yang sekadar mencetak log (`console.log`) atau kosong (`() => {}`).
    - Ketiadaan fungsi penyambung data (contohnya `fetch`, `api.get`, `useAuth`) dalam komponen yang sepatutnya interaktif.
3.  **Kemas Kini Roadmap**:
    Jika terdapat _mockup_ yang dikesan dan belum disenaraikan untuk disambung API, sistem akan **secara automatik mengubah fail `roadmap.md`** Tuan dan menambahkan senarai tugasan tersebut di bawah seksyen khas (**Sub-Fasa 13: Auto-Detection & Mockup Integration**).
    Memastikan tiada satupun UI yang tersalah anggap sebagai `100% siap` selagi ia tidak bersambung dengan pangkalan data atau backend sebenar. Tuan sentiasa mendapat kualiti produk akhir yang benar-benar berfungsi!

---

### 📖 Modul 5: Sistem Diari Sesi (Save Diary)

Sistem pembalakan (_logging_) berterusan yang merakam naratif perbualan dan proses pemikiran pembangunan ke dalam sebuah fail diari harian.

#### Arahan Penting:

- `save diary` — Arahkan Abu Hanifah untuk merumuskan apa yang telah dicapai dalam sesi tersebut dan merekodkannya ke dalam fail diari harian di `abu_hanifah/daily-diary/current/`.

#### Matlamat:

Memastikan konteks perancangan dan keputusan teknikal tidak hilang jika projek ditinggalkan lama. Ia bertindak sebagai "jurnal pembangun" automatik.

---

### 🔍 Modul 6: Sistem Carian Memori Sejarah (Echo Recall)

Enjin carian pintar yang membolehkan Abu Hanifah membaca semula arkib diari dan mengingat kembali perbincangan lama secara naratif.

#### Arahan Penting (Trigger Phrases):

- `do you remember...` atau `recall...` atau `check history...` — Bertanya kepada Abu Hanifah mengenai sejarah lepas.

#### Cara Berfungsi:

1.  Abu Hanifah akan mengekstrak kata kunci dari soalan Tuan.
2.  Mencari rekod dalam `daily-diary/current/` dan `daily-diary/archived/`.
3.  Jika dijumpai, Abu Hanifah akan membacakan kembali memori tersebut dalam bentuk penceritaan.
4.  **Peraturan Utama**: Abu Hanifah sama sekali dilarang mereka-reka (hallucinate) memori masa lalu tanpa bukti bertulis yang sah dari fail diari.

---

### 🛡️ Modul 7: Pentest Tool (Abu Hanifah Safeguard)

Alat pengimbas keselamatan (dihasilkan menggunakan Rust) yang dibina khusus untuk mengaudit kod sumber projek Tuan secara tempatan, pantas, dan tepat.

#### Arahan Penting (Cara Penggunaan):

- `jalankan abu hanifah safeguard pada projek [Laluan Projek]` — Arahkan Abu Hanifah untuk memulakan pengimbasan penuh ke atas direktori projek sasaran. Abu Hanifah akan menjalankan aplikasi dan mengemaskini fail jadual `security_audit.md`.

#### Ciri-ciri & Cara Berfungsi:

1. **Analisis Sintaks Abstrak (AST):** Menggunakan enjin _tree-sitter_ untuk menghuraikan struktur kod secara mendalam (seperti pengecaman variabel dalam rentetan HTML/PHP).
2. **Penapisan Amaran Palsu (False Positives):** Dilengkapi dengan logik bijak yang mengecam kunci tatasusunan (array keys seperti `['Price']`) dan fungsi selamat (seperti `number_format`, `date`, `t()`). Ini mengelakkan output selamat seperti nombor atau tarikh dilabel secara membuta tuli sebagai kerentanan XSS.
3. **Penjanaan Laporan Automatik:** Segala kelemahan sebenar akan disenaraikan dalam fail `security_audit.md` untuk memudahkan proses pembaikan (patching) oleh Abu Hanifah tanpa menyusahkan Tuan untuk menyemak ribuan log.

---

### 🎨 Modul 8: Resepi Halaman Utama (Landing Page Recipe)

Rangka kerja reka bentuk piawai (standard) untuk memastikan setiap antaramuka (Frontend) projek terutama sekali Halaman Utama (Landing Page) mencapai kesan "WOW" dan mematuhi estetika premium moden.

#### Arahan Penting (Cara Penggunaan):

- `bina halaman utama menggunakan resepi landing page` — Arahkan Abu Hanifah untuk merujuk kriteria ketat ini secara automatik semasa membina dan menyusun atur reka bentuk fail CSS/HTML/React Tuan.

#### Kriteria Pematuhan Resepi:

1. **Estetika Premium & Dinamik:** Wajib menggunakan gabungan palet warna yang harmoni (bukan warna asas generik seperti merah/biru rata), tipografi moden (seperti Inter atau Outfit), kesan _glassmorphism_, mod gelap yang elegan, dan kecerunan (gradients) yang licin.
2. **Pengalaman Interaktif:** Mengutamakan kesan _hover_, peralihan lancar (smooth transitions), dan _micro-animations_ yang membuatkan halaman terasa 'hidup'.
3. **Bahagian (Sections) Standard:** Setiap _Landing Page_ MESTI merangkumi: _Hero Section_ dengan _Call-To-Action (CTA)_ yang jelas, penerangan ciri (Features), bukti sosial (Testimonials/Partners), dan Footer yang komprehensif.
4. **Praktis Terbaik SEO:** Struktur _Semantic HTML_, tag tajuk (_Heading Structure_) yang betul (hanya satu H1), serta kelajuan muat turun dan responsif yang cemerlang di pelbagai saiz skrin.

---

### 💰 Modul 9: Penjana Sebut Harga (Price Quotation Generator)

Sistem terbina dalam untuk menghasilkan jadual anggaran kos projek berdasarkan templat harga yang telah ditetapkan dalam memori skil Abu Hanifah.

#### Arahan Penting (Cara Penggunaan):

- `buatkan price quotation A dari skill awak Abu hanifah` — Arahkan Abu Hanifah untuk menjana jadual sebut harga menggunakan struktur **Quotation A** (Kadar: Small RM50, Medium RM200, Large RM500).
- `buatkan price quotation B dari skill awak Abu hanifah` — Arahkan Abu Hanifah untuk menjana jadual sebut harga menggunakan struktur **Quotation B** (Kadar: Small RM200, Medium RM500, Large RM1500).

#### Cara Berfungsi:

1.  Abu Hanifah akan membaca senarai ciri-ciri (_features_) projek semasa.
2.  Mengkategorikan setiap ciri kepada skala _Large_ (contoh: Sistem Auth, Enjin Teras), _Medium_ (contoh: Papan Pemuka, CRUD), dan _Small_ (contoh: Alert UI, Konfigurasi Docker).
3.  Mendarabkan kuantiti ciri dengan Harga Unit mengikut versi _Quotation_ yang Tuan minta.
4.  Menghasilkan jadual Markdown profesional yang memaparkan Kategori, Ciri-Ciri Utama, Kuantiti, Harga Unit, dan Jumlah berserta anggaran harga keseluruhan projek.



---

### 🚀 Modul 10: Mod Autonomi Penuh (Zero-Questions Mode)

Kemahiran khas di mana Abu Hanifah dibenarkan untuk membuat keputusan secara autonomi tanpa meminta pendapat teknikal yang remeh dari Tuan.

#### Arahan Penting (Cara Penggunaan):

- `"Abu Hanifah, jalankan Autonomous Mode"` atau `buat keputusan sendiri` atau `bertindak secara autonomi` atau `tak perlu tanya, terus buat` — Arahkan Abu Hanifah untuk mengaktifkan mod ini.

#### Cara Berfungsi:

1. **Keputusan Autonomi**: AI secara automatik menggunakan amalan terbaik industri untuk menetapkan butiran teknikal (seperti penamaan pembolehubah, fail, atau pustaka/library) yang tidak disebut dalam skop asal tanpa menyoal kembali kepada Tuan.
2. **Kitaran Pembetulan Kendiri**: Jika berlaku masalah kompilasi, linting, atau kegagalan Unit Test, AI akan menganalisis log ralat dan membetulkannya sendiri tanpa henti sehingga berjaya, sebelum memulangkan jawapan akhir.

---

## 4. 8 Fasa Pembangunan Sistem Abu Hanifah

Sistem pembangunan berturutan ini memastikan kualiti kod, seni bina yang mantap, dan keselamatan yang tinggi dapat dicapai dalam setiap projek:

### 📑 Fasa 1: Perancangan Prompt Utama (Prompt Planning)

Fasa ini adalah tapak asas (_foundation_) setiap projek baru. Abu Hanifah akan mencipta fail `prompt_planning.md` di direktori utama projek. Struktur dokumen ini wajib dipenuhi sepenuhnya:

1.  **Visi & Misi:** Penyataan masalah dan solusi yang ditawarkan secara jelas.
2.  **Tech Stack:** Jadual terperinci teknologi frontend, backend, storan, auth, database, dan payment.
3.  **Sitemap & Aliran Halaman:** Peta navigasi struktur halaman serta tab.
4.  **Skema Storan & Database:** Jadual struktur kolum, jenis data, dan hubungan (foreign key).
5.  **Carta Alir Mermaid:** Visualisasi kitaran logik teras (_core loop_) bagi fitur utama.
6.  **Integrasi API:** Jadual endpoint, method (GET/POST), payload, authentication, dan peranan.
7.  **Protokol Keselamatan:** Pematuhan terhadap 33 Peraturan Keselamatan Global.
8.  **Struktur Folder Projek:** Pohon direktori projek modular.
9.  **UI/UX & Garis Panduan Estetika:** Keperluan reka bentuk premium, mod gelap, dan animasi mikro.
10. **Roadmap MVP:** Pelan tindakan pembangunan yang dibahagikan kepada fasa-fasa.

> [!IMPORTANT]
> Fail `prompt_planning.md` ini bertindak sebagai _Source of Truth_ utama. Sebelum memulakan fasa-fasa berikutnya, pastikan struktur ini lengkap dan disetujui sepenuhnya oleh Tuan.

### 🗺️ Fasa 2: Project Mapping & Perancangan Ujian

Mengekstrak spesifikasi, gambar rajah, atau halaman yang dikehendaki ke dalam fail `roadmap.md`. Setiap ciri/halaman dipecahkan kepada senarai tugasan unik yang bernombor (cth: Fasa 1 hingga Fasa 40) bagi memudahkan pemantauan kemajuan kerja.

Pada fasa ini, skop dan keperluan **Unit/Feature Test** untuk setiap ciri backend dan frontend mesti ditentukan dan ditulis secara jelas di dalam `roadmap.md` sebagai dwi-semakan (double-check). Fail **`features.md`** juga wajib dibina/diinisiasikan pada direktori utama sebagai daftar induk kesemua fitur yang akan diuji berserta status semakan kotak centang (checkbox).

### 🎨 Fasa 3: Mockup Design

Membina komponen Frontend dan struktur UI visual dengan menggunakan data mockup (data olok-olok) berdasarkan senarai dalam `roadmap.md`. Ini membolehkan Tuan melihat dan menyempurnakan reka bentuk visual (aesthetics) terlebih dahulu sebelum menghubungkan sistem backend/pangkalan data.

### 💻 Fasa 4: Pelaksanaan Kod & Ujian (Dwi-Semakan)

Mula mengimplementasikan setiap ciri secara berturutan berdasarkan senarai di dalam `roadmap.md`. Fail `dependency_map.json` akan dijana secara dalaman bagi memastikan susunan fail yang dibina adalah betul (dependencies diutamakan).

**Setiap kali satu ciri backend dibina, unit/feature test wajib dibina bersamanya. Bagi frontend, unit test wajib dibina untuk logik kritikal (seperti aliran kuiz dan autentikasi). Sebarang ciri baharu yang dicipta secara spontan juga wajib dipasang unit test.**

Setiap kali unit test dijalankan, fail **`features.md`** mestilah dikemas kini secara automatik:

- Menanda `[x]` (atau `✔️`) bagi fitur yang lulus (pass).
- Memadam tanda menjadi `[ ]` (atau `❌`) bagi fitur yang gagal/rosak.
  Fail `roadmap.md` juga akan dikemas kini secara automatik setelah tugasan selesai.

### 🛡️ Fasa 5: Imbasan Keselamatan (Security Scan)

Menjalankan audit keselamatan menyeluruh terhadap kod projek bagi mematuhi 33 Peraturan Keselamatan Global (_RULE[user_global]_). Sebarang kelemahan seperti SQL Injection, XSS, kebocoran token, atau isu OLAC (Object-Level Access Control) dibetulkan serta-merta, dan laporan audit disimpan dalam `security_audit.md`.

### ⚙️ Fasa 6: Konfigurasi Produksi & Nginx

Penyediaan fail `docker-compose.prod.yml` untuk tujuan deployment serta konfigurasi Nginx reverse proxy yang lengkap dengan tuning buffer, keselamatan header, dan rate-limiting bagi mensimulasikan persekitaran server sebenar.

### ⚡ Fasa 7: Ujian Bebanan (Load Testing)

Membina fail pengujian k6 untuk menguji ketahanan dan prestasi sistem di bawah bebanan tinggi. Sebelum dijalankan, persekitaran pembangunan (_development server_) dimatikan dan aplikasi dijalankan sepenuhnya di dalam bekas (_container_) Docker produksi terlebih dahulu. Fail ralat binaan Docker wajib dibetulkan sehingga berjaya sebelum pengujian bebanan boleh diteruskan.

### 🤖 Fasa 8: Ujian Automasi E2E (Playwright)

Memasang dan mengkonfigurasi kerangka kerja Playwright di dalam projek frontend (seperti Next.js) menggunakan TypeScript. Fasa ini melibatkan pembinaan skrip ujian E2E yang menyeluruh untuk menyimulasikan tingkah laku pengguna sebenar, menguji aliran kritikal aplikasi, dan memastikan persekitaran web selamat daripada regresi (regression). Ia mesti dilaksanakan secara berterusan sehingga semua ujian disahkan berfungsi dengan baik.
