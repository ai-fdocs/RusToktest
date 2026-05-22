#!/usr/bin/env python3
from __future__ import annotations

import pathlib
import subprocess
import sys
import tempfile
import textwrap
import unittest


SCRIPT = pathlib.Path(__file__).with_name("check-dependabot-directories.py")


class DependabotDirectoryCheckTests(unittest.TestCase):
    def run_script(self, root: pathlib.Path, config: pathlib.Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(SCRIPT), "--root", str(root), "--config", str(config)],
            text=True,
            capture_output=True,
            check=False,
        )

    def test_returns_zero_when_all_directories_exist(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / "apps" / "server").mkdir(parents=True)
            (root / "crates").mkdir()

            config = root / ".github" / "dependabot.yml"
            config.parent.mkdir(parents=True)
            config.write_text(
                textwrap.dedent(
                    """
                    version: 2
                    updates:
                      - package-ecosystem: "cargo"
                        directory: "/"
                      - package-ecosystem: "cargo"
                        directory: "/apps/server"
                      - package-ecosystem: "cargo"
                        directory: "/crates"
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )

            result = self.run_script(root, config)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn("All Dependabot update directories exist.", result.stdout)

    def test_fails_when_directory_is_missing(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / "apps" / "server").mkdir(parents=True)

            config = root / ".github" / "dependabot.yml"
            config.parent.mkdir(parents=True)
            config.write_text(
                textwrap.dedent(
                    """
                    version: 2
                    updates:
                      - package-ecosystem: "cargo"
                        directory: "/apps/server"
                      - package-ecosystem: "cargo"
                        directory: "/apps/mcp"
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )

            result = self.run_script(root, config)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("Dependabot directories do not exist:", result.stderr)
            self.assertIn("/apps/mcp", result.stderr)


if __name__ == "__main__":
    unittest.main()
