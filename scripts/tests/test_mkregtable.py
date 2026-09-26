# SPDX-License-Identifier: MIT
"""Original-C differentials and real Kbuild coverage for Radeon mkregtable."""

import os
from pathlib import Path
import random
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
COMPONENT = Path("drivers/gpu/drm/radeon")


class MkregtableTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="mkregtable-tests-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.env = {**os.environ, "LC_ALL": "C"}
        cls.tools = []
        commands = [("c", cls.cc + ["-O2", "-Wall", str(ROOT / COMPONENT / "mkregtable.c")]),
                    ("rust", cls.rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
                                         "-Wunreachable-pub", "-Wrust_2018_idioms",
                                         "-Dunsafe_op_in_unsafe_fn", str(ROOT / COMPONENT / "mkregtable.rs")])]
        if shutil.which("clang"):
            commands.append(("clang", ["clang", "-O2", "-Wall", str(ROOT / COMPONENT / "mkregtable.c")]))
        if "MKREGTABLE_I686_CC" in os.environ:
            compiler = Path(os.environ["MKREGTABLE_I686_CC"])
            if not compiler.is_absolute() or not os.access(compiler, os.X_OK):
                raise ValueError("MKREGTABLE_I686_CC must be an executable absolute path")
            commands += [("c-i686", [str(compiler), "-O2", "-Wall", str(ROOT / COMPONENT / "mkregtable.c")]),
                         ("rust-i686", cls.rustc + ["--edition=2021", "-O", "-Dwarnings",
                                                   "-Wmissing-docs", "-Wunreachable-pub", "-Wrust_2018_idioms",
                                                   "-Dunsafe_op_in_unsafe_fn",
                                                   "--target=i686-unknown-linux-gnu", "-Clinker=" + str(compiler),
                                                   str(ROOT / COMPONENT / "mkregtable.rs")])]
        for name, command in commands:
            output = cls.work / name
            result = subprocess.run(command + ["-o", str(output)], capture_output=True,
                                    env=cls.env, cwd=cls.work, timeout=60)
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)
            if name.endswith("-i686") and output.read_bytes()[:5] != b"\x7fELF\x01":
                raise RuntimeError("i686 compiler did not produce an actual ELF32 executable: " + name)
            cls.tools.append(output)

    def invoke(self, tool, args, **kwargs):
        return subprocess.run([b"mkregtable", *args], executable=tool,
                              stdout=kwargs.pop("stdout", subprocess.PIPE), stderr=subprocess.PIPE,
                              env=self.env, cwd=self.work, timeout=15, **kwargs)

    def compare(self, args, **kwargs):
        outcomes = []
        for tool in self.tools:
            result = self.invoke(tool, args, **kwargs)
            outcomes.append((result.returncode, result.stdout, result.stderr))
            self.assertEqual(outcomes[0], outcomes[-1], (tool, args))
        return outcomes[0]

    def test_all_checked_in_register_sources(self):
        inputs = sorted((ROOT / COMPONENT / "reg_srcs").iterdir())
        self.assertEqual(len(inputs), 10)
        for source in inputs:
            with self.subTest(source=source.name):
                result = self.compare([source])
                self.assertEqual((result[0], result[2]), (0, b""))
                self.assertTrue(result[1].startswith(b"static const unsigned " + source.name.encode()))

    def test_usage_open_and_read_errors_with_raw_names(self):
        for args in ([], [b"one", b"two"]):
            self.assertEqual(self.compare(args), (1, b"", b"Usage: mkregtable <authfile>\n"))
        missing = os.fsencode(self.work) + b"/missing-\xff"
        self.assertEqual(self.compare([missing]),
                         (255, b"", b"Failed to open: " + missing + b"\nFailed to parse file " + missing + b"\n"))
        for number, data in enumerate((b"", b"gpu 0x100\n", b"gpu 0x100")):
            path = self.work / ("unreadable-" + str(number))
            path.write_bytes(data)
            self.assertEqual(self.compare([path]),
                             (255, b"", b"Failed to parse file " + os.fsencode(path) + b"\n"))
        result = self.compare([self.work])
        self.assertEqual(result[0], 255)

    def test_header_width_regex_chunks_nuls_and_final_newline(self):
        fixtures = [
            b"gpu 0x204\n0x4 REG\n0x80 NAME\n0xFC LAST\n",
            b"ABCDEFGHI025 0x100\n0x4 register\n",
            b"\x80gpu\t0x104\r\n0x0 A\r\n0x7 B\r\n0x83 C",
            b"\v\fgpu\r 104\nno match\n0Xf is ignored\n0xZ means zero\nnoise 0x7 REG\n",
            b"gpu 0x180\n" + b"x" * 1021 + b"0x20 NAME\n",
            b"gpu 0x104\n\0ignored 0x8\n0x4 NAME\0ignored 0x10\n",
            b"gpu 0x104\0 ignored\n#0x8 NAME\n0x8 NAME\n0x4\n",
            b"gpu 0x104\n0x" + b"0" * 200 + b"4 label\n",
            b"gpu 0\n\n",
            b"gpu 0x100\n0x4 A 0x8 B\n0xC_AFTER\n",
        ]
        path = self.work / "parser-input"
        for number, fixture in enumerate(fixtures):
            with self.subTest(number=number):
                path.write_bytes(fixture)
                result = self.compare([path])
                self.assertEqual((result[0], result[2]), (0, b""))

    def test_random_registers_and_duplicate_xor(self):
        rng = random.Random(0x1ADEC0DE)
        path = self.work / "random-input"
        for number in range(100):
            offsets = [rng.randrange(0x400) for _ in range(rng.randrange(1, 100))]
            offsets += offsets[:rng.randrange(len(offsets) + 1)]
            lines = [b"chip 0x404\n"]
            for offset in offsets:
                lines.append(("ignored prefix 0x%X REGISTER\n" % offset).encode())
            path.write_bytes(b"".join(lines))
            with self.subTest(number=number):
                result = self.compare([path])
                self.assertEqual((result[0], result[2]), (0, b""))
        path.write_bytes(b"gpu 0x84\n0x4 A\n0x7 SAME_BIT\n")
        self.assertEqual(self.compare([path])[1],
                         b"static const unsigned gpu_reg_safe_bm[2] = {\n\t0xFFFFFFFF, 0xFFFFFFFF,\n};\n")

    def test_pipe_input_retains_ftell_end_behavior(self):
        result = self.compare(["/dev/stdin"], input=b"gpu 0x84\n0x4 FIRST\n0x8 IGNORED\n")
        self.assertEqual(result, (0, b"static const unsigned gpu_reg_safe_bm[2] = {\n"
                                    b"\t0xFFFFFFFD, 0xFFFFFFFF,\n};\n", b""))

    def test_stdio_errors_and_inherited_sigpipe(self):
        source = ROOT / COMPONENT / "reg_srcs/evergreen"
        for tool in self.tools:
            with self.subTest(tool=tool.name), open("/dev/full", "wb") as output:
                result = self.invoke(tool, [source], stdout=output)
                self.assertEqual((result.returncode, result.stderr), (0, b""))
            for restore in (True, False):
                reader, writer = os.pipe()
                os.close(reader)
                try:
                    result = self.invoke(tool, [source], stdout=writer, restore_signals=restore)
                finally:
                    os.close(writer)
                self.assertEqual((result.returncode, result.stderr),
                                 (-signal.SIGPIPE if restore else 0, b""))

    def test_kbuild_c_rust_c_and_rust_c_rust(self):
        for languages in (("c", "rust", "c"), ("rust", "c", "rust")):
            with self.subTest(languages=languages):
                self.kbuild(languages)

    def test_kbuild_ignores_inherited_parent_environment(self):
        parent = {**self.env, "MAKEFLAGS": "--directory=/missing-parent", "MAKELEVEL": "7",
                  "KBUILD_HOSTCFLAGS": "--bad-parent-c-flag", "KBUILD_HOSTRUSTFLAGS": "--bad-parent-rust-flag",
                  "srctree": "/missing-source", "objtree": "/missing-build", "ARCH": "arm64",
                  "SRCARCH": "arm64", "sub_make_done": "1", "LLVM": "1"}
        self.kbuild(("c", "rust", "c"), parent)

    def kbuild(self, languages, parent=None):
        env = dict(self.env if parent is None else parent)
        for key in list(env):
            if key.startswith("KBUILD_") or key in (
                    "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                    "sub_make_done", "srctree", "srcroot", "objtree", "VPATH", "ARCH", "SRCARCH",
                    "SUBARCH", "CROSS_COMPILE", "LLVM", "LLVM_IAS"):
                env.pop(key)
        temporary = tempfile.TemporaryDirectory(prefix="kbuild-", dir=self.work)
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        source, build = work / "source", work / "build"
        component = source / COMPONENT
        component.mkdir(parents=True)
        for name in ("Makefile", "mkregtable.c", "mkregtable.rs"):
            shutil.copyfile(ROOT / COMPONENT / name, component / name)
        shutil.copytree(ROOT / COMPONENT / "reg_srcs", component / "reg_srcs")
        (build / COMPONENT).mkdir(parents=True)
        (build / "scripts/basic").mkdir(parents=True)
        subprocess.run(self.cc + ["-O2", "-I" + str(ROOT / "scripts/include"),
                                 str(ROOT / "scripts/basic/fixdep.c"), "-o", str(build / "scripts/basic/fixdep")],
                       cwd=work, env=env, check=True, capture_output=True, timeout=30)
        headers = [path.name + "_reg_safe.h" for path in sorted((component / "reg_srcs").iterdir())]
        base = shlex.split(os.environ.get("MAKE", "make")) + [
            "-j4", "-C", str(build), "-f", str(ROOT / "scripts/Makefile.build"),
            "srctree=" + str(ROOT), "srcroot=" + str(source), "objtree=.", "VPATH=" + str(source),
            "building_out_of_srctree=1", "CONFIG_SHELL=/bin/sh", "obj=" + str(COMPONENT),
            "HOSTCC=" + shlex.join(self.cc), "KBUILD_HOSTCFLAGS=-O2",
            "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings -Dunsafe_op_in_unsafe_fn",
            *[str(COMPONENT / name) for name in headers]]
        binary = build / COMPONENT / "mkregtable"
        products = [binary, *[build / COMPONENT / name for name in headers]]

        def run(language):
            command = base + ["HOST_TOOLS_LANG=" + language,
                              "HOSTRUSTC=" + (shlex.join(self.rustc) if language == "rust" else "false")]
            result = subprocess.run(command, cwd=work, env=env, capture_output=True, timeout=60)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            for name in headers:
                original = ROOT / COMPONENT / "reg_srcs" / name.removesuffix("_reg_safe.h")
                self.assertEqual((build / COMPONENT / name).read_bytes(), self.invoke(self.tools[0], [original]).stdout)

        def stamps():
            return [path.stat().st_mtime_ns for path in products]

        previous = None
        for language in languages:
            run(language)
            current = stamps()
            if previous is not None:
                self.assertTrue(all(new > old for new, old in zip(current, previous)))
            record = (binary.parent / ".mkregtable.cmd").read_text()
            self.assertIn("mkregtable." + ("rs" if language == "rust" else "c"), record)
            self.assertEqual("--emit=link=" in record, language == "rust")
            run(language)
            self.assertEqual(stamps(), current)
            (component / ("mkregtable.c" if language == "rust" else "mkregtable.rs")).touch()
            run(language)
            self.assertEqual(stamps(), current)
            (component / ("mkregtable.rs" if language == "rust" else "mkregtable.c")).touch()
            run(language)
            changed = stamps()
            self.assertTrue(all(new > old for new, old in zip(changed, current)))
            run(language)
            self.assertEqual(stamps(), changed)
            # An input register list rebuilds its own header, not the generator.
            (component / "reg_srcs/r100").touch()
            run(language)
            after_input = stamps()
            for path, old, new in zip(products, changed, after_input):
                if path.name == "r100_reg_safe.h":
                    self.assertGreater(new, old)
                else:
                    self.assertEqual(new, old)
            previous = after_input


if __name__ == "__main__":
    unittest.main()
