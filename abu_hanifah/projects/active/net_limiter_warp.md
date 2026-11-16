# net_limiter_warp - Cloudflare One Bandwidth Pacer
*Lightweight Windows Desktop Bandwidth Limiter khusus untuk warp-svc.exe*

## Project Overview
- **Type**: Windows Desktop Utility App
- **Period**: 2026-11-13 - Active
- **Tech Stack**: Tauri v2 + Rust (Backend) + WinDivert + Windows IP Helper API + React + TypeScript + Tailwind CSS
- **Completion**: 100% (Siap & Teruji)
- **Duration**: ~45 min

## Current Status
- **Last Session**: 2026-11-13 - Pengekodan penuh, ujian unit 6/6 lulus, binari `net-limiter-warp.exe` siap dibina bersama pemacu kernel WinDivert64.sys.
- **Next Steps**: Pelancaran aplikasi oleh pengguna (`Start-Process .\src-tauri\target\debug\net-limiter-warp.exe -Verb RunAs`).
- **Known Issues**: None

## Session History (Last 5)

### 2026-11-13 - Pembinaan Penuh & Ujian Berjaya (100%)
- **Changes**:
  1. Menyiapkan arkitektur modular Rust: `process_tracker.rs`, `pacer.rs`, `commands.rs`, `state.rs`, `lib.rs`, `main.rs`.
  2. Membina UI React + Tailwind lengkap: `App.tsx`, `TitleBar.tsx`, `WarpStatusCard.tsx`, `MasterToggle.tsx`, `LimitControlCard.tsx`, `LiveTelemetryCard.tsx`.
  3. Memasang pemacu kernel Microsoft-signed `WinDivert.dll` dan `WinDivert64.sys` dengan automasi salinan di `build.rs` dan manifes `requireAdministrator`.
  4. Lulus 100% ujian unit automasi (6/6 ujian `cargo test`).
  5. Berjaya membina binari sedia guna `src-tauri\target\debug\net-limiter-warp.exe`.
- **Time Spent**: ~45 min

## Historical Summary
[No history yet -- this section is populated when session count exceeds 5]

## Technical Notes
- **Repository**: c:\Users\mypc\Desktop\net_limiter_own_version
- **Key Dependencies**: tauri v2, windivert (vendored), windows (IpHelper, ToolHelp, WinSock), sysinfo, spin_sleep, react, lucide-react, tailwindcss
- **Security Requirement**: Windows Administrator elevation (UAC `requireAdministrator` manifest) untuk akses driver WinDivert64.sys.

---
**Last Updated**: 2026-11-13 | **Position**: #1/10 Active
