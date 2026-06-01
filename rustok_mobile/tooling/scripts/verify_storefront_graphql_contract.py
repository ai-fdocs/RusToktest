#!/usr/bin/env python3
"""Verify Flutter storefront GraphQL documents against server-owned surfaces.

The check is intentionally source-backed: it does not require a Flutter SDK or a
running RusTok server, but it validates that mobile operation documents keep
matching the existing storefront/search APIs and commerce runtime parity flow.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


@dataclass(frozen=True)
class GraphQlContract:
    const_name: str
    operation_name: str
    operation_kind: str
    root_field: str
    server_markers: tuple[tuple[str, str], ...]
    runtime_builder: str | None = None
    runtime_error_marker: str | None = None


CONTRACTS: tuple[GraphQlContract, ...] = (
    GraphQlContract(
        const_name="storefrontMobileCatalogQuery",
        operation_name="StorefrontMobileCatalog",
        operation_kind="query",
        root_field="storefrontSearch",
        server_markers=(
            (
                "crates/rustok-search/storefront/src/api.rs",
                "query StorefrontSearch($input: SearchPreviewInput!)",
            ),
            ("crates/rustok-search/storefront/src/api.rs", "storefrontSearch(input: $input)"),
            ("crates/rustok-search/storefront/src/api.rs", "struct SearchPreviewInput"),
        ),
    ),
    GraphQlContract(
        const_name="storefrontMobileCartQuery",
        operation_name="StorefrontMobileCart",
        operation_kind="query",
        root_field="storefrontCart",
        server_markers=(
            ("crates/rustok-commerce/src/graphql/query.rs", "async fn storefront_cart"),
            ("crates/rustok-commerce/src/graphql/query.rs", "id: Uuid"),
        ),
        runtime_builder="storefront_cart_query",
        runtime_error_marker="unexpected cart query GraphQL errors",
    ),
    GraphQlContract(
        const_name="storefrontMobileCreateCartMutation",
        operation_name="StorefrontMobileCreateCart",
        operation_kind="mutation",
        root_field="createStorefrontCart",
        server_markers=(
            (
                "crates/rustok-commerce/src/graphql/mutation.rs",
                "async fn create_storefront_cart",
            ),
            ("crates/rustok-commerce/src/graphql/types.rs", "pub struct CreateStorefrontCartInput"),
        ),
        runtime_builder="storefront_cart_flow_mutation",
        runtime_error_marker="unexpected create cart GraphQL errors",
    ),
    GraphQlContract(
        const_name="storefrontMobileAddCartLineMutation",
        operation_name="StorefrontMobileAddCartLine",
        operation_kind="mutation",
        root_field="addStorefrontCartLineItem",
        server_markers=(
            (
                "crates/rustok-commerce/src/graphql/mutation.rs",
                "async fn add_storefront_cart_line_item",
            ),
            (
                "crates/rustok-commerce/src/graphql/types.rs",
                "pub struct AddStorefrontCartLineItemInput",
            ),
        ),
        runtime_builder="storefront_cart_add_line_item_mutation",
        runtime_error_marker="unexpected add line item GraphQL errors",
    ),
    GraphQlContract(
        const_name="storefrontMobileUpdateCartLineMutation",
        operation_name="StorefrontMobileUpdateCartLine",
        operation_kind="mutation",
        root_field="updateStorefrontCartLineItem",
        server_markers=(
            (
                "crates/rustok-commerce/src/graphql/mutation.rs",
                "async fn update_storefront_cart_line_item",
            ),
            (
                "crates/rustok-commerce/src/graphql/types.rs",
                "pub struct UpdateStorefrontCartLineItemInput",
            ),
        ),
        runtime_builder="storefront_cart_update_line_item_mutation",
        runtime_error_marker="unexpected update line item GraphQL errors",
    ),
    GraphQlContract(
        const_name="storefrontMobileRemoveCartLineMutation",
        operation_name="StorefrontMobileRemoveCartLine",
        operation_kind="mutation",
        root_field="removeStorefrontCartLineItem",
        server_markers=(
            (
                "crates/rustok-commerce/src/graphql/mutation.rs",
                "async fn remove_storefront_cart_line_item",
            ),
            ("crates/rustok-commerce/src/graphql/mutation.rs", "line_id: Uuid"),
        ),
        runtime_builder="storefront_cart_remove_line_item_mutation",
        runtime_error_marker="unexpected remove line item GraphQL errors",
    ),
)

MOBILE_REPOSITORY_PATH = Path(
    "rustok_mobile/apps/rustok_frontend_mobile/lib/data/storefront_catalog_repository.dart"
)
MOBILE_CONTEXT_PATH = Path(
    "rustok_mobile/apps/rustok_frontend_mobile/lib/app_shell/storefront_context.dart"
)
COMMERCE_RUNTIME_TEST_PATH = Path("crates/rustok-commerce/tests/graphql_runtime_parity_test.rs")
FORBIDDEN_MOBILE_MARKERS = ("/api/flutter", "/api/mobile", "tenantId:", "$tenantId")


class ContractError(RuntimeError):
    pass


def read(repo_root: Path, path: Path | str) -> str:
    return (repo_root / path).read_text(encoding="utf-8")


def extract_dart_raw_string_const(source: str, const_name: str) -> str:
    pattern = re.compile(
        rf"const\s+{re.escape(const_name)}\s*=\s*r'''(?P<body>.*?)''';",
        re.DOTALL,
    )
    match = pattern.search(source)
    if match is None:
        raise ContractError(f"missing Dart GraphQL const `{const_name}`")
    return match.group("body")


def assert_contains(source: str, marker: str, context: str) -> None:
    if marker not in source:
        raise ContractError(f"missing marker `{marker}` in {context}")


def assert_runtime_builder_is_executed(runtime_source: str, builder: str) -> None:
    assert_contains(runtime_source, f"fn {builder}", str(COMMERCE_RUNTIME_TEST_PATH))
    pattern = re.compile(rf"schema\s*\.execute\(Request::new\(\s*{re.escape(builder)}")
    if pattern.search(runtime_source) is None:
        raise ContractError(
            f"runtime parity test defines `{builder}` but does not execute it through schema.execute"
        )


def verify_contract(repo_root: Path, contract: GraphQlContract, mobile_source: str) -> dict[str, str]:
    document = extract_dart_raw_string_const(mobile_source, contract.const_name)
    assert_contains(
        document,
        f"{contract.operation_kind} {contract.operation_name}",
        contract.const_name,
    )
    assert_contains(document, f"{contract.root_field}(", contract.const_name)

    for forbidden in FORBIDDEN_MOBILE_MARKERS:
        if forbidden in document:
            raise ContractError(
                f"mobile GraphQL document `{contract.const_name}` must not contain `{forbidden}`"
            )

    checked_paths: set[str] = set()
    for path, marker in contract.server_markers:
        server_source = read(repo_root, path)
        assert_contains(server_source, marker, path)
        checked_paths.add(path)

    if contract.runtime_builder is not None:
        runtime_source = read(repo_root, COMMERCE_RUNTIME_TEST_PATH)
        assert_runtime_builder_is_executed(runtime_source, contract.runtime_builder)
        checked_paths.add(str(COMMERCE_RUNTIME_TEST_PATH))
    if contract.runtime_error_marker is not None:
        runtime_source = read(repo_root, COMMERCE_RUNTIME_TEST_PATH)
        assert_contains(runtime_source, contract.runtime_error_marker, str(COMMERCE_RUNTIME_TEST_PATH))
        checked_paths.add(str(COMMERCE_RUNTIME_TEST_PATH))

    return {
        "const": contract.const_name,
        "operation": contract.operation_name,
        "kind": contract.operation_kind,
        "root_field": contract.root_field,
        "server_evidence": ",".join(sorted(checked_paths)),
    }


def verify(repo_root: Path) -> list[dict[str, str]]:
    mobile_source = read(repo_root, MOBILE_REPOSITORY_PATH)
    mobile_context = read(repo_root, MOBILE_CONTEXT_PATH)

    for forbidden in ("/api/flutter", "/api/mobile"):
        if forbidden in mobile_source or forbidden in mobile_context:
            raise ContractError(f"mobile storefront transport must not define `{forbidden}`")

    assert_contains(mobile_source, "GraphQlStorefrontCatalogRepository", str(MOBILE_REPOSITORY_PATH))
    assert_contains(mobile_context, "GraphQlClientFactory().create", str(MOBILE_CONTEXT_PATH))

    return [verify_contract(repo_root, contract, mobile_source) for contract in CONTRACTS]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path.cwd(),
        help="Repository root. Defaults to the current working directory.",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Print machine-readable evidence instead of a short OK line.",
    )
    return parser


def main(argv: Iterable[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    repo_root = args.repo_root.resolve()
    try:
        evidence = verify(repo_root)
    except ContractError as error:
        print(f"ERROR: {error}")
        return 1

    if args.json:
        print(json.dumps({"storefront_graphql_contracts": evidence}, indent=2, sort_keys=True))
    else:
        print(f"OK: verified {len(evidence)} storefront mobile GraphQL contracts")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
