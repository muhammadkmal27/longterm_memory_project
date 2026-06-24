# Roadmap Projek: Safeguard AI (Rust SAST Tool)

## Fasa 1: Perancangan Utama
- [x] Visi & Misi Ditetapkan
- [x] Senibina AST (Tree-sitter) Dipersetujui

## Fasa 2: Persediaan Teras (Core Setup)
- [ ] Tugasan 2.1: Inisialisasi projek Cargo & dependencies (`clap`, `tree-sitter`, dll).
  - *Ujian (Test)*: Memastikan `cargo check` lulus tanpa ralat.

## Fasa 3: Enjin Travers Direktori
- [ ] Tugasan 3.1: Membina modul yang membaca fail rekursif (berulang) dalam folder menggunakan `walkdir`.
  - *Ujian (Test)*: Menulis unit test yang membaca mock folder dan me-return senarai fail `.php` dan `.tsx` sahaja.

## Fasa 4: Pengimbas XSS (TSX/Next.js)
- [ ] Tugasan 4.1: Mengkonfigurasi `tree-sitter-typescript` (TSX).
- [ ] Tugasan 4.2: Membina AST Query untuk `dangerouslySetInnerHTML`.
  - *Ujian (Test)*: Menulis unit test `test_detect_xss_tsx` menggunakan fail mock `.tsx`.

## Fasa 5: Pengimbas XSS & Race Condition (PHP/Laravel)
- [ ] Tugasan 5.1: Mengkonfigurasi `tree-sitter-php`.
- [ ] Tugasan 5.2: Membina AST Query untuk pembolehubah mentah `{!! !!}`.
- [ ] Tugasan 5.3: Membina AST Query untuk mengesan ketiadaan `DB::transaction` pada `->update()` atau `->insert()`.
  - *Ujian (Test)*: Menulis unit test `test_detect_xss_php` dan `test_detect_race_condition_php`.

## Fasa 6: Modul Pelaporan Akhir
- [ ] Tugasan 6.1: Membina antaramuka laporan terminal berwarna (`cli_output.rs`).
  - *Ujian (Test)*: Menguji output stdout untuk kepastian format.
