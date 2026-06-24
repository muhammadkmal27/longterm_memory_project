# Panduan Penggunaan Abu Hanifah Safeguard (Pentest Tool)

Alat ini adalah sistem **Static Application Security Testing (SAST)** pantas yang dikuasakan oleh Rust dan *Abstract Syntax Tree* (AST). Ia bertindak sebagai pengawal keselamatan peribadi anda bagi mencari kerentanan berbahaya ("halusinasi AI") di dalam kod sumber sebelum dihantar ke pelayan produksi.

## Teknologi Sokongan (5 Tech Stack)

Abu Hanifah Safeguard kini mampu mengimbas dan menyokong senibina berikut:
1. **Next.js / React (Frontend)**
2. **Nest.js (Backend)**
3. **Laravel (PHP)**
4. **PHP Native**
5. **Rust Axum (Backend)**
6. **Java Spring Boot (Backend)**

---

## 🛡️ Senarai Semakan Keselamatan (Security Rules)

### [FASA 1] SAST Asas (Pengecaman AST Statik)
Menjejaki 27 jenis ancaman melalui corak pengekodan berisiko tinggi secara terus di dalam kod sumber:
- #4 (Cryptographic Failures)
- #5 (Injection)
- #10 (Mishandling of Exceptional Conditions)
- #11 (API Key Leaks)
- #16 (CRLF Injection)
- #17 (CSS Injection)
- #18 (CSV Injection)
- #22 (Command Injection)
- #25 (DOM Clobbering)
- #28 (Directory Traversal)
- #31 (File Inclusion LFI/RFI)
- #37 (Insecure Deserialization)
- #40 (Insecure Randomness)
- #55 (Regular Expression ReDoS)
- #59 (SQL Injection)
- #60 (Server Side Include Injection SSI)
- #61 (Server Side Request Forgery SSRF)
- #62 (Server Side Template Injection SSTI)
- #64 (Type Juggling)
- #65 (Upload Insecure Files)
- #68 (Web Sockets)
- #69 (XPATH Injection)
- #71 (XSLT Injection)
- #72 (XSS Injection)
- #73 (XXE Injection)
- #74 (Zip Slip)
- #79 (Buffer Overflow)

### [FASA 2] SAST Lanjutan (Taint Analysis / Data Flow)
Melindungi 14 ancaman *Advanced Injection* dengan menjejaki aliran data daripada input pengguna ke fungsi berbahaya:
- #21 (Client Side Path Traversal)
- #29 (Encoding Transformations)
- #30 (External Variable Modification)
- #34 (HTTP Parameter Pollution)
- #43 (Java RMI)
- #44 (LDAP Injection)
- #45 (LaTeX Injection)
- #48 (NoSQL Injection)
- #51 (Open Redirect)
- #53 (Prototype Pollution)
- #56 (Request Smuggling)
- #58 (SAML Injection)
- #67 (Web Cache Deception)
- #70 (XS-Leak)

### [FASA 3] Semantic Dependency & Configuration Auditor (Pengimbas White-Box)
Menganalisis 25 jenis ancaman berkaitan fail tetapan persekitaran (Environment), infrastruktur web, dan rantaian pembekal (Dependencies):
- #2 (Security Misconfiguration)
- #3 (Software Supply Chain Failures)
- #8 (Software or Data Integrity Failures)
- #9 (Security Logging and Alerting Failures)
- #15 (CORS Misconfiguration)
- #19 (CVE Exploits)
- #20 (Clickjacking)
- #23 (Cross-Site Request Forgery CSRF)
- #24 (DNS Rebinding)
- #26 (Denial of Service DoS)
- #27 (Dependency Confusion)
- #32 (Google Web Toolkit)
- #33 (GraphQL Injection)
- #35 (Headless Browser)
- #39 (Insecure Management Interface)
- #41 (Insecure Source Code Management)
- #42 (JSON Web Token JWT)
- #47 (Methodology and Resources)
- #49 (OAuth Misconfiguration)
- #57 (Reverse Proxy Misconfigurations)
- #63 (Tabnabbing)
- #66 (Virtual Hosts)
- #76 (_template_vuln)
- #78 (Man-in-the-Middle MitM)
- #81 (Watering Hole Attack)

