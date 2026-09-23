#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Byte-for-byte host oracle for the allocation-free kernel hexdump port.

The C oracle compiles the unchanged lib/hexdump.c and lib/ctype.c with small
host-environment adapters. Conversion, classification, hexadecimal digit
helpers, and unaligned reads are the original kernel implementations. The
printk capture function implements the actual variadic ABI; it records format
strings and typed arguments rather than replacing kernel pointer policy.
"""

import ctypes
import itertools
import json
import os
from pathlib import Path
import random
import shlex
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SYMBOLS = (
    "hex_asc", "hex_asc_upper", "hex_to_bin", "hex2bin", "bin2hex",
    "hex_dump_to_buffer", "print_hex_dump",
)


def command_env(name, default):
    return shlex.split(os.environ.get(name, default))


def run(command):
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            timeout=120)
    if result.returncode:
        raise RuntimeError(f"{command!r}\n{result.stdout.decode(errors='replace')}"
                           f"{result.stderr.decode(errors='replace')}")
    return result


def c_environment(directory):
    """Install host adapters, retaining the real hex/ctype/unaligned bodies."""
    directory = Path(directory)
    include = directory / "include" / "linux"
    include.mkdir(parents=True, exist_ok=True)
    headers = {
        "types.h": """
#ifndef HOST_TEST_TYPES_H
#define HOST_TEST_TYPES_H
#include <stdbool.h>
#include <stddef.h>
typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned int u32;
typedef unsigned long long u64;
#define __must_check
#define __packed __attribute__((packed))
#endif
""",
        "compiler.h": "#define unlikely(x) __builtin_expect(!!(x), 0)\n",
        "errno.h": '#include "' + str(ROOT / "include/uapi/asm-generic/errno-base.h") + '"\n',
        "hex.h": '#include "' + str(ROOT / "include/linux/hex.h") + '"\n',
        "ctype.h": '#include "' + str(ROOT / "include/linux/ctype.h") + '"\n',
        "export.h": "#define EXPORT_SYMBOL(symbol)\n",
        "minmax.h": "#define min(x, y) ((x) < (y) ? (x) : (y))\n",
        "kernel.h": """
#include <stdio.h>
extern int _printk(const char *, ...);
#define printk _printk
enum { DUMP_PREFIX_NONE, DUMP_PREFIX_ADDRESS, DUMP_PREFIX_OFFSET };
static inline int is_power_of_2(unsigned long value)
{
    return value && !(value & (value - 1));
}
""",
        "unaligned.h": '#include "' + str(ROOT / "include/linux/unaligned/packed_struct.h") + """"
#define get_unaligned(ptr) _Generic(*(ptr), \\
    u16: __get_unaligned_cpu16, u32: __get_unaligned_cpu32, \\
    u64: __get_unaligned_cpu64)(ptr)
""",
    }
    for name, source in headers.items():
        (include / name).write_text(source)
    return directory / "include"


CAPTURE_C = r"""
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static char capture[131072];
static size_t used;
static uintptr_t input_start;

void capture_reset(const void *input)
{
    used = 0;
    capture[0] = 0;
    input_start = (uintptr_t)input;
}

const char *capture_data(void) { return capture; }

static void append(const char *format, ...)
{
    va_list ap;
    va_start(ap, format);
    int result = vsnprintf(capture + used, sizeof(capture) - used, format, ap);
    va_end(ap);
    if (result < 0 || (size_t)result >= sizeof(capture) - used)
        abort();
    used += result;
}

int _printk(const char *format, ...)
{
    va_list ap;
    va_start(ap, format);
    const char *level = va_arg(ap, const char *);
    const char *prefix = va_arg(ap, const char *);
    append("format=%slevel=%s\nprefix=%s\n", format,
           level ? level : "<null>", prefix ? prefix : "<null>");
    if (!strcmp(format, "%s%s%p: %s\n")) {
        const void *address = va_arg(ap, const void *);
        append("address=%zu\n", (size_t)((uintptr_t)address - input_start));
    } else if (!strcmp(format, "%s%s%.8x: %s\n")) {
        unsigned int offset = va_arg(ap, unsigned int);
        append("offset=%08x\n", offset);
    } else if (strcmp(format, "%s%s%s\n")) {
        abort();
    }
    const char *line = va_arg(ap, const char *);
    append("line=%s\n", line);
    va_end(ap);
    return 0;
}
"""


def rust_environment(directory, *, printk=True):
    """Build core-only source with the real host panic runtime for linking."""
    directory = Path(directory)
    wrapper = directory / "hexdump_wrapper.rs"
    wrapper.write_text("""//! Standalone kernel hexdump ABI oracle.
