# Senarai Induk Ujian Ciri (Features & Unit Tests Checklist)

- [ ] `[ ]` **Ujian Modul Travers**: Pastikan modul `walkdir` memulangkan senarai fail yang betul dengan ekstensi `.php` dan `.tsx`.
- [ ] `[ ]` **Ujian TSX XSS**: Pastikan Tree-sitter AST berjaya mengesan fail `.tsx` olok-olok yang mengandungi `dangerouslySetInnerHTML`.
- [ ] `[ ]` **Ujian PHP XSS**: Pastikan Tree-sitter AST berjaya mengesan fail `.php` (Blade) yang mengandungi tag `{!! !!}`.
- [ ] `[ ]` **Ujian Race Condition PHP**: Pastikan Tree-sitter AST berjaya mengesan panggilan fungsi `$model->update()` yang tidak berada di dalam kurungan (closure) `DB::transaction()`.
- [ ] `[ ]` **Ujian Laporan CLI**: Pastikan laporan memaparkan nama fail, nombor baris, dan mesej ralat dengan format yang betul.
