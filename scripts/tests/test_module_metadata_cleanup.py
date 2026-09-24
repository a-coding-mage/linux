# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the top-level clean rule on private module-metadata outputs."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

from test_migration_invariants import environment


ROOT = Path(__file__).resolve().parents[2]


class ModuleMetadataCleanupTests(unittest.TestCase):
    def check_clean(self, external, separate_output=False):
        with tempfile.TemporaryDirectory(prefix="module-metadata-clean-") as name:
            work = Path(name)
            kernel = work / "kernel"
            kernel.mkdir()
            (kernel / "include/config").mkdir(parents=True)
            # Cleaning must also remove the inactive language's old products.
            (kernel / "include/config/auto.conf").write_text(
                "# CONFIG_RUST_MODULE_METADATA is not set\n")
            command = shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", str(ROOT), "O=" + str(kernel), "ARCH=x86",
                "HOST_TOOLS_LANG=c", "HOSTRUSTC=false", "clean"]
            if external:
                source = work / "module"
                source.mkdir()
                (source / "Makefile").write_text("obj-m := unit.o\n")
                output = work / "module-output" if separate_output else source
                output.mkdir(exist_ok=True)
                command += ["M=" + str(source)]
                if separate_output:
                    command += ["MO=" + str(output)]
            else:
                source = output = kernel
            kept = (source / "unit.rs", source / "unit.c", source / "notes.txt")
            for path in kept:
                path.write_text("unrelated source must survive clean\n")
            before = {path: (path.read_bytes(), path.stat().st_mtime_ns) for path in kept}
            generated = []
            for directory in (output, output / "nested"):
                directory.mkdir(exist_ok=True)
                for suffix in ("mod.c", "mod.h", "mod.rs", "mod.rs.input", "mod.rs.tmp",
                               "mod.o", "ko"):
                    generated.append(directory / ("unit." + suffix))
                generated += [directory / ".unit.mod.rs.d.validate",
                              directory / ".unit.mod.rs.cmd", directory / ".unit.mod.o.cmd"]
            for path in generated:
                path.write_text("private generated clean fixture\n")
            result = subprocess.run(command, env=environment(), capture_output=True,
                                    text=True, timeout=90)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual([path for path in generated if path.exists()], [])
            self.assertEqual(before, {path: (path.read_bytes(), path.stat().st_mtime_ns)
                                      for path in kept})

    def test_normal_out_of_tree_clean_with_selection_disabled(self):
        self.check_clean(False)

    def test_external_same_directory_clean_with_selection_disabled(self):
        self.check_clean(True)

    def test_external_separate_output_clean_with_selection_disabled(self):
        self.check_clean(True, separate_output=True)


if __name__ == "__main__":
    unittest.main()