### [FASA 4] Deep Architectural & Business Logic Enforcer (Pengimbas White-Box)
Bertindak sebagai *Senior Engineer* maya untuk menghalang 15 kerentanan maut di peringkat logik kod dan pangkalan data yang mustahil dikesan oleh SAST biasa:
- #1 (Broken Access Control)
- #6 (Insecure Design)
- #7 (Authentication Failures)
- #12 (Account Takeover)
- #13 (Brute Force Rate Limit)
- #14 (Business Logic Errors)
- #36 (Hidden Parameters)
- #38 (Insecure Direct Object References IDOR)
- #46 (Mass Assignment)
- #50 (ORM Leak)
- #52 (Prompt Injection)
- #54 (Race Condition)
- #75 (_LEARNING_AND_SOCIALS)
- #77 (Credential Stuffing)
- #80 (Reentrancy Attack)


---

## 🚀 Cara Penggunaan Sistem 4-Fasa (Integrasi AI Automatik)

Kini, Abu Hanifah Safeguard dikuasakan sepenuhnya oleh enjin **Antigravity (AI Verify)**. Anda tidak perlu lagi menaip arahan terminal secara manual. Cukup sekadar memberikan arahan (Prompt) kepada agen AI (Abu Hanifah) di ruangan sembang.

### Kata Kunci Arahan (Trigger Phrases)

Anda boleh mengarahkan ujian mengikut fasa tunggal, atau menjalankan kesemuanya serentak:

1. **Ujian Penuh (Kesemua 4 Fasa Serentak)**:
   > *"Abu Hanifah, jalankan abu hanifah safeguard pada projek {nama projek}"*
2. **Fasa 1 Sahaja (SAST Asas)**:
   > *"Abu Hanifah, jalankan fasa 1 Abu Hanifah Safeguard"*
3. **Fasa 2 Sahaja (Taint Analysis)**:
   > *"Abu Hanifah, jalankan fasa 2 Abu Hanifah Safeguard"*
4. **Fasa 3 Sahaja (Konfigurasi & Infrastruktur)**:
   > *"Abu Hanifah, jalankan fasa 3 Abu Hanifah Safeguard"*
5. **Fasa 4 Sahaja (Logik Perniagaan & Senibina)**:
   > *"Abu Hanifah, jalankan fasa 4 Abu Hanifah Safeguard"*

### Aliran Kerja "AI Verify" (Jaring Luas)
Alat ini menggunakan falsafah **Sensitiviti Maksimum (Jaring Luas)**. Ini bermakna skrip teras Rust akan memuntahkan segala potensi ancaman secara buta tanpa menapisnya, dan menyimpannya di dalam fail log sementara `raw_safeguard_output.json`.

Secara automatik, **Otak AI Abu Hanifah (Antigravity)** akan membaca log mentah tersebut, melawat sendiri setiap baris kod yang disenaraikan, dan membuang segala *False Positives* sebelum merumuskan hasil akhir yang tepat.

### Hasil Imbasan Akhir (pentest_audit.md)
Selepas proses tapisan *AI Verify* selesai, saya (Abu Hanifah) akan menjana sebuah fail induk bernama `pentest_audit.md` di dalam folder projek klien anda.

Fail audit tersebut mengandungi format **Senarai Semak (Checklist)** berserta **"Alasan AI"** yang menjelaskan rasional di sebalik setiap penemuan. Anda wajib membetulkan setiap kotak tanda `[ ]` kepada `[x]` sebelum membenarkan projek itu *go live*.

> *Peringatan: Alat ini adalah untuk tujuan pertahanan dan semakan jaminan kualiti (QA) mengikut kerangka Secure-by-Design. Sentiasa pastikan anda mematuhi 32 Peraturan Keselamatan Global apabila menulis kod asas.*
