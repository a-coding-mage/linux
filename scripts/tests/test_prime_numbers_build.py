# SPDX-License-Identifier: GPL-2.0-only
"""Real prime-provider Kconfig and Kbuild selection, isolated from native outputs.

The compiler transport is shared with the differential cache tests; these checks
prove build selection and dependency behavior, not native allocator/RCU locking.
Unrelated math objects are private empty archive members, never substitute prime
implementations. The selected prime C/Rust source is compiled directly.
"""

import os
from pathlib import Path
import re
import shlex
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from test_prime_numbers import Fixture
from test_rational_build import environment, run


ROOT = Path(__file__).resolve().parents[2]
PREFIX = ["div64.o", "gcd.o", "lcm.o", "int_log.o", "int_pow.o", "int_sqrt.o", "reciprocal_div.o"]


def stanza(name, source):
    match = re.search(r"(?ms)^config " + re.escape(name) + r"\n.*?(?=^config |\Z)", source.read_text())
    if match is None:
        raise AssertionError("missing actual Kconfig stanza: " + name)
    return match.group()


class PrimeSelectionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="prime-selection-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def test_actual_kconfig_defaults_rust_off_and_keeps_original_tristate(self):
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "rust"\n' +
                          stanza("RUST_PRIME_NUMBERS", ROOT / "lib/Kconfig") + "\n" +
                          stanza("PRIME_NUMBERS", ROOT / "lib/math/Kconfig"))
        for tool in cached_conf_tools():
            for rust, requested in (("n", "y"), ("y", None), ("y", "n"), ("y", "y")):
                for state in ("n", "m", "y"):
                    with self.subTest(tool=tool, rust=rust, requested=requested, state=state):
                        text = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_PRIME_NUMBERS={state}\n"
                        if requested is not None:
                            text += "CONFIG_RUST_PRIME_NUMBERS=" + requested + "\n"
                        (self.work / ".config").write_text(text)
                        run([tool, "--olddefconfig", config], cwd=self.work,
                            env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                        actual = (self.work / ".config").read_text().splitlines()
                        self.assertEqual("CONFIG_RUST_PRIME_NUMBERS=y" in actual, rust == requested == "y")
                        self.assertEqual("CONFIG_PRIME_NUMBERS=" + state in actual, state != "n")

    def test_actual_makefile_keeps_object_order_and_independent_host_language(self):
        makefile = self.work / "Makefile"
        makefile.write_text(f"include {ROOT}/lib/math/Makefile\n.PHONY: selection\nselection:\n"
                            "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)' '$(prime_numbers-y)'\n")
        for host in ("c", "rust"):
            for rust in ("", "y"):
                for state in ("", "m", "y"):
                    result = run(["make", "--no-print-directory", "-rR", "-f", makefile, "selection",
                                  "HOST_TOOLS_LANG=" + host, "CONFIG_RUST_PRIME_NUMBERS=" + rust,
                                  "CONFIG_PRIME_NUMBERS=" + state, "CONFIG_CORDIC=y",
                                  "CONFIG_POLYNOMIAL=y", "CONFIG_RATIONAL=y"], cwd=self.work, env=environment())
                    builtin, modules, composite = result.stdout.decode().splitlines()
                    self.assertEqual(builtin.split(), PREFIX + ["cordic.o", "polynomial.o"] +
                                     (["prime_numbers.o"] if state == "y" else []) + ["rational.o", "tests/"])
                    self.assertEqual(modules.split(), ["prime_numbers.o"] if state == "m" else [])
                    self.assertEqual(composite, "")


class PrimeKbuildTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="prime-kbuild-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.fixture = Fixture(cls.work / "fixture", 64)
        cls.obj = cls.work / "lib/math"
        (cls.obj / "tests").mkdir(parents=True)
        (cls.work / "scripts/basic").mkdir(parents=True)
        run([*cls.fixture.rustc, "--edition=2021", "-O", "-Dwarnings",
             ROOT / "scripts/basic/fixdep.rs", "-o", cls.work / "scripts/basic/fixdep"])
        cls.ar = shlex.split(os.environ.get("AR", "ar"))
        empty = cls.work / "empty.S"
        empty.write_text('.section .note.GNU-stack,"",@progbits\n')
        for name in PREFIX:
            run([*cls.fixture.cc, "-c", empty, "-o", cls.obj / name])
        run([*cls.ar, "cr", cls.obj / "tests/built-in.a"])
        (cls.obj / "tests/modules.order").write_bytes(b"")
        cflags = [flag for flag in cls.fixture.flags if not flag.startswith(
            ("-DMODULE", "-DKBUILD_MODNAME=", "-DKBUILD_MODFILE="))]
        cflags += ['-DKBUILD_MODNAME="$(target-stem)"', '-DKBUILD_MODFILE="$(modfile)"']
        flags = shlex.join(cflags) + " -O2 $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile)"
        rust = cls.fixture.rustc + cls.fixture.target + ["--extern", "kernel=" + str(cls.fixture.library)]
        cls.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                       "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                       "srctree=" + str(ROOT), "objtree=" + str(cls.work), "VPATH=" + str(ROOT),
                       "need-builtin=1", "need-modorder=1", "KBUILD_BUILTIN=1", "KBUILD_MODULES=1",
                       "CONFIG_MODULES=y", "AR=" + shlex.join(cls.ar), "NM=" + os.environ.get("NM", "nm"),
                       "LD=" + os.environ.get("LD", "ld"), "AWK=" + os.environ.get("AWK", "awk"),
                       "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(rust) +
                       " --edition=2021 --crate-type=rlib -O -Cpanic=abort -Dwarnings -Wmissing-docs"
                       " -Zbinary_dep_depinfo=y"
                       " -Wunreachable-pub -Wrust-2018-idioms -Dunsafe-op-in-unsafe-fn -Zcrate-attr=no_std"
                       " --cfg CONFIG_PRIME_NUMBERS_KUNIT_TEST $(if $(part-of-module),--cfg MODULE)"
                       " --emit=dep-info=$(depfile)",
                       "cmd_cc_o_c=" + shlex.join(cls.fixture.cc) + " " + flags + " -c $< -o $@",
                       "cmd_cc_s_c=" + shlex.join(cls.fixture.cc) + " " + flags + " -S $< -o $@",
                       "cmd_cc_ll_c=" + shlex.join(shlex.split(os.environ.get("CLANG", "clang"))) +
                       " " + flags + " -emit-llvm -S $< -o $@"]
        for name in (*PREFIX, "tests/built-in.a", "tests/modules.order"):
            cls.command += ["-o", "lib/math/" + name]

    def make(self, *targets, rust=True, state="m", extra=()):
        return run([*self.command, "CONFIG_RUST_PRIME_NUMBERS=" + ("y" if rust else ""),
                    "CONFIG_PRIME_NUMBERS=" + state, *extra, *targets], cwd=self.work, env=self.fixture.env)

    def test_parallel_object_assembly_ir_source_switch_and_noop(self):
        targets = tuple("lib/math/prime_numbers." + ext for ext in ("o", "s", "ll"))
        for rust in (False, True, False, True):
            self.make(*targets, "lib/math/prime_numbers.mod", "lib/math/modules.order", rust=rust)
            self.assertEqual((self.obj / "prime_numbers.mod").read_text(), "lib/math/prime_numbers.o\n")
            self.assertEqual((self.obj / "modules.order").read_text(), "lib/math/prime_numbers.o\n")
            for ext in ("o", "s", "ll"):
                command = (self.obj / (".prime_numbers." + ext + ".cmd")).read_text()
                source = ROOT / ("lib/math/prime_numbers." + ("rs" if rust else "c"))
                self.assertIn("source_lib/math/prime_numbers." + ext + " := " + str(source), command)
            before = [(self.work / name).stat().st_mtime_ns for name in targets]
            self.make(*targets, rust=rust)
            self.assertEqual(before, [(self.work / name).stat().st_mtime_ns for name in targets])

    def test_original_builtin_module_disabled_archive_membership(self):
        for rust in (False, True, False):
            for state in ("y", "m", ""):
                self.make("lib/math/built-in.a", "lib/math/modules.order", rust=rust, state=state)
                members = run([*self.ar, "t", self.obj / "built-in.a"]).stdout.decode().splitlines()
                self.assertEqual([Path(name).name for name in members],
                                 PREFIX + (["prime_numbers.o"] if state == "y" else []))
                self.assertEqual((self.obj / "modules.order").read_text(),
                                 "lib/math/prime_numbers.o\n" if state == "m" else "")

    def test_actual_provider_mutex_export_and_metadata_dependencies(self):
        targets = tuple("lib/math/prime_numbers." + ext for ext in ("o", "s", "ll"))
        self.make(*targets)
        for dependency in (ROOT / "lib/math/prime_numbers.rs", ROOT / "lib/math/prime_numbers_mutex.rs",
                           ROOT / "rust/ffi_export.rs", ROOT / "include/linux/export_header.rs", self.fixture.library):
            # rustc preserves ../ components from #[path]. GNU make's -W
            # matches the recorded spelling; it does not canonicalize aliases.
            command = (self.obj / ".prime_numbers.o.cmd").read_text()
            spellings = [word for word in command.split() if word.startswith("/") and
                         Path(word).resolve() == dependency.resolve()]
            self.assertTrue(spellings, str(dependency))
            before = [(self.work / name).stat().st_mtime_ns for name in targets]
            self.make(*targets, extra=("-W", spellings[0]))
            after = [(self.work / name).stat().st_mtime_ns for name in targets]
            self.assertTrue(all(new > old for new, old in zip(after, before)), str(dependency))
            self.make(*targets)
            self.assertEqual(after, [(self.work / name).stat().st_mtime_ns for name in targets])
        command = (self.obj / ".prime_numbers.o.cmd").read_text()
        self.assertIn(str(self.fixture.library), command)
        self.assertIn(str(ROOT / "lib/math/prime_numbers_mutex.rs"), command)


if __name__ == "__main__":
    unittest.main()
