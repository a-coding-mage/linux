#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Original-C differential and real Kbuild coverage for RAID-6 mktables."""

import os
from pathlib import Path
import re
import shlex
import signal
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
# A private overlay can use an untouched reference tree for C and Kbuild.
REFERENCE = Path(os.environ.get("RAID6_REFERENCE_ROOT", ROOT))
COMPONENT = Path("lib/raid/raid6")


class Raid6TableGeneratorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="raid6-tables-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.tools = (cls.work / "mktables-c", cls.work / "mktables-rust")
        commands = (
            cls.cc + ["-O2", "-Wall", str(REFERENCE / COMPONENT / "mktables.c")],
            cls.rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
                         "-Wunreachable-pub", "-Wrust_2018_idioms",
                         "-Dunsafe_op_in_unsafe_fn", str(ROOT / COMPONENT / "mktables.rs")],
        )
        for command, output in zip(commands, cls.tools):
            result = subprocess.run(command + ["-o", str(output)], capture_output=True, timeout=60)
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)
        cls.oracle = subprocess.run([cls.tools[0]], capture_output=True, timeout=15)
        if cls.oracle.returncode or cls.oracle.stderr:
            raise RuntimeError("C oracle failed", cls.oracle)

    def test_byte_exact_output_and_ignored_arguments(self):
        for args in ([], ["--help"], ["--bad-option", "", "output.c"], [b"\xff"]):
            with self.subTest(args=args):
                for tool in self.tools:
                    result = subprocess.run([tool, *args], input=b"ignored stdin\n",
                                            capture_output=True, timeout=15)
                    self.assertEqual((result.returncode, result.stdout, result.stderr),
                                     (0, self.oracle.stdout, b""))

    def test_all_table_algebra(self):
        output = subprocess.check_output([self.tools[1]], timeout=15).decode("ascii")
        arrays = re.findall(r"(raid6_\w+)\[[^=]+?=\n\{(.*?)\n\};", output, re.S)
        tables = {name: [int(value, 16) for value in re.findall(r"0x([0-9a-f]{2})", body)]
                  for name, body in arrays}
        self.assertEqual(set(tables), {"raid6_" + name for name in
                                      ("gfmul", "vgfmul", "gfexp", "gflog", "gfinv", "gfexi")})

        def multiply(a, b):
            # Polynomial multiplication followed by reduction modulo x^8+x^4+x^3+x^2+1.
            product = 0
            for bit in range(8):
                if b & (1 << bit):
                    product ^= a << bit
            for bit in range(14, 7, -1):
                if product & (1 << bit):
                    product ^= 0x11d << (bit - 8)
            return product

        mul, vector, exp, log, inv, exi = (tables["raid6_" + name] for name in
                                         ("gfmul", "vgfmul", "gfexp", "gflog", "gfinv", "gfexi"))
        self.assertEqual([len(x) for x in (mul, vector, exp, log, inv, exi)],
                         [65536, 8192, 256, 256, 256, 256])
        for a in range(256):
            for b in range(256):
                self.assertEqual(mul[256 * a + b], multiply(a, b))
            for b in range(16):
                self.assertEqual(vector[32 * a + b], mul[256 * a + b])
                self.assertEqual(vector[32 * a + 16 + b], mul[256 * a + (b << 4)])
            self.assertEqual(multiply(a, inv[a]), int(a != 0))
            self.assertEqual(exi[a], inv[exp[a] ^ 1])
            self.assertEqual(exp[log[a]], a)
        self.assertEqual((exp[0], exp[255], log[0], inv[0], exi[0], exi[255]),
                         (1, 0, 255, 0, 0, 1))
        for index in range(254):
            self.assertEqual(exp[index + 1], multiply(exp[index], 2))
        self.assertEqual(multiply(exp[254], 2), 1)

    @unittest.skipUnless(Path("/dev/full").exists(), "requires /dev/full")
    def test_ignored_stdout_errors(self):
        for tool in self.tools:
            with open("/dev/full", "wb") as output:
                result = subprocess.run([tool], stdout=output, stderr=subprocess.PIPE, timeout=15)
            self.assertEqual((result.returncode, result.stderr), (0, b""))

    def test_broken_pipe(self):
        for tool in self.tools:
            reader, writer = os.pipe()
            os.close(reader)
            try:
                result = subprocess.run([tool], stdout=writer, stderr=subprocess.PIPE, timeout=15)
            finally:
                os.close(writer)
            self.assertEqual((result.returncode, result.stderr), (-signal.SIGPIPE, b""))

    def test_inherited_ignored_sigpipe(self):
        for tool in self.tools:
            reader, writer = os.pipe()
            os.close(reader)
            try:
                # Python ignores SIGPIPE; preserve that disposition in the child.
                result = subprocess.run([tool], stdout=writer, stderr=subprocess.PIPE,
                                        restore_signals=False, timeout=15)
            finally:
                os.close(writer)
            self.assertEqual((result.returncode, result.stderr), (0, b""))

    def test_kbuild_c_rust_c_noop_and_source_dependencies(self):
        self.check_kbuild_cycle(("c", "rust", "c"))

    def test_kbuild_rust_c_rust_noop_and_source_dependencies(self):
        self.check_kbuild_cycle(("rust", "c", "rust"))

    def test_kbuild_ignores_inherited_parent_build_environment(self):
        parent = os.environ.copy()
        parent.update({
            "KBUILD_CFLAGS": "-falign-functions= -Wframe-larger-than=",
            "KBUILD_HOSTCFLAGS": "--invalid-parent-host-flag",
            "KBUILD_HOSTRUSTFLAGS": "--invalid-parent-rust-flag",
            "MAKEFLAGS": "--directory=/nonexistent-parent-output",
            "MAKELEVEL": "7", "sub_make_done": "1",
            "srctree": "/nonexistent-parent-source", "objtree": "/nonexistent-parent-output",
            "ARCH": "arm64", "SRCARCH": "arm64", "LLVM": "1",
        })
        self.check_kbuild_cycle(("c", "rust", "c"), parent)

    def check_kbuild_cycle(self, languages, inherited_environment=None):
        # The test is also discovered inside make rust-host-tests. Do not
        # inherit the caller's output paths, recursive flags, or architecture.
        env = (os.environ if inherited_environment is None else inherited_environment).copy()
        for key in list(env):
            if key.startswith("KBUILD_") or key in (
                    "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                    "sub_make_done", "srctree", "srcroot", "objtree", "VPATH", "ARCH", "SRCARCH",
                    "SUBARCH", "CROSS_COMPILE", "LLVM", "LLVM_IAS"):
                env.pop(key)
        env["LC_ALL"] = "C"
        temporary = tempfile.TemporaryDirectory(prefix="raid6-kbuild-", dir=self.work)
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        source = work / "source"
        component = source / COMPONENT
        component.mkdir(parents=True)
        for name, root in (("Makefile", ROOT), ("mktables.rs", ROOT), ("mktables.c", REFERENCE)):
            (component / name).write_bytes((root / COMPONENT / name).read_bytes())
        build = work / "build"
        (build / COMPONENT).mkdir(parents=True)
        (build / "scripts/basic").mkdir(parents=True)
        result = subprocess.run(self.cc + ["-O2", "-I", str(REFERENCE / "scripts/include"),
                                str(REFERENCE / "scripts/basic/fixdep.c"),
                                "-o", str(build / "scripts/basic/fixdep")], capture_output=True, timeout=30)
        self.assertEqual(result.returncode, 0, result.stderr)
        base = shlex.split(os.environ.get("MAKE", "make")) + [
            "-j8", "-C", str(build), "-f", str(REFERENCE / "scripts/Makefile.build"),
            "srctree=" + str(REFERENCE), "srcroot=" + str(source), "objtree=.",
            "VPATH=" + str(source), "building_out_of_srctree=1", "CONFIG_SHELL=/bin/sh",
            "obj=" + str(COMPONENT), "HOSTCC=" + shlex.join(self.cc),
            "KBUILD_HOSTCFLAGS=-O2", "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings",
            str(COMPONENT / "tables.c")]
        binary = build / COMPONENT / "mktables"
        generated = build / COMPONENT / "tables.c"

        def run(language):
            command = base + ["HOST_TOOLS_LANG=" + language,
                              "HOSTRUSTC=" + (shlex.join(self.rustc) if language == "rust" else "false")]
            result = subprocess.run(command, env=env, capture_output=True, text=True, timeout=60)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(generated.read_bytes(), self.oracle.stdout)
            return result.stdout

        def stamps():
            return (binary.stat().st_mtime_ns, generated.stat().st_mtime_ns)

        previous = None
        for language in languages:
            with self.subTest(language=language):
                run(language)
                now = stamps()
                if previous is not None:
                    self.assertTrue(all(a > b for a, b in zip(now, previous)))
                record = (binary.parent / ".mktables.cmd").read_text()
                self.assertIn("mktables." + ("rs" if language == "rust" else "c"), record)
                self.assertEqual("--emit=link=" in record, language == "rust")
                table_record = (binary.parent / ".tables.c.cmd").read_text()
                self.assertIn("mktables >", table_record)
                run(language)
                self.assertEqual(stamps(), now)

                inactive = component / ("mktables.c" if language == "rust" else "mktables.rs")
                inactive.touch()
                run(language)
                self.assertEqual(stamps(), now, "inactive source must not rebuild generator")

                active = component / ("mktables.rs" if language == "rust" else "mktables.c")
                active.touch()
                run(language)
                changed = stamps()
                self.assertTrue(all(a > b for a, b in zip(changed, now)),
                                "active source must rebuild generator and dependent tables")
                run(language)
                self.assertEqual(stamps(), changed)
                previous = changed


if __name__ == "__main__":
    unittest.main()
