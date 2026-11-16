# 🔥 Post-Mortems — Failure Learning Log
*What went wrong, why, and what we'll do differently. No blame, only growth.*

---

## Log

### PM-001: WinDivert Blocking Deadlock on UDP Port Migration (2026-11-14)
- **Gejala / Kegagalan:** Walaupun Master Limiter berstatus "ENGAGED" dan had ditetapkan kepada 750 KB/s (~6 Mbps), ujian Fast.com masih mencatatkan 22-34 Mbps manakala telemetri memaparkan `0.0 KB/s` (0.0 MB).
- **Punca Tersembunyi (Root Cause):**
  1. WinDivert handle dibuka sebagai blocking socket secara default.
  2. Apabila `warp-svc` memulakan semula soket atau menukar port UDP tempatan (contohnya dari 61968 ke 62855 semasa sambungan MASQUE), panggilan `divert.recv()` terperangkap dalam keadaan sekatan kernel (*kernel blocking deadlock*) menunggu paket pada port lama yang sudah sunyi sepi.
  3. Kerana benang pembatas (*pacer thread*) terbeku dalam `recv()`, gelung utama tidak dapat berjalan untuk membaca port baharu dari `state.tracked_ports`.
- **Tindakan Pembaikan (Preventative Fix):**
  1. Laksanakan mekanisme *Immediate Unblock Signal* menggunakan `WinDivertShutdown(handle, WinDivertShutdownMode::Recv)`. Apabila `process_tracker` mengesan perubahan port, ia mematikan handle lama secara atomik, menyebabkan `recv()` terkeluar serta-merta dengan ralat terkawal dan membuka semula filter baharu.
  2. Perluaskan penapis WinDivert untuk menyasarkan terus IP gerbang Cloudflare WARP Anycast (`162.159.198.2`): `(ip.DstAddr == 162.159.198.2 or ip.SrcAddr == 162.159.198.2)`. Ini memastikan semua paket WARP dipintas secara konsisten walaupun port soket tempatan berpindah.
