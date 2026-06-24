# 🤝 Relationship Memory (User Preferences)

*Fail ini menyimpan sejarah projek pengguna, pantang-larang (rules), dan spesifikasi teknikal.*

## Pilihan Teknologi Teras (Core Stack)
- **Frontend**: Next.js, TailwindCSS
- **Backend**: Laravel (PHP) atau Rust (Axum)
- **Bahasa Komunikasi**: Bahasa Malaysia (BM) untuk perbualan, Implementation Plan, `roadmap.md`, dan `features.md`.
- **Bahasa Pengekodan**: English (EN) sepenuhnya untuk kod sumber, komen, dokumentasi API, serta semua data teknikal dalam SQL (seperti default values, trigger messages, dan seed data).
- **Flutter Development**: Menggunakan `device_preview` secara lalai (default) untuk projek Flutter bagi menyokong pratonton peranti yang luas (termasuk Apple iOS dan Android) semasa pembangunan.
- **Singkatan Khas**: `t` bermaksud "Teruskan" (Continue).
- **Protokol Kemajuan**: SETIAP KALI fitur baharu siap diimplementasi (tanpa mengira fasa), Abu Hanifah WAJIB segera mengemaskini fail `features.md` (untuk senarai semak Unit Test) dengan nama fitur yang spesifik dan granular. Fail `roadmap.md` pula WAJIB dikemaskini dengan status ✅ sejurus selepas fasa disiapkan. Kegagalan mengemaskini `features.md` atau `roadmap.md` dianggap melanggar protokol memori utama.
- **Protokol Pelaksanaan Berterusan (Continuous Execution Mode)**: Apabila pengguna memberi arahan pukal seperti "Siapkan Fasa 4", Abu Hanifah WAJIB menggunakan mod **/goal** dan melaksanakan kesemua Sub-Fasa di dalamnya secara berterusan dan berurutan dari mula hingga akhir (termasuk Sub-Fasa Mockup). JANGAN berhenti untuk meminta pengesahan kecil antara Sub-Fasa kecuali terdapat ralat kritikal yang memerlukan keputusan drastik daripada pengguna.
- **Protokol Pembersihan Kod**: SETIAP KALI ralat atau isu teknikal telah diselesaikan dan disahkan oleh pengguna (Cth: "Alhamdulillah" atau "dah boleh"), bersihkan semua kod debugging (`console.log`, `console.error`) dengan segera untuk menjaga integriti kod produksi.

## Protokol Deployment (Workflow)
- **Deployment Method**: GitHub Actions (CI/CD) -> GHCR -> Docker Compose Pull.
- **Git Protocol**: Hanya `git pull` untuk fail konfigurasi (`.yml`, `.conf`, `.env`). Dilarang `git pull` kod sumber (src) di server produksi.
- **Pencegahan Disk Penuh (Server Disk Cleanup)**: Setiap kali proses deployment selesai dijalankan oleh GitHub Actions, sistem WAJIB menjalankan `docker system prune -f` dan `docker builder prune -f` secara automatik di server produksi untuk mengelakkan disk storan penuh yang boleh menyebabkan database terhenti atau lumpuh dalam recovery mode.

## Piawaian Infrastruktur & Docker
- **Data Persistence (Volume)**: Setiap pangkalan data (PostgreSQL, Redis, dsb.) di dalam Docker wajib menggunakan Docker Volume atau Bind Mount yang tersendiri bagi mengelakkan kehilangan data semasa proses rebuild atau deployment (CI/CD).
- **Local Dev Environment Docker Dependency**: Setiap kali memulakan persekitaran pembangunan tempatan (local dev), Abu Hanifah WAJIB memeriksa dan memastikan perkhidmatan kontena (Docker Desktop) bagi semua pangkalan data (database), caching layer, dan middleware pihak ketiga yang diperlukan oleh projek tersebut telah dihidupkan terlebih dahulu sebelum menjalankan server aplikasi.

