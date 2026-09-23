#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Differential coverage of s390 facility masks and disassembler generators."""

import os
from pathlib import Path
import random
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
TOOLS = ROOT / "arch/s390/tools"
CONFIGS = ("Z196", "ZEC12", "Z13", "Z14", "Z15", "Z17")
FORMATS = (
    ("MII", "RR", "RS", "RSI", "RX", "SI", "SMI", "SS"),
    ("RI", "RIL", "SSF"),
    ("E", "IE", "RRE", "RRF", "RRR", "S", "SIL", "SSE"),
    ("RIE", "RIS", "RRS", "RSE", "RSL", "RSY", "RXE", "RXF", "RXY", "SIY",
     "VRI", "VRR", "VRS", "VRV", "VRX", "VSI"),
)


class S390TableTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="s390-tables-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.opcode = cls.compile("gen_opcode_table")
        cls.facilities = cls.compile("gen_facilities")

    @classmethod
    def compile(cls, name, configs=()):
        suffix = "-".join(configs)
        outputs = (cls.work / (name + suffix + "-c"), cls.work / (name + suffix + "-rust"))
        c = cls.cc + ["-O2", "-Wall"]
        rust = cls.rustc + ["--edition=2021", "-Dwarnings", "-Wmissing_docs", "-Wunreachable_pub", "-Wrust_2018_idioms"]
        for config in configs:
            symbol = "CONFIG_HAVE_MARCH_" + config + "_FEATURES"
            c.append("-D" + symbol)
            rust += ["--cfg", symbol]
        for command, extension, output in ((c, ".c", outputs[0]), (rust, ".rs", outputs[1])):
            result = subprocess.run(command + [str(TOOLS / (name + extension)), "-o", str(output)], capture_output=True)
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)
        return outputs

    def compare(self, tools, data=b"", args=()):
        results = [subprocess.run([str(tool), *args], input=data, capture_output=True, timeout=15) for tool in tools]
        compared = [(result.returncode, result.stdout, result.stderr) for result in results]
        self.assertEqual(compared[0], compared[1])
        return compared[1]

    def test_facility_all_64_configuration_combinations(self):
        feature_bits = ((45,), (49, 52), (53, 129), (58,), (61,), (84,))
        base = (0, 1, 3, 18, 21, 25, 27, 32, 33, 34, 35)
        for mask in range(64):
            configs = tuple(config for index, config in enumerate(CONFIGS) if mask & (1 << index))
            with self.subTest(configs=configs):
                tools = self.compile("gen_facilities", configs) if mask else self.facilities
                status, output, errors = self.compare(tools, args=("ignored", "--help"))
                self.assertEqual((status, errors), (0, b""))
                bits = list(base) + [bit for i, group in enumerate(feature_bits) if mask & (1 << i) for bit in group]
                words = [0] * (max(bits) // 64 + 1)
                for bit in bits:
                    words[bit // 64] |= 1 << (63 - bit % 64)
                expected = "#define FACILITIES_ALS " + ",".join(f"_AC(0x{word:016x},UL)" for word in words)
                self.assertIn(expected.encode(), output.splitlines())

    def test_full_checked_in_opcode_table(self):
        data = (TOOLS / "opcodes.txt").read_bytes()
        status, output, errors = self.compare(self.opcode, data)
        self.assertEqual((status, errors), (0, b""))
        self.assertEqual(output.count(b".opfrag ="), len(data.splitlines()))
        # The actual generated header must remain valid C for the disassembler.
        generated = self.work / "dis-defs.h"
        generated.write_bytes(output)
        source = b'''#include <stddef.h>
#include "dis-defs.h"
struct instruction { unsigned int opfrag, format; union { char name[6]; unsigned int offset; }; };
struct group { unsigned int opcode, mask, byte, offset, count; };
static const char names[][20] = LONG_INSN_INITIALIZER;
static const struct instruction instructions[] = OPCODE_TABLE_INITIALIZER;
static const struct group groups[] = OPCODE_OFFSET_INITIALIZER;
int main(void) { return !(sizeof(names) && sizeof(instructions) && sizeof(groups)); }
'''
        result = subprocess.run(self.cc + ["-Wall", "-Werror", "-x", "c", "-", "-I", str(self.work), "-o", str(self.work / "consumer")],
                                input=source, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(subprocess.run([str(self.work / "consumer")]).returncode, 0)

    def test_every_format_base_and_suffix_with_scanf_whitespace(self):
        rows = []
        for kind, formats in enumerate(FORMATS):
            for index, fmt in enumerate(formats):
                code = f"{0x40 + index:02x}" if kind == 0 else f"{0xa0 + kind:02x}{index:x}"
                for suffix in ("", "_", "_R1_R2", "__U0"):
                    rows.append([code.encode(), ("insn" + str(index)).encode(), (fmt + suffix).encode()])
        random.Random(12345).shuffle(rows)
        fields = [word for row in rows for word in row]
        whitespace = (b" ", b"\t", b"\n", b"\r", b"\v", b"\f", b" \t\n\v\f\r")
        data = b"".join(word + whitespace[index % len(whitespace)] for index, word in enumerate(fields))
        self.assertEqual(self.compare(self.opcode, data)[0], 0)

    def test_random_orders_duplicate_keys_and_large_tables(self):
        rng = random.Random(0x5390)
        for count in (1, 2, 20, 200, 3000):
            rows = []
            for _ in range(count):
                kind = rng.randrange(4)
                opcode = rng.choice(("04", "05", "60")) if kind == 0 else rng.choice(("a70", "a71", "b902", "e300", "e301"))
                name = rng.choice(("foo", "bar", "a", "abcde", "abcdef", "dup", "long_duplicate", "n" * 19))
                fmt = rng.choice(FORMATS[kind]) + rng.choice(("", "_R1", "_R2"))
                rows.append(f"{opcode} {name} {fmt}\n".encode())
            with self.subTest(count=count):
                self.assertEqual(self.compare(self.opcode, b"".join(rows))[0], 0)

    def test_group_transitions_and_shared_first_byte(self):
        fixtures = (
            b"04 a RR\n05 b RX\nff c SI\n",
            b"a70 longname RI\na71 second RIL_R1\nb901 short RRE\n",
            b"ff00 multi E\nff one RR\n",  # C merges this into the previous group.
            b"0000 illegal E\n0001 other S\n01ff last E\n04 first RR\n",
        )
        for data in fixtures:
            with self.subTest(data=data):
                self.assertEqual(self.compare(self.opcode, data)[0], 0)

    def test_kapi_generated_headers_configuration_rebuilds_and_noop_mtimes(self):
        env = os.environ.copy()
        for key in list(env):
            if key.startswith("KBUILD_") or key in (
                    "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                    "sub_make_done", "srctree", "srcroot", "objtree", "VPATH"):
                env.pop(key)
        env["LC_ALL"] = "C"
        with tempfile.TemporaryDirectory(prefix="s390-kapi-") as temporary:
            output = Path(temporary)
            basic = output / "scripts/basic"
            basic.mkdir(parents=True)
            result = subprocess.run(self.rustc + ["--edition=2021", "-Dwarnings",
                                    str(ROOT / "scripts/basic/fixdep.rs"), "-o", str(basic / "fixdep")],
                                    capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            command = shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", temporary, "-f", str(ROOT / "scripts/Makefile.build"),
                "srctree=" + str(ROOT), "srcroot=" + str(ROOT), "objtree=.",
                "obj=arch/s390/tools", "ARCH=s390", "CC=" + shlex.join(self.cc),
                "HOSTCC=" + shlex.join(self.cc), "HOSTRUSTC=" + shlex.join(self.rustc),
                "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings -Wmissing_docs -Wunreachable_pub -Wrust_2018_idioms",
                "kapi"]
            for configs in ((), CONFIGS, (), ("Z13",)):
                flags = ["CONFIG_HAVE_MARCH_" + config + "_FEATURES=y" for config in configs]
                with self.subTest(configs=configs):
                    result = subprocess.run(command + flags, env=env, capture_output=True, timeout=30)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertNotIn(b"  HOSTCC ", result.stdout)
                    deps = (output / "arch/s390/tools/.gen_facilities.cmd").read_text()
                    for config in CONFIGS:
                        self.assertEqual("--cfg CONFIG_HAVE_MARCH_" + config + "_FEATURES" in deps, config in configs)
                    for name, generated, data in (
                            ("gen_facilities", "facility-defs.h", b""),
                            ("gen_opcode_table", "dis-defs.h", (TOOLS / "opcodes.txt").read_bytes())):
                        c = output / (name + "-c")
                        result = subprocess.run(self.cc + ["-O2", *["-DCONFIG_HAVE_MARCH_" + config + "_FEATURES" for config in configs],
                                                        str(TOOLS / (name + ".c")), "-o", str(c)], capture_output=True)
                        self.assertEqual(result.returncode, 0, result.stderr)
                        expected = subprocess.run([str(c)], input=data, capture_output=True, check=True).stdout
                        header = output / "arch/s390/include/generated/asm" / generated
                        self.assertEqual(header.read_bytes(), expected)
                    headers = [output / "arch/s390/include/generated/asm" / name for name in ("facility-defs.h", "dis-defs.h")]
                    mtimes = [header.stat().st_mtime_ns for header in headers]
                    result = subprocess.run(command + flags, env=env, capture_output=True, timeout=30)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertNotIn(b"HOSTRUSTC", result.stdout)
                    self.assertEqual(mtimes, [header.stat().st_mtime_ns for header in headers])

    def test_raw_bytes_ascii_case_conversion_nul_and_length_boundaries(self):
        fixtures = (
            b"04 a\xff RR\na70 abcde\xff RI\nb901 mixed_CASE RRE\n",
            b"04\0ignored name\0ignored RR\0ignore\n",
            b"04 \0 RR\na70 abcdef RI_\xff\n",
            b"04 " + b"n" * 19 + b" RR_" + b"F" * 16 + b"\n",
            b"04 a RR\na70 aaaaa RI\na71 aaaaaa RIL\n",
            b"04 quote' RR\na70 slash\\ E\n",
            b"0 single RR\n\0 empty RR\n",
        )
        for data in fixtures:
            with self.subTest(data=data):
                self.assertEqual(self.compare(self.opcode, data)[0], 0)

    def test_empty_ignored_arguments_and_malformed_input(self):
        for data in (b"", b" \n\r\t\v\f"):
            self.assertEqual(self.compare(self.opcode, data, ("--help", "ignored"))[0], 0)
        for data in (b"04", b"04 foo", b"04 foo NONE", b"04 foo rr", b"04 foo _RR", b"04 foo RR\nbad", b"04 foo RR\n04 b XYZ"):
            with self.subTest(data=data):
                self.assertEqual(self.compare(self.opcode, data), (1, b"", b""))
        # The original overflows fixed 20-byte arrays or reads an uninitialized
        # opcode fragment here; Rust refuses these without producing a header.
        for data in (b"04 " + b"x" * 20 + b" RR", b"x" * 10000 + b" a RR", b"0 a E", b"\0 a E"):
            result = subprocess.run([str(self.opcode[1])], input=data, capture_output=True)
            self.assertEqual((result.returncode, result.stdout, result.stderr), (1, b"", b""))

    def test_output_and_input_errors_are_reported(self):
        if not Path("/dev/full").exists():
            self.skipTest("/dev/full is unavailable")
        for tools in (self.opcode, self.facilities):
            with open("/dev/full", "wb") as full:
                result = subprocess.run([str(tools[1])], input=b"04 foo RR\n", stdout=full, stderr=subprocess.PIPE)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"No space left on device", result.stderr)
            self.assertNotIn(b"panicked", result.stderr)
        directory = os.open(self.work, os.O_RDONLY)
        try:
            result = subprocess.run([str(self.opcode[1])], stdin=directory, capture_output=True)
        finally:
            os.close(directory)
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout, b"")
        self.assertIn(b"Is a directory", result.stderr)


if __name__ == "__main__":
    unittest.main()
