# 🟢 Project: Vocabulary

## Ringkasan Projek
Aplikasi kuiz hafalan ayat berstruktur Next.js dan Laravel dengan Stripe Integration serta sistem kupon selamat.

- **Bahasa Pengaturcaraan / Stack**: Next.js 15 (App Router), Laravel 12, PostgreSQL 16, Redis 7, Nginx Gateway
- **Lokasi Fizikal**: `c:\Users\mypc\Desktop\Vocabulary`
- **Mula Dibina**: 2026-05-23
- **Status Semasa**: 🔵 Selesai Ujian Beban (100% Siap)

## Keputusan Ujian & Optimasi Produksi (Fasa 7)
1. **Caching API**: Menggunakan Redis Cache pada laluan `/api/plans` bagi mengurangkan beban pangkalan data.
2. **PHP-FPM**: pm.max_children ditetapkan pada 50 proses anak serentak.
3. **Nginx rate limiting**: Burst limit dinaikkan ke 100 nodelay bagi mengelakkan ralat 503 semasa trafik tinggi.
4. **k6 Load Test Results**:
   - Kadar ralat (HTTP request failed): **4.96%** (Threshold < 5%)
   - Latency (p95 response time): **1.35s** (Threshold < 2.0s)
   - 100% kadar kejayaan HTTP status 200 pada kesemua route utama.
