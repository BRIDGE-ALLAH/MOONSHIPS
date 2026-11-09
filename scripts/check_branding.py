#!/usr/bin/env python3
"""Deterministic public-brand consistency gate for MOONSHIPS."""
from __future__ import annotations

import argparse
import fnmatch
import json
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INVENTORY_PATH = ROOT / "branding" / "brand-inventory.json"
TEXT_SUFFIXES = {".md", ".html", ".yml", ".yaml", ".toml", ".sh", ".py", ".rs", ".js", ".json", ".svg", ".css", ".xml", ".service"}
PIER_RE = re.compile(r"pier", re.IGNORECASE)
PLACEHOLDER_RE = re.compile(r"%\{[A-Za-z_][A-Za-z0-9_]*\}|\{\{\s*[^}]+\s*\}\}")

def load_inventory(path: Path = INVENTORY_PATH) -> dict:
    data = json.loads(path.read_text(encoding="utf-8"))
    if data.get("schema_version") != 1 or not data.get("brand", {}).get("name"):
        raise ValueError("unsupported or incomplete branding inventory")
    for entry in data.get("allowlist", []):
        if not entry.get("path") or not entry.get("pattern") or not entry.get("reason"):
            raise ValueError("every compatibility allowlist entry needs path, pattern, and reason")
        if entry["path"] in {"*", "**", "**/*"}:
            raise ValueError("global compatibility allowlists are forbidden")
        re.compile(entry["pattern"])
    return data

def allowed_legacy_reference(path: str, line: str, allowlist: list[dict]) -> dict | None:
    for entry in allowlist:
        if not fnmatch.fnmatchcase(path, entry["path"]) or not re.search(entry["pattern"], line):
            continue
        # In rendered templates and application locales, compatibility identifiers
        # may remain, but a standalone/title-case Pier or PIER word is public-brand
        # leakage and must be corrected rather than allowlisted.
        if path.startswith("crates/pier-core/assets/templates/") or path.startswith("crates/pier-core/locales/"):
            if re.search(r"(?<![A-Za-z0-9])Pier(?![A-Za-z0-9])", line):
                return None
            if re.search(r"(?<![A-Za-z0-9])PIER(?![_A-Za-z0-9])", line):
                return None
        return entry
    return None

def parse_locale(path: Path) -> tuple[set[str], dict[str, tuple[str, ...]]]:
    keys: set[str] = set()
    signatures: dict[str, tuple[str, ...]] = {}
    stack: list[tuple[int, str]] = []
    mapping = re.compile(r"^( *)([A-Za-z0-9_-]+):(?:[ \t]+(.*)|[ \t]*)$")
    for raw in path.read_text(encoding="utf-8").splitlines():
        match = mapping.match(raw)
        if not match:
            continue
        indent = len(match.group(1))
        key = match.group(2)
        value = match.group(3) or ""
        while stack and stack[-1][0] >= indent:
            stack.pop()
        stack.append((indent, key))
        full_key = ".".join(item[1] for item in stack)
        keys.add(full_key)
        signatures[full_key] = tuple(sorted(set(PLACEHOLDER_RE.findall(value))))
    return keys, signatures

def expand_targets(targets: list[str]) -> list[Path]:
    files: set[Path] = set()
    for target in targets:
        path = ROOT / target
        if not path.exists():
            raise FileNotFoundError(f"branding scan target does not exist: {target}")
        if path.is_dir():
            files.update(p for p in path.rglob("*") if p.is_file() and p.suffix.lower() in TEXT_SUFFIXES)
        else:
            files.add(path)
    return sorted(files, key=lambda p: p.relative_to(ROOT).as_posix())

