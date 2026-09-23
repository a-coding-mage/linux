#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Independent ABI, protected-memory and constant-time hexdump audit."""

import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
RUST_WRAPPER = r'''
//! Independent no-std ABI audit wrapper, using the public packed C layout.
#![no_std]
#[cfg(all(CONFIG_PRINTK, CONFIG_RUST))]
extern crate self as kernel;
#[allow(missing_docs, non_camel_case_types)]
pub mod bindings {
    #[cfg(native_char)]
    use kernel_ffi::{c_char, c_int};
    #[cfg(not(native_char))]
    use core::ffi::c_int;
    #[cfg(not(native_char))]
    type c_char = i8;
    #[repr(C, packed)]
    pub struct pi_entry {
        pub fmt: *const c_char,
        pub func: *const c_char,
        pub file: *const c_char,
        pub line: u32,
        pub level: *const c_char,
        pub subsys_fmt_prefix: *const c_char,
    }
    unsafe extern "C" { pub fn _printk(format: *const c_char, ...) -> c_int; }
}
#[path = "@SOURCE@"]
mod production;
pub use production::*;
'''

GUARD_RUNNER = r'''
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

static unsigned char *end_page(void) {
    size_t page = sysconf(_SC_PAGESIZE);
    void *memory = mmap(NULL, page * 3, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (memory == MAP_FAILED || mprotect((char *)memory + page, page, PROT_READ | PROT_WRITE)) abort();
    memset((char *)memory + page, 0xa5, page);
    return (unsigned char *)memory + page * 2;
}

int main(int argc, char **argv) {
    if (argc != 4) return 2;
    void *library = dlopen(argv[1], RTLD_NOW);
    if (!library) { fputs(dlerror(), stderr); return 2; }
    char name[128];
    #define LOOKUP(type, var, symbol) \
        snprintf(name, sizeof(name), "%s%s", argv[2], symbol); \
        type var = (void *)dlsym(library, name); if (!var) { fputs(name, stderr); return 2; }
    typedef int (*decode_type)(unsigned char *, const char *, size_t);
    typedef char *(*encode_type)(char *, const void *, size_t);
    typedef int (*format_type)(const void *, size_t, int, int, char *, size_t, bool);
    typedef void (*print_type)(const char *, const char *, int, int, int, const void *, size_t, bool);
    LOOKUP(decode_type, decode, "hex2bin");
    LOOKUP(encode_type, encode, "bin2hex");
    LOOKUP(format_type, format, "hex_dump_to_buffer");
    LOOKUP(print_type, print, "print_hex_dump");
    unsigned char *source = end_page(), *output = end_page();
    int test = atoi(argv[3]), result = 0;
    size_t capacity = 0;
    switch (test) {
    case 0:
        printf("%d %d %d %d\n", decode(NULL, NULL, 0), encode(NULL, NULL, 0) == NULL,
               decode(output, (char *)source, 0), encode((char *)output, source, 0) == (char *)output);
        print(NULL, NULL, 0, 16, 1, NULL, 0, false);
        return 0;
    case 1:
        source[-1] = 'X'; result = decode(NULL, (char *)source - 1, SIZE_MAX); break;
    case 2:
        source[-2] = '1'; source[-1] = 'X'; result = decode(NULL, (char *)source - 2, SIZE_MAX); break;
    case 3:
        memcpy(source - 3, "a0X", 3); capacity = 2;
        result = decode(output - capacity, (char *)source - 3, 2); break;
    case 4:
        memcpy(source - 4, "a01X", 4); capacity = 2;
        result = decode(output - capacity, (char *)source - 4, 2); break;
    case 5:
        memcpy(source - 2, "ff", 2); capacity = 1;
        result = decode(output - capacity, (char *)source - 2, 1); break;
    case 6:
        source[-1] = 0xab; capacity = 2;
        result = encode((char *)output - capacity, source - 1, 1) == (char *)output; break;
    case 7:
        result = format(NULL, SIZE_MAX, 16, 8, NULL, 0, true); break;
    case 8:
        result = format(NULL, 0, 16, 1, NULL, 0, false); break;
    case 9:
        result = format(NULL, 0, 32, 1, NULL, 0, true); break;
    case 10:
        capacity = 1; result = format(NULL, 0, 32, 8, (char *)output - capacity, capacity, true); break;
    case 11:
        capacity = 1; result = format(source, 16, 1, 1, (char *)output - capacity, capacity, false); break;
    case 12:
        source[-1] = 0xab; capacity = 2;
        result = format(source - 1, 16, 16, 1, (char *)output - capacity, capacity, false); break;
    case 13:
        source[-1] = 0xab; capacity = 3;
        result = format(source - 1, 16, 16, 1, (char *)output - capacity, capacity, false); break;
    default: {
        int group = 1 << ((test - 14) / 4 + 1);
        if (group > 8) return 2;
        capacity = (test - 14) % 4 + 1;
        for (int index = 0; index < group; ++index) source[index - group] = 0x12 + index * 7;
        result = format(source - group, 32, 32, group, (char *)output - capacity, capacity, true);
        break;
    }
    }
    printf("%d", result);
    for (size_t index = 0; index < capacity; ++index) printf(" %02x", (output - capacity)[index]);
    putchar('\n');
    return 0;
}
'''


