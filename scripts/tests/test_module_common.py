#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Original-C/Rust common module metadata, with real ELF widths and byte order.

Only temporary test directories receive files. INT_MATH_I686_SYSROOT and
MODULE_COMMON_{ARM64,POWERPC}_SYSROOT can supply matching target core crates;
an explicitly requested but unusable sysroot fails instead of being skipped.
No target executable, module, or native output tree is modified or executed.
"""

import json
import os
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "scripts/module-common.rs"
GENERATOR = ROOT / "scripts/module-common-data.rs"
INPUT = ROOT / "scripts/module-common-data.h"
ORIGINAL = ROOT / "scripts/module-common.c"
OPTS = ("0", "2", "s")


def command(name, fallback):
    return shlex.split(os.environ.get(name, fallback))


def invoke(arguments, **kwargs):
    return subprocess.run([str(arg) for arg in arguments], capture_output=True,
                          timeout=120, **kwargs)


def run(arguments, **kwargs):
    result = invoke(arguments, **kwargs)
    if result.returncode:
        raise AssertionError(shlex.join(map(str, arguments)) + "\n" +
                             result.stderr.decode(errors="replace"))
    return result.stdout


def macro(source, name):
    lines = source.splitlines(keepends=True)
    for start, line in enumerate(lines):
        if line.startswith("#define " + name + "("):
            end = start + 1
            while lines[end - 1].rstrip().endswith("\\"):
                end += 1
            return "".join(lines[start:end])
    raise ValueError(name)


def elf(path):
    blob = path.read_bytes()
    if blob[:4] != b"\x7fELF" or blob[4] not in (1, 2) or blob[5] not in (1, 2):
        raise ValueError("expected a 32/64-bit ELF object")
    wide, order = blob[4] == 2, "<" if blob[5] == 1 else ">"

    def unpack(fmt, offset):
        return struct.unpack_from(order + fmt, blob, offset)

    offset = unpack("Q" if wide else "I", 40 if wide else 32)[0]
    stride, count, strings = unpack("HHH", 58 if wide else 46)
    sections = [unpack("IIQQQQIIQQ" if wide else "IIIIIIIIII", offset + stride * i)
                for i in range(count)]
    names_header = sections[strings]
    names = blob[names_header[4]:names_header[4] + names_header[5]]
    result = {}
    for index, header in enumerate(sections):
        name = names[header[0]:names.index(0, header[0])].decode()
        result[name] = dict(type=header[1], flags=header[2], align=header[8],
                            data=blob[header[4]:header[4] + header[5]],
                            index=index, header=header)
    return dict(bits=64 if wide else 32, order=order, machine=unpack("H", 18)[0],
                sections=result)


def notes(raw, order):
    result, offset = [], 0
    while offset < len(raw):
        namesz, descsz, kind = struct.unpack_from(order + "III", raw, offset)
        if namesz != 6 or raw[offset + 12:offset + 20] != b"Linux\0\0\0":
            raise ValueError("wrong note owner or name padding")
        end = offset + 20 + descsz
        result.append((kind, raw[offset + 20:end]))
        offset = (end + 3) & ~3
        if raw[end:offset] != bytes(offset - end) or offset > len(raw):
            raise ValueError("invalid note descriptor/padding")
    return sorted(result)


class ModuleCommonTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="module-common-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.rustc = command("HOSTRUSTC", "rustc")
        cls.cc = command("HOSTCC", "cc")
        cls.clang = command("CLANG", "clang")
        cls.flags = ["--edition=2021", "-Dwarnings", "-Wmissing-docs",
                     "-Wrust-2018-idioms", "-Wunreachable-pub", "-Cpanic=abort",
                     "-Coverflow-checks=yes"]
        cls.generator = cls.work / "generator"
        run(cls.rustc + cls.flags + [GENERATOR, "-o", cls.generator])
        cls.targets = {}
        for name, target, variable, cflags, arch, config in [
            ("x86_64", "x86_64-unknown-linux-gnu", None, cls.cc + ["-m64"], "x86", ["CONFIG_X86_64"]),
            ("i686", "i686-unknown-linux-gnu", "INT_MATH_I686_SYSROOT", cls.cc + ["-m32"], "x86", ["CONFIG_X86_32", "CONFIG_M686"]),
            ("arm64", "aarch64-unknown-linux-musl", "MODULE_COMMON_ARM64_SYSROOT", cls.clang + ["--target=aarch64-linux-gnu"], "arm64", []),
            ("powerpc", "powerpc-unknown-linux-gnu", "MODULE_COMMON_POWERPC_SYSROOT", cls.clang + ["--target=powerpc-linux-gnu"], "powerpc", ["CONFIG_PPC32", "CONFIG_FUNCTION_TRACER", "CONFIG_MPROFILE_KERNEL", "CONFIG_RELOCATABLE"]),
        ]:
            rust_flags = ["--target", target]
            requested = os.environ.get(variable) if variable else None
            if requested:
                rust_flags += ["--sysroot", requested]
            libdir = Path(run(cls.rustc + rust_flags + ["--print=target-libdir"]).decode().strip())
            if any(libdir.glob("libcore*.rlib")):
                cls.targets[name] = (rust_flags, cflags, arch, config)
            elif requested:
                raise AssertionError(f"explicit {variable} lacks matching target core: {libdir}")

    def fixture(self, name, target="x86_64", config=(), salt=b'""', release=b'"6.test"', compiler=None):
        directory = self.work / name
        directory.mkdir()
        include = directory / "include"
        for part in ("linux", "asm", "generated"):
            (include / part).mkdir(parents=True)
        # Primitive compiler/types plumbing only. The metadata macros, ELF
        # structures, architecture vermagic, and original source are unchanged.
        (include / "linux/compiler.h").write_text("""#pragma once
