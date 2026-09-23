#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Independent C/Rust MIPS relocation outputs and complete-file mutation parity."""

import itertools
import os
from pathlib import Path
import random
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/mips/boot/tools"
FORMATS = tuple(itertools.product((32, 64), (False, True)))
EMITTED = {2, 4, 5, 18}
IGNORED = {0, 3, 6, 10, 28, 29, 60, 61, 248}


def records(bits, big, entries):
    """Entries are absolute address, type, symbol index, aux bytes, addend."""
    order = ">" if big else "<"
    result = bytearray()
    for address, kind, symbol, auxiliary, addend in entries:
        if bits == 32:
            result += struct.pack(order + "II", address & 0xffffffff, symbol << 8 | kind)
        else:
            # MIPS64 r_info is not an ordinary endian-encoded u64: r_sym is
            # one target-endian word, followed by ssym/type3/type2/type bytes.
            result += struct.pack(order + "QI", address & 0xffffffffffffffff, symbol)
            result += bytes((*auxiliary, kind))
            result += struct.pack(order + "q", addend)
    return result


def fixture(bits=64, big=False, entries=None, symbols=None, reserved=128,
            text_name=b".text", reserved_name=b".data.reloc", text_flags=6,
            base=None, symbol_tail=b"", relocation_tail=b"", extra=()):
    order = ">" if big else "<"
    wide = bits == 64
    ehsize, phsize, shsize, symsize, relsize = (64, 56, 64, 24, 24) if wide else (52, 32, 40, 16, 8)
    if base is None:
        base = 0xffffffff80000000 if wide else 0x80000000
    if entries is None:
        entries = [(base + 8, 2, 1, (0, 0, 0), 0)]
    if symbols is None:
        symbols = [(b"", 0, 0, 0), (b"target", base + 16, 1, 4)]
    strings = bytearray(b"\0")
    symtab = bytearray()
    for name, value, binding, section in symbols:
        offset = len(strings) if name else 0
        if name:
            strings += name + b"\0"
        value &= 2**bits - 1
        symtab += struct.pack(order + "IBBHQQ", offset, binding << 4, 0, section, value, 0) if wide else \
            struct.pack(order + "IIIBBH", offset, value, 0, binding << 4, 0, section)
    sections = [
        dict(name=b"", kind=0, data=b""),
        dict(name=b".shstrtab", kind=3, data=b""),
        dict(name=b".strtab", kind=3, data=strings),
        dict(name=b".symtab", kind=2, data=symtab + symbol_tail, link=2, stride=symsize),
        dict(name=text_name, kind=1, flags=text_flags, address=base, data=bytes(range(64))),
        dict(name=reserved_name, kind=1, flags=3, data=b"\xa5" * reserved),
        dict(name=b".rela.text" if wide else b".rel.text", kind=4 if wide else 9,
             data=records(bits, big, entries) + relocation_tail, link=3, info=4, stride=relsize),
        dict(name=b"__ex_table", kind=1, flags=2, data=bytes(16)),
        dict(name=b".debug_info", kind=1, data=b"debug"),
    ]
    sections += [dict(section) for section in extra]
    names = bytearray(b"\0")
    for section in sections:
        section["name_offset"] = len(names) if section["name"] else 0
        if section["name"]:
            names += section["name"] + b"\0"
    sections[1]["data"] = names
    data = bytearray(ehsize)
    for section in sections:
        data += bytes(-len(data) % 8)
        section["offset"] = len(data)
        data += section["data"]
    data += bytes(-len(data) % 8)
    shoff = len(data)
    for section in sections:
        data += struct.pack(order + ("IIQQQQIIQQ" if wide else "IIIIIIIIII"),
                            section["name_offset"], section["kind"], section.get("flags", 0),
                            section.get("address", 0), section["offset"], len(section["data"]),
                            section.get("link", 0), section.get("info", 0), 8, section.get("stride", 0))
    ident = b"\x7fELF" + bytes((2 if wide else 1, 2 if big else 1, 1)) + bytes(9)
    data[:ehsize] = struct.pack(order + ("16sHHIQQQIHHHHHH" if wide else "16sHHIIIIIHHHHHH"),
                               ident, 2, 8, 1, base, 0, shoff, 0, ehsize, phsize, 0, shsize,
                               len(sections), 1)
    return data, sections, shoff


