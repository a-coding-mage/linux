#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""MIPS vDSO byte repairs and generated C output, compared with the C tool."""

import contextlib
import os
from pathlib import Path
import random
import re
import shlex
import shutil
import stat
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/mips/vdso/genvdso"
SYMBOLS = [(b"__vdso_sigreturn", 0x120), (b"__vdso_rt_sigreturn", 0x340)]


def fixture(bits=64, big=False, abi2=False, symbols=SYMBOLS,
            names=(b".mips_abiflags",), extra=(), extended=False):
    endian = ">" if big else "<"
    wide = bits == 64
    ehsize, shsize, symsize = (64, 64, 24) if wide else (52, 40, 16)
    sections = [dict(name=b"", kind=0, data=b""), dict(name=b".shstrtab", kind=3, data=b"")]
    sections += [dict(name=name, kind=1, data=bytes(range(24))) for name in names]
    sections += [dict(name=name, kind=kind, data=bytes(24)) for name, kind in extra]
    if symbols is not None:
        strings = bytearray(b"\0")
        symtab = bytearray(symsize)
        for name, value in symbols:
            offset = len(strings)
            strings += name + b"\0"
            value &= 2**bits - 1
            symtab += struct.pack(endian + "IBBHQQ", offset, 0x12, 0, 0xfff1, value, 0) if wide else \
                struct.pack(endian + "IIIBBH", offset, value, 0, 0x12, 0, 0xfff1)
        sections += [dict(name=b".symtab", kind=2, data=symtab, link=len(sections) + 1, stride=symsize),
                     dict(name=b".strtab", kind=3, data=strings)]
    names_data = bytearray(b"\0")
    for section in sections:
        section["name_offset"] = len(names_data)
        names_data += section["name"] + b"\0"
    sections[1]["data"] = names_data
    data = bytearray(ehsize)
    for section in sections:
        data += bytes(-len(data) % 8)
        section["offset"] = len(data)
        data += section["data"]
    data += bytes(-len(data) % 8)
    shoff = len(data)
    for index, section in enumerate(sections):
        fields = (section["name_offset"], section["kind"], 0, 0, section["offset"],
                  len(section["data"]), section.get("link", 0), 0, 1, section.get("stride", 0))
        if index == 0 and extended:
            fields = (0, 0, 0, 0, 0, len(sections), 1, 0, 0, 0)
        data += struct.pack(endian + ("IIQQQQIIQQ" if wide else "IIIIIIIIII"), *fields)
    ident = b"\x7fELF" + bytes((2 if wide else 1, 2 if big else 1, 1)) + bytes(9)
    data[:ehsize] = struct.pack(endian + ("16sHHIQQQIHHHHHH" if wide else "16sHHIIIIIHHHHHH"),
                               ident, 3, 8, 1, 0, 0, shoff, 0x20 if abi2 else 0,
                               ehsize, 56 if wide else 32, 0, shsize,
                               0 if extended else len(sections), 0xffff if extended else 1)
    return data, sections, shoff


class MipsGenvdsoTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="mips-genvdso-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.tools = [cls.work / name for name in ("c", "rust")]
        commands = [shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-Wall", "-Werror", str(SOURCE.with_suffix(".c")), "-o", str(cls.tools[0])],
            shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
            str(SOURCE.with_suffix(".rs")), "-o", str(cls.tools[1])]]
        for command in commands:
            result = subprocess.run(command, capture_output=True, text=True)
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)

    @contextlib.contextmanager
    def case(self, debug, stripped=None):
        with tempfile.TemporaryDirectory(prefix="mips-vdso-case-") as temporary:
            paths = [Path(temporary) / side for side in ("c", "rust")]
            for path in paths:
                path.mkdir()
                (path / "debug").write_bytes(debug)
                (path / "stripped").write_bytes(debug if stripped is None else stripped)
                (path / "out").write_bytes(b"previous output")
            yield paths

    def compare(self, paths, args=("debug", "stripped", "out")):
        results = []
        for path, tool in zip(paths, self.tools):
            result = subprocess.run([tool, *args], cwd=path, capture_output=True, timeout=10)
            files = {os.fsencode(file.name): (file.read_bytes() if file.is_file() else None,
                     stat.S_IMODE(file.stat().st_mode)) for file in path.iterdir() if file.exists()}
            results.append((result.returncode, result.stdout,
                            result.stderr.replace(os.fsencode(tool), b"<tool>"), files))
        self.assertEqual(*results)
        return results[1]

    def test_all_abis_byte_orders_names_and_exact_generated_source(self):
        for bits, abi2 in ((32, False), (32, True), (64, False)):
            for big in (False, True):
                for name in ("", "o32", "n32", "name.with-punctuation", os.fsdecode(b"name-\xff")):
                    raw = fixture(bits, big, abi2)[0]
                    stripped = fixture(bits, big, abi2, symbols=None)[0]
                    with self.subTest(bits=bits, big=big, name=name), self.case(raw, stripped) as paths:
                        result = self.compare(paths, ("debug", "stripped", "out", name))
                        self.assertEqual(result[0], 0)
                        output = result[3][b"out"][0]
                        self.assertIn(b".off_rt_sigreturn = 0x340,", output)
                        self.assertEqual(b".off_sigreturn" in output, bits == 32 and not abi2)
                        embedded = bytes(int(value, 16) for value in re.findall(rb"0x([0-9a-f]{2}), ", output))
                        self.assertEqual(embedded, result[3][b"stripped"][0])

    def test_multiple_and_shared_abi_names_partial_error_order(self):
        for bits in (32, 64):
            for big in (False, True):
                for names, extra in (((b".mips_abiflags", b".mips_abiflags"), ()),
                                     ((b".mips_abiflags", b".MIPS.abiflags"), ()),
                                     ((b".mips_abiflags",), ((b".rela.text", 4),)),
                                     ((b".MIPS.abiflags",), ()), ((), ())):
                    data, _, _ = fixture(bits, big, names=names, extra=extra)
                    with self.case(data) as paths:
                        self.compare(paths)
                data, sections, table = fixture(bits, big, names=(b".mips_abiflags",) * 2)
                struct.pack_into((">" if big else "<") + "I", data, table + 3 * (64 if bits == 64 else 40), sections[2]["name_offset"])
                with self.case(data) as paths:
                    self.assertEqual(self.compare(paths)[0], 1)

    def test_missing_symbols_first_table_and_duplicate_names(self):
        for bits, abi2 in ((32, False), (32, True), (64, False)):
            for big in (False, True):
                for symbols in (None, [], SYMBOLS[:1], SYMBOLS[1:], SYMBOLS[::-1],
                                [(b"__vdso_rt_sigreturn", 0), *SYMBOLS],
                                [(b"__vdso_sigreturn", -1), (b"__vdso_rt_sigreturn", -2)]):
                    with self.case(fixture(bits, big, abi2, symbols=symbols)[0]) as paths:
                        result = self.compare(paths)
                        if result[0]:
                            self.assertNotIn(b"out", result[3])

    def test_relocation_rejection_and_ordered_two_file_repairs(self):
        for kind in (4, 9):
            valid = fixture()[0]
            bad = fixture(extra=((b".rel", kind),))[0]
            for debug, stripped in ((valid, bad), (bad, valid)):
                with self.case(debug, stripped) as paths:
                    result = self.compare(paths)
                    self.assertEqual(result[0], 1)
                    self.assertEqual(result[3][b"out"][0], b"previous output")

    def test_paths_modes_symlinks_hardlinks_and_aliased_inputs(self):
        for alias in ("symbolic", "hard", "same", None):
            with self.case(fixture()[0]) as paths:
                inodes = []
                for path in paths:
                    (path / "debug").chmod(0o751)
                    if alias == "symbolic":
                        (path / "linked").symlink_to("debug")
                    elif alias == "hard":
                        os.link(path / "debug", path / "linked")
                    (path / os.fsdecode(b"output-\xff")).symlink_to("out")
                    inodes.append((path / "debug").stat().st_ino)
                second = "linked" if alias in ("symbolic", "hard") else "debug" if alias == "same" else "stripped"
                result = self.compare(paths, ("debug", second, os.fsdecode(b"output-\xff")))
                self.assertEqual(result[0], 0 if alias is None else 1)
                for path, inode in zip(paths, inodes):
                    self.assertEqual((path / "debug").stat().st_ino, inode)
                    self.assertEqual(stat.S_IMODE((path / "debug").stat().st_mode), 0o751)

    def test_usage_open_and_header_errors(self):
        for args in ((), ("debug",), ("debug", "stripped", "out", "n", "extra"),
                     ("missing", "stripped", "out"), ("debug", "missing", "out"),
                     ("debug", "stripped", "."), (".", "stripped", "out")):
            with self.case(fixture()[0]) as paths:
                self.assertEqual(self.compare(paths, args)[0], 1)
        data = fixture()[0]
        for offset, value in ((0, 0), (4, 0), (5, 0), (16, 0), (18, 0)):
            bad = data.copy()
            bad[offset] = value
            with self.case(bad, data) as paths:
                self.assertEqual(self.compare(paths)[0], 1)
        with self.case(b"", data) as paths:
            self.assertEqual(self.compare(paths)[0], 1)

    def test_random_symbol_tables(self):
        rng = random.Random(0xD50)
        for index in range(60):
            bits = rng.choice((32, 64))
            big = rng.choice((False, True))
            symbols = [(b"unused_" + str(i).encode(), rng.getrandbits(bits)) for i in range(rng.randrange(100))]
            symbols += [(name, rng.getrandbits(bits)) for name, _ in SYMBOLS]
            rng.shuffle(symbols)
            with self.subTest(index=index), self.case(fixture(bits, big, symbols=symbols)[0]) as paths:
                self.assertEqual(self.compare(paths)[0], 0)

    def test_malformed_inputs_and_output_alias_are_safe(self):
        valid, _, shoff = fixture()
        cases = [valid[:size] for size in (0, 4, 16, 63, shoff, len(valid) - 1)]
        for offset, code, value in ((40, "Q", 2**64 - 1), (62, "H", 0xffff),
                                    (shoff + 2 * 64, "I", 0xffffffff),
                                    (shoff + 3 * 64 + 24, "Q", 2**64 - 1)):
            data = valid.copy()
            struct.pack_into("<" + code, data, offset, value)
            cases.append(data)
        for bad in cases:
            with self.case(bad, valid) as paths:
                run = subprocess.run([self.tools[1], "debug", "stripped", "out"], cwd=paths[1], capture_output=True, timeout=2)
                self.assertEqual(run.returncode, 1)
                self.assertNotIn(b"panicked", run.stderr)
        for output in ("debug", "stripped", "alias"):
            with self.case(valid) as paths:
                os.link(paths[1] / "debug", paths[1] / "alias")
                run = subprocess.run([self.tools[1], "debug", "stripped", output], cwd=paths[1], capture_output=True)
                self.assertEqual(run.returncode, 1)
                self.assertEqual((paths[1] / "debug").read_bytes(), valid)
                self.assertEqual((paths[1] / "stripped").read_bytes(), valid)

    def test_write_errors_and_mismatched_input_formats(self):
        if Path("/dev/full").exists():
            with self.case(fixture()[0]) as paths:
                run = subprocess.run([self.tools[1], "debug", "stripped", "/dev/full"], cwd=paths[1], capture_output=True)
                self.assertEqual(run.returncode, 1)
                self.assertIn(b"Failed to write", run.stderr)
        for stripped in (fixture(32)[0], fixture(big=True)[0]):
            debug = fixture()[0]
            with self.case(debug, stripped) as paths:
                run = subprocess.run([self.tools[1], "debug", "stripped", "out"], cwd=paths[1], capture_output=True)
                self.assertEqual(run.returncode, 1)
                self.assertEqual((paths[1] / "debug").read_bytes(), debug)
                self.assertEqual((paths[1] / "stripped").read_bytes(), stripped)

    def test_extended_section_counts_and_large_symbol_tables(self):
        # C truncates both counts to u16 and cannot safely decode SHN_XINDEX.
        # Rust follows ELF extended numbering and searches every symbol.
        for bits in (32, 64):
            for big in (False, True):
                data = fixture(bits, big, extended=True)[0]
                with self.case(data) as paths:
                    run = subprocess.run([self.tools[1], "debug", "stripped", "out"], cwd=paths[1], capture_output=True)
                    self.assertEqual(run.returncode, 0, run.stderr)
                    self.assertIn(b".off_rt_sigreturn = 0x340,", (paths[1] / "out").read_bytes())
        data = fixture(symbols=[(b"unused", 0)] * 65536 + SYMBOLS)[0]
        with self.case(data) as paths:
            run = subprocess.run([self.tools[1], "debug", "stripped", "out"], cwd=paths[1], capture_output=True, timeout=10)
            self.assertEqual(run.returncode, 0, run.stderr)
            self.assertIn(b".off_rt_sigreturn = 0x340,", (paths[1] / "out").read_bytes())

    def test_random_malformed_images_do_not_panic(self):
        rng = random.Random(0xBAD50)
        for bits in (32, 64):
            for big in (False, True):
                original = fixture(bits, big)[0]
                with self.case(original) as paths:
                    for _ in range(120):
                        data = original.copy()
                        for _ in range(rng.randrange(1, 6)):
                            data[rng.randrange(len(data))] = rng.randrange(256)
                        (paths[1] / "debug").write_bytes(data)
                        (paths[1] / "stripped").write_bytes(original)
                        run = subprocess.run([self.tools[1], "debug", "stripped", "out"], cwd=paths[1], capture_output=True, timeout=2)
                        self.assertIn(run.returncode, (0, 1), run.stderr)
                        self.assertNotIn(b"panicked", run.stderr)

    def test_real_mips_objects_and_generated_c_execution(self):
        clang = shutil.which("clang")
        linker = shutil.which("ld.lld")
        objcopy = shutil.which("llvm-objcopy")
        if not all((clang, linker, objcopy)):
            self.skipTest("requires LLVM MIPS compiler, linker and objcopy")
        with tempfile.TemporaryDirectory(prefix="mips-vdso-real-") as temporary:
            work = Path(temporary)
            assembly = work / "vdso.s"
            assembly.write_text('.set noreorder\n.text\n.globl __vdso_sigreturn\n'
                                '.globl __vdso_rt_sigreturn\n__vdso_sigreturn:\njr $31\nnop\n'
                                '__vdso_rt_sigreturn:\njr $31\nnop\n')
            (work / "linux").mkdir()
            (work / "asm").mkdir()
            (work / "linux/linkage.h").write_text('#define __page_aligned_data __attribute__((aligned(4096)))\n')
            (work / "linux/mm.h").write_text('''#define PAGE_SIZE 4096
#define PAGE_ALIGN(x) (((x) + 4095) & ~4095)
struct page;
struct vm_area_struct { unsigned long vm_start; };
struct vm_special_mapping { const char *name; struct page **pages;
 int (*mremap)(const struct vm_special_mapping *, struct vm_area_struct *); };
struct mm_struct { struct { void *vdso; } context; };
struct task_struct { struct mm_struct *mm; };
static struct mm_struct test_mm;
static struct task_struct test_task = { &test_mm };
static struct task_struct *current = &test_task;
''')
            (work / "asm/vdso.h").write_text('''struct mips_vdso_image {
 unsigned char *data; unsigned long size; struct vm_special_mapping mapping;
 unsigned long off_sigreturn, off_rt_sigreturn;
};
''')
            for target, flags in (("mips-linux-gnu", []), ("mipsel-linux-gnu", []),
                                  ("mips64-linux-gnuabi64", []), ("mips64el-linux-gnuabi64", []),
                                  ("mips64-linux-gnuabi64", ["-mabi=n32"]),
                                  ("mips64el-linux-gnuabi64", ["-mabi=n32"])):
                with self.subTest(target=target, flags=flags):
                    obj, image, stripped = (work / name for name in ("vdso.o", "debug", "stripped"))
                    commands = ([clang, "--target=" + target, *flags, "-c", assembly, "-o", obj],
                                [linker, "-shared", "--hash-style=sysv", obj, "-o", image],
                                [objcopy, "--rename-section", ".MIPS.abiflags=.mips_abiflags", image],
                                [objcopy, "-S", image, stripped])
                    for command in commands:
                        result = subprocess.run(command, capture_output=True)
                        self.assertEqual(result.returncode, 0, result.stderr)
                    with self.case(image.read_bytes(), stripped.read_bytes()) as paths:
                        result = self.compare(paths)
                        self.assertEqual(result[0], 0, result[2])
                        generated = result[3][b"out"][0]
                        (work / "image.c").write_bytes(generated)
                        length = len(result[3][b"stripped"][0])
                        (work / "main.c").write_text('#include <stdio.h>\n#include "image.c"\n'
                                                     'int main(void) { return fwrite(vdso_image.data, 1, '
                                                     + str(length) + ', stdout) != ' + str(length) + '; }\n')
                        compile_result = subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
                            "-O2", "-I", str(work), str(work / "main.c"), "-o", str(work / "verify")], capture_output=True)
                        self.assertEqual(compile_result.returncode, 0, compile_result.stderr)
                        executed = subprocess.run([work / "verify"], check=True, capture_output=True)
                        self.assertEqual(executed.stdout, result[3][b"stripped"][0])

    def test_kernel_vdso_build(self):
        build = os.environ.get("MIPS_VDSO_KERNEL_BUILD")
        if not build:
            self.skipTest("set MIPS_VDSO_KERNEL_BUILD to a MIPS vDSO build")
        directory = Path(build) / "arch/mips/vdso"
        images = sorted(directory.glob("*.so.dbg.raw"))
        self.assertTrue(images, "no raw vDSO images found")
        for image in images:
            prefix = image.name.removesuffix(".so.dbg.raw")
            stripped = directory / (prefix + ".so.raw")
            name = prefix.removeprefix("vdso").lstrip("-")
            with self.subTest(prefix=prefix), self.case(image.read_bytes(), stripped.read_bytes()) as paths:
                result = self.compare(paths, ("debug", "stripped", "out", name))
                self.assertEqual(result[0], 0)
                self.assertEqual(result[3][b"out"][0], (directory / (prefix + "-image.c")).read_bytes())
                self.assertEqual(result[3][b"debug"][0], (directory / (prefix + ".so.dbg")).read_bytes())
                self.assertEqual(result[3][b"stripped"][0], (directory / (prefix + ".so")).read_bytes())


if __name__ == "__main__":
    unittest.main()
