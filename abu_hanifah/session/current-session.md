# ⏳ Current Session RAM: NetLimiter WARP (Abu Hanifah)

_Nota: Fail ini bertindak sebagai memori sementara (scratchpad) semasa Antigravity melaksanakan tugasan._

## Active Project
- **Name**: NetLimiter WARP (Kawalan Jalur Lebar Khusus Cloudflare One / `warp-svc.exe`)
- **Status Terkini**: Enjin Pacing Berjaya 100% (5.2 Mbps @ 750 KB/s limit) & Butang Minimize/Close Selesai Dibaiki (2026-11-14 19:12).
- **Lokasi Projek Sumber**: `c:\Users\mypc\Desktop\net_limiter_own_version`
- **Lokasi Aplikasi Desktop Siap**: `C:\Users\mypc\Desktop\NetLimiter-WARP-App\net-limiter-warp.exe`
- **Pintasan Desktop**: `C:\Users\mypc\Desktop\NetLimiter WARP.lnk`

---

## 🔍 Penambahbaikan Terbaharu (Butang Minimize & Close):
1. **Pemisahan Kawasan Seret (Drag Region) & Butang:**
   - Atribut `data-tauri-drag-region` sebelum ini diletakkan pada keseluruhan `<header>`, menyebabkan klik tetikus pada butang Minimize dan Close ditafsirkan sebagai peristiwa seret tetingkap (*window drag event*) oleh enjin WebView2.
   - Telah dibaiki: Kawasan seret kini dihadkan khusus pada bahagian tajuk sahaja (`flex-1`), manakala bekas butang diasingkan sepenuhnya dengan `data-tauri-drag-region="false"` dan `z-10`.
2. **Panggilan Arahan Terus Tauri IPC (Native Window Commands):**
   - Menggantikan API JavaScript mentah dengan arahan teras Rust Tauri: `commands::minimize_window` dan `commands::close_window` (`app_handle.exit(0)`).
   - Menambah kebenaran `core:window:allow-minimize` dan `core:window:allow-close` dalam `capabilities/default.json`.
3. **Penyebaran Binari Baharu:**
   - Binari release terbaharu (`10,118,656 bytes`) telah berjaya dikompilasi dan diletakkan di `C:\Users\mypc\Desktop\NetLimiter-WARP-App\net-limiter-warp.exe`.
