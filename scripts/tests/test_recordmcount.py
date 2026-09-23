#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Ftrace table and instruction-rewrite parity against the original C tool."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest

from modpost_test_support import Elf, ROOT, build


def append_aligned(data, explicit=True):
    """Avoid C's uninitialized allocation gap between names and new headers."""
    order = "<" if data[5] == 1 else ">"
    word = 8 if data[4] == 2 else 4
    shoff = struct.unpack_from(order + ("Q" if word == 8 else "I"), data, 40 if word == 8 else 32)[0]
    shsize, _, strings = struct.unpack_from(order + "HHH", data, 58 if word == 8 else 46)
    if strings == 0xffff:
        strings = struct.unpack_from(order + "I", data, shoff + (40 if word == 8 else 24))[0]
    size = struct.unpack_from(order + ("Q" if word == 8 else "I"), data,
                              shoff + strings * shsize + (32 if word == 8 else 20))[0]
    names = len(b".rela__mcount_loc\0" if explicit else b".rel__mcount_loc\0")
    return data + bytes((-len(data) - size - names) % word)


class RecordmcountTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="recordmcount-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.c = cls.work / "recordmcount-c"
        cls.rust = cls.work / "recordmcount-rust"
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        build(cls.cc + ["-O2", str(ROOT / "scripts/recordmcount.c"), "-o", str(cls.c)])
        build(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + ["--edition=2021", "-O", "-Dwarnings",
              "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
              str(ROOT / "scripts/recordmcount.rs"), "-o", str(cls.rust)])

    def compare(self, files, args=None, status=0, env=None):
        results = []
        with tempfile.TemporaryDirectory(prefix="recordmcount-parity-") as work:
            for executable, name in ((self.c, "c"), (self.rust, "rust")):
                directory = Path(work) / name
                directory.mkdir()
                for name, data in files.items():
                    path = directory / os.fsdecode(name)
                    path.parent.mkdir(parents=True, exist_ok=True)
                    if data is None:
                        path.mkdir()
                    else:
                        path.write_bytes(data)
                        path.chmod(0o751)
                arguments = args if args is not None else list(files)
                result = subprocess.run([os.fsencode(executable), *map(os.fsencode, arguments)], cwd=directory,
                                        capture_output=True, env=dict(os.environ, **(env or {})))
                stderr = result.stderr.replace(os.fsencode(executable), b"recordmcount")
                tree = {str(path.relative_to(directory)): (path.read_bytes(), path.stat().st_mode & 0o7777)
                        for path in directory.rglob("*") if path.is_file()}
                results.append((result.returncode, result.stdout, stderr, tree))
        self.assertEqual(results[0][:3], results[1][:3])
        self.assertEqual(results[0][3].keys(), results[1][3].keys())
        for path in results[0][3]:
            self.assertEqual(results[0][3][path], results[1][3][path], path)
        self.assertEqual(results[0][0], status, results[0][2])
        return results[1]

    def fixture(self, bits=64, endian="little", machine=None, explicit=None, name=None,
                offsets=(1, 17), section_name=".text", base_value=0, base_kind=3,
                base_binding=0, extra_names=(), relocation_kind=None, extended=False):
        machine = machine or (62 if bits == 64 else 3)
        explicit = bits == 64 if explicit is None else explicit
        elf = Elf(bits, endian, machine)
        section = elf.section(section_name, bytes(80), flags=6)
        elf.symbol("base", section, value=base_value, kind=base_kind, binding=base_binding)
        name = name or ("mcount" if machine in (3, 40, 42, 62) else "_mcount")
        symbol = elf.symbol(name)
        default_kind = 28 if machine == 40 else 283 if machine == 183 else 20 if machine == 258 else 1
        for offset in offsets:
            elf.relocation(section, symbol, offset, kind=relocation_kind or default_kind, addend=0 if explicit else None)
        for extra_name in extra_names:
            other = elf.symbol(extra_name)
            elf.relocation(section, other, 24, kind=default_kind, addend=0 if explicit else None)
        return append_aligned(elf.build(extended=extended), explicit)

    def test_all_architectures_classes_byte_orders_and_relocation_styles(self):
        architectures = {3: (32,), 8: (32, 64), 20: (32,), 21: (64,), 22: (32, 64),
                         40: (32,), 42: (32,), 43: (64,), 62: (64,), 183: (64,), 258: (32, 64)}
        for machine, widths in architectures.items():
            for bits in widths:
                for endian in ("little", "big"):
                    for explicit in (False, True):
                        with self.subTest(machine=machine, bits=bits, endian=endian, explicit=explicit):
                            self.compare({"fixture.o": self.fixture(bits, endian, machine, explicit)})

    def test_recorded_section_names_and_symbol_names(self):
        sections = (".text", ".text.hot", ".text.unlikely", ".textual", ".init.text", ".ref.text",
                    ".sched.text", ".spinlock.text", ".irqentry.text", ".softirqentry.text",
                    ".kprobes.text", ".cpuidle.text", ".entry.text", ".noinstr.text")
        for section in sections:
            for name in ("mcount", ".mcount", "__fentry__", "_mcount", "unrelated"):
                self.compare({"fixture.o": self.fixture(section_name=section, name=name)})
        self.compare({"fixture.o": self.fixture(32, machine=40, name="__gnu_mcount_nc")})
        self.compare({"fixture.o": self.fixture(machine=21, name="._mcount")})

    def test_first_mcount_symbol_is_cached_per_section(self):
        for first in ("mcount", "__fentry__", "other"):
            self.compare({"fixture.o": self.fixture(name=first, extra_names=("mcount", "__fentry__", "mcount"))})

    def test_section_base_values_bindings_and_arm_function_exclusion(self):
        for bits in (32, 64):
            for value in (0, 4, 64, 0xffffffff):
                for binding in (0, 1, 2):
                    self.compare({"fixture.o": self.fixture(bits, base_value=value, base_binding=binding)},
                                 status=1 if binding == 2 else 0)
        self.compare({"fixture.o": self.fixture(32, machine=40, base_kind=2)}, status=1)

    def test_mips_pairs_and_architecture_specific_relocation_filters(self):
        for bits in (32, 64):
            for endian in ("little", "big"):
                self.compare({"fixture.o": self.fixture(bits, endian, 8, offsets=(0,4,16,20,36,40,44))})
        for machine, bits, kinds in ((40,32,(1,10,28,2,29)), (183,64,(283,257,0)), (258,32,(20,29,1)), (258,64,(20,29,2))):
            for kind in kinds:
                self.compare({"fixture.o": self.fixture(bits, machine=machine, relocation_kind=kind)})

    def test_x86_ignored_calls_are_nopped_and_relocations_disabled(self):
        for bits in (32, 64):
            for explicit in (False, True):
                elf = Elf(bits)
                traced = elf.section(".text", bytes(64), flags=6)
                elf.symbol("base", traced, kind=3, binding=0)
                mcount = elf.symbol("mcount")
                elf.relocation(traced, mcount, 1, addend=0 if explicit else None)
                ignored = elf.section(".noinstr.text", b"\xe8\0\0\0\0" * 2 + b"\xe8\x01\0\0\0" + bytes(16), flags=6)
                for offset in (1,6,11):
                    elf.relocation(ignored, mcount, offset, addend=-4 if explicit else None)
                self.compare({"fixture.o": append_aligned(elf.build(), explicit)}, ["-w", "fixture.o"])

    def test_arm_arm64_and_thumb_nops(self):
        for endian in ("little", "big"):
            patterns = ((bytes.fromhex("04e02de5 feffffeb"), 4, 28), (bytes.fromhex("00000000 feffffeb"), 4, 1),
                        (bytes.fromhex("00b5 fff7 feff"), 2, 10), (bytes(8), 4, 28))
            for data, offset, kind in patterns:
                if endian == "big":
                    width = 2 if kind == 10 else 4
                    data = b"".join(data[index:index+width][::-1] for index in range(0, len(data), width))
                elf = Elf(32, endian, 40)
                traced = elf.section(".text", bytes(16), flags=6)
                elf.symbol("base", traced, kind=3, binding=0)
                mcount = elf.symbol("__gnu_mcount_nc")
                elf.relocation(traced, mcount, 0, kind=28, addend=None)
                ignored = elf.section(".entry.text", data, flags=6)
                elf.relocation(ignored, mcount, offset, kind=kind, addend=None)
                self.compare({"fixture.o": append_aligned(elf.build(), False)}, ["-w", "fixture.o"])
            elf = Elf(64, endian, 183)
            traced = elf.section(".text", bytes(16), flags=6)
            elf.symbol("base", traced, kind=3, binding=0)
            mcount = elf.symbol("_mcount")
            elf.relocation(traced, mcount, 0, kind=283)
            ignored = elf.section(".entry.text", b"\0\0\0\x94" + bytes(4), flags=6)
            elf.relocation(ignored, mcount, 0, kind=283)
            self.compare({"fixture.o": append_aligned(elf.build())}, ["-w", "fixture.o"])

    def test_ignored_sections_without_recorded_relocations_do_nothing(self):
        self.compare({"fixture.o": self.fixture(section_name=".noinstr.text")}, ["-w", "fixture.o"])
        elf = Elf()
        traced = elf.section(".text", bytes(16), flags=6)
        elf.symbol("base", traced, kind=3, binding=0)
        unrelated = elf.symbol("other")
        elf.relocation(traced, unrelated, 1)
        mcount = elf.symbol("mcount")
        ignored = elf.section(".ignored", b"\xe8\0\0\0\0", flags=6)
        elf.relocation(ignored, mcount, 1)
        self.compare({"fixture.o": append_aligned(elf.build())}, ["-w", "fixture.o"])

    def test_reprocessing_preserves_bytes_and_reports_existing_table(self):
        result = self.compare({"fixture.o": self.fixture()})
        changed = result[3]["fixture.o"][0]
        again = self.compare({"fixture.o": changed})
        self.assertEqual(again[3]["fixture.o"][0], changed)
        self.assertIn(b"warning: __mcount_loc already exists", again[2])

    def test_extended_indices_and_section_count_transition(self):
        for bits in (32, 64):
            self.compare({"fixture.o": self.fixture(bits, extended=True)})
        elf = Elf()
        text = elf.section(".text", bytes(16), flags=6)
        elf.symbol("base", text, kind=3, binding=0)
        mcount = elf.symbol("mcount")
        elf.relocation(text, mcount, 1)
        while len(elf.sections) < 0xff00 - 1 - 4:
            elf.section("", flags=0, kind=8)
        self.compare({"fixture.o": append_aligned(elf.build())})

    def test_non_utf8_paths_names_and_cli_errors(self):
        self.compare({b"obj-\xff.o": self.fixture()}, [b"obj-\xff.o"])
        data = self.fixture(section_name=".text.bytes")
        self.compare({"fixture.o": data.replace(b".text.bytes", b".text.byt\xffs")})
        for arguments in ([], ["-h"], ["--help"], ["-w"], ["-wwz"], ["missing.o"],
                          ["/missing/ftrace.o"], ["directory"], ["--", "missing.o"]):
            self.compare({"directory": None}, arguments, status=1 if "missing.o" in arguments or arguments == ["directory"] else 0)
        self.compare({"fixture.o": self.fixture(), "fixture.o.rc": None}, ["fixture.o"], status=1)
        self.compare({"fixture.o": self.fixture()}, ["fixture.o", "missing.o"], status=1)
        self.compare({"fixture.o": self.fixture()}, ["fixture.o", "-w"], status=1, env={"POSIXLY_CORRECT": "1"})

    def test_validation_errors_leave_input_untouched(self):
        original = self.fixture()
        for offset, byte in ((0,0), (5,0), (6,2), (16,2), (18,0), (4,0), (52,0), (58,0)):
            data = bytearray(original)
            data[offset] = byte
            self.compare({"fixture.o": bytes(data)}, status=1)
        with tempfile.TemporaryDirectory(prefix="recordmcount-malformed-") as work:
            path = Path(work) / "fixture.o"
            for data in (b"", original[:8], original[:64], original[:100]):
                path.write_bytes(data)
                result = subprocess.run([str(self.rust), str(path)], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertNotIn(b"panicked", result.stderr)
                self.assertEqual(path.read_bytes(), data)

    def test_real_compiler_objects_and_linked_table_relocations(self):
        source = b"int function(int x) { return x + 1; }\nint other(void) { return function(3); }\n"
        for bits in (32, 64):
            path = self.work / f"compiled-{bits}.o"
            build(self.cc + [f"-m{bits}", "-fno-pic", "-fno-pie", "-pg", "-O2", "-ffunction-sections", "-c", "-x", "c", "-", "-o", str(path)], input=source)
            result = self.compare({"fixture.o": append_aligned(path.read_bytes(), bits == 64)})
            path.write_bytes(result[3]["fixture.o"][0])
            linked = self.work / f"linked-{bits}.o"
            build(self.cc + [f"-m{bits}", "-nostdlib", "-no-pie", "-Wl,-r", str(path), "-o", str(linked)])
            self.assertIn(b"__mcount_loc", linked.read_bytes())

    def test_randomized_relocations_and_section_bases(self):
        randomizer = random.Random(0x464F554E54)
        for trial in range(80):
            bits = (32,64)[trial % 2]
            endian = ("little","big")[(trial // 2) % 2]
            explicit = bool(trial % 3)
            elf = Elf(bits, endian)
            for section_index in range(randomizer.randrange(1, 5)):
                section = elf.section(".text." + str(section_index), bytes(128), flags=6)
                elf.symbol("weak", section, kind=2, binding=2)
                elf.symbol("base", section, value=randomizer.randrange(80), kind=3, binding=randomizer.randrange(2))
                mcount = elf.symbol(randomizer.choice(("mcount", "__fentry__")))
                other = elf.symbol("other")
                for _ in range(randomizer.randrange(1, 24)):
                    elf.relocation(section, randomizer.choice((mcount, other)), randomizer.randrange(128),
                                   kind=randomizer.randrange(1, 8), addend=0 if explicit else None)
            self.compare({"fixture.o": append_aligned(elf.build(), explicit)})

    def test_multi_file_architecture_and_mips_state_are_isolated(self):
        # Compare with separate original-C invocations: its process-global
        # callbacks and MIPS previous-offset variable leak across argv files.
        files = {"mips-first.o": self.fixture(machine=8, offsets=(16,)),
                 "mips-second.o": self.fixture(machine=8, offsets=(20,)),
                 "x86.o": self.fixture(), "arm.o": self.fixture(32, machine=40),
                 "ppc.o": self.fixture(machine=21)}
        with tempfile.TemporaryDirectory(prefix="recordmcount-multi-") as work:
            directories = [Path(work) / name for name in ("c", "rust")]
            for directory in directories:
                directory.mkdir()
                for name, data in files.items():
                    (directory / name).write_bytes(data)
            expected_stdout, expected_stderr = b"", b""
            for name in files:
                result = subprocess.run([str(self.c), "-w", name], cwd=directories[0], capture_output=True)
                self.assertEqual(result.returncode, 0)
                expected_stdout += result.stdout
                expected_stderr += result.stderr
            actual = subprocess.run([str(self.rust), "-w", *files], cwd=directories[1], capture_output=True)
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr), (0, expected_stdout, expected_stderr))
            for name in files:
                self.assertEqual((directories[0] / name).read_bytes(), (directories[1] / name).read_bytes())

    def test_big_endian_extended_indices_are_decoded_safely(self):
        # C's find_symtab compares on-disk integers without endian conversion;
        # its ELF64 get_shnum also truncates sh_size before conversion.
        for bits in (32, 64):
            data = self.fixture(bits, "big", extended=True)
            with tempfile.TemporaryDirectory(prefix="recordmcount-extended-") as work:
                path = Path(work) / "fixture.o"
                path.write_bytes(data)
                result = subprocess.run([str(self.rust), str(path)], capture_output=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                output = path.read_bytes()
                self.assertGreater(len(output), len(data))
                self.assertIn(b"__mcount_loc\0", output[len(data):])
                again = subprocess.run([str(self.rust), str(path)], capture_output=True)
                self.assertEqual(again.returncode, 0, again.stderr)
                self.assertIn(b"__mcount_loc already exists", again.stderr)
                self.assertEqual(output, path.read_bytes())

    def test_uninitialized_c_alignment_gap_is_zero_filled(self):
        data = self.fixture()
        word = 8
        shoff = struct.unpack_from("<Q", data, 40)[0]
        strings = struct.unpack_from("<H", data, 62)[0]
        name_size = struct.unpack_from("<Q", data, shoff + strings * 64 + 32)[0]
        # Force exactly seven bytes of padding in the appended section table.
        data += b"X"
        with tempfile.TemporaryDirectory(prefix="recordmcount-padding-") as work:
            paths = [Path(work) / name for name in ("c.o", "rust.o")]
            for executable, path in zip((self.c, self.rust), paths):
                path.write_bytes(data)
                subprocess.run([str(executable), str(path)], check=True, capture_output=True)
            expected, actual = (bytearray(path.read_bytes()) for path in paths)
            gap = len(data) + name_size + len(b".rela__mcount_loc\0")
            end = (gap + word - 1) & ~(word - 1)
            self.assertEqual(actual[gap:end], bytes(end - gap))
            # C does not initialize this allocation-only gap. Every byte it
            # actually writes must still match the deterministic Rust output.
            expected[gap:end] = bytes(end - gap)
            self.assertEqual(expected, actual)

    def test_late_errors_do_not_commit_instruction_edits(self):
        elf = Elf()
        ignored = elf.section(".noinstr.text", b"\xe8\0\0\0\0", flags=6)
        mcount = elf.symbol("mcount")
        elf.relocation(ignored, mcount, 1)
        traced = elf.section(".text", bytes(16), flags=6)
        elf.relocation(traced, mcount, 1)
        self.compare({"fixture.o": append_aligned(elf.build())}, ["-w", "fixture.o"], status=1)

    @unittest.skipUnless(os.environ.get("RECORDMCOUNT_KERNEL_BUILD"), "set RECORDMCOUNT_KERNEL_BUILD for a real kernel compilation unit")
    def test_existing_kernel_compilation_unit_with_instrumentation(self):
        directory = Path(os.environ["RECORDMCOUNT_KERNEL_BUILD"])
        object_name = Path(os.environ.get("RECORDMCOUNT_KERNEL_OBJECT", "lib/test_hexdump.o"))
        command_file = directory / object_name.with_name("." + object_name.name + ".cmd")
        saved = command_file.read_text().splitlines()[0].split(":=", 1)[1]
        # Only re-run the compiler, not objtool/genksyms shell postprocessing.
        lexer = shlex.shlex(saved, posix=True, punctuation_chars=";&|")
        lexer.whitespace_split = True
        command = []
        for token in lexer:
            if token in (";", "&&", "||", "|"):
                break
            command.append(token)
        self.assertIn("-c", command)
        self.assertIn("-o", command)
        with tempfile.TemporaryDirectory(prefix="recordmcount-kernel-") as work:
            output = Path(work) / "unit.o"
            command[command.index("-o") + 1] = str(output)
            command = ["-Wp,-MMD," + str(Path(work) / "unit.d") if argument.startswith("-Wp,-MMD,")
                       else argument for argument in command]
            command = [argument for argument in command if argument not in ("-mrecord-mcount", "-mnop-mcount")
                       and not argument.startswith("-fpatchable-function-entry=")]
            command += ["-pg", "-fno-omit-frame-pointer"]
            build(command, cwd=directory)
            data = output.read_bytes()
            self.assertNotIn(b"__mcount_loc\0", data)
            result = self.compare({"unit.o": append_aligned(data, data[4] == 2)})
            self.assertGreater(len(result[3]["unit.o"][0]), len(data))
            self.assertIn(b"__mcount_loc\0", result[3]["unit.o"][0])


if __name__ == "__main__":
    unittest.main()
