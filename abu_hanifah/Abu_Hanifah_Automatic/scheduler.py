"""Hourly Scheduler with Humanized Jitter for Threads Automatic Posting."""

import time
import random
from datetime import datetime, timedelta
from config import POST_INTERVAL_MINUTES, POST_INTERVAL_JITTER_MINUTES
from poster import dispatch_post


def run_scheduler(headless: bool = False, dry_run: bool = False) -> None:
    """Run recurring posting loop every 1 hour."""
    print("\n=======================================================")
    print("   ABU HANIFAH AUTOMATIC THREADS SCHEDULER AKTIF")
    print("=======================================================")
    print(f"Jarak Masa Asas: {POST_INTERVAL_MINUTES} Minit (1 Jam)")
    print(f"Variasi Masa Rawak (Jitter): +/- {POST_INTERVAL_JITTER_MINUTES} Minit")
    print("Tekan [Ctrl + C] pada bila-bila masa untuk berhenti.")
    print("=======================================================\n")

    iteration = 1
    while True:
        try:
            print(f"\n>>> [Pusingan #{iteration}] Memulakan hantaran post baru...")
            success = dispatch_post(headless=headless, dry_run=dry_run)
            
            if success:
                print(f"[+] Pusingan #{iteration} selesai dengan jaya.")
            else:
                print(f"[!] Pusingan #{iteration} mengalami masalah. Akan mencuba lagi pusingan seterusnya.")

            # Calculate next run time with humanized jitter
            jitter_secs = random.randint(
                -POST_INTERVAL_JITTER_MINUTES * 60,
                POST_INTERVAL_JITTER_MINUTES * 60
            )
            sleep_duration = max(1800, (POST_INTERVAL_MINUTES * 60) + jitter_secs)
            next_run = datetime.now() + timedelta(seconds=sleep_duration)

            print(f"\n[*] Menunggu pusingan seterusnya ({sleep_duration // 60} minit)...")
            print(f"[*] Hantaran seterusnya dijadualkan pada: {next_run.strftime('%Y-%m-%d %I:%M:%S %p')}")
            print("------------------------------------------------------------")

            iteration += 1
            time.sleep(sleep_duration)

        except KeyboardInterrupt:
            print("\n[!] Penjadual dihentikan oleh pengguna (Ctrl+C). Selamat tinggal!")
            break
        except Exception as e:
            print(f"[!] Ralat tidak dijangka dalam penjadual: {e}")
            time.sleep(60)