#define __used __attribute__((used))
#define __section(s) __attribute__((section(s)))
#define __aligned(n) __attribute__((aligned(n)))
#define __CAT(a,b) a##b
#define __XCAT(a,b) __CAT(a,b)
#define __UNIQUE_ID(a) __XCAT(a,__COUNTER__)
#define __stringify_1(x) #x
#define __stringify(x) __stringify_1(x)
#define static_assert _Static_assert
""")
        (include / "linux/module.h").write_text(
            '#include <linux/compiler.h>\n#define __MODULE_INFO_PREFIX\n' +
            macro((ROOT / "include/linux/moduleparam.h").read_text(), "MODULE_INFO"))
        (include / "linux/types.h").write_text("""#pragma once
typedef unsigned char u8;
typedef unsigned char __u8; typedef signed char __s8;
typedef unsigned short __u16; typedef signed short __s16;
typedef unsigned int __u32; typedef signed int __s32;
typedef unsigned long long __u64; typedef signed long long __s64;
""")
        rust_flags, cc, arch, arch_config = self.targets[target]
        config = list(arch_config) + list(config)
        (directory / "config.h").write_bytes(
            "".join(f"#define {value} 1\n" for value in config).encode() +
            b"#define CONFIG_BUILD_SALT " + salt + b"\n")
        (include / "generated/utsrelease.h").write_bytes(b"#define UTS_RELEASE " + release + b"\n")
        (include / "asm/vermagic.h").write_text(
            f'#include "{ROOT}/arch/{arch}/include/asm/vermagic.h"\n')
        (include / "asm/orc_header.h").write_text(
            f'#include "{ROOT}/arch/x86/include/asm/orc_header.h"\n')
        (include / "asm/orc_hash.h").write_text("#define ORC_HASH " +
            ", ".join(hex((i * 37 + 11) % 256) for i in range(20)) + "\n")
        flags = list(compiler or cc) + ["-ffreestanding", "-std=gnu11", "-nostdinc",
            "-I", include, "-I", ROOT / "include", "-I", ROOT / "include/uapi",
            "-include", directory / "config.h"]
        return dict(directory=directory, include=include, flags=flags,
                    rust_flags=rust_flags, config=config)

    def generate(self, case, extra=()):
        directory, flags = case["directory"], case["flags"] + list(extra)
        # Same original source used by the production rule. No C object is
        # emitted during validation or preprocessing.
        run(flags + ["-fsyntax-only", ORIGINAL])
        source = run(flags + ["-E", "-P", "-x", "c", "-MMD", "-MF",
            directory / "metadata.d", "-MT", directory / "metadata.rs", INPUT])
        data = run([self.generator], input=source)
        (directory / "metadata.rs").write_bytes(data)
        return source, data

    def objects(self, case, optimize="0"):
        directory = case["directory"]
        c, rust = directory / f"c-{optimize}.o", directory / f"rust-{optimize}.o"
        run(case["flags"] + ["-O" + optimize, "-c", ORIGINAL, "-o", c])
        cfgs = sum((["--cfg", c] for c in case["config"]
                    if c in ("CONFIG_UNWINDER_ORC", "CONFIG_MITIGATION_RETPOLINE")), [])
        run(self.rustc + self.flags + case["rust_flags"] + cfgs + ["--crate-name=module_common",
            "--crate-type=rlib", "--emit=obj", "--emit=dep-info=" + str(directory / "rust.d"),
            "-Copt-level=" + optimize, SOURCE, "-o", rust],
            env=dict(os.environ, MODULE_COMMON_DATA=str(directory / "metadata.rs")))
        return c, rust

    def compare(self, c, rust):
        original, translated = elf(c), elf(rust)
        self.assertEqual((original["bits"], original["order"], original["machine"]),
                         (translated["bits"], translated["order"], translated["machine"]))
        original, sections = original["sections"], translated["sections"]
        owned = {".modinfo", ".note.Linux", ".orc_header"}
        self.assertEqual(set(original) & owned, set(sections) & owned)
        for name in owned & set(original):
            a, b = original[name], sections[name]
            expected = (7, 2, 4) if name == ".note.Linux" else (1, 2, 4 if name == ".orc_header" else 1)
            self.assertEqual(tuple(a[k] for k in ("type", "flags", "align")), expected)
            self.assertEqual(tuple(b[k] for k in ("type", "flags", "align")), expected)
            if name == ".note.Linux":
                self.assertEqual(notes(a["data"], translated["order"]), notes(b["data"], translated["order"]))
            elif name == ".modinfo":
                # GCC can reverse declaration order; loader records are the
                # same inline strings, not an array of pointers/descriptors.
                self.assertEqual(sorted(a["data"].split(b"\0")), sorted(b["data"].split(b"\0")))
            else:
                self.assertEqual(len(b["data"]), 20)
                self.assertEqual(a["data"], b["data"])
            for relocation in sections.values():
                if relocation["type"] in (4, 9):
                    self.assertNotEqual(relocation["header"][7], b["index"])
        self.assertEqual(run(command("NM", "nm") + ["-u", rust]).strip(), b"")
        self.assertNotIn(".export_symbol", sections)
        self.assertNotIn("__ksymtab", sections)
        self.assertFalse(any(s["flags"] & 4 and s["data"] for s in sections.values()))

    def matrix(self, target):
        if target not in self.targets:
            self.skipTest(f"matching {target} Rust core unavailable")
        salts = [b'""', b'"abc"', b'"abcd"', br'"a\"\\\n\t\101\x42" "09"',
                 br'"a\0b\377"', '"Unicode-λ"'.encode(), br'"\u1234\U0001f642\u0024"', br'"\q\8\`"']
        for index, salt in enumerate(salts):
            config = ["CONFIG_SMP", "CONFIG_MODULE_UNLOAD", "CONFIG_MODVERSIONS"] if index & 1 else []
            config += ["CONFIG_PREEMPT_RT", "CONFIG_PREEMPT_BUILD"] if index & 2 else (["CONFIG_PREEMPT_BUILD"] if index & 1 else [])
            config += ["CONFIG_LTO", "CONFIG_MITIGATION_RETPOLINE"] if index & 4 else []
            if target in ("x86_64", "i686") and index & 1:
                config.append("CONFIG_UNWINDER_ORC")
            case = self.fixture(f"matrix-{target}-{index}", target, config, salt, br'"6.test-\"\\\t"')
            if index == 7:
                with (case["directory"] / "config.h").open("ab") as out:
                    out.write(b'#define RANDSTRUCT\n#define RANDSTRUCT_HASHED_SEED "012345deadbeef"\n')
            self.generate(case)
            for optimize in OPTS:
                with self.subTest(target=target, config=index, opt=optimize):
                    self.compare(*self.objects(case, optimize))

    def test_x86_64_original_records_configurations_and_optimizations(self):
        self.matrix("x86_64")

    def test_genuine_i686_original_records(self):
        self.matrix("i686")

    def test_genuine_arm64_original_records(self):
        self.matrix("arm64")

    def test_genuine_big_endian_powerpc_original_records(self):
        self.matrix("powerpc")

    def test_generator_unit_tests_and_exact_original_provenance(self):
        binary = self.work / "generator-unit-tests"
        run(self.rustc + [flag for flag in self.flags if flag != "-Cpanic=abort"] +
            ["--test", GENERATOR, "-o", binary])
        self.assertIn(b"4 passed", run([binary]))
        marker = "// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783"
        self.assertEqual(SOURCE.read_text().count(marker), 1)
        self.assertIn("scripts/module-common.rs " + marker.split()[-1],
                      (ROOT / "scripts/tests/translated_sources.txt").read_text().splitlines())

    def test_malformed_generator_input_fails_without_partial_output(self):
        valid = b'LUPOS_EXEC_CHARSET "UTF-8"\nLUPOS_VERMAGIC "v"\nLUPOS_BUILD_SALT ""\nLUPOS_LTO 0\n'
        mutations = [b"", valid + b"LUPOS_LTO 0\n", valid + b"LUPOS_UNKNOWN 1\n",
            valid.replace(b'LUPOS_LTO 0', b'LUPOS_LTO 2'), valid + b"LUPOS_ORC 0x00\n",
            valid.replace(b'"v"', b'"bad\\0tail"'), valid.replace(b'"v"', b'UNRESOLVED'),
            valid.replace(b'"v"', b'L"wide"'), valid.replace(b'"v"', b'"\\uD800"'),
            valid.replace(b'"v"', b'"\\x"'), valid.replace(b'"v"', b'"\\U80000000"'),
            valid.replace(b'LUPOS_EXEC_CHARSET "UTF-8"\n', b''),
            valid.replace(b'"UTF-8"', b'"ISO-8859-1"')]
        for value in mutations:
            with self.subTest(value=value):
                result = invoke([self.generator], input=value)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(result.stdout, b"")

    def test_selected_compiler_literal_bytes_diagnostics_and_werror(self):
        literals = [br'"\u1234"', br'"\U0001f642"', br'"\u0024"', br'"\400"',
                    br'"\777"', br'"\x123456789abcdef01234567890"', br'"\U7fffffff"',
                    br'"\q\8\`"', br'"\e"', br'"\u0000"', br'"\uD800"',
                    br'"\x"', br'"\u12"', br'"\U80000000"', b'"raw-\xff\xfe"']
        for compiler_index, compiler in enumerate((self.cc, self.clang)):
            for index, literal in enumerate(literals):
                case = self.fixture(f"literal-{compiler_index}-{index}", salt=literal, compiler=compiler)
                for werror in (False, True):
                    extra = ["-Werror"] if werror else []
                    reference = invoke(case["flags"] + extra + ["-c", ORIGINAL, "-o", case["directory"] / "reference.o"])
                    with self.subTest(compiler=compiler, literal=literal, werror=werror):
                        if reference.returncode:
                            with self.assertRaises(AssertionError):
                                self.generate(case, extra)
                        else:
                            self.generate(case, extra)
                            self.compare(*self.objects(case))

    def test_original_module_info_rejects_nul_but_salt_preserves_tail(self):
        bad = self.fixture("nul-vermagic", release=br'"v\0tail"')
        result = invoke(bad["flags"] + ["-fsyntax-only", ORIGINAL])
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"contains embedded NUL byte", result.stderr)
        with self.assertRaises(AssertionError):
            self.generate(bad)
        good = self.fixture("nul-salt", salt=br'"x\0tail"')
        self.generate(good)
        c, rust = self.objects(good)
        self.compare(c, rust)
        value = elf(rust)
        self.assertEqual(notes(value["sections"][".note.Linux"]["data"], value["order"])[0],
                         (0x100, b"x\0tail\0"))

    def test_original_header_and_generated_data_dependencies(self):
        case = self.fixture("deps", config=["CONFIG_UNWINDER_ORC"])
        _, first = self.generate(case)
        self.compare(*self.objects(case))
        dependencies = (case["directory"] / "metadata.d").read_text()
        for path in (INPUT, ROOT / "include/linux/vermagic.h",
                     ROOT / "arch/x86/include/asm/vermagic.h", case["directory"] / "config.h",
                     case["include"] / "generated/utsrelease.h", case["include"] / "asm/orc_hash.h"):
            self.assertIn(str(path), dependencies)
        rust_deps = (case["directory"] / "rust.d").read_text()
        self.assertIn(str(SOURCE), rust_deps)
        self.assertIn(str(case["directory"] / "metadata.rs"), rust_deps)
        (case["include"] / "generated/utsrelease.h").write_text('#define UTS_RELEASE "changed"\n')
        (case["include"] / "asm/orc_hash.h").write_text("#define ORC_HASH " + ", ".join(["0xaa"] * 20) + "\n")
        _, second = self.generate(case)
        self.assertNotEqual(first, second)
        self.compare(*self.objects(case))

    def test_execution_charset_guard_and_original_input_charset_conversion(self):
        gcc = command("GCC", "gcc")
        case = self.fixture("encoding", salt=b'"\xe9 \\u1234"', compiler=gcc)
        case["flags"] += ["-finput-charset=ISO-8859-1"]
        self.generate(case)
        c, rust = self.objects(case)
        self.compare(c, rust)
        value = elf(rust)
        self.assertEqual(notes(value["sections"][".note.Linux"]["data"], value["order"])[0],
                         (0x100, "é ሴ\0".encode()))
        # The original C path genuinely supports this encoding. The Rust
        # selection must fail closed, not silently use different string bytes.
        latin = self.fixture("non-utf8-execution", salt=br'"\u00e9"', compiler=gcc)
        latin["flags"] += ["-fexec-charset=ISO-8859-1"]
        original = latin["directory"] / "original.o"
        run(latin["flags"] + ["-c", ORIGINAL, "-o", original])
        value = elf(original)
        self.assertEqual(notes(value["sections"][".note.Linux"]["data"], value["order"])[0],
                         (0x100, b"\xe9\0"))
        with self.assertRaisesRegex(AssertionError, "requires UTF-8 execution encoding"):
            self.generate(latin)
        # Unknown frontends without the GCC charset macro cannot fall through
        # to the known-Clang UTF-8 default.
        unknown = self.fixture("unknown-charset-frontend", compiler=gcc)
        unknown["flags"] += ["-U__GNUC_EXECUTION_CHARSET_NAME"]
        with self.assertRaisesRegex(AssertionError, "known narrow execution character set"):
            self.generate(unknown)


if __name__ == "__main__":
    unittest.main()
