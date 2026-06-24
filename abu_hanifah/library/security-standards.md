# 🛡️ Standard Keselamatan (Security Standards)

*Fail ini merupakan rujukan kekal di dalam Library untuk memastikan semua kod yang dijana oleh Antigravity mematuhi piawaian keselamatan tertinggi.*

## 1. Perlindungan Suntikan Pangkalan Data (Database Injection)
*   **Prepared Statements**: Semua kueri pangkalan data WAJIB menggunakan parametrisasi (parameterized queries) atau Prepared Statements.
*   **Tiada Pemalar String (No String Concatenation)**: Dilarang sama sekali menyambung pembolehubah terus ke dalam rentetan SQL.

## 2. Pencegahan XSS (Cross-Site Scripting)
*   **Sanitasi Universal**: Semua data yang diterima dari pengguna mesti disanitasi sebelum disimpan (contoh: membuang tag skrip).
*   **Escaping**: Semua output yang dipaparkan di bahagian antaramuka (UI) mesti di-escape menggunakan fungsi terbina-dalam framework (contoh: `{{ }}` dalam Blade/Laravel, atau raw teks dalam React).

## 3. Kawalan Akses & Pengesahan (Authorization)
*   **Object-Level Access Control (OLAC)**: Setiap kueri (terutamanya UPDATE atau DELETE) mesti menyemak pemilikan data.
    *   *Contoh Betul*: `SELECT * FROM tickets WHERE id = ? AND user_id = ?`
    *   *Contoh Salah*: `SELECT * FROM tickets WHERE id = ?`

## 4. Pengurusan ID
*   **Opaque IDs**: Gunakan UUID v4 atau ULID sebagai Primary Key (atau Public Identifier) untuk sebarang sumber yang boleh diakses melalui URL. JANGAN dedahkan Incremental Integer IDs (contoh: `id=123`) untuk mengelakkan *Insecure Direct Object Reference (IDOR)*.