def section_field(data, index, field, value):
    wide = data[4] == 2
    order = ">" if data[5] == 2 else "<"
    shoff = struct.unpack_from(order + ("Q" if wide else "I"), data, 40 if wide else 32)[0]
    offsets = {"name": (0, 0, 4), "kind": (4, 4, 4), "offset": (24, 16, 0),
               "size": (32, 20, 0), "link": (40, 24, 4), "info": (44, 28, 4),
               "stride": (56, 36, 0)}
    offset64, offset32, width = offsets[field]
    width = width or (8 if wide else 4)
    struct.pack_into(order + ("Q" if width == 8 else "I"), data,
                     shoff + index * (64 if wide else 40) + (offset64 if wide else offset32), value)


class MipsRelocsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="mips-relocs-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.tools = [cls.work / "relocs-c", cls.work / "relocs-rust"]
        commands = [shlex.split(os.environ.get("HOSTCC", "cc")) + ["-O2", "-Wall",
            *[str(SOURCE / (name + ".c")) for name in ("relocs_main", "relocs_32", "relocs_64")],
            "-o", str(cls.tools[0])],
            shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + ["--edition=2021", "-O", "-Dwarnings",
            "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms", str(SOURCE / "relocs.rs"),
            "-o", str(cls.tools[1])]]
        for command in commands:
            result = subprocess.run(command, capture_output=True)
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)

    def compare(self, data=None, options=(), filename=b"vmlinux", args=None, same_file=True):
        path = self.work / os.fsdecode(filename)
        results = []
        for tool in self.tools:
            if data is not None:
                path.write_bytes(data)
                path.chmod(0o751)
                inode = path.stat().st_ino
            argv = [*options, filename] if args is None else args
            run = subprocess.run([tool, *argv], cwd=self.work, capture_output=True, timeout=20)
            content = path.read_bytes() if data is not None else None
            if data is not None:
                self.assertEqual(path.stat().st_ino, inode)
                self.assertEqual(path.stat().st_mode & 0o777, 0o751)
            results.append((run.returncode, run.stdout, run.stderr, content))
        self.assertEqual(results[0][:3], results[1][:3])
        if same_file:
            self.assertEqual(results[0][3], results[1][3])
        return results[1]

    def test_all_modes_precedence_and_complete_file_edits(self):
        flags = ("--reloc-info", "--text", "--bin", "--keep")
        for bits, big in FORMATS:
            data, sections, table = fixture(bits, big)
            for enabled in itertools.product((False, True), repeat=4):
                options = [flag for flag, active in zip(flags, enabled) if active]
                with self.subTest(bits=bits, big=big, options=options):
                    result = self.compare(data, options)
                    self.assertEqual(result[0], 0)
                    if enabled[0] or enabled[3] and (enabled[1] or enabled[2]):
                        self.assertEqual(result[3], data)
                    if not any(enabled[:3]):
                        self.assertEqual(result[1], b"")
                        encoded = struct.pack((">" if big else "<") + "II", 0x02000002, 0)
                        offset = sections[5]["offset"]
                        self.assertEqual(result[3][offset:offset + 8], encoded)
                        self.assertEqual(result[3][offset + 8:offset + 128], b"\xa5" * 120)
            self.compare(data, args=[b"vmlinux", b"--bin", b"--text", b"--keep", b"--text"])

    def test_all_256_relocation_types_in_each_format(self):
        for bits, big in FORMATS:
            base = 0x80000000
            for kind in range(256):
                with self.subTest(bits=bits, big=big, kind=kind):
                    data = fixture(bits, big, base=base, entries=[(base + 12, kind, 1, (7, 5, 18), -9)])[0]
                    result = self.compare(data, ("--text",))
                    self.assertEqual(result[0], 0 if kind in EMITTED | IGNORED else 1)
                    if kind not in EMITTED | IGNORED:
                        self.assertEqual(result[3], data)
                        self.assertEqual(result[1], b"")
            entries = [(base + index * 4, kind, 1, (0xff, 0x80, 1), -index)
                       for index, kind in enumerate(range(256))]
            result = self.compare(fixture(bits, big, entries=entries, base=base)[0], ("--reloc-info",))
            self.assertEqual(result[0], 0)
            self.assertEqual(len(result[1].splitlines()), 257)

    def test_empty_tables_ignored_records_and_missing_required_sections(self):
        for bits, big in FORMATS:
            for entries in ([], [(0x80000000 + 4 * n, kind, 1, (0, 0, 0), 0)
                                for n, kind in enumerate(sorted(IGNORED))]):
                data = fixture(bits, big, entries=entries, base=0x80000000, reserved=0)[0]
                for options in ((), ("--text",), ("--bin",), ("--reloc-info",)):
                    self.assertEqual(self.compare(data, options)[0], 0)
            for kwargs, diagnostic in (({"text_name": b"not-text"}, b"Could not find .text section\n"),
                                       ({"reserved_name": b"not-reserved"}, b"Could not find relocation section\n")):
                result = self.compare(fixture(bits, big, **kwargs)[0])
                self.assertEqual(result[:3], (1, b"", diagnostic))
            self.assertEqual(self.compare(fixture(bits, big, reserved_name=b"missing")[0],
                                          ("--reloc-info",))[0], 0)

    def test_weak_zero_crc_prefix_and_info_does_not_apply_symbol_filters(self):
        for bits, big in FORMATS:
            for name, value, binding, skipped in ((b"target", 0, 2, True), (b"target", 1, 2, False),
                    (b"target", 0, 1, False), (b"__crc_", 1, 1, True), (b"__crc_\xff", 1, 0, True),
                    (b"__crc", 1, 1, False), (b"prefix__crc_name", 1, 1, False)):
                symbols = [(b"", 0, 0, 0), (name, value, binding, 4)]
                data = fixture(bits, big, symbols=symbols,
                               entries=[(0, 255, 1, (1, 2, 3), 0)])[0]
                with self.subTest(bits=bits, big=big, symbol=symbols[1]):
                    self.assertEqual(self.compare(data)[0], 0 if skipped else 1)
                    info = self.compare(data, ("--reloc-info",))
                    self.assertEqual(info[0], 0)
                    self.assertIn(name, info[1])

    def test_exception_nonalloc_wrong_kind_filters_and_order(self):
        for bits, big in FORMATS:
            relkind = 9 if bits == 32 else 4
            payload = records(bits, big, [(0, 255, 0xffffff, (0, 0, 0), 0)])
            extra = [dict(name=b".rel.ex", kind=relkind, data=payload, info=7, link=0xffffffff),
                     dict(name=b".rel.debug", kind=relkind, data=payload, info=8, link=0xffffffff),
                     dict(name=b".wrong-kind", kind=4 if bits == 32 else 9, data=payload,
                          info=0xffffffff, link=0xffffffff)]
            for options in ((), ("--text",), ("--reloc-info",)):
                self.assertEqual(self.compare(fixture(bits, big, extra=extra)[0], options)[0], 0)
            self.assertEqual(self.compare(fixture(bits, big, text_flags=0,
                entries=[(0, 255, 0xffffff, (0, 0, 0), 0)])[0])[0], 0)

    def test_offsets_truncation_unaligned_and_maximum_image_size(self):
        for bits, big in FORMATS:
            for offset in (0, 1, 2, 3, 4, 0x3fffffc, 0x3ffffff, 0x4000000,
                           0xffffffff, 0x100000000, 0x100000007):
                data = fixture(bits, big, base=0x80000000,
                               entries=[(0x80000000 + offset, 2, 1, (0, 0, 0), 0)])[0]
                result = self.compare(data, ("--bin",))
                self.assertEqual(result[0], int((offset & 0xffffffff) >> 2 > 0xffffff))
                if result[0] == 0:
                    self.assertEqual(result[1], struct.pack((">" if big else "<") + "II",
                                     0x02000000 | (offset & 0xffffffff) >> 2, 0))

    def test_fixed_strides_and_partial_record_tails(self):
        for bits, big in FORMATS:
            for stride in (0, 1, 8, 16, 24, 80, 65536):
                data = fixture(bits, big)[0]
                section_field(data, 3, "stride", stride)
                section_field(data, 6, "stride", stride)
                self.assertEqual(self.compare(data, ("--text",))[0], 0)
            for size in (1, 7, 15):
                tail = bytes(range(size))
                if bits == 32:
                    tail = tail[:7]
                self.assertEqual(self.compare(fixture(bits, big, symbol_tail=tail,
                                                      relocation_tail=tail)[0], ("--text",))[0], 0)

    def test_symbol_fallback_names_reserved_indices_and_raw_bytes(self):
        for bits, big in FORMATS:
            for section in (0, 4, 7, 0xfff1, 0xfff2, 0xffff):
                data = fixture(bits, big, symbols=[(b"", 0, 0, 0), (b"", 1, 1, section)])[0]
                self.assertEqual(self.compare(data, ("--reloc-info",), filename=b"input-\xff")[0], 0)
            name = b"long-symbol-" + b"x" * 100 + b"\xff"
            self.compare(fixture(bits, big, symbols=[(b"", 0, 0, 0), (name, 1, 1, 4)])[0],
                         ("--reloc-info",), filename=b"input-\xfe")

    def test_multiple_tables_high_symbol_indices_and_ordering(self):
        for bits, big in FORMATS:
            symbols = [(b"", 0, 0, 0)] + [(f"symbol_{i}".encode(), i, 1, 4) for i in range(300)]
            base = 0x80000000
            entries = [(base + n * 4, kind, 300 - n, (255, 0, 128), 0)
                       for n, kind in enumerate((2, 18, 5, 4))]
            extra = [dict(name=b"other-allocated-\xff", kind=1, flags=2, data=b""),
                     dict(name=b".second-rel", kind=9 if bits == 32 else 4, link=3, info=9,
                          data=records(bits, big, [(base + 52, 5, 259, (0, 0, 0), -1)]))]
            for options in ((), ("--text",), ("--reloc-info",)):
                result = self.compare(fixture(bits, big, base=base, symbols=symbols,
                                              entries=entries, extra=extra)[0], options)
                self.assertEqual(result[0], 0)
                if options == ("--reloc-info",):
                    self.assertIn(b"symbol_258", result[1])
                    self.assertIn(b"other-allocated-\xff", result[1])

    def test_unsigned_reserved_capacity_and_page_suggestion(self):
        for bits, big in FORMATS:
            for size in (0, 0x7fffffff):
                data = fixture(bits, big, entries=[])[0]
                section_field(data, 5, "size", size)
                self.compare(data, ("--text", "--keep"))
            for size in (0x80000000, 0xffffffff):
                # C narrows the declared capacity to signed int and reports
                # false overflow. Rust intentionally retains a checked size.
                data = fixture(bits, big, entries=[])[0]
                section_field(data, 5, "size", size)
                path = self.work / "large-capacity"
                results = []
                for tool in self.tools:
                    path.write_bytes(data)
                    run = subprocess.run([tool, "--text", "--keep", path], capture_output=True)
                    results.append(run)
                    self.assertEqual(path.read_bytes(), data)
                self.assertEqual([run.returncode for run in results], [1, 0])
                self.assertEqual(results[0].stdout, results[1].stdout)
                self.assertIn(b"Relocations overflow", results[0].stderr)
                self.assertEqual(results[1].stderr, b"")
            entries = [(0x80000000 + n * 4, 2, 1, (0, 0, 0), 0) for n in range(1023)]
            data = fixture(bits, big, entries=entries, base=0x80000000, reserved=4095)[0]
            result = self.compare(data, ("--text",))
            self.assertEqual(result[0], 1)
            self.assertIn(b"at least 0x00002000", result[2])

    def test_invalid_target_is_rejected_when_no_exception_table(self):
        for bits, big in FORMATS:
            data = fixture(bits, big)[0]
            section_field(data, 7, "name", 0)  # No __ex_table means C extab_index=-1.
            section_field(data, 6, "info", 0xffffffff)
            section_field(data, 6, "link", 0xffffffff)
            # C accidentally treats this invalid unsigned index as its -1
            # exception-table sentinel. Rust rejects the malformed reference.
            path = self.work / "bad-target"
            for tool, status in zip(self.tools, (0, 1)):
                path.write_bytes(data)
                run = subprocess.run([tool, "--text", "--keep", path], capture_output=True)
                self.assertEqual(run.returncode, status)
                self.assertEqual(path.read_bytes(), data)
                if status:
                    self.assertIn(b"invalid ELF section index", run.stderr)

    def test_overflow_stdout_parity_and_non_destructive_default(self):
        for bits, big in FORMATS:
            for reserved in (0, 1, 4, 7):
                data = fixture(bits, big, reserved=reserved)[0]
                for options in (("--text",), ("--bin",), ("--text", "--bin", "--keep")):
                    result = self.compare(data, options)
                    self.assertEqual(result[0], 1)
                    self.assertEqual(result[3], data)
                    self.assertIn(b"CONFIG_RELOCATION_TABLE_SIZE to at least 0x00001000", result[2])
                # C writes past its reserved section before noticing overflow;
                # Rust intentionally validates capacity before any file edit.
                result = self.compare(data, same_file=False)
                self.assertEqual(result[0], 1)
                self.assertEqual(result[3], data)

    def test_usage_paths_and_header_validation(self):
        for args in ([], [b"--text"], [b"--help"], [b"--"], [b"-"],
                     [b"vmlinux", b"second"], [b"missing-\xff"], [b"."]):
            self.assertEqual(self.compare(args=args)[0], 1)
        for bits, big in FORMATS:
            data = fixture(bits, big)[0]
            order = ">" if big else "<"
            for length in (0, 1, 15, 16, 30, 51, *([52, 63] if bits == 64 else [])):
                self.assertEqual(self.compare(data[:length])[0], 1)
            for offset, width, value in ((0, 1, 0), (4, 1, 0), (5, 1, 0), (6, 1, 0),
                    (16, 2, 1), (18, 2, 0), (20, 4, 0),
                    (52 if bits == 64 else 40, 2, 0), (54 if bits == 64 else 42, 2, 0),
                    (58 if bits == 64 else 46, 2, 0), (62 if bits == 64 else 50, 2, 0xfffe)):
                changed = data.copy()
                struct.pack_into(order + {1: "B", 2: "H", 4: "I"}[width], changed, offset, value)
                with self.subTest(bits=bits, big=big, offset=offset):
                    self.assertEqual(self.compare(changed)[0], 1)
            struct.pack_into(order + "H", data, 16, 3)
            self.assertEqual(self.compare(data)[0], 0)

    def test_long_mixed_relocation_stream_preserves_order_and_duplicates(self):
        rng = random.Random(0x64c0)
        stream = [(rng.randrange(0x1000000) * 4, rng.choice(sorted(EMITTED | IGNORED)))
                  for _ in range(100001)]
        for bits, big in FORMATS:
            base = 0x80000000
            entries = [(base + offset, kind, 1, (17, 99, 33), -index)
                       for index, (offset, kind) in enumerate(stream)]
            data = fixture(bits, big, entries=entries, base=base, reserved=500000)[0]
            result = self.compare(data, ("--bin", "--keep"))
            expected = [kind << 24 | offset >> 2 for offset, kind in stream if kind in EMITTED] + [0]
            self.assertEqual(result[:3], (0, struct.pack((">" if big else "<") + "I" * len(expected),
                                                        *expected), b""))
            self.assertEqual(result[3], data)

    def test_malformed_bounds_indices_names_and_random_mutations(self):
        rng = random.Random(0xbad64)
        for bits, big in FORMATS:
            original, sections, shoff = fixture(bits, big)
            cases = [original[:size] for size in (52 if bits == 32 else 64, shoff, len(original) - 1)]
            for index, field, value in ((3, "offset", 2**bits - 1), (6, "offset", 2**bits - 1),
                    (6, "link", 0xffffffff), (6, "info", 0xffffffff), (3, "link", 0xffffffff),
                    (4, "name", 0xffffffff), (5, "offset", 2**bits - 1),
                    (5, "offset", shoff + (64 if bits == 64 else 40) * 6 + (32 if bits == 64 else 20))):
                data = original.copy()
                section_field(data, index, field, value)
                cases.append(data)
            for _ in range(150):
                data = original.copy()
                for _ in range(rng.randrange(1, 7)):
                    data[rng.randrange(len(data))] = rng.randrange(256)
                cases.append(data)
            path = self.work / "malformed"
            for data in cases:
                path.write_bytes(data)
                run = subprocess.run([self.tools[1], path], capture_output=True, timeout=3)
                self.assertIn(run.returncode, (0, 1), run.stderr)
                self.assertNotIn(b"panicked", run.stderr)
                if run.returncode:
                    self.assertEqual(path.read_bytes(), data)

    def test_stdout_failure_preserves_input(self):
        if not Path("/dev/full").exists():
            self.skipTest("requires /dev/full")
        path = self.work / "full-output"
        data = fixture()[0]
        path.write_bytes(data)
        with open("/dev/full", "wb") as output:
            run = subprocess.run([self.tools[1], "--text", path], stdout=output, stderr=subprocess.PIPE)
        self.assertEqual(run.returncode, 1)
        self.assertIn(b"Cannot write output", run.stderr)
        self.assertEqual(path.read_bytes(), data)

    def test_actual_kernel_build(self):
        """Opt in to a retained, unprocessed link image; never edit the build."""
        source = os.environ.get("MIPS_RELOCS_VMLINUX")
        build = os.environ.get("MIPS_RELOCS_KERNEL_BUILD")
        if not source and not build:
            self.skipTest("set MIPS_RELOCS_VMLINUX or MIPS_RELOCS_KERNEL_BUILD")
        if source:
            path = Path(source)
        else:
            candidates = [Path(build) / (".tmp_vmlinux" + str(number)) for number in (3, 2, 1)]
            path = next((candidate for candidate in candidates if candidate.is_file()), None)
            self.assertIsNotNone(path, "build has no retained .tmp_vmlinux{1,2,3} input")
        before = path.stat()
        data = path.read_bytes()
        self.assertEqual(data[:4], b"\x7fELF")
        self.assertEqual(struct.unpack_from((">" if data[5] == 2 else "<") + "H", data, 18)[0], 8)
        for enabled in itertools.product((False, True), repeat=4):
            options = [flag for flag, active in zip(("--reloc-info", "--text", "--bin", "--keep"),
                                                   enabled) if active]
            with self.subTest(options=options):
                result = self.compare(data, options)
                self.assertEqual(result[0], 0, result[2])
                if enabled[0]:
                    self.assertGreater(len(result[1].splitlines()), 1)
                    self.assertEqual(result[3], data)
                elif enabled[1] or enabled[2]:
                    self.assertTrue(result[1])
                    if enabled[3]:
                        self.assertEqual(result[3], data)
                else:
                    self.assertNotEqual(result[3], data)
        after = path.stat()
        self.assertEqual((before.st_ino, before.st_size, before.st_mtime_ns, before.st_mode),
                         (after.st_ino, after.st_size, after.st_mtime_ns, after.st_mode))
        self.assertEqual(path.read_bytes(), data)

    @unittest.skipUnless(shutil.which("clang") and shutil.which("ld.lld"), "requires LLVM MIPS tools")
    def test_real_linked_mips_images(self):
        with tempfile.TemporaryDirectory(prefix="mips-relocs-link-") as tmp:
            work = Path(tmp)
            (work / "input.s").write_text(
                '.set noreorder\n.text\n.globl _start\n_start:\nnop\n.word target\n'
                'lui $2,%hi(target)\naddiu $2,$2,%lo(target)\n.data\n.globl target\n'
                'target:\n.word _start\n.section .data.reloc,"a",@progbits\n.space 4096\n')
            (work / "layout.lds").write_text(
                'SECTIONS { . = 0x80000000; .text : { *(.text) } '
                '.data.reloc : { *(.data.reloc) } .data : { *(.data) } }\n')
            for bits, big in FORMATS:
                triple = ("mips64" if bits == 64 else "mips") + ("" if big else "el") + "-linux-gnu"
                emulation = ("elf64" if bits == 64 else "elf32") + ("bt" if big else "lt") + "smip"
                for command in (["clang", "--target=" + triple, "-c", "input.s", "-o", "input.o"],
                                ["ld.lld", "-m", emulation, "-T", "layout.lds", "--emit-relocs",
                                 "input.o", "-o", "vmlinux"]):
                    run = subprocess.run(command, cwd=work, capture_output=True)
                    self.assertEqual(run.returncode, 0, run.stderr)
                data = (work / "vmlinux").read_bytes()
                self.assertEqual(data[4:6], bytes((2 if bits == 64 else 1, 2 if big else 1)))
                for options in ((), ("--text",), ("--bin",), ("--reloc-info",), ("--keep",)):
                    self.assertEqual(self.compare(data, options)[0], 0)


if __name__ == "__main__":
    unittest.main()
