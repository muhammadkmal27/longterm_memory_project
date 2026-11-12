"""Unit tests for BijakEnglish Content Generator."""

import unittest
from generator import (
    generate_post_content,
    get_next_post,
    record_published_post,
    load_history_data,
    TEMPLATES,
    CLOSING_CALLS,
)
from config import TARGET_URL


class TestGenerator(unittest.TestCase):
    def test_generate_post_contains_target_url(self):
        """Ensure every generated post contains the target website link."""
        for _ in range(10):
            post = generate_post_content()
            self.assertIn(TARGET_URL, post)
            self.assertIn("RM50", post)

    def test_templates_integrity(self):
        """Ensure all templates have hook and core properties."""
        self.assertGreaterEqual(len(TEMPLATES), 6)
        for t in TEMPLATES:
            self.assertIn("hook", t)
            self.assertIn("core", t)
            self.assertTrue(len(t["hook"]) > 10)
            self.assertTrue(len(t["core"]) > 50)

    def test_closing_calls_contain_url(self):
        """Ensure all closing calls contain the target link."""
        for call in CLOSING_CALLS:
            self.assertIn(TARGET_URL, call)

    def test_rotation_avoids_immediate_duplicates(self):
        """Verify that get_next_post avoids repeating used angles in the cycle."""
        seen_indices = set()
        for _ in range(len(TEMPLATES)):
            idx, name, hook, content, images = get_next_post(variation=1)
            seen_indices.add(idx)
        # Should have touched multiple unique templates
        self.assertGreater(len(seen_indices), 1)

    def test_variation_2_includes_images(self):
        """Verify that Variasi 2 attaches the 2 specific images."""
        idx, name, hook, content, images = get_next_post(variation=2)
        self.assertEqual(len(images), 2)
        self.assertTrue(any("variasi2a.png" in img for img in images))
        self.assertTrue(any("variasi2b.png" in img for img in images))
        self.assertTrue(len(content) < 480)


if __name__ == "__main__":
    unittest.main()
