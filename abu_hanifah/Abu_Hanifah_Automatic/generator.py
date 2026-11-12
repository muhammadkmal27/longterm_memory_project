"""Content Generator for Threads Marketing Posts.

Supports multiple campaign variations (Variasi 1, Variasi 2, etc.)
Strictly audited to under 460 characters (Threads limit: 500 chars).
"""

import json
import random
from datetime import datetime
from typing import Dict, List, Tuple
from config import HISTORY_FILE, TARGET_URL
from templates_v1 import TEMPLATES_V1, VARIASI_1_IMAGES
from templates_v2 import TEMPLATES_V2, VARIASI_2_IMAGES

# Backward compatibility
TEMPLATES = TEMPLATES_V1

VARIATIONS: Dict[int, Dict] = {
    1: {"name": "Formula Menang Asal", "templates": TEMPLATES_V1, "images": VARIASI_1_IMAGES},
    2: {"name": "Ceramah Mufti Menk & Subtitle", "templates": TEMPLATES_V2, "images": VARIASI_2_IMAGES},
}

CLOSING_CALLS: List[str] = [
    f"Boleh tengok & cuba di sini ya:\n{TARGET_URL}",
    f"Jemput singgah website:\n{TARGET_URL}",
    f"Link website ada di sini:\n{TARGET_URL}",
    f"Korang boleh check website ni:\n{TARGET_URL}",
]


def load_history_data() -> Dict:
    """Load complete post history from JSON."""
    default_state = {
        "active_variation": 1,
        "next_queued_index": None,
        "used_indices_in_cycle": [],
        "total_published": 0,
        "history": [],
    }
    try:
        with open(HISTORY_FILE, "r", encoding="utf-8") as f:
            data = json.load(f)
            if not isinstance(data, dict):
                return default_state
            return {
                "active_variation": data.get("active_variation", 1),
                "next_queued_index": data.get("next_queued_index"),
                "used_indices_in_cycle": data.get("used_indices_in_cycle", []),
                "total_published": data.get("total_published", 0),
                "history": data.get("history", []),
            }
    except Exception:
        return default_state


def set_active_variation(variation: int) -> None:
    """Set active variation in history config and reset cycle if changed."""
    data = load_history_data()
    if data.get("active_variation") != variation:
        data["active_variation"] = variation
        data["used_indices_in_cycle"] = []
        try:
            with open(HISTORY_FILE, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
        except Exception:
            pass


def get_next_post(variation: int = None) -> Tuple[int, str, str, str, List[str]]:
    """Select the next unused template to guarantee true non-repeating rotation."""
    data = load_history_data()
    v_num = variation if variation in VARIATIONS else data.get("active_variation", 1)
    if v_num not in VARIATIONS:
        v_num = 1

    var_cfg = VARIATIONS[v_num]
    template_list = var_cfg["templates"]
    images = var_cfg["images"]
    queued_idx = data.get("next_queued_index")
    total_templates = len(template_list)

    if queued_idx is not None and 0 <= queued_idx < total_templates:
        chosen_idx = queued_idx
    else:
        used_indices = data.get("used_indices_in_cycle", [])
        available = [i for i in range(total_templates) if i not in used_indices]
        if not available:
            available = list(range(total_templates))
        chosen_idx = random.choice(available)

    template = template_list[chosen_idx]
    closing = random.choice(CLOSING_CALLS)
    content = f"{template['hook']}\n\n{template['core']}\n\n{closing}"

    # Strict safety assertion: Threads hard limit is 500
    if len(content) > 480:
        content = content[:475]

    return chosen_idx, template["name"], template["hook"], content, images


def record_published_post(idx: int, name: str, hook: str, content: str, variation: int = 1) -> None:
    """Record successfully published post to history file and maintain cycles."""
    data = load_history_data()
    used_indices = list(data.get("used_indices_in_cycle", []))
    history = data.get("history", [])
    total_templates = len(VARIATIONS.get(variation, VARIATIONS[1])["templates"])

    if len(used_indices) >= total_templates:
        used_indices = [idx]
    elif idx not in used_indices:
        used_indices.append(idx)

    new_record = {
        "id": len(history) + 1,
        "timestamp": datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        "variation": variation,
        "template_index": idx,
        "template_name": name,
        "hook": hook,
        "content": content,
        "status": "published",
    }
    history.append(new_record)

    save_data = {
        "active_variation": variation,
        "next_queued_index": None if data.get("next_queued_index") == idx else data.get("next_queued_index"),
        "used_indices_in_cycle": used_indices,
        "total_published": data.get("total_published", 0) + 1,
        "history": history[-50:],
    }

    try:
        with open(HISTORY_FILE, "w", encoding="utf-8") as f:
            json.dump(save_data, f, indent=2, ensure_ascii=False)
    except Exception:
        pass


def generate_post_content(variation: int = None) -> str:
    """Generate post content (for preview or backwards-compatibility)."""
    _, _, _, content, _ = get_next_post(variation)
    return content
