import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from check_branding import allowed_legacy_reference, parse_locale

class BrandingGateTests(unittest.TestCase):
    def test_legacy_reference_requires_exact_path_and_pattern(self):
        allowlist = [{
            "path": "scripts/install.sh",
            "pattern": r"(?i)/opt/pier",
            "reason": "legacy install root",
        }]
        self.assertIsNotNone(allowed_legacy_reference("scripts/install.sh", "ROOT=/opt/pier", allowlist))
        self.assertIsNone(allowed_legacy_reference("README.md", "ROOT=/opt/pier", allowlist))
        self.assertIsNone(allowed_legacy_reference("scripts/install.sh", "Product: Pier", allowlist))

    def test_locale_parser_tracks_nested_keys_and_placeholders(self):
        import tempfile
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "locale.yml"
            path.write_text("app:\n  welcome: Hello %{name}\n  nested:\n    count: Items %{count}\n", encoding="utf-8")
            keys, signatures = parse_locale(path)
        self.assertEqual(keys, {"app", "app.welcome", "app.nested", "app.nested.count"})
        self.assertEqual(signatures["app.welcome"], ("%{name}",))
        self.assertEqual(signatures["app.nested.count"], ("%{count}",))

    def test_global_allowlist_is_not_a_valid_design(self):
        from check_branding import load_inventory
        import tempfile, json
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "inventory.json"
            path.write_text(json.dumps({
                "schema_version": 1,
                "brand": {"name": "MOONSHIPS"},
                "allowlist": [{"path": "**", "pattern": "(?i)pier", "reason": "too broad"}],
            }), encoding="utf-8")
            with self.assertRaises(ValueError):
                load_inventory(path)

if __name__ == "__main__":
    unittest.main()