## Prinsip Pengekodan UI (Mobile vs Desktop)
- **Mobile-Specific Isolation**: Segala perubahan UI untuk versi mudah alih (mobile) MESTI diasingkan menggunakan responsive classes (contoh: `hidden lg:block`).
- **Desktop Integrity**: DILARANG keras mengubah rupa atau fungsi versi Desktop/Laptop semasa membuat pelarasan untuk Mobile.

## Pantang-Larang & Standard Keselamatan (User Global Rules)
*Sila aplikasikan peraturan ini pada setiap kod atau fungsi yang dijana:*

1.  **Strict Input Validation**: Implement rigorous server-side validation for all user-provided data (POST, GET, Cookies, Headers). Enforce specific data types, maximum length limits, and format constraints (e.g., Regex).
2.  **Universal Sanitization**: All data rendered in the UI or stored in a database must be sanitized or escaped using industry-standard methods (e.g., HTML escaping, DOM purification) to prevent XSS and injection attacks.
3.  **Prepared Statements Only**: Never use string concatenation for database queries. Always use Prepared Statements, Parameterized Queries, or safe ORM methods to prevent SQL Injection.
4.  **Business Logic Integrity**:
     - Include logical checks before critical operations.
     - Never trust client-side input for sensitive values.
5. **Object-Level Access Control (OLAC)**: Every SELECT, UPDATE, or DELETE query involving a user-supplied ID MUST explicitly include a user_id or tenant_id filter.
6. **Opaque/Randomized Resource Identifiers**: Use UUID v4 or ULID as the Primary Key or Public Identifier. Never expose Incremental Integer IDs.
7.  **Fail-Safe Error Handling**: Use `try-catch` blocks. Never expose detailed system errors (stack traces) to the end-user.
8.  **Secure Default State**: Apply the "Deny by Default" principle for authorization.
9.  **Data Integrity**: Use Atomic Transactions (Begin/Commit/Rollback) for any operation involving multiple tables.
10. **Cryptographic Best Practices**: Use modern algorithms (Argon2, Bcrypt, AES-256-GCM). Strictly avoid MD5/SHA1.
11. **Framework-Native Security**: Prioritize built-in security features (CSRF, Middleware).
12. **Modular Code & Separation of Concerns**: Atomic Functionality, Single Responsibility Principle, File Length Constraint (<250 lines), Interface-Based Interaction.
13. **Mandatory Automated Testing**: Test-Driven Logic, Boundary & Edge Cases, Dependency Isolation.
14. **Atomic Transactions & Concurrency Control**: Use Database Transactions and Row-Level Locking (e.g., SELECT FOR UPDATE).
15. **Database Resource Management**: Implement Connection Pooling and Indexing.
16. **Asynchronous Task Execution**: Time-consuming operations MUST be offloaded to Background Jobs or Message Queues.
17. **Distributed Caching Layer**: Integrate caching (Redis/Memcached) for "hot data".
18. **Advanced Attack Surface Reduction**: SSRF Prevention, Mass Assignment Protection, Safe Serialization (JSON), Information Leakage Prevention.
19. **CSRF Protection**: Valid CSRF token verification for all data modification requests.
20. **Session & Cookie Security**: Use `HttpOnly`, `Secure`, and `SameSite=Lax/Strict`.
21. **Rate Limiting**: Implement globally and strictly on sensitive functions.
22. **File Upload Integrity**: Validate MIME type via Magic Bytes. Store outside public directory with randomized names.
23. **Security Headers & CSP**: Set `Content-Security-Policy`, `X-Content-Type-Options: nosniff`, and `HSTS`.
24. **Security Event Logging**: Record important security activities securely.
25. **Webhook Signature Verification**: Authenticate incoming webhooks using Secret Signatures.
26. **Secure Password Requirements**: Enforce strong policies and avoid Plaintext storage.
27. **Dependency Vulnerability Scanning**: Periodically check for Known Vulnerabilities.
28. **Least Privilege Principle**: Minimum necessary permissions for DB and Cloud.
29. **API Documentation Security**: Do not expose sensitive info in Swagger/OpenAPI.
30. **Environment Variable Protection**: Never leak `.env` or hardcode secrets. Use secret managers.
31. **Mandatory Bot Protection**: Setiap borang (form) terutamanya yang melibatkan operasi penting seperti Log Masuk, Pendaftaran, atau Transaksi Pembayaran MESTI diintegrasikan dengan Cloudflare Turnstile di bahagian UI, serta disertakan dengan pengesahan token API (verify_turnstile) di bahagian Backend.
32. **Waiting Room Strategy**: Jika pengguna bertanya tentang Waiting Room, sentiasa ingatkan untuk menggunakan 2 pendekatan utama: (1) Admin Secret Query Parameter (bypass token kuki) untuk admin bypass mudah alih, dan (2) Sliding Window menggunakan Redis Sorted Set (ZSET) di Upstash Redis untuk mengira pelawat aktif dengan tepat tanpa kebocoran kaunter.
33. **Payment Gateway Minimum Amount Enforcement**: Wajib laksanakan semakan berlapis (Defense in Depth) sebelum menghantar bil/invois ke *Payment Gateway* (seperti ToyyibPay/Stripe). Nilai `billAmount` atau harga akhir MESTILAH melepasi semakan syarat ketat (contohnya `if ($billAmount <= 0) { throw new \Exception(); }`) walaupun harga itu diambil secara langsung dari *database*. Ini bagi menghalang bil RM0 terjana akibat kecuaian manusia (Admin tersalah letak harga RM0 di database) mahupun sisa manipulasi sesi *Sandbox/Testing*.


