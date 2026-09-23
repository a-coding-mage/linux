#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Byte-exact IPE boot-policy generation, error handling, and compiled output."""

import contextlib
import os
from pathlib import Path
import random
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "scripts/ipe/polgen/polgen"


class PolgenTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="polgen-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        work = Path(cls.temporary.name)
        cls.c, cls.rust = work / "polgen-c", work / "polgen-rust"
        for command in (
                shlex.split(os.environ.get("HOSTCC", "cc")) + [
                    "-O2", "-Wall", "-Wextra", "-Werror", str(SOURCE.with_suffix(".c")), "-o", str(cls.c)],
                shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
                    "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                    "-Wrust-2018-idioms", str(SOURCE.with_suffix(".rs")), "-o", str(cls.rust)]):
            result = subprocess.run(command, capture_output=True, text=True)
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)

    @contextlib.contextmanager
    def case(self, contents=None, name="policy"):
        with tempfile.TemporaryDirectory(prefix="polgen-case-") as temporary:
            paths = [Path(temporary) / side for side in ("c", "rust")]
            for path in paths:
                path.mkdir()
                if contents is not None:
                    (path / name).write_bytes(contents)
            yield paths

    def compare(self, paths, arguments=("output.c", "policy")):
        results = []
        for tool, work in zip((self.c, self.rust), paths):
            result = subprocess.run([str(tool), *arguments], cwd=work, capture_output=True, timeout=30)
            streams = [stream.replace(os.fsencode(tool), b"<tool>") for stream in (result.stdout, result.stderr)]
            files = {}
            for path in sorted(work.iterdir()):
                if path.is_file():
                    files[path.name] = path.read_bytes()
                elif path.is_dir():
                    files[path.name] = None
            results.append((result.returncode, *streams, files))
        self.assertEqual(results[0], results[1])
        return results[1]

    def test_empty_and_absent_policy(self):
        with self.case() as paths:
            result = self.compare(paths, ("output.c",))
            self.assertEqual(result[0], 0)
            self.assertTrue(result[3]["output.c"].endswith(b"\tNULL;\n"))
        with self.case(b"") as paths:
            result = self.compare(paths)
            self.assertTrue(result[3]["output.c"].endswith(b"\tNULL;\n"))

    def test_all_bytes_escaping_and_line_boundaries(self):
        cases = (bytes(range(256)), b'"\'\\??/\t\n', b"\n", b"\n\n", b"last line\n",
                 b"last line", b"a\0b\0", b"\r\n\v\f", "policy 日本語 café\n".encode())
        for content in cases:
            with self.subTest(content=content), self.case(content) as paths:
                self.assertEqual(self.compare(paths)[0], 0)
        with self.case(b'"\'\\?\t\n') as paths:
            generated = self.compare(paths)[3]["output.c"]
            self.assertTrue(generated.endswith(b'\t"\\"\'\\\\\\?\\t\\n"\n\t"";\n'))

    def test_random_binary_and_large_policy(self):
        rng = random.Random(0x1FE)
        for size in (1, 2, 17, 255, 256, 4095, 4096, 65537, 2_000_000):
            with self.subTest(size=size), self.case(rng.randbytes(size)) as paths:
                self.assertEqual(self.compare(paths)[0], 0)

    def test_extra_arguments_dash_paths_and_non_utf8_names(self):
        for name in ("-policy", "with spaces", "日本語", os.fsdecode(b"policy_\xff")):
            output = os.fsdecode(b"output_\xfe.c")
            with self.subTest(name=name), self.case(b"test\n", name) as paths:
                self.assertEqual(self.compare(paths, (output, name, "ignored", "--help"))[0], 0)
        with self.case() as paths:
            self.assertEqual(self.compare(paths, ("--help",))[0], 0)

    def test_same_file_and_symlink_output(self):
        with self.case(b"same file\n") as paths:
            self.compare(paths, ("policy", "policy"))
        with self.case(b"policy\n") as paths:
            for path in paths:
                (path / "target").write_bytes(b"previous contents")
                (path / "output.c").symlink_to("target")
            self.compare(paths)
            for path in paths:
                self.assertTrue((path / "output.c").is_symlink())
                self.assertTrue((path / "target").read_bytes().startswith(b"/*"))

    def test_usage_and_read_open_failures_preserve_existing_output(self):
        with self.case() as paths:
            result = self.compare(paths, ())
            self.assertEqual(result[:3], (22, b"Usage: <tool> OutputFile (PolicyFile)\n", b""))
        with self.case() as paths:
            for path in paths:
                (path / "output.c").write_bytes(b"keep this")
            result = self.compare(paths)
            self.assertEqual(result[0], 2)
            self.assertEqual(result[3]["output.c"], b"keep this")
        for arguments in (("missing/output.c", "policy"), ("", "policy"), ("directory", "policy")):
            with self.subTest(arguments=arguments), self.case(b"test") as paths:
                for path in paths:
                    (path / "directory").mkdir()
                self.assertNotEqual(self.compare(paths, arguments)[0], 0)

    def test_seekable_pseudo_files_and_directory_input(self):
        for source in ("/dev/null", "/proc/version", "."):
            if source != "." and not Path(source).exists():
                continue
            with self.subTest(source=source), self.case() as paths:
                self.compare(paths, ("output.c", source))

    def test_generated_c_compiles_and_preserves_policy_bytes(self):
        content = b'policy_name=BootPolicy policy_version=1.0.0\nDEFAULT action=DENY\n# "quotes" \t ??/ \\\n'
        with self.case(content) as paths:
            self.compare(paths)
            work = paths[1]
            (work / "driver.c").write_text(
                '#include <stdio.h>\nextern const char *const ipe_boot_policy;\n'
                f'int main(void) {{ return fwrite(ipe_boot_policy, 1, {len(content)}, stdout) != {len(content)}; }}\n')
            result = subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
                "-std=c11", "-Wall", "-Wextra", "-Werror", "-I", str(ROOT / "include"),
                str(work / "output.c"), str(work / "driver.c"), "-o", str(work / "driver")],
                capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            result = subprocess.run([str(work / "driver")], capture_output=True)
            self.assertEqual((result.returncode, result.stdout, result.stderr), (0, content, b""))

    def test_output_write_failure_is_reported(self):
        if not Path("/dev/full").exists():
            self.skipTest("requires /dev/full")
        with self.case(b"policy") as paths:
            result = subprocess.run([str(self.rust), "/dev/full", "policy"], cwd=paths[1], capture_output=True)
            self.assertEqual(result.returncode, 28)
            self.assertIn(b"No space left on device", result.stderr)


if __name__ == "__main__":
    unittest.main()
