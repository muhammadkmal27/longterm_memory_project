"""Browser Automation Manager for Threads using Playwright Persistent Context."""

import os
import sys
import time
import logging
from pathlib import Path
from playwright.sync_api import sync_playwright, BrowserContext, Page
from config import (
    PROFILE_DIR,
    CHROME_EXECUTABLE,
    THREADS_URL,
    PAGE_TIMEOUT_MS,
    TYPING_DELAY_MS,
    LOG_FILE,
)

# Setup logging
logging.basicConfig(
    filename=LOG_FILE,
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
)


def get_browser_args():
    """Return stealth arguments to prevent bot detection."""
    return [
        "--disable-blink-features=AutomationControlled",
        "--start-maximized",
        "--no-sandbox",
        "--disable-infobars",
    ]


def open_persistent_context(p, headless: bool = False) -> BrowserContext:
    """Launch or resume persistent browser context."""
    PROFILE_DIR.mkdir(parents=True, exist_ok=True)
    
    kwargs = {
        "user_data_dir": str(PROFILE_DIR),
        "headless": headless,
        "channel": "chrome",
        "args": get_browser_args(),
        "viewport": {"width": 1280, "height": 800},
    }
    return p.chromium.launch_persistent_context(**kwargs)


def interactive_login() -> bool:
    """Open visible browser to allow the user to log in manually once with auto-detection."""
    print("\n=======================================================")
    print("      SESI LOG MASUK MANUAL THREADS (SEKALI SAHAJA)")
    print("=======================================================")
    print("1. Tetingkap Google Chrome dibuka sekarang.")
    print("2. Sila log masuk ke akaun Threads / Instagram anda.")
    print("3. Sistem akan mengesan status log masuk anda secara automatik.")
    print("=======================================================\n")

    with sync_playwright() as p:
        context = open_persistent_context(p, headless=False)
        page = context.pages[0] if context.pages else context.new_page()
        page.goto(f"{THREADS_URL}/login", timeout=PAGE_TIMEOUT_MS, wait_until="commit")

        print("[*] Menunggu anda log masuk di pelayar Chrome...")
        start_time = time.time()
        logged_in = False
        
        # Monitor for successful login up to 10 minutes (600 seconds)
        while time.time() - start_time < 600:
            time.sleep(3)
            try:
                if page.is_closed():
                    print("[!] Tetingkap pelayar telah ditutup.")
                    break
                curr_url = page.url.lower()
                if any(dom in curr_url for dom in ["threads.net", "threads.com"]) and "/login" not in curr_url:
                    if is_user_logged_in(page) or curr_url.rstrip("/") in [
                        THREADS_URL.lower(),
                        "https://www.threads.net",
                        "https://threads.net",
                        "https://www.threads.com",
                        "https://threads.com",
                    ]:
                        time.sleep(4)  # Allow cookies and storage to settle
                        logged_in = True
                        break
            except Exception:
                pass

        if logged_in:
            print("[+] Log masuk dikesan! Menyimpan sesi ke folder profil...")
            time.sleep(2)
            context.close()
            print("[+] Selesai! Sesi anda telah berjaya disimpan.\n")
            return True
        else:
            print("[!] Masa log masuk tamat atau tetingkap ditutup.")
            context.close()
            return False


def is_user_logged_in(page: Page) -> bool:
    """Check if the current page has an active logged-in session."""
    current_url = page.url.lower()
    if "/login" in current_url:
        return False
    # If create button, navigation icons or feed elements are visible
    create_indicators = [
        '[aria-label="New thread"]',
        '[aria-label*="compose a new post"]',
        '[aria-label="Profile"]',
        '[aria-label="Activity"]',
        'a[href*="/@"]',
        'svg[aria-label="Threads"]',
        'div[role="button"][aria-label*="compose"]',
    ]
    for sel in create_indicators:
        try:
            if page.locator(sel).first.is_visible(timeout=3000):
                return True
        except Exception:
            continue
    return "threads.com" in current_url and "/login" not in current_url


def post_to_threads(post_text: str, image_paths: list = None, headless: bool = False) -> bool:
    """Publish a new post to Threads with optional image attachments."""
    with sync_playwright() as p:
        context = open_persistent_context(p, headless=headless)
        try:
            page = context.pages[0] if context.pages else context.new_page()
            page.set_default_timeout(PAGE_TIMEOUT_MS)
            
            print("[*] Membuka laman utama Threads...")
            page.goto(THREADS_URL, wait_until="commit")
            try:
                page.wait_for_selector("#barcelona-splash-screen", state="detached", timeout=15000)
            except Exception:
                pass
            time.sleep(2)

            # Check authentication
            if not is_user_logged_in(page):
                msg = "Sesi belum log masuk! Sila jalankan 'python main.py --login' terlebih dahulu."
                print(f"[!] {msg}")
                logging.warning(msg)
                context.close()
                return False

            print("[*] Mencari kotak 'Start a thread' / Butang Create...")
            elem = None
            for _ in range(15):
                for cand in page.locator(
                    '[aria-label="New thread"], div[role="button"][aria-label*="compose"]'
                ).all():
                    if cand.is_visible():
                        elem = cand
                        break
                if elem:
                    break
                time.sleep(1)

            if not elem:
                elem = page.locator('[aria-label="New thread"]').first

            elem.click()
            time.sleep(2)

            # Locate the editable text box
            print("[*] Mengisi kandungan ayat promosi...")
            textbox = page.wait_for_selector(
                'div[role="textbox"][contenteditable="true"]', timeout=15000
            )
            textbox.click()
            time.sleep(1)

            # Type text smoothly to emulate human interaction
            lines = post_text.split("\n")
            for i, line in enumerate(lines):
                if line:
                    textbox.type(line, delay=TYPING_DELAY_MS)
                if i < len(lines) - 1:
                    page.keyboard.press("Shift+Enter")
                    time.sleep(0.05)

            time.sleep(4)  # Allow Threads to attach link preview card

            # Attach images if specified
            if image_paths:
                valid_imgs = [str(Path(p).resolve()) for p in image_paths if Path(p).exists()]
                if valid_imgs:
                    print(f"[*] Melampirkan {len(valid_imgs)} imej ke pos Threads...")
                    page.locator('input[type="file"]').first.set_input_files(valid_imgs)
                    time.sleep(4)

            # Find Post button inside composer dialog
            print("[*] Menekan butang 'Post' di tetingkap dialog...")
            dialog = page.locator('div[role="dialog"]').first
            post_button = None
            if dialog.is_visible(timeout=4000):
                for b in dialog.locator('div[role="button"]').all():
                    if b.inner_text().strip() == "Post":
                        post_button = b
                        break

            if not post_button:
                post_button = page.locator('div[role="button"]:has-text("Post")').last

            post_button.click(force=True)
            print("[*] Menunggu hantaran selesai dimuat naik...")
            time.sleep(8)

            print("[+] Alhamdulillah! Post berjaya dihantar ke Threads.")
            logging.info("Successfully published new post to Threads.")
            context.close()
            return True

        except Exception as e:
            error_msg = f"Ralat semasa memuat naik post: {e}"
            print(f"[!] {error_msg}")
            logging.error(error_msg)
            context.close()
            return False