class ElfRecords:
    """Read target-endian relocatable ELF records without host ABI assumptions."""

    def __init__(self, path):
        self.data = Path(path).read_bytes()
        self.order = "<" if self.data[5] == 1 else ">"
        self.word = 8 if self.data[4] == 2 else 4
        if self.word == 8:
            offset = self.unpack("Q", 40)[0]
            size, count, strings = self.unpack("HHH", 58)
            layout = "IIQQQQIIQQ"
        else:
            offset = self.unpack("I", 32)[0]
            size, count, strings = self.unpack("HHH", 46)
            layout = "IIIIIIIIII"
        self.sections = [self.unpack(layout, offset + i * size) for i in range(count)]
        names = self.section(strings)
        self.names = [self.string(names, section[0]) for section in self.sections]
        self.symbols = []
        self.relocations = {}
        for section in self.sections:
            if section[1] != 2:
                continue
            strings = self.section(section[6])
            for at in range(section[4], section[4] + section[5], section[9]):
                if self.word == 8:
                    name, info, other, index, value, size = self.unpack("IBBHQQ", at)
                else:
                    name, value, size, info, other, index = self.unpack("IIIBBH", at)
                self.symbols.append((self.string(strings, name), index, value, size, info))
        for section in self.sections:
            if section[1] not in (4, 9):
                continue
            for at in range(section[4], section[4] + section[5], section[9]):
                offset, info = self.unpack("QQ" if self.word == 8 else "II", at)
                symbol = info >> (32 if self.word == 8 else 8)
                if section[1] == 4:
                    addend = self.unpack("q" if self.word == 8 else "i", at + 2 * self.word)[0]
                else:
                    addend = struct.unpack_from(self.order + ("Q" if self.word == 8 else "I"),
                                                self.section(section[7]), offset)[0]
                self.relocations[(section[7], offset)] = (self.symbols[symbol][1], self.symbols[symbol][2] + addend)

    def unpack(self, layout, offset):
        return struct.unpack_from(self.order + layout, self.data, offset)

    def section(self, index):
        section = self.sections[index]
        return self.data[section[4]:section[4] + section[5]]

    @staticmethod
    def string(data, offset):
        return data[offset:data.index(0, offset)]

    def pointer_string(self, section, offset):
        target = self.relocations.get((section, offset))
        if target is None:
            assert self.section(section)[offset:offset + self.word] == bytes(self.word)
            return None
        section, offset = target
        return self.string(self.section(section), offset)


class HexdumpAbiTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="hexdump-abi-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.work = Path(cls.temp.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.wrapper = cls.work / "abi.rs"
        cls.wrapper.write_text(RUST_WRAPPER.replace("@SOURCE@", str(ROOT / "lib/hexdump_rust.rs")))
        # Kernel C is built with -funsigned-char, even on architectures where
        # core::ffi::c_char is signed. Import the actual kernel alias crate so
        # the native bindings test cannot silently regress to the host ABI.
        cls.ffi = cls.work / "libkernel_ffi.rlib"
        subprocess.run([*cls.rustc, "--edition=2021", "--crate-name=kernel_ffi",
                        "--crate-type=rlib", "-Dwarnings", ROOT / "rust/ffi.rs",
                        "-o", cls.ffi], check=True)

    def rust_object(self, name, flags=(), optimize="2"):
        output = self.work / name
        subprocess.run([*self.rustc, "--edition=2021", "--crate-type=lib", "--emit=obj",
                        "-Cpanic=abort", "-Copt-level=" + optimize, "-Dwarnings", "-Dunsafe-op-in-unsafe-fn",
                        "-Wmissing-docs", "-Wrust-2018-idioms", "-Wunreachable-pub",
                        "--extern", "kernel_ffi=" + str(self.ffi),
                        *flags, self.wrapper, "-o", output], check=True)
        return output

    def test_hex_to_bin_machine_code_has_no_input_branches_or_loads(self):
        objdump = shlex.split(os.environ.get("OBJDUMP", "objdump"))
        for optimization in ("1", "2", "3", "s", "z"):
            image = self.rust_object("constant-time-" + optimization + ".o", optimize=optimization)
            elf = ElfRecords(image)
            if elf.unpack("H", 18)[0] not in (3, 62):
                self.skipTest("branchless instruction audit currently supports native x86 machine code")
            result = subprocess.run([*objdump, "-d", "--no-show-raw-insn", "-M", "intel",
                                     "--disassemble=hex_to_bin", image], capture_output=True, check=True)
            instructions = []
            for line in result.stdout.decode().splitlines():
                match = re.match(r"\s*[0-9a-f]+:\s+([a-z][a-z0-9]*)\s*(.*)", line)
                if match:
                    instructions.append(match.groups())
            self.assertTrue(instructions, result.stdout)
            for mnemonic, operands in instructions:
                self.assertFalse(mnemonic.startswith(("j", "loop", "call")), (optimization, mnemonic, operands))
                # LEA computes arithmetic without reading the apparent memory
                # operand. Actual loads/stores would violate the source's
                # stronger guarantee that no address depends on the digit.
                self.assertTrue("[" not in operands or mnemonic == "lea", (optimization, mnemonic, operands))

    def test_printk_configuration_and_packed_index_records(self):
        for native_char in (False, True):
            with self.subTest(char="kernel unsigned" if native_char else "signed"):
                self.check_printk_configuration_and_packed_index_records(native_char)

    def check_printk_configuration_and_packed_index_records(self, native_char):
        for printk, index in ((False, False), (False, True), (True, False), (True, True)):
            flags = ["--cfg", "CONFIG_RUST"]
            if native_char:
                flags += ["--cfg", "native_char"]
            if printk:
                flags += ["--cfg", "CONFIG_PRINTK"]
            if index:
                flags += ["--cfg", "CONFIG_PRINTK_INDEX"]
            image = ElfRecords(self.rust_object(f"index-{native_char}-{printk}-{index}.o", flags))
            self.assertEqual(any(symbol[0] == b"print_hex_dump" for symbol in image.symbols), printk)
            self.assertEqual(b".printk_index" in image.names, printk and index)
            if not (printk and index):
                continue
            section = image.names.index(b".printk_index")
            self.assertEqual(len(image.section(section)), 3 * image.word)
            formats, lines = set(), set()
            for offset in range(0, 3 * image.word, image.word):
                target, start = image.relocations[(section, offset)]
                sizes = [symbol[3] for symbol in image.symbols if symbol[1] == target and symbol[2] == start]
                self.assertIn(image.word * 5 + 4, sizes, "printk record must retain its packed C size")
                formats.add(image.pointer_string(target, start))
                self.assertEqual(image.pointer_string(target, start + image.word), b"print_hex_dump")
                filename = image.pointer_string(target, start + 2 * image.word)
                self.assertTrue(filename.endswith(b"lib/hexdump.rs"), filename)
                line = struct.unpack_from(image.order + "I", image.section(target), start + 3 * image.word)[0]
                self.assertIn("dump_printk!", (ROOT / "lib/hexdump.rs").read_text().splitlines()[line - 1])
                lines.add(line)
                self.assertIsNone(image.pointer_string(target, start + 3 * image.word + 4))
                self.assertIsNone(image.pointer_string(target, start + 4 * image.word + 4))
            self.assertEqual(formats, {b"%s%s%p: %s\n", b"%s%s%.8x: %s\n", b"%s%s%s\n"})
            self.assertEqual(len(lines), 3)

    def test_native_c_printk_index_layout(self):
        header = (ROOT / "include/linux/printk.h").read_text()
        definition = header[header.index("struct pi_entry {"):]
        definition = definition[:definition.index("} __packed;") + len("} __packed;")]
        source = self.work / "layout.c"
        source.write_text('#include <stdio.h>\n#include <stddef.h>\n#define __packed __attribute__((packed))\n'
                          + definition + '\nint main(void) { printf("%zu %zu %zu %zu %zu %zu %zu %zu\\n", '
                          'sizeof(void *), sizeof(struct pi_entry), offsetof(struct pi_entry, fmt), '
                          'offsetof(struct pi_entry, func), offsetof(struct pi_entry, file), '
                          'offsetof(struct pi_entry, line), offsetof(struct pi_entry, level), '
                          'offsetof(struct pi_entry, subsys_fmt_prefix)); }\n')
        binary = self.work / "layout"
        subprocess.run([*self.cc, "-O2", source, "-o", binary], check=True)
        word, size, *offsets = map(int, subprocess.check_output([binary]).split())
        self.assertEqual(size, word * 5 + 4)
        self.assertEqual(offsets, [0, word, 2 * word, 3 * word, 3 * word + 4, 4 * word + 4])

    def test_protected_pages_zero_lengths_and_partial_accesses(self):
        from test_hexdump_translation import build_hexdump_tools
        directory = self.work / "guard-library"
        directory.mkdir()
        library = build_hexdump_tools(directory)
        source = self.work / "guard.c"
        source.write_text(GUARD_RUNNER)
        binary = self.work / "guard"
        subprocess.run([*self.cc, "-O2", "-Wall", "-Wextra", source, "-ldl", "-o", binary], check=True)
        for case in range(26):
            results = [subprocess.run([binary, library, prefix, str(case)], capture_output=True, timeout=5)
                       for prefix in ("c_", "")]
            for result in results:
                self.assertEqual(result.returncode, 0, (case, result.returncode, result.stderr))
            self.assertEqual(results[0].stdout, results[1].stdout, case)
            self.assertEqual(results[0].stderr, results[1].stderr, case)


if __name__ == "__main__":
    unittest.main()
