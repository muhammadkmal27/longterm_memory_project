# 🔍 Protokol: Sistem Pengesan Mockup UI Automatik

Sistem ini membantu Abu Hanifah (anda) mengesan UI atau komponen Frontend yang hanyalah sekadar paparan (*mockup*) dan tidak berfungsi secara sebenar (contohnya tiada interaksi API atau menggunakan data statik).

Sistem ini memastikan Abu Hanifah **tidak tersalah anggap** bahawa komponen telah siap 100% dan akan mengemas kini `roadmap.md` secara automatik.

## ⚙️ Syarat Pencetus Protokol (Trigger Condition)

Protokol ini **WAJIB** dijalankan apabila:
- Pengguna (User) mengarahkan: `"Laksanakan Fasa 4"`, `"Selesaikan UI"`, atau `"Semak mockup"`.
- Anda diminta untuk mengesahkan status keseluruhan `roadmap.md`.
- Sebelum anda menanda apa-apa komponen Mockup sebagai siap (✅) dalam senarai semak.
- Selepas anda selesai wire-up mana-mana halaman mockup — jalankan semula untuk pengesahan.

## 📋 Arahan Pelaksanaan (Execution Steps)

Apabila syarat di atas dipenuhi, jalankan arahan berikut di terminal dengan *Current Working Directory (cwd)* di `c:\Users\mypc\Desktop\Vocabulary\`:

```bash
node scripts/detect-mockups.mjs
```

### Selepas Skrip Dijalankan:
1. Skrip v3 akan mengimbas fail `.tsx` menggunakan 5 Enjin Pengesanan Utama:
   - **Engine 1 (Static Mock Arrays)**: Mengesan array statik dengan data olok-olok seperti `const LANGUAGES = [...]`.
   - **Engine 2 (Empty Handlers)**: Mengesan `onClick`/`onSubmit` yang kosong atau sekadar menggunakan `console.log`/`alert`.
   - **Engine 3 & 4 (Inline Hardcoded Data & Stats Display)**: Mengesan teks statik statistik (seperti "Jumlah Pengguna", "RM20.00", atau nombor `1,234`) sekiranya fail tidak mempunyai sebarang panggilan API.
   - **Engine 5 (Interactive Without API)**: Mengesan butang/suis interaktif di dalam fail yang tidak memanggil sebarang backend API.
   *Pengecualian Pintar: Fail yang menggunakan hook `useAuth` untuk autentikasi dikecualikan daripada Engine 5 untuk mengelakkan false-positives (seperti register/page.tsx).*
2. Jika ada *mockup* baharu ditemui, skrip akan mengemas kini `roadmap.md` secara automatik dengan menambahkannya di bawah **Sub-Fasa 13: Auto-Detection & Mockup Integration**.
3. **Penting:** Anda mesti memuat semula / membaca fail `roadmap.md` jika ia telah dikemas kini oleh skrip.
4. Jangan tandakan task Sub-Fasa 13 sebagai ✅ sehinggalah anda benar-benar menyambungkannya (wire-up) dengan API backend dan melengkapkan ujiannya.

## 🐛 Bug Yang Telah Dibetulkan (v2 — 2026-05-26)

### Bug 1: Baris jadual baru dimasukkan selepas `---` separator
- **Punca**: Regex `subFasa13SectionRegex` menggunakan pola greedy `[\s\S]*?(?=###|## |$)` yang menangkap termasuk `---` separator, menyebabkan baris baharu diletakkan di luar jadual Markdown.
- **Pembetulan**: Gantikan regex dengan pola yang tepat menangkap header jadual dan baris-baris data sahaja: `/(### Sub-Fasa 13:.*\n(?:\s*\n)*\|[^\n]+\|\s*\n\|[-|\s]+\|\s*\n)((?:\|[^\n]+\|\s*\n)*)/`

### Bug 2: Path menggunakan backslash `\` (Windows) dalam roadmap.md
- **Punca**: `path.relative()` di Windows menghasilkan path dengan `\` (backslash) yang rosak apabila dimasukkan ke Markdown.
- **Pembetulan**: Tambah `.replace(/\\/g, "/")` untuk normalisasi semua path ke forward slash.

### Bug 3: Progress counter tidak dikemas kini dengan betul
- **Punca**: Logik kemas kini hanya menambah baris baharu tanpa mengira semula jumlah keseluruhan.
- **Pembetulan**: Selepas menambah baris, kira semula jumlah dari semua counter `✅ X/Y` dalam jadual progress dan kemas kini baris `**Total**`.

## 🚀 Naik Taraf Sistem (v3 — 2026-05-27)
Sistem telah dinaik taraf ke versi 3 dengan 5 enjin pengesanan baharu yang lebih agresif bagi mengesan mockups yang sebelum ini terlepas:
1. **Engine 1: Static Mock Arrays** - Mengimbas deklarasi pemalar senarai statik (contohnya `LANGUAGES` dalam profil).
2. **Engine 2: Empty Handlers** - Mengesan onClick/onSubmit kosong atau placeholder console/alert.
3. **Engine 3/4: Inline Hardcoded Data & Display** - Mengesan paparan data/stats (seperti "Jumlah Pengguna", "RM20.00", atau pattern nombor `1,234`) tanpa sebarang panggilan API.
4. **Engine 5: Interactive Without API** - Mengesan kewujudan komponen interaktif (Switch/Button) di dalam fail halaman tanpa integrasi API backend.

Pengecualian pintar dibuat untuk fail yang menggunakan hook `useAuth` yang sah untuk mengelakkan false-positive (seperti pada `register/page.tsx`).

## ✅ Status Semasa (2026-05-27)
- Semua 15 mockup dalam Sub-Fasa 13 telah berjaya di-wire-up ke API backend sebenar.
- Skrip detect-mockups v3 mengeluarkan: `✅ No mockup UIs detected! All components seem functional.`
- Roadmap.md: `💪 157/157` — 100% siap.
