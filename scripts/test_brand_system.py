#!/usr/bin/env python3
"""Regression tests for the canonical MOONSHIPS brand system."""
from __future__ import annotations

import json
import re
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class BrandSystemTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.tokens = json.loads((ROOT / "branding/brand-tokens.json").read_text(encoding="utf-8"))
        cls.css = (ROOT / "crates/pier-core/assets/static/moonships-brand.css").read_text(encoding="utf-8")
        cls.book = (ROOT / "docs/BRAND-BOOK.md").read_text(encoding="utf-8")

    def test_brand_tokens_are_valid_and_have_roles(self) -> None:
        self.assertEqual(self.tokens["brand"], "MOONSHIPS")
        self.assertEqual(self.tokens["direction"], "Space Punk / Underground Future")
        self.assertEqual(
            self.tokens["taglines"]["master"],
            "Your Infrastructure. Your Rules.",
        )
        for name, token in self.tokens["colors"].items():
            with self.subTest(color=name):
                self.assertRegex(token["value"], r"^#[0-9A-Fa-f]{6}$")
                self.assertTrue(token["role"].strip())

    def test_css_variables_match_machine_readable_tokens(self) -> None:
        aliases = {
            "void": "--ms-void",
            "carbon": "--ms-carbon",
            "paper": "--ms-paper",
            "chalk": "--ms-chalk",
            "acid": "--ms-acid",
            "orbit": "--ms-orbit",
            "ultraviolet": "--ms-ultraviolet",
            "riot": "--ms-riot",
            "muted": "--ms-muted",
        }
        for key, variable in aliases.items():
            with self.subTest(token=key):
                self.assertRegex(
                    self.css,
                    rf"{re.escape(variable)}:\s*{re.escape(self.tokens['colors'][key]['value'])};",
                )

    def test_product_shell_and_auth_templates_load_brand_styles(self) -> None:
        base = (ROOT / "crates/pier-core/assets/templates/base.html").read_text(encoding="utf-8")
        auth = (ROOT / "crates/pier-core/assets/templates/auth_base.html").read_text(encoding="utf-8")
        self.assertIn('/static/moonships-brand.css', base)
        self.assertIn('/static/moonships-brand.css', auth)
        self.assertIn('ms-shell-header', base)
        self.assertIn('ms-shell-header', auth)
        self.assertIn('Your Infrastructure. Your Rules.', base)
        self.assertIn('Your Infrastructure. Your Rules.', auth)

    def test_login_and_dashboard_use_controlled_brand_accents(self) -> None:
        login = (ROOT / "crates/pier-core/assets/templates/login.html").read_text(encoding="utf-8")
        dashboard = (ROOT / "crates/pier-core/assets/templates/dashboard.html").read_text(encoding="utf-8")
        self.assertIn('ms-auth-card', login)
        self.assertEqual(login.count('ms-primary-action'), 2)
        self.assertIn('NO PERMISSION NEEDED', (ROOT / "crates/pier-core/assets/templates/auth_base.html").read_text(encoding="utf-8"))
        self.assertIn('YOUR INFRASTRUCTURE. YOUR RULES.', dashboard)

    def test_brand_book_defines_all_four_dna_traits(self) -> None:
        for term in ("GRUNGE", "RIOT", "FUNKY", "GEN-Z"):
            with self.subTest(term=term):
                self.assertIn(term, self.book)
        self.assertIn("Operational UI stays legible", self.book)

    def test_social_preview_assets_are_valid_and_identical(self) -> None:
        docs_banner = ROOT / "docs/og-banner.svg"
        static_banner = ROOT / "crates/pier-core/assets/static/moonships-og-banner.svg"
        for path in (docs_banner, static_banner):
            root = ET.parse(path).getroot()
            self.assertEqual(root.tag.split("}")[-1], "svg")
            self.assertIn("MOONSHIPS", ET.tostring(root, encoding="unicode"))
        self.assertEqual(docs_banner.read_bytes(), static_banner.read_bytes())


if __name__ == "__main__":
    unittest.main()


    def test_auth_tagline_is_canonical_in_all_locales(self) -> None:
        expected = {
            "en.yml": "Your Infrastructure. Your Rules.",
            "es.yml": "Tu infraestructura. Tus reglas.",
            "zh-CN.yml": "你的基础设施，由你做主。",
        }
        for filename, tagline in expected.items():
            with self.subTest(locale=filename):
                locale = (ROOT / "crates/pier-core/locales" / filename).read_text(encoding="utf-8")
                self.assertIn(f"tagline: {tagline}", locale)

    def test_brand_messaging_guide_covers_core_narrative(self) -> None:
        guide = (ROOT / "docs/BRAND-MESSAGING.md").read_text(encoding="utf-8")
        for phrase in (
            "Your Infrastructure. Your Rules.",
            "No Permission Needed.",
            "self-hosted control plane",
            "GRUNGE",
            "RIOT",
            "FUNKY",
            "GEN-Z",
            "Proof before promise",
        ):
            with self.subTest(phrase=phrase):
                self.assertIn(phrase, guide)
