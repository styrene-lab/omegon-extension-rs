#!/usr/bin/env python3
"""Verify all Omegon extension SDK ports carry the canonical Rust contract."""
from __future__ import annotations

import json
import pathlib
import sys

RUST_REPO = pathlib.Path(__file__).resolve().parents[1]
ROOT = RUST_REPO.parent
CONTRACTS = [
    RUST_REPO / "schema" / "sdk-contract.json",
    ROOT / "omegon-extension-py" / "src" / "omegon_extension" / "schema" / "sdk-contract.json",
    ROOT / "omegon-extension-ts" / "src" / "schema" / "sdk-contract.json",
    ROOT / "omegon-extension-ts" / "dist" / "schema" / "sdk-contract.json",
]


def normalized(path: pathlib.Path) -> str:
    return json.dumps(json.loads(path.read_text()), sort_keys=True, separators=(",", ":"))


def main() -> int:
    missing = [path for path in CONTRACTS if not path.is_file()]
    if missing:
        for path in missing:
            print(f"missing contract: {path}", file=sys.stderr)
        return 1

    canonical_path = CONTRACTS[0]
    canonical = normalized(canonical_path)
    failed = False
    for path in CONTRACTS[1:]:
        if normalized(path) != canonical:
            print(f"contract drift: {path} differs from {canonical_path}", file=sys.stderr)
            failed = True
        else:
            print(f"ok: {path.relative_to(ROOT)}")
    if failed:
        return 1
    print(f"all SDK contracts match {canonical_path.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
