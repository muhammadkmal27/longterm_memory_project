"""Configuration settings for Abu Hanifah Automatic Threads Poster."""

import os
from pathlib import Path

# Base directories
BASE_DIR = Path(__file__).resolve().parent
PROFILE_DIR = BASE_DIR / "threads_profile"
HISTORY_FILE = BASE_DIR / "posts_history.json"
LOG_FILE = BASE_DIR / "bot_activity.log"

# Target Marketing Website
TARGET_URL = "http://bijakenglish.imat.my/"
THREADS_URL = "https://www.threads.com"

# Preferred Chrome installation path on Windows
CHROME_EXECUTABLE = r"C:\Program Files\Google\Chrome\Application\chrome.exe"

# Schedule settings (1 Hour = 60 minutes)
POST_INTERVAL_MINUTES = 60
POST_INTERVAL_JITTER_MINUTES = 5  # Random jitter between -5 to +5 minutes

# Browser behavior settings
PAGE_TIMEOUT_MS = 45000
TYPING_DELAY_MS = 25
HEADLESS_MODE = False  # Set to True once verified, or False to see browser

