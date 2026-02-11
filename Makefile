.PHONY: docs-sync-loco docs-check-loco

# Refresh metadata for the local Loco upstream docs snapshot.
docs-sync-loco:
	python3 scripts/loco_upstream_snapshot.py sync

# Validate that the upstream snapshot metadata exists and is fresh enough.
docs-check-loco:
	python3 scripts/loco_upstream_snapshot.py check
