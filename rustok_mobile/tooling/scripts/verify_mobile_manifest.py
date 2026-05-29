#!/usr/bin/env python3
"""Verify generated mobile manifest is up to date."""

from __future__ import annotations

import argparse
import difflib
import json
import pathlib
import re
import sys

if __package__:
    from .generate_mobile_manifest import (
        render,
        render_snapshot_json,
        scan_modules,
        to_snapshot,
    )
else:
    current_dir = pathlib.Path(__file__).resolve().parent
    if str(current_dir) not in sys.path:
        sys.path.insert(0, str(current_dir))
    from generate_mobile_manifest import (
        render,
        render_snapshot_json,
        scan_modules,
        to_snapshot,
    )


_SNAKE_CASE_RE = re.compile(r"^[a-z0-9_]+$")
_PERMISSION_RE = re.compile(r"^[a-z0-9_.:]+$")
_CAPABILITY_RE = re.compile(r"^[a-z0-9_.:-]+$")


def _is_snake_case(value: str) -> bool:
    return bool(value) and bool(_SNAKE_CASE_RE.fullmatch(value))


def _is_permission_key(value: str) -> bool:
    return bool(value) and bool(_PERMISSION_RE.fullmatch(value))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Path to repository root containing crates/*/rustok-module.toml",
    )
    parser.add_argument(
        "--manifest",
        default=(
            "rustok_mobile/apps/rustok_admin_mobile/lib/registry/mobile_manifest.g.dart"
        ),
        help="Path to generated Dart manifest",
    )
    parser.add_argument(
        "--snapshot",
        default=("rustok_mobile/tooling/snapshots/mobile_manifest.snapshot.json"),
        help="Path to generated registry snapshot JSON",
    )
    return parser.parse_args()


def _validate_snapshot_schema(entries: object) -> str | None:
    if not isinstance(entries, list):
        return "snapshot root must be an array"

    required_keys = {
        "module_slug",
        "surface_kind",
        "route_segment",
        "nav_icon",
        "permissions",
        "locale_namespace",
        "child_pages",
    }
    optional_keys = {"builder_surface"}
    seen_route_segments: set[str] = set()
    seen_module_slugs: set[str] = set()
    previous_route_segment: str | None = None

    for index, item in enumerate(entries):
        if not isinstance(item, dict):
            return f"snapshot entry #{index} is not an object"

        missing = required_keys.difference(item.keys())
        if missing:
            return f"snapshot entry #{index} missing keys: {sorted(missing)}"
        unknown = set(item.keys()).difference(required_keys | optional_keys)
        if unknown:
            return f"snapshot entry #{index} has unknown keys: {sorted(unknown)}"

        module_slug = item["module_slug"]
        route_segment = item["route_segment"]
        surface_kind = item["surface_kind"]
        nav_icon = item["nav_icon"]
        locale_namespace = item["locale_namespace"]
        permissions = item["permissions"]
        child_pages = item["child_pages"]
        builder_surface = item.get("builder_surface")

        if not isinstance(module_slug, str) or not module_slug.strip():
            return f"snapshot entry #{index} has invalid module_slug"
        if module_slug != module_slug.strip():
            return f"snapshot entry #{index} module_slug must be trimmed"
        if not _is_snake_case(module_slug):
            return f"snapshot entry #{index} module_slug must be snake_case"
        if module_slug in seen_module_slugs:
            return f"snapshot entry #{index} duplicates module_slug '{module_slug}'"
        seen_module_slugs.add(module_slug)
        if not isinstance(route_segment, str) or not route_segment.strip():
            return f"snapshot entry #{index} has invalid route_segment"
        if route_segment != route_segment.strip():
            return f"snapshot entry #{index} route_segment must be trimmed"
        if not _is_snake_case(route_segment):
            return f"snapshot entry #{index} route_segment must be snake_case"
        if route_segment in seen_route_segments:
            return f"snapshot entry #{index} duplicates route_segment '{route_segment}'"
        if (
            previous_route_segment is not None
            and route_segment < previous_route_segment
        ):
            return "snapshot entries must be sorted by route_segment"
        seen_route_segments.add(route_segment)
        previous_route_segment = route_segment

        if not isinstance(surface_kind, str) or surface_kind != surface_kind.strip():
            return f"snapshot entry #{index} has invalid surface_kind"
        if surface_kind != "admin_mobile":
            return (
                f"snapshot entry #{index} has unsupported surface_kind '{surface_kind}'"
            )
        if not isinstance(nav_icon, str) or not nav_icon.strip():
            return f"snapshot entry #{index} has invalid nav_icon"
        if nav_icon != nav_icon.strip():
            return f"snapshot entry #{index} nav_icon must be trimmed"
        if not _is_snake_case(nav_icon):
            return f"snapshot entry #{index} nav_icon must be snake_case"
        if not isinstance(locale_namespace, str) or not locale_namespace.strip():
            return f"snapshot entry #{index} has invalid locale_namespace"
        if locale_namespace != locale_namespace.strip():
            return f"snapshot entry #{index} locale_namespace must be trimmed"
        if not _is_snake_case(locale_namespace):
            return f"snapshot entry #{index} locale_namespace must be snake_case"
        if not isinstance(permissions, list):
            return f"snapshot entry #{index} permissions must be an array"
        seen_permissions: set[str] = set()
        previous_permission: str | None = None
        for permission_index, permission in enumerate(permissions):
            if not isinstance(permission, str) or not permission.strip():
                return (
                    f"snapshot entry #{index} permission #{permission_index} is invalid"
                )
            if permission != permission.strip():
                return f"snapshot entry #{index} permission #{permission_index} must be trimmed"
            if not _is_permission_key(permission):
                return f"snapshot entry #{index} permission #{permission_index} must use [a-z0-9_.:]"
            if permission in seen_permissions:
                return f"snapshot entry #{index} duplicates permission '{permission}'"
            if previous_permission is not None and permission < previous_permission:
                return f"snapshot entry #{index} permissions must be sorted ascending"
            seen_permissions.add(permission)
            previous_permission = permission
        if not isinstance(child_pages, list):
            return f"snapshot entry #{index} child_pages must be an array"

        seen_subpaths: set[str] = set()
        previous_subpath: str | None = None
        for child_index, child in enumerate(child_pages):
            if not isinstance(child, dict):
                return f"snapshot entry #{index} child #{child_index} is not an object"
            required_child_keys = {"subpath", "title", "nav_label"}
            missing_child = required_child_keys.difference(child.keys())
            if missing_child:
                return f"snapshot entry #{index} child #{child_index} missing keys: {sorted(missing_child)}"
            unknown_child = set(child.keys()).difference(required_child_keys)
            if unknown_child:
                return f"snapshot entry #{index} child #{child_index} has unknown keys: {sorted(unknown_child)}"
            for key in ("subpath", "title", "nav_label"):
                value = child.get(key)
                if not isinstance(value, str) or not value.strip():
                    return f"snapshot entry #{index} child #{child_index} has invalid {key}"
                if value != value.strip():
                    return f"snapshot entry #{index} child #{child_index} {key} must be trimmed"
            subpath = child["subpath"]
            if not _is_snake_case(subpath):
                return f"snapshot entry #{index} child #{child_index} subpath must be snake_case"
            if subpath in seen_subpaths:
                return f"snapshot entry #{index} child #{child_index} duplicates subpath '{subpath}'"
            if previous_subpath is not None and subpath < previous_subpath:
                return f"snapshot entry #{index} child_pages must be sorted by subpath"
            seen_subpaths.add(subpath)
            previous_subpath = subpath

        builder_surface_error = _validate_builder_surface(index, builder_surface)
        if builder_surface_error is not None:
            return builder_surface_error

    return None


def _validate_builder_surface(index: int, builder_surface: object) -> str | None:
    if builder_surface is None:
        return None
    if not isinstance(builder_surface, dict):
        return f"snapshot entry #{index} builder_surface must be null or an object"

    required_keys = {
        "provider_module",
        "contract",
        "contract_version",
        "builder_contract_version",
        "capabilities",
        "degraded_modes",
        "toggle_profiles",
    }
    missing = required_keys.difference(builder_surface.keys())
    if missing:
        return (
            f"snapshot entry #{index} builder_surface missing keys: {sorted(missing)}"
        )
    unknown = set(builder_surface.keys()).difference(required_keys)
    if unknown:
        return f"snapshot entry #{index} builder_surface has unknown keys: {sorted(unknown)}"

    for key in ("provider_module", "contract_version", "builder_contract_version"):
        value = builder_surface.get(key)
        if not isinstance(value, str) or not value.strip():
            return f"snapshot entry #{index} builder_surface has invalid {key}"
        if value != value.strip():
            return f"snapshot entry #{index} builder_surface {key} must be trimmed"

    contract = builder_surface.get("contract")
    if not isinstance(contract, str) or contract != contract.strip():
        return (
            f"snapshot entry #{index} builder_surface contract must be a trimmed string"
        )

    capabilities = builder_surface.get("capabilities")
    if not isinstance(capabilities, list):
        return f"snapshot entry #{index} builder_surface capabilities must be an array"
    previous_capability: str | None = None
    seen_capabilities: set[str] = set()
    for capability_index, capability in enumerate(capabilities):
        if not isinstance(capability, str) or not capability.strip():
            return (
                f"snapshot entry #{index} builder_surface capability "
                f"#{capability_index} is invalid"
            )
        if capability != capability.strip() or not _CAPABILITY_RE.fullmatch(capability):
            return (
                f"snapshot entry #{index} builder_surface capability "
                f"#{capability_index} must use [a-z0-9_.:-]"
            )
        if capability in seen_capabilities:
            return (
                f"snapshot entry #{index} builder_surface duplicates "
                f"capability '{capability}'"
            )
        if previous_capability is not None and capability < previous_capability:
            return f"snapshot entry #{index} builder_surface capabilities must be sorted ascending"
        seen_capabilities.add(capability)
        previous_capability = capability

    degraded_modes = builder_surface.get("degraded_modes")
    if not isinstance(degraded_modes, dict):
        return (
            f"snapshot entry #{index} builder_surface degraded_modes must be an object"
        )
    for key, value in degraded_modes.items():
        if not isinstance(key, str) or not _is_snake_case(key):
            return f"snapshot entry #{index} builder_surface degraded_modes key must be snake_case"
        if not isinstance(value, str) or not value.strip() or value != value.strip():
            return f"snapshot entry #{index} builder_surface degraded_modes value is invalid"

    toggle_profiles = builder_surface.get("toggle_profiles")
    if not isinstance(toggle_profiles, dict):
        return (
            f"snapshot entry #{index} builder_surface toggle_profiles must be an object"
        )
    for key, values in toggle_profiles.items():
        if not isinstance(key, str) or not _is_snake_case(key):
            return f"snapshot entry #{index} builder_surface toggle_profiles key must be snake_case"
        if not isinstance(values, list) or not values:
            return f"snapshot entry #{index} builder_surface toggle profile '{key}' must be a non-empty array"
        previous_value: str | None = None
        seen_values: set[str] = set()
        for value_index, value in enumerate(values):
            if (
                not isinstance(value, str)
                or not value.strip()
                or value != value.strip()
            ):
                return (
                    f"snapshot entry #{index} builder_surface toggle profile "
                    f"'{key}' value #{value_index} is invalid"
                )
            if value in seen_values:
                return (
                    f"snapshot entry #{index} builder_surface toggle profile "
                    f"'{key}' duplicates value '{value}'"
                )
            if previous_value is not None and value < previous_value:
                return (
                    f"snapshot entry #{index} builder_surface toggle profile "
                    f"'{key}' values must be sorted ascending"
                )
            seen_values.add(value)
            previous_value = value

    return None


def _print_regeneration_command(repo_root: pathlib.Path) -> None:
    print("Run:")
    print(
        "  python3 rustok_mobile/tooling/scripts/generate_mobile_manifest.py "
        f"--repo-root {repo_root}"
    )


def _print_stale_diff(
    *,
    label: str,
    path: pathlib.Path,
    current: str,
    expected: str,
    repo_root: pathlib.Path,
) -> None:
    print(f"ERROR: {label} is stale: {path}")
    print("Diff (current -> expected):")
    for line in difflib.unified_diff(
        current.splitlines(),
        expected.splitlines(),
        fromfile=f"{path} (current)",
        tofile=f"{path} (expected)",
        lineterm="",
    ):
        print(line)
    _print_regeneration_command(repo_root)


def main() -> int:
    args = parse_args()
    repo_root = pathlib.Path(args.repo_root).resolve()
    manifest_path = pathlib.Path(args.manifest).resolve()
    snapshot_path = pathlib.Path(args.snapshot).resolve()

    modules = scan_modules(repo_root)
    expected = render(modules)
    expected_snapshot_entries = to_snapshot(modules)
    schema_error = _validate_snapshot_schema(expected_snapshot_entries)
    if schema_error is not None:
        print(f"ERROR: generated snapshot schema is invalid: {schema_error}")
        return 1
    expected_snapshot = render_snapshot_json(modules)
    if not manifest_path.exists():
        print(f"Manifest file is missing: {manifest_path}")
        return 1

    current = manifest_path.read_text(encoding="utf-8")
    if current != expected:
        _print_stale_diff(
            label="mobile manifest",
            path=manifest_path,
            current=current,
            expected=expected,
            repo_root=repo_root,
        )
        return 1

    if not snapshot_path.exists():
        print(f"Snapshot file is missing: {snapshot_path}")
        return 1

    snapshot_current = snapshot_path.read_text(encoding="utf-8")
    if snapshot_current != expected_snapshot:
        _print_stale_diff(
            label="mobile manifest snapshot",
            path=snapshot_path,
            current=snapshot_current,
            expected=expected_snapshot,
            repo_root=repo_root,
        )
        return 1

    try:
        parsed = json.loads(snapshot_current)
    except json.JSONDecodeError as exc:
        print(f"ERROR: snapshot is not valid JSON: {exc}")
        return 1

    schema_error = _validate_snapshot_schema(parsed)
    if schema_error is not None:
        print(f"ERROR: snapshot schema invalid: {schema_error}")
        return 1

    print(f"OK: mobile manifest and snapshot are up to date ({manifest_path})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
