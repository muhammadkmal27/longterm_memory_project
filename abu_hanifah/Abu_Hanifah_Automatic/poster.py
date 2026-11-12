"""Dispatcher module linking the content generator and browser publisher."""

from datetime import datetime
from pathlib import Path
from generator import get_next_post, record_published_post, set_active_variation, load_history_data
from browser import post_to_threads


def dispatch_post(variation: int = None, headless: bool = False, dry_run: bool = False) -> bool:
    """Generate and dispatch one marketing thread."""
    if variation:
        set_active_variation(variation)

    v_active = variation or load_history_data().get("active_variation", 1)
    idx, name, hook, content, images = get_next_post(v_active)
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    print("\n------------------------------------------------------------")
    print(f"[{timestamp}] Ayat Promosi Dijana [{name}] (Variasi {v_active}):")
    print("------------------------------------------------------------")
    print(content)
    if images:
        print(f"[*] Lampiran Imej ({len(images)}): {[Path(img).name for img in images]}")
    print("------------------------------------------------------------\n")

    if dry_run:
        print("[INFO] Mod Pratonton (Dry Run). Tiada hantaran dibuat ke Threads.")
        return True

    success = post_to_threads(content, image_paths=images, headless=headless)
    if success:
        record_published_post(idx, name, hook, content, variation=v_active)
        print("[+] Sejarah pos telah dikemaskini dalam posts_history.json.")
    return success
