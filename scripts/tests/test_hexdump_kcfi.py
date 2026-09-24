#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Original C/KCFI type identity and protected calls across the native boundary.

The real kernel's unsigned-char alias comes from rust/ffi.rs. Negative controls
use the identical production implementation with an explicitly signed fixture
alias: its ordinary ABI and results still work, but protected C calls must trap.
All compiler fixtures and unchanged-original-C builds are temporary.
"""

import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest

from check_hexdump_kernel import SOURCE
from test_hexdump_abi import RUST_WRAPPER
from test_hexdump_translation import CAPTURE_C, c_environment


ROOT = Path(__file__).resolve().parents[2]
POINTER_APIS = ("bin2hex", "hex2bin", "hex_dump_to_buffer", "print_hex_dump")
C_KCFI = ("-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers")
RUST_KCFI = ("-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers")

CALLER = r'''
#include <linux/hex.h>
#include <linux/printk.h>
#include <stdlib.h>
#include <string.h>
extern void capture_reset(const void *);
extern const char *capture_data(void);
static typeof(&bin2hex) volatile encode = bin2hex;
static typeof(&hex2bin) volatile decode = hex2bin;
static typeof(&hex_dump_to_buffer) volatile format = hex_dump_to_buffer;
static typeof(&print_hex_dump) volatile print = print_hex_dump;
int main(int argc, char **argv)
{
    const unsigned char input[] = {0xab, 0, 0xff};
    unsigned char decoded[3];
    char output[128] = {0};
    if (argc != 2) return 10;
    switch (atoi(argv[1])) {
    case 0:
        if (encode(output, input, 3) != output + 6 || strcmp(output, "ab00ff")) return 11;
        break;
    case 1:
        if (decode(decoded, "ab00ff", 3) || memcmp(decoded, input, 3)) return 12;
        break;
    case 2:
        if (format(input, 3, 16, 1, output, sizeof(output), false) != 8 ||
            strcmp(output, "ab 00 ff")) return 13;
        break;
    case 3:
        capture_reset(input);
        print("", "checked: ", DUMP_PREFIX_NONE, 16, 1, input, 3, false);
        if (!strstr(capture_data(), "prefix=checked: \n") ||
            !strstr(capture_data(), "line=ab 00 ff\n")) return 14;
        break;
    default: return 15;
    }
    return 0;
}
'''


def type_ids(path):
    """Read emitted function type metadata, not handwritten expected hashes."""
    text = path.read_text()
    values = {index: int(value) & 0xffffffff for index, value in
              re.findall(r"(?m)^!(\d+) = !\{i32 (-?\d+)\}", text)}
    return {name.strip('"'): values[index] for name, index in
            re.findall(r"(?m)^define [^\n]*?@([^ (]+)\([^\n]*!kcfi_type !(\d+)", text)}


def no_core_dump():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


class HexdumpKcfiTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.clang = shlex.split(os.environ.get("CLANG", "clang"))
        if not shutil.which(cls.clang[0]):
            if "CLANG" in os.environ:
                raise RuntimeError("explicit CLANG executable is unavailable")
            raise unittest.SkipTest("Clang is required for protected KCFI calls")
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        host = subprocess.check_output([*cls.rustc, "-vV"], text=True)
        target = re.search(r"(?m)^host: (.*)$", host).group(1)
        if not target.startswith(("x86_64-", "aarch64-")) or "linux" not in target:
            raise unittest.SkipTest("protected native KCFI fixture requires Linux x86-64 or AArch64")
        temporary = tempfile.TemporaryDirectory(prefix="hexdump-kcfi-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.env = {**os.environ, "RUSTC_BOOTSTRAP": "1"}
        cls.include = c_environment(cls.work)
        # Pull the exact two declarations from the original header. All other
        # hexdump declarations are included directly through linux/hex.h.
        header = (ROOT / "include/linux/printk.h").read_text()
        declarations = []
        for name in ("hex_dump_to_buffer", "print_hex_dump"):
            matches = re.findall(r"extern (?:int|void) " + name + r"\([^;]+;", header)
            if len(matches) != 1:
                raise AssertionError("ambiguous original declaration: " + name)
            declarations.append(matches[0])
        (cls.include / "linux/printk.h").write_text(
            '#include <linux/types.h>\n#include <linux/kernel.h>\n' +
            "\n".join(declarations) + '\n#define KERN_INFO ""\n'
            '#define pr_info(...) printf(__VA_ARGS__)\n'
            '#define pr_err(...) fprintf(stderr, __VA_ARGS__)\n')
        (cls.include / "linux/init.h").write_text("#define __init\n#define __exit\n")
        (cls.include / "linux/module.h").write_text(
            "#define module_init(fn) int main(void) { return fn(); }\n"
            "#define module_exit(fn)\n#define MODULE_LICENSE(value)\n"
            "#define MODULE_DESCRIPTION(value)\n")
        (cls.include / "linux/string.h").write_text("#include <string.h>\n")
        cls.wrapper = cls.work / "wrapper.rs"
        cls.wrapper.write_text(RUST_WRAPPER.replace("@SOURCE@", str(ROOT / "lib/hexdump_rust.rs")) +
                               "\n// Real host runtime, only for linking the independent executable.\n"
                               "extern crate std;\n")
        cls.ffi = cls.work / "libkernel_ffi.rlib"
        cls.run_command([*cls.rustc, "--edition=2021", "--crate-name=kernel_ffi", "--crate-type=rlib",
                         "-Dwarnings", ROOT / "rust/ffi.rs", "-o", cls.ffi])
        cls.capture = cls.work / "capture.c"
        cls.capture.write_text(CAPTURE_C)
        cls.capture_obj, cls.ctype_obj = cls.work / "capture.o", cls.work / "ctype.o"
        cls.run_command([*cls.clang, "-O2", "-funsigned-char", "-c", cls.capture, "-o", cls.capture_obj])
        cls.run_command([*cls.clang, "-O2", "-funsigned-char", "-I" + str(cls.include),
                         "-c", ROOT / "lib/ctype.c", "-o", cls.ctype_obj])
        cls.caller = cls.work / "caller.c"
        cls.caller.write_text(CALLER)
        cls.native_fixture = cls.work / "native-fixture.c"
        cls.native_fixture.write_text(SOURCE)
        cls.originals, cls.rust_owners = {}, {}

    @classmethod
    def run_command(cls, command):
        result = subprocess.run(list(map(str, command)), env=cls.env,
                                capture_output=True, timeout=120)
        if result.returncode:
            raise AssertionError(shlex.join(list(map(str, command))) + "\n" +
                                 result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
        return result

    @classmethod
    def original(cls, optimize, unsigned=True):
        key = optimize, unsigned
        if key not in cls.originals:
            stem = f"original-{optimize}-{unsigned}"
            obj, ir = cls.work / (stem + ".o"), cls.work / (stem + ".ll")
            common = [*cls.clang, "-O" + optimize, "-funsigned-char" if unsigned else "-fsigned-char",
                      *C_KCFI, "-I" + str(cls.include), "-DCONFIG_PRINTK", ROOT / "lib/hexdump.c"]
            cls.run_command([*common, "-c", "-o", obj])
            cls.run_command([*common, "-S", "-emit-llvm", "-o", ir])
            cls.originals[key] = obj, ir
        return cls.originals[key]

    @classmethod
    def translated(cls, optimize, unsigned=True):
        key = optimize, unsigned
        if key not in cls.rust_owners:
            stem = f"native-{optimize}-{unsigned}"
            library, ir = cls.work / (stem + ".a"), cls.work / (stem + ".ll")
            cls.run_command([*cls.rustc, "--edition=2021", "--crate-name=hexdump_kcfi",
                             "--crate-type=staticlib", "-Copt-level=" + optimize, "-Cpanic=abort",
                             "-Coverflow-checks=yes", "-Dwarnings", "-Wmissing-docs",
                             "-Wrust-2018-idioms", "-Wunreachable-pub", *RUST_KCFI,
                             "--extern", "kernel_ffi=" + str(cls.ffi), "--cfg", "CONFIG_RUST",
                             "--cfg", "CONFIG_PRINTK", *(["--cfg", "native_char"] if unsigned else []),
                             "--emit=link=" + str(library) + ",llvm-ir=" + str(ir), cls.wrapper])
            cls.rust_owners[key] = library, ir
        return cls.rust_owners[key]

    @classmethod
    def executable(cls, owner, label, protected=True, fixture=False):
        binary = cls.work / label
        cls.run_command([*cls.clang, "-O2", "-funsigned-char", *(C_KCFI if protected else ()),
                         "-I" + str(cls.include), cls.native_fixture if fixture else cls.caller,
                         owner, cls.ctype_obj, cls.capture_obj, "-ldl", "-lpthread", "-lm", "-o", binary])
        return binary

    def result(self, binary, case=None):
        return subprocess.run([binary, *([] if case is None else [str(case)])], capture_output=True,
                              timeout=10, preexec_fn=no_core_dump)

    def test_actual_original_c_normalized_type_ids_and_signedness_negative_control(self):
        for optimize in ("0", "2", "s"):
            original = type_ids(self.original(optimize)[1])
            native = type_ids(self.translated(optimize)[1])
            wrong = type_ids(self.translated(optimize, unsigned=False)[1])
            signed_c = type_ids(self.original(optimize, unsigned=False)[1])
            for name in (*POINTER_APIS, "hex_to_bin"):
                with self.subTest(optimize=optimize, symbol=name):
                    self.assertIn(name, original)
                    self.assertEqual(native[name], original[name])
                    self.assertEqual(wrong[name], signed_c[name])
                    if name in POINTER_APIS:
                        self.assertNotEqual(wrong[name], original[name])
                    else:
                        self.assertEqual(wrong[name], original[name])

    def test_protected_original_and_native_calls_and_four_wrong_type_traps(self):
        for optimize in ("0", "2", "s"):
            for kind, owner in (("original", self.original(optimize)[0]),
                                ("native", self.translated(optimize)[0])):
                binary = self.executable(owner, kind + "-" + optimize)
                for case, name in enumerate(POINTER_APIS):
                    result = self.result(binary, case)
                    self.assertEqual(result.returncode, 0, (optimize, kind, name, result.stderr))
            wrong = self.translated(optimize, unsigned=False)[0]
            unchecked = self.executable(wrong, "wrong-unchecked-" + optimize, protected=False)
            checked = self.executable(wrong, "wrong-checked-" + optimize)
            for case, name in enumerate(POINTER_APIS):
                # This exact wrong-typed owner still computes correct bytes.
                # A failure with checks enabled must come from type protection.
                self.assertEqual(self.result(unchecked, case).returncode, 0, (optimize, name))
                result = self.result(checked, case)
                self.assertIn(result.returncode, (-signal.SIGILL, -signal.SIGTRAP),
                              (optimize, name, result.returncode, result.stderr))

    def test_exact_native_kernel_fixture_retains_all_four_protected_calls(self):
        for name in POINTER_APIS:
            self.assertIn(f"static typeof(&{name}) volatile call_{name} = {name};", SOURCE)
            self.assertNotRegex(SOURCE, r"(?<![\w])" + name + r"\s*\(")
        ir = self.work / "native-fixture.ll"
        self.run_command([*self.clang, "-O2", "-funsigned-char", *C_KCFI,
                          "-I" + str(self.include), "-S", "-emit-llvm", self.native_fixture, "-o", ir])
        checks = {int(value) & 0xffffffff for value in
                  re.findall(r'"kcfi"\(i32 (-?\d+)\)', ir.read_text())}
        expected = type_ids(self.original("2")[1])
        self.assertEqual(checks, {expected[name] for name in POINTER_APIS})
        for kind, owner in (("original", self.original("2")[0]), ("native", self.translated("2")[0])):
            result = self.result(self.executable(owner, "whole-fixture-" + kind, fixture=True))
            self.assertEqual(result.returncode, 0, (kind, result.stderr))
            self.assertEqual(result.stdout, b"LUPOS_HEX_ABI_OK\n")


if __name__ == "__main__":
    unittest.main()
