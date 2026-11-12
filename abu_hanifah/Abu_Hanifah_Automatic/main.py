"""CLI Entrypoint for Abu_Hanifah_Automatic Threads Marketing Bot."""

import argparse
import sys
from browser import interactive_login
from poster import dispatch_post
from scheduler import run_scheduler
from generator import generate_post_content


def print_banner():
    banner = r"""
    ===============================================================
       ABU HANIFAH AUTOMATIC THREADS POSTER v1.0
       Enjin Pemasaran Automatik BijakEnglish (Setiap 1 Jam)
    ===============================================================
    """
    print(banner)


def preview_posts(count: int = 3, variation: int = None):
    v_label = f" (Variasi {variation})" if variation else ""
    print(f"\n[*] Menjana {count} contoh variasi ayat promosi BijakEnglish{v_label}:\n")
    for i in range(1, count + 1):
        print(f"--- [ CONTOH #{i} ] ---")
        print(generate_post_content(variation))
        print()


def main():
    parser = argparse.ArgumentParser(
        description="Abu Hanifah Automatic Threads Poster for BijakEnglish."
    )
    parser.add_argument(
        "--login",
        action="store_true",
        help="Buka pelayar Chrome untuk log masuk Threads secara manual sekali sahaja.",
    )
    parser.add_argument(
        "--post-now",
        action="store_true",
        help="Jana dan hantar 1 post ke Threads dengan segera (tanpa menunggu 1 jam).",
    )
    parser.add_argument(
        "--preview",
        action="store_true",
        help="Papar contoh variasi ayat iklan yang dijana tanpa membuat sebarang hantaran.",
    )
    parser.add_argument(
        "--variation",
        type=int,
        default=None,
        help="Pilih nombor variasi iklan rujukan (contoh: 1 atau 2).",
    )
    parser.add_argument(
        "--start",
        action="store_true",
        help="Mulakan penjadual automatik setiap 1 jam secara berterusan.",
    )
    parser.add_argument(
        "--headless",
        action="store_true",
        help="Jalankan pelayar di latar belakang tanpa memaparkan tetingkap Chrome.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Simulasikan proses penjanaan ayat tanpa menekan butang hantar di Threads.",
    )

    args = parser.parse_args()
    print_banner()

    if args.login:
        interactive_login()
    elif args.preview:
        preview_posts(3, variation=args.variation)
    elif args.post_now:
        dispatch_post(variation=args.variation, headless=args.headless, dry_run=args.dry_run)
    elif args.start:
        run_scheduler(headless=args.headless, dry_run=args.dry_run)
    else:
        parser.print_help()
        print("\nContoh Arahan Pantas:")
        print("  1. Log Masuk:   python main.py --login")
        print("  2. Uji Ayat:    python main.py --preview")
        print("  3. Hantar 1x:   python main.py --post-now")
        print("  4. Jadual 1 Jam: python main.py --start\n")


if __name__ == "__main__":
    main()

