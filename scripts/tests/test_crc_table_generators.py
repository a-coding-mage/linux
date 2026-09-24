# SPDX-License-Identifier: GPL-2.0-only
"""Compare CRC host generators with original C and exercise real host rules."""

import os
from pathlib import Path
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
TOOLS = ("gen_crc32table", "gen_crc64table")
RUSTFLAGS = ["--edition=2021", "-Dwarnings", "-Wmissing-docs",
             "-Wunreachable-pub", "-Wrust_2018_idioms", "-Dunsafe_op_in_unsafe_fn"]


def environment():
    env = os.environ.copy()
    for key in list(env):
        if key.startswith("KBUILD_") or key in (
                "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                "sub_make_done", "srctree", "srcroot", "objtree", "VPATH", "ARCH", "SRCARCH",
                "SUBARCH", "CROSS_COMPILE", "LLVM", "LLVM_IAS"):
            env.pop(key)
    env["LC_ALL"] = "C"
    return env


class CrcTableGeneratorsTest(unittest.TestCase):
    def run_command(self, command, **kwargs):
        result = subprocess.run([arg if isinstance(arg, bytes) else str(arg) for arg in command],
                                env=environment(),
                                capture_output=True, timeout=60, **kwargs)
        self.assertEqual(result.returncode, 0,
                         f"{command!r}\n{result.stdout!r}\n{result.stderr!r}")
        return result

    def fixture(self, directory):
        source = directory / "source"
        for name in ("lib/crc/Makefile", "include/linux/crc32poly.h",
                     "include/linux/crc32poly_header.rs",
                     *(f"lib/crc/{tool}.{ext}" for tool in TOOLS for ext in ("c", "rs"))):
            dest = source / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, dest)
        config = source / "include/generated/autoconf.h"
        config.parent.mkdir(parents=True)
        config.write_text("")
        return source

    def test_original_c_bytes(self):
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        with tempfile.TemporaryDirectory(prefix="crc-differential-") as tmp:
            work = Path(tmp)
            source = self.fixture(work)
            baseline = {}
            # There are no option-controlled tables in these C sources. Check
            # ignored argv and representative configs explicitly, including
            # old CRC slice-width defines which must not change current output.
            for configured in (False, True):
                config = source / "include/generated/autoconf.h"
                config.write_text("#define CONFIG_CRC32 1\n#define CONFIG_CRC64 1\n"
                                  "#define CONFIG_CPU_BIG_ENDIAN 1\n" if configured else "")
                for tool in TOOLS:
                    c_binary = work / (tool + "-c")
                    flags = ["-O2", "-DCRC_LE_BITS=64", "-DCRC_BE_BITS=64"] if configured else ["-O0"]
                    self.run_command(cc + flags + [source / f"lib/crc/{tool}.c", "-o", c_binary])
                    expected = self.run_command([c_binary]).stdout
                    if tool in baseline:
                        self.assertEqual(expected, baseline[tool])
                    baseline[tool] = expected
                    for optimize in (False, True):
                        rust_binary = work / (tool + "-rust")
                        self.run_command(rustc + RUSTFLAGS + (["-O"] if optimize else []) + [
                            source / f"lib/crc/{tool}.rs", "-o", rust_binary])
                        for args in ([], ["--help"], ["ignored", "--anything"], [b"\xff", b""]):
                            c = self.run_command([c_binary, *args])
                            rust = self.run_command([rust_binary, *args])
                            self.assertEqual((rust.stdout, rust.stderr), (c.stdout, c.stderr))
                            self.assertEqual(rust.stdout, expected)
                    # Compile the generated header too: catches punctuation
                    # regressions even if the differential comparison changes.
                    inc = work / "include/linux"
                    inc.mkdir(parents=True, exist_ok=True)
                    (inc / "types.h").write_text("typedef unsigned long long u64;\n")
                    (inc / "cache.h").write_text("#define ____cacheline_aligned\n")
                    self.run_command(cc + ["-x", "c", "-fsyntax-only", "-I", work / "include", "-"],
                                     input=b"typedef unsigned int u32;\n#define ____cacheline_aligned\n" + expected)

    def test_output_errors_and_negative_controls(self):
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.assertTrue(Path("/dev/full").exists(), "Linux /dev/full is required")

        def failed_output(binary, mode):
            if mode == "full":
                with open("/dev/full", "wb") as output:
                    return subprocess.run([binary], stdout=output, stderr=subprocess.PIPE,
                                          env=environment(), timeout=15)
            reader, writer = os.pipe()
            os.close(reader)
            try:
                # Python ignores SIGPIPE; restore_signals=True restores its
                # default disposition at exec, False preserves SIG_IGN.
                self.assertEqual(signal.getsignal(signal.SIGPIPE), signal.SIG_IGN)
                return subprocess.run([binary], stdout=writer, stderr=subprocess.PIPE,
                                      restore_signals=(mode == "default"),
                                      env=environment(), timeout=15)
            finally:
                os.close(writer)

        with tempfile.TemporaryDirectory(prefix="crc-errors-") as tmp:
            work = Path(tmp)
            source = self.fixture(work)
            for tool in TOOLS:
                c_binary = work / (tool + "-c")
                self.run_command(cc + ["-O2", source / f"lib/crc/{tool}.c", "-o", c_binary])
                for optimize in (False, True):
                    rust_binary = work / (tool + "-rust")
                    flags = RUSTFLAGS + (["-O"] if optimize else [])
                    self.run_command(rustc + flags + [source / f"lib/crc/{tool}.rs", "-o", rust_binary])
                    # Reconstruct the prior normal-runtime/println behavior as
                    # a negative control using the exact same table code.
                    candidate = (source / f"lib/crc/{tool}.rs").read_text()
                    start = candidate.index("// C startup")
                    end = candidate.index("\n}\n", candidate.index("macro_rules! println")) + 3
                    negative = candidate[:start] + candidate[end:]
                    negative = negative.replace(
                        '#[no_mangle]\nextern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {',
                        'fn main() {\n    assert_eq!(std::env::args().collect::<Vec<_>>().len(), 1);'
                    ).replace("\n    0\n}", "\n}")
                    negative_source = source / f"lib/crc/{tool}_negative.rs"
                    negative_source.write_text(negative)
                    negative_binary = work / (tool + "-negative")
                    self.run_command(rustc + flags + [negative_source, "-o", negative_binary])
                    self.assertEqual(self.run_command([negative_binary]).stdout,
                                     self.run_command([c_binary]).stdout)
                    # Deliberately consuming argv is a separate negative
                    # control: options and invalid UTF-8 must remain ignored.
                    for args in (["--help"], [b"\xff"]):
                        expected = self.run_command([c_binary, *args])
                        actual = self.run_command([rust_binary, *args])
                        self.assertEqual((actual.stdout, actual.stderr),
                                         (expected.stdout, expected.stderr))
                        rejected = subprocess.run([negative_binary, *args],
                                                  capture_output=True, timeout=15,
                                                  env=environment())
                        self.assertEqual(rejected.returncode, 101)
                        self.assertTrue(rejected.stderr)
                    for mode in ("full", "default", "ignored"):
                        with self.subTest(tool=tool, optimize=optimize, mode=mode):
                            expected = (-signal.SIGPIPE if mode == "default" else 0, b"")
                            for binary in (c_binary, rust_binary):
                                result = failed_output(binary, mode)
                                self.assertEqual((result.returncode, result.stderr), expected)
                            result = failed_output(negative_binary, mode)
                            self.assertEqual(result.returncode, 101)
                            self.assertIn(b"failed printing to stdout", result.stderr)

    def test_parent_make_environment_isolation(self):
        # Use a real parallel parent make so MAKEFLAGS/jobserver/MAKELEVEL
        # come from make, rather than merely synthesizing those variables.
        with tempfile.TemporaryDirectory(prefix="crc-parent-make-") as tmp:
            work = Path(tmp)
            poisoned = work / "must-not-be-used"
            launcher = work / "launch.py"
            launcher.write_text(
                "import os, runpy, sys\n"
                "assert int(os.environ['MAKELEVEL']) > 0\n"
                "assert '--jobserver-auth=' in os.environ['MAKEFLAGS']\n"
                "assert os.environ['ARCH'] == 'invalid-crc-test-arch'\n"
                "assert os.environ['KBUILD_OUTPUT'].endswith('must-not-be-used')\n"
                "sys.argv = [sys.argv[1], 'CrcTableGeneratorsTest.test_cold_parallel_kbuild_and_dependencies', '-v']\n"
                "runpy.run_path(sys.argv[0], run_name='__main__')\n")
            makefile = work / "Makefile"
            makefile.write_text(
                ".PHONY: check\n"
                + "".join(f"export {key} := {poisoned}\n" for key in (
                    "KBUILD_OUTPUT", "KBUILD_SRC", "srctree", "srcroot", "objtree", "VPATH"))
                + "export ARCH := invalid-crc-test-arch\n"
                + "export SRCARCH := invalid-crc-test-arch\n"
                + "export sub_make_done := 1\n"
                + "export CARGO_MAKEFLAGS := invalid-crc-test-flags\n"
                + "check:\n\t+" + shlex.join([sys.executable, str(launcher), str(Path(__file__).resolve())]) + "\n")
            self.run_command(shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", work, "-f", makefile, "-j2", "HOST_TOOLS_LANG=c", "check"])
            self.assertFalse(poisoned.exists())

    def test_cold_parallel_kbuild_and_dependencies(self):
        cc = os.environ.get("HOSTCC", "cc")
        rustc = os.environ.get("HOSTRUSTC", "rustc")
        make = shlex.split(os.environ.get("MAKE", "make"))
        with tempfile.TemporaryDirectory(prefix="crc-kbuild-") as tmp:
            work = Path(tmp)
            source = self.fixture(work)
            (source / "include/generated/autoconf.h").unlink()
            for initial in ("c", "rust"):
                out = work / ("out-" + initial)
                (out / "scripts/basic").mkdir(parents=True)
                (out / "include/generated").mkdir(parents=True)
                (out / "include/generated/autoconf.h").write_text("")
                # Only bootstrap fixdep; all CRC compilation/generation uses
                # the actual Makefile.build and Makefile.host rules.
                self.run_command(shlex.split(cc) + ["-O2", "-I", ROOT / "scripts/include",
                                                   ROOT / "scripts/basic/fixdep.c",
                                                   "-o", out / "scripts/basic/fixdep"])
                targets = ["lib/crc/crc32table.h", "lib/crc/crc64table.h"]
                base = make + ["-C", str(out), "-f", str(ROOT / "scripts/Makefile.build"),
                               "-j8", "srctree=" + str(ROOT), "srcroot=" + str(source),
                               "objtree=.", "VPATH=" + str(source), "building_out_of_srctree=1",
                               "CONFIG_SHELL=/bin/sh", "obj=lib/crc", "CC=" + cc, "HOSTCC=" + cc,
                               "KBUILD_HOSTCFLAGS=-O2",
                               "KBUILD_HOSTRUSTFLAGS=" + shlex.join(RUSTFLAGS + ["-O"]), *targets]
                expected = None
                for language in (initial, "rust" if initial == "c" else "c", initial):
                    command = base + ["HOST_TOOLS_LANG=" + language,
                                      "HOSTRUSTC=" + (rustc if language == "rust" else "false")]
                    self.run_command(command)
                    generated = [(out / target).read_bytes() for target in targets]
                    if expected is None:
                        expected = generated
                    self.assertEqual(generated, expected)
                    tracked = [out / target for target in targets]
                    for tool in TOOLS:
                        binary = out / "lib/crc" / tool
                        tracked.append(binary)
                        record = binary.with_name("." + tool + ".cmd").read_text()
                        suffix = ".rs" if language == "rust" else ".c"
                        self.assertIn(tool + suffix, record)
                        if language == "rust":
                            self.assertIn("-Dwarnings", record)
                    before = [path.stat().st_mtime_ns for path in tracked]
                    self.run_command(command)
                    self.assertEqual(before, [path.stat().st_mtime_ns for path in tracked])
                    # A change to the retained, unselected language must not
                    # force recompilation after a language switch.
                    other = ".c" if language == "rust" else ".rs"
                    self.run_command(command + [arg for tool in TOOLS for arg in
                                               ("-W", str(source / f"lib/crc/{tool}{other}"))])
                    self.assertEqual(before, [path.stat().st_mtime_ns for path in tracked])
                    # Force an actual rebuild of each selected source, checking
                    # that the unrelated generator and header remain untouched.
                    for index, tool in enumerate(TOOLS):
                        selected = source / f"lib/crc/{tool}{suffix}"
                        self.run_command(command + ["-W", str(selected)])
                        after = [path.stat().st_mtime_ns for path in tracked]
                        self.assertGreater(after[index], before[index])
                        self.assertGreater(after[index + 2], before[index + 2])
                        self.assertEqual(after[1 - index], before[1 - index])
                        self.assertEqual(after[3 - index], before[3 - index])
                        before = after
                    # Verify the polynomial header is recorded and rebuilds
                    # CRC32 only. C's generated autoconf include is unused.
                    header = source / ("include/linux/crc32poly_header.rs" if language == "rust"
                                       else "include/linux/crc32poly.h")
                    record = (out / "lib/crc/.gen_crc32table.cmd").read_text()
                    self.assertIn(header.name, record)
                    # rustc dep-info preserves the module's relative spelling.
                    dependency = str(source / "lib/crc/../../include/linux" / header.name)
                    self.run_command(command + ["-W", dependency])
                    after = [path.stat().st_mtime_ns for path in tracked]
                    self.assertGreater(after[0], before[0])
                    self.assertGreater(after[2], before[2])
                    self.assertEqual(after[1], before[1])
                    self.assertEqual(after[3], before[3])
                    self.assertEqual([(out / target).read_bytes() for target in targets], expected)


if __name__ == "__main__":
    unittest.main()
