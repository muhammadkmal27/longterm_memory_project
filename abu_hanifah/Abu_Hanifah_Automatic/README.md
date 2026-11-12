# 🤖 Abu_Hanifah_Automatic: Enjin Pemasaran Automatik Threads

Sistem automasi hantaran promosi ke **Threads** khas untuk mempromosikan **BijakEnglish** (`http://bijakenglish.imat.my/`) setiap 1 jam secara berterusan tanpa perlu buat manual.

---

## ⚡ Ciri-Ciri Utama

1. **Variasi Ayat Pintar (Dynamic Copywriting)**:
   - Berasaskan formula ayat pemenang (*winning copy*) yang telah terbukti menghasilkan jualan.
   - Mengubah cangkuk (*hook*) dan susunan ayat pada setiap jam agar Meta tidak mengesan hantaran sebagai robot/spam berulang.
   - Sentiasa menyertakan poin teras: **teori hafal ayat demi ayat**, **video 2 subtitle (Melayu/English)**, **harga RM50 setahun**, dan link laman web.
2. **Sesi Log Masuk Kekal (*Persistent Context*)**:
   - Anda hanya perlu log masuk sekali sahaja. Kuki dan sesi Threads disimpan selamat di dalam folder tempatan `threads_profile/`.
3. **Penjadual 1 Jam dengan Jitter Semula Jadi**:
   - Memuat naik post secara automatik setiap 1 jam (bersama variasi rawak +/- 5 minit) agar menyerupai tingkah laku manusia sebenar.

---

## 🚀 Panduan Penggunaan (4 Langkah Mudah)

Buka terminal **PowerShell** dan masuk ke folder ini:
```powershell
cd "c:\Users\mypc\Desktop\Longterm Memory Project\abu_hanifah\Abu_Hanifah_Automatic"
```

### Langkah 1: Log Masuk Threads (Sekali Sahaja)
Jalankan arahan ini:
```powershell
python main.py --login
```
- Tetingkap Google Chrome akan terbuka ke laman Threads.
- Sila log masuk ke akaun Threads / Instagram anda seperti biasa.
- Selepas selesai log masuk dan berada di halaman Feed Threads, kembali ke terminal dan tekan **[ENTER]**.
- Sesi anda telah disimpan selamat!

---

### Langkah 2: Uji & Lihat Contoh Ayat yang Dijana
Jika anda ingin melihat contoh variasi ayat yang akan diposkan:
```powershell
python main.py --preview
```

---

### Langkah 3: Uji Hantar 1 Post Serta-Merta (Ujian Pertama)
Untuk menguji sama ada bot berjaya membuka Threads dan menekan butang hantar:
```powershell
python main.py --post-now
```

---

### Langkah 4: Jalankan Automasi Setiap 1 Jam Berterusan
Bila anda sudah bersedia untuk membiarkan bot bekerja secara automatik setiap 1 jam:
```powershell
python main.py --start
```
- Bot akan menghantar 1 post sekarang, kemudian menunggu 1 jam sebelum menghantar variasi post baharu.
- Tekan **[Ctrl + C]** di terminal pada bila-bila masa jika ingin memberhentikan bot.

---

## 💬 Arahan Siap Guna (Template Prompt Harian Akhi Bersama Abu Hanifah)

Akhi boleh mengawal automasi ini terus di dalam ruangan sembang Antigravity tanpa perlu buka terminal setiap kali:

### 1. Arahan Memulakan Kempen (Setiap Pagi)
Salin dan hantar prompt ini di chat Antigravity:
> **"Abu Hanifah, mulakan kempen iklan Threads hari ini guna Variasi 1 (setiap 1 jam)."**  
> *(Atau: "Abu Hanifah, hari ini guna Variasi 2.")*

- Rujukan variasi diambil terus daripada fail [`variasi_iklan.md`](variasi_iklan.md).
- Abu Hanifah akan menghantar pos pusingan pertama serta-merta.
- Teks pos akan dipaparkan di ruangan chat untuk semakan akhi.
- Pemasa 1 jam (`schedule`) diaktifkan di latar belakang secara jimat token.
- Setiap 1 jam, sistem akan kejutkan Abu Hanifah untuk hantar variasi ayat seterusnya dan melapor di chat.

### 2. Rujukan Variasi Iklan
Akhi boleh menambah atau mengemas kini variasi iklan pada bila-bila masa di:
👉 [`variasi_iklan.md`](variasi_iklan.md)

### 2. Arahan Menyesuaikan Gaya Ayat (Bila-Bila Masa)
Jika akhi mahu mengubah gaya ayat promosi pada bila-bila masa:
> **"Abu Hanifah, pos seterusnya fokus pada golongan pekerja pejabat / santai / mak ayah."**

### 3. Arahan Berhenti / Rehat (Waktu Malam)
Bila tiba waktu malam atau ingin berehat:
> **"Abu Hanifah, rehat untuk hari ini."** *(atau tutup sahaja Antigravity)*.

---

## 🛠️ Pilihan Tambahan (Flags Terminal)

| Arahan | Fungsi |
| :--- | :--- |
| `python main.py --login` | Buka pelayar untuk log masuk akaun Threads kali pertama |
| `python main.py --preview` | Papar 3 contoh variasi ayat promosi tanpa memuat naik |
| `python main.py --post-now` | Jana dan pos 1 kandungan serta-merta |
| `python main.py --start` | Mulakan jadual automatik setiap 1 jam |
| `--headless` | Jalankan pelayar di latar belakang (tanpa paparan tetingkap Chrome) |
| `--dry-run` | Uji simulasi tanpa menekan butang hantar di Threads sebenar |