def run_checks(inventory: dict) -> dict:
    brand = inventory["brand"]["name"]
    matches: list[dict] = []
    failures: list[str] = []
    for file_path in expand_targets(inventory["scan_targets"]):
        rel = file_path.relative_to(ROOT).as_posix()
        try:
            text = file_path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            failures.append(f"{rel}: scanned file is not valid UTF-8 text")
            continue
        for line_no, line in enumerate(text.splitlines(), 1):
            if PIER_RE.search(line):
                entry = allowed_legacy_reference(rel, line, inventory["allowlist"])
                if entry is None:
                    failures.append(f"{rel}:{line_no}: unclassified legacy brand reference")
                else:
                    matches.append({
                        "path": rel,
                        "line": line_no,
                        "reason": entry["reason"],
                        "text": line.strip()[:240],
                    })

    locale_files = [ROOT / p for p in inventory["locale_files"]]
    locale_data = [(p, *parse_locale(p)) for p in locale_files]
    baseline_path, baseline_keys, baseline_signatures = locale_data[0]
    for path, keys, signatures in locale_data[1:]:
        rel = path.relative_to(ROOT).as_posix()
        missing = sorted(baseline_keys - keys)
        extra = sorted(keys - baseline_keys)
        if missing or extra:
            failures.append(f"{rel}: locale key parity mismatch; missing={missing[:10]} extra={extra[:10]}")
        for key in sorted(baseline_keys & keys):
            if baseline_signatures.get(key, ()) != signatures.get(key, ()):
                failures.append(f"{rel}: placeholder mismatch for {key}")

    for path in locale_files:
        if brand.casefold() not in path.read_text(encoding="utf-8").casefold():
            failures.append(f"{path.relative_to(ROOT).as_posix()}: required brand name is missing")

    for asset in inventory["assets"]:
        path = ROOT / asset["path"]
        if not path.is_file():
            failures.append(f"{asset['path']}: required brand asset is missing")
            continue
        try:
            root = ET.parse(path).getroot()
        except ET.ParseError as exc:
            failures.append(f"{asset['path']}: invalid SVG/XML: {exc}")
            continue
        if root.tag.split("}")[-1].lower() != "svg":
            failures.append(f"{asset['path']}: root element is not SVG")
        if asset.get("must_contain_brand"):
            serialized = ET.tostring(root, encoding="unicode")
            if brand.casefold() not in serialized.casefold():
                failures.append(f"{asset['path']}: SVG has no accessible MOONSHIPS brand label")
        for ref in asset.get("references", []):
            target = ROOT / ref["path"]
            if not target.is_file() or ref["needle"] not in target.read_text(encoding="utf-8"):
                failures.append(f"{asset['path']}: expected reference missing from {ref['path']}")

    required_urls = [
        ("README.md", inventory["brand"]["repository_url"]),
        ("INSTALL.md", inventory["brand"]["repository_url"]),
        ("CONTRIBUTING.md", inventory["brand"]["issues_url"]),
        ("SECURITY.md", inventory["brand"]["security_url"]),
    ]
    for rel, needle in required_urls:
        if needle not in (ROOT / rel).read_text(encoding="utf-8"):
            failures.append(f"{rel}: canonical MOONSHIPS URL missing: {needle}")

    css = (ROOT / "crates/pier-core/assets/static/tailwind.css").read_text(encoding="utf-8")
    if ".bg-moonships-600" not in css or ".hover\\:bg-moonships-700" not in css:
        failures.append("compiled Tailwind CSS is missing the MOONSHIPS brand palette utilities")

    return {
        "brand": brand,
        "scanned_files": len(expand_targets(inventory["scan_targets"])),
        "classified_legacy_references": matches,
        "classified_legacy_reference_count": len(matches),
        "failures": failures,
        "status": "passed" if not failures else "failed",
    }

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--report", default="branding-scan-report.json")
    args = parser.parse_args()
    try:
        inventory = load_inventory()
        report = run_checks(inventory)
    except Exception as exc:
        print(f"branding gate error: {exc}", file=sys.stderr)
        return 2
    report_path = ROOT / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({k: v for k, v in report.items() if k != "classified_legacy_references"}, indent=2, ensure_ascii=False))
    for item in report["classified_legacy_references"]:
        print(f"ALLOW {item['path']}:{item['line']} — {item['reason']}")
    return 0 if report["status"] == "passed" else 1

if __name__ == "__main__":
    raise SystemExit(main())