#![no_std]
// Supply the real host panic personality for libcore's prebuilt unwind data.
// The production module itself imports only core and contains no allocator.
extern crate std;
#[path = """ + json.dumps(str(ROOT / "lib/hexdump_rust.rs")) + """]
mod production;
pub use production::*;
""")
    command = command_env("HOSTRUSTC", "rustc") + [
        "--edition=2021", "--crate-name=hexdump_oracle",
        "--crate-type=staticlib", "-Copt-level=2", "-Cpanic=abort",
        "-Coverflow-checks=yes", "-Dwarnings", "-Wmissing_docs",
        "-Wrust_2018_idioms", "-Wunreachable_pub",
        "--check-cfg=cfg(CONFIG_PRINTK)",
        "--check-cfg=cfg(CONFIG_PRINTK_INDEX)",
        "--check-cfg=cfg(CONFIG_RUST)",
    ]
    if printk:
        command += ["--cfg=CONFIG_PRINTK"]
    output = directory / "hexdump-rust.a"
    run(command + [str(wrapper), "-o", str(output)])
    return output


def build_hexdump_tools(directory):
    """Return one shared library exporting both C and Rust C-ABI functions."""
    directory = Path(directory)
    include = c_environment(directory)
    common = command_env("HOSTCC", "cc") + [
        "-O2", "-fPIC", "-fno-strict-aliasing", "-DCONFIG_PRINTK",
        "-I" + str(include),
    ]
    original = directory / "hexdump-c.o"
    run(common + ["-D" + name + "=c_" + name for name in SYMBOLS] +
        ["-c", str(ROOT / "lib/hexdump.c"), "-o", str(original)])
    ctype = directory / "ctype.o"
    run(common + ["-c", str(ROOT / "lib/ctype.c"), "-o", str(ctype)])
    capture = directory / "capture.c"
    capture.write_text(CAPTURE_C)
    archive = rust_environment(directory)
    library = directory / "hexdump.so"
    run(command_env("HOSTCC", "cc") + [
        "-shared", "-fPIC", "-Wl,--gc-sections", "-Wl,-z,defs",
        str(capture), str(original), str(ctype), str(archive),
        *["-Wl,-u," + name for name in SYMBOLS],
        "-o", str(library),
    ])
    return library


class HexdumpTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="hexdump-translation-")
        cls.directory = Path(cls.temporary.name)
        cls.library = ctypes.CDLL(str(build_hexdump_tools(cls.directory)))
        cls.functions = {}
        for prefix in ("c_", ""):
            conversion = getattr(cls.library, prefix + "hex_to_bin")
            conversion.argtypes = [ctypes.c_ubyte]
            conversion.restype = ctypes.c_int
            decode = getattr(cls.library, prefix + "hex2bin")
            decode.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_size_t]
            decode.restype = ctypes.c_int
            encode = getattr(cls.library, prefix + "bin2hex")
            encode.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_size_t]
            encode.restype = ctypes.c_void_p
            dump = getattr(cls.library, prefix + "hex_dump_to_buffer")
            dump.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int,
                             ctypes.c_int, ctypes.c_void_p, ctypes.c_size_t,
                             ctypes.c_bool]
            dump.restype = ctypes.c_int
            printing = getattr(cls.library, prefix + "print_hex_dump")
            printing.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_int,
                                 ctypes.c_int, ctypes.c_int, ctypes.c_void_p,
                                 ctypes.c_size_t, ctypes.c_bool]
            printing.restype = None
            cls.functions[prefix] = conversion, decode, encode, dump, printing
        cls.library.capture_reset.argtypes = [ctypes.c_void_p]
        cls.library.capture_reset.restype = None
        cls.library.capture_data.argtypes = []
        cls.library.capture_data.restype = ctypes.c_char_p

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def test_all_digits_and_digit_arrays(self):
        for prefix, functions in self.functions.items():
            for name, expected in (
                    ("hex_asc", b"0123456789abcdef\0"),
                    ("hex_asc_upper", b"0123456789ABCDEF\0")):
                actual = (ctypes.c_ubyte * 17).in_dll(self.library, prefix + name)
                self.assertEqual(bytes(actual), expected)
            for byte in range(256):
                expected = int(chr(byte), 16) if byte in b"0123456789abcdefABCDEF" else -1
                self.assertEqual(functions[0](byte), expected, (prefix, byte))

    def test_all_hex_pairs_and_stopping_before_invalid_pair(self):
        source = (ctypes.c_ubyte * 12)(*b"12345678abcd")
        buffers = [(ctypes.c_ubyte * 16)() for _ in range(2)]
        for high, low in itertools.product(range(256), repeat=2):
            source[4], source[5] = high, low
            results = []
            for output, prefix in zip(buffers, ("c_", "")):
                ctypes.memset(output, 0xa5, len(output))
                status = self.functions[prefix][1](output, source, 6)
                results.append((status, bytes(output)))
            self.assertEqual(*results, (high, low))
            expected = high in b"0123456789abcdefABCDEF" and low in b"0123456789abcdefABCDEF"
            self.assertEqual(results[0][0], 0 if expected else -22)
            self.assertEqual(results[0][1][:2], b"\x12\x34")
            if not expected:
                self.assertEqual(results[0][1][2:], b"\xa5" * 14)

    def test_binary_encoding_large_and_unterminated(self):
        rng = random.Random(0xc001d00d)
        for length in (0, 1, 2, 15, 16, 17, 31, 32, 33, 255, 256, 4096, 65536):
            data = bytes(range(256)) * (length // 256) + rng.randbytes(length % 256)
            source = ctypes.create_string_buffer(data)
            for prefix in self.functions:
                output = (ctypes.c_ubyte * (length * 2 + 17))()
                ctypes.memset(output, 0xa5, len(output))
                end = self.functions[prefix][2](output, source, length)
                self.assertEqual(end, ctypes.addressof(output) + length * 2)
                self.assertEqual(bytes(output), data.hex().encode() + b"\xa5" * 17)

    def check_overlap(self, kind, initial, source, destination, count, *parameters):
        results = []
        for prefix in ("c_", ""):
            data = ctypes.create_string_buffer(initial, len(initial))
            begin = ctypes.addressof(data)
            if kind == 3:
                row, group, capacity, ascii_output = parameters
                status = self.functions[prefix][kind](
                    begin + source, count, row, group, begin + destination,
                    capacity, ascii_output)
            else:
                status = self.functions[prefix][kind](
                    begin + destination, begin + source, count)
                if kind == 2:
                    status -= begin
            results.append((status, bytes(data)))
        self.assertEqual(*results, (kind, source, destination, count, parameters))

    def test_forward_overlapping_conversions(self):
        rng = random.Random(0x5afe)
        for source, destination, count in itertools.product(
                range(0, 17), range(0, 17), range(0, 33)):
            binary = rng.randbytes(128)
            self.check_overlap(2, binary, source, destination, count)
            hexadecimal = bytes(rng.choice(b"0123456789aBcDeF") for _ in range(128))
            self.check_overlap(1, hexadecimal, source, destination, count)

    def compare_dump(self, data, length, row, group, capacity, ascii_output, offset=0):
        source = ctypes.create_string_buffer(b"?" * offset + data)
        values = []
        for prefix in ("c_", ""):
            output = (ctypes.c_ubyte * 160)()
            ctypes.memset(output, 0xa5, len(output))
            status = self.functions[prefix][3](
                ctypes.addressof(source) + offset, length, row, group,
                ctypes.addressof(output) + 7, capacity, ascii_output)
            values.append((status, bytes(output)))
        self.assertEqual(*values, (length, row, group, capacity, ascii_output, offset))
        self.assertEqual(values[0][1][:7], b"\xa5" * 7)
        self.assertEqual(values[0][1][7 + capacity:], b"\xa5" * (153 - capacity))
        return values[0]

    def test_every_short_capacity_row_group_and_length(self):
        data = bytes((i * 37 + 11) & 255 for i in range(64))
        for length, row, group, ascii_output, capacity in itertools.product(
                range(41), (16, 32), (1, 2, 4, 8), (False, True), range(134)):
            self.compare_dump(data, length, row, group, capacity, ascii_output,
                              offset=(length + group) % 8)

    def test_invalid_parameters_normalization_and_size_t_lengths(self):
        data = bytes(range(64))
        for row, group, length, capacity, ascii_output in itertools.product(
                (-2**31, -1, 0, 1, 15, 17, 31, 33, 2**31 - 1),
                (-2**31, -32, -8, -1, 0, 3, 5, 7, 9, 16, 2**31 - 1),
                (0, 1, 7, 8, 16, 32, 33, 2**32 + 1, ctypes.c_size_t(-1).value),
                (0, 1, 2, 16, 65, 130), (False, True)):
            self.compare_dump(data, length, row, group, capacity, ascii_output)

    def test_all_ascii_values_and_native_endian_groups(self):
        for first in range(0, 256, 16):
            data = bytes(range(first, first + 16))
            for group in (1, 2, 4, 8):
                status, output = self.compare_dump(data, 16, 16, group, 130, True)
                ascii_column = 32 + 16 // group + 1
                digits = b" ".join(
                    int.from_bytes(data[i:i + group], sys.byteorder).to_bytes(
                        group, "big").hex().encode() for i in range(0, 16, group))
                text = digits.ljust(ascii_column) + bytes(
                    byte if 32 <= byte < 127 else 46 for byte in data)
                self.assertEqual(status, len(text))
                self.assertEqual(output[7:7 + len(text) + 1], text + b"\0")

    def test_overlapping_dump_short_writes_and_ascii_second_pass(self):
        rng = random.Random(0xb0ff)
        for source, destination, length, row, group, capacity, ascii_output in itertools.product(
                (0, 1, 7, 32, 64), (0, 1, 7, 16, 64), (0, 1, 8, 16, 32),
                (16, 32), (1, 2, 4, 8), (0, 1, 2, 3, 8, 9, 17, 33, 65, 130),
                (False, True)):
            self.check_overlap(3, rng.randbytes(256), source, destination,
                               length, row, group, capacity, ascii_output)

    def test_printk_formats_prefixes_rows_and_argument_types(self):
        data = bytes(range(256)) * 4
        source = ctypes.create_string_buffer(data)
        for length, row, group, ascii_output, prefix_type in itertools.product(
                (0, 1, 7, 8, 15, 16, 17, 31, 32, 33, 65, 257, 1024),
                (-1, 0, 16, 32, 33), (-1, 0, 1, 2, 4, 8, 16),
                (False, True), (-2**31, -1, 0, 1, 2, 3, 2**31 - 1)):
            captures = []
            for prefix in ("c_", ""):
                self.library.capture_reset(source)
                self.functions[prefix][4](
                    b"\x017", b"raw:\xff", prefix_type, row, group,
                    source, length, ascii_output)
                captures.append(self.library.capture_data())
            self.assertEqual(*captures, (length, row, group, ascii_output, prefix_type))
            expected_rows = (length + (32 if row == 32 else 16) - 1) // (
                32 if row == 32 else 16)
            self.assertEqual(captures[0].count(b"format="), expected_rows)

    def test_null_and_zero_size_contracts(self):
        for prefix, functions in self.functions.items():
            self.assertEqual(functions[1](None, None, 0), 0, prefix)
            self.assertIsNone(functions[2](None, None, 0), prefix)
            for length, row, group, ascii_output in itertools.product(
                    (0, 1, 16, 32, ctypes.c_size_t(-1).value),
                    (0, 16, 32), (1, 2, 4, 8), (False, True)):
                # No source or output access is permitted with zero capacity.
                result = functions[3](None, length, row, group, None, 0, ascii_output)
                expected = self.functions["c_"][3](
                    None, length, row, group, None, 0, ascii_output)
                self.assertEqual(result, expected, (prefix, length, row, group))
            output = ctypes.c_ubyte(0xa5)
            self.assertEqual(functions[3](None, 0, 32, 8, ctypes.byref(output), 1, True), 0)
            self.assertEqual(output.value, 0)
            output.value = 0xa5
            self.assertEqual(functions[3](None, 16, 16, 1, ctypes.byref(output), 1, False), 47)
            self.assertEqual(output.value, 0)
            self.library.capture_reset(None)
            functions[4](None, None, 1, 16, 8, None, 0, True)
            self.assertEqual(self.library.capture_data(), b"")

    def test_printk_null_strings_are_passed_through(self):
        data = ctypes.create_string_buffer(b"0123456789abcdef")
        for level, prefix_text, prefix_type in itertools.product(
                (None, b"", b"\x016"), (None, b"prefix"), (0, 1, 2)):
            outputs = []
            for prefix in ("c_", ""):
                self.library.capture_reset(data)
                self.functions[prefix][4](level, prefix_text, prefix_type, 16, 1,
                                         data, 16, True)
                outputs.append(self.library.capture_data())
            self.assertEqual(*outputs)


if __name__ == "__main__":
    unittest.main()