## Log Sejarah Evolusi Memori
- **2026-05-07**: Berjaya menggabungkan memori AI ke dalam sistem Antigravity tanpa API Key/Docker tambahan.
- **2026-05-07**: Naiktaraf infrastruktur memori kepada "MemoryCore Modular Architecture" berdasarkan inspirasi projek luaran.
- **2026-05-11**: Menambah Protokol Deployment GitHub Actions dan Peraturan Emas "Mobile-Specific Isolation" ke dalam Memori Utama.
- **2026-05-12**: Menambah Peraturan Emas "SQL Technical English" ke dalam Memori Utama (Abu Hanifah) untuk memastikan keselarasan data teknikal dalam database.
- **2026-05-12 (Update)**: Menetapkan Protokol Tegar Pembaruan `features.md` secara automatik bagi setiap fitur granular yang siap diimplementasi.
- **2026-05-14**: Menambah arahan mandatori (Rule 31) untuk sentiasa menggunakan Cloudflare Turnstile pada sebarang borang kritikal sebagai langkah perlindungan anti-bot standard.
- **2026-05-19**: Menambah peraturan 'Waiting Room Strategy' (Rule 32) pada user-preferences.md untuk membolehkan ingatan pasif apabila pengguna bertanya kelak.
- **2026-05-25**: Menambah "Protokol Pelaksanaan Berterusan (Continuous Execution Mode)" untuk memastikan arahan menyiapkan Fasa secara pukal dilakukan tanpa henti (menggunakan `/goal`) sehingga fasa penamat.
- **2026-05-29**: Menambah piawaian "Data Persistence (Volume)" bagi Docker database ke dalam memori utama (user preferences) untuk memastikan keselamatan data semasa deployment CI/CD.
- **2026-05-29 (Update)**: Menambah peraturan "Pencegahan Disk Penuh (Server Disk Cleanup)" di dalam Protokol Deployment untuk mengelakkan pengumpulan imej dan cache Docker lama yang memenuhi storan server.
- **2026-05-30**: Menambah peraturan umum "Local Dev Environment Docker Dependency" di bawah Piawaian Infrastruktur & Docker untuk mengelakkan ralat sambungan pangkalan data/middleware semasa permulaan local development.
- **2026-06-04**: Menambah ketetapan "Flutter Development" untuk menyarankan `device_preview` bagi mana-mana projek Flutter di masa hadapan.


