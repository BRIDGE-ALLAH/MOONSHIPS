#!/usr/bin/env python3
"""Validate that a completed CI run is authorized to produce a release."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


def release_allowed(run: dict[str, Any], repository: str) -> bool:
    head_repository = run.get("head_repository")
    return bool(
        run.get("conclusion") == "success"
        and run.get("head_branch") == "main"
        and run.get("event") == "push"
        and isinstance(head_repository, dict)
        and head_repository.get("full_name") == repository
        and isinstance(run.get("head_sha"), str)
        and len(run["head_sha"]) == 40
    )


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: release_gate.py EVENT_JSON REPOSITORY", file=sys.stderr)
        return 2

    event = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    run = event.get("workflow_run", {})
    repository = sys.argv[2]
    if not release_allowed(run, repository):
        print("release denied: CI provenance is not eligible", file=sys.stderr)
        return 1

    print(run["head_sha"])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
