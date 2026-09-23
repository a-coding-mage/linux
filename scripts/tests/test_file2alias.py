#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Target-layout and alias parity against the original file2alias implementation."""

import os
from pathlib import Path
import random
import re
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "scripts/mod/file2alias.c"
C_WRAPPER = r'''
#include "file2alias.c"
bool target_is_big_endian;
bool host_is_big_endian = __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__;
static void *contents;
static bool errors;
void *sym_get_data(const struct elf_info *info, const Elf_Sym *sym) {
    (void)info; (void)sym; return contents;
}
void modpost_log(bool is_error, struct module *mod, const char *format, ...) {
    (void)mod;
    va_list args;
    errors |= is_error;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);
}
int main(int argc, char **argv) {
    if (argc < 5) return 2;
    target_is_big_endian = strcmp(argv[1], "big") == 0;
    struct module *mod = calloc(1, sizeof(*mod) + 16);
    strcpy(mod->name, "fixture");
    mod->is_vmlinux = strcmp(argv[2], "builtin") == 0;
    INIT_LIST_HEAD(&mod->aliases);
    Elf_Shdr sections[2] = { 0 };
    sections[1].sh_type = SHT_PROGBITS;
    struct elf_info info = { .sechdrs = sections, .num_sections = 2 };
    for (int i = 3; i + 1 < argc; i += 2) {
        FILE *file = fopen(argv[i+1], "rb");
        if (!file) return 2;
        fseek(file, 0, SEEK_END);
        long len = ftell(file);
        rewind(file);
        contents = calloc(1, len + 1);
        if (fread(contents, 1, len, file) != (size_t)len) return 2;
        fclose(file);
        Elf_Sym sym = { .st_shndx = 1, .st_info = STT_OBJECT, .st_size = len };
        handle_moddevtable(mod, &info, &sym, argv[i]);
        free(contents);
    }
    struct module_alias *alias;
    list_for_each_entry(alias, &mod->aliases, node)
        printf("%s\t%s\n", alias->builtin_modname ? alias->builtin_modname : "-", alias->str);
    return errors ? 1 : 0;
}
'''
RUST_WRAPPER = r'''
//! Device alias differential harness.
#[path = "__SOURCE__"]
mod file2alias;
use file2alias::{Tables, Severity};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let header = std::fs::read_to_string(&args[1]).unwrap();
    let tables = Tables::parse(&header).unwrap();
    let little = args[2] == "little";
    let bits = args[3].parse().unwrap();
    let builtin = args[4] == "builtin";
    let mut aliases = Vec::new();
    let mut errors = false;
    for pair in args[5..].chunks_exact(2) {
        let data = std::fs::read(&pair[1]).unwrap();
        for diagnostic in tables.handle("fixture", builtin, &pair[0], &data, little, bits, &mut aliases) {
            eprint!("{}", diagnostic.message);
            errors |= diagnostic.severity != Severity::Warning;
            if diagnostic.severity == Severity::Fatal { std::process::exit(1); }
        }
    }
    for alias in aliases {
        println!("{}\t{}", alias.builtin_modname.as_deref().unwrap_or("-"), alias.text);
    }
    std::process::exit(i32::from(errors));
}
'''


def build(command):
    result = subprocess.run(command, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"{shlex.join(map(str, command))}\n{result.stdout}{result.stderr}")


class File2AliasTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="file2alias-tests-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.buses = dict(re.findall(r'\{"([^"]+)", SIZE_(\w+), do_\w+\}', SOURCE.read_text()))
        offsets = (ROOT / "scripts/mod/devicetable-offsets.c").read_text()
        prelude = '''#include <stddef.h>
typedef unsigned char __u8;
typedef unsigned short __u16;
typedef unsigned int __u32;
typedef unsigned long kernel_ulong_t;
typedef struct { __u8 b[16]; } guid_t;
typedef guid_t uuid_t;
typedef guid_t uuid_le;
#define UUID_STRING_LEN 36
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
'''
        offsets = offsets.replace(
            "DEFINE(OFF_##devid##_##field, offsetof(struct devid, field))",
            "DEFINE(OFF_##devid##_##field, offsetof(struct devid, field)); "
            "DEFINE(WIDTH_##devid##_##field, sizeof(((struct devid *)0)->field))")
        generator = cls.work / "offsets.c"
        generator.write_text(prelude + offsets)
        wrapper = cls.work / "wrapper.c"
        wrapper.write_text(C_WRAPPER)
        cls.headers, cls.layout, cls.c = {}, {}, {}
        for bits in (32, 64):
            directory = cls.work / str(bits)
            directory.mkdir()
            assembly = directory / "offsets.s"
            build(cc + [f"-m{bits}", "-S", "-I", str(ROOT / "include"),
                        str(generator), "-o", str(assembly)])
            layout = {name: int(value) for name, value in
                      re.findall(r'->(\w+) [#$]?(\d+) ', assembly.read_text())}
            if not layout:
                raise RuntimeError("compiler did not emit target device-table offsets")
            header = directory / "devicetable-offsets.h"
            header.write_text("\n".join(f"#define {key} {value}" for key, value in layout.items()) + "\n")
            (directory / "elfconfig.h").write_text(f"#define KERNEL_ELFCLASS ELFCLASS{bits}\n")
            reference = directory / "alias-c"
            build(cc + ["-std=gnu11", "-O2", "-Wall", "-Wextra", "-Wno-sign-compare",
                        "-I", str(directory), "-I", str(ROOT / "scripts/mod"),
                        "-I", str(ROOT / "scripts/include"), str(wrapper), "-o", str(reference)])
            cls.headers[bits], cls.layout[bits], cls.c[bits] = header, layout, reference
        driver = cls.work / "driver.rs"
        driver.write_text(RUST_WRAPPER.replace("__SOURCE__", str(SOURCE.with_suffix(".rs"))))
        cls.rust = cls.work / "alias-rust"
        build(rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                       "-Wrust-2018-idioms", str(driver), "-o", str(cls.rust)])

    def entry(self, bus, bits):
        return bytearray(self.layout[bits]["SIZE_" + self.buses[bus]])

    def field(self, bus, bits, field):
        key = self.buses[bus] + "_" + field
        return self.layout[bits]["OFF_" + key], self.layout[bits]["WIDTH_" + key]

    def set_number(self, data, bus, bits, endian, field, value):
        offset, width = self.field(bus, bits, field)
        data[offset:offset + width] = value.to_bytes(width, endian)

    def set_string(self, data, bus, bits, field, value):
        offset, width = self.field(bus, bits, field)
        value = value.encode() if isinstance(value, str) else value
        self.assertLess(len(value), width)
        data[offset:offset + width] = value + bytes(width - len(value))

    def compare(self, bus, data, bits=64, endian="little", builtin=False, symbol=None, extra=()):
        file = self.work / "table.bin"
        file.write_bytes(data)
        symbol = symbol or f"__mod_device_table__kmod_module_one__{bus}__ids"
        arguments = [symbol, str(file)]
        for index, (name, contents) in enumerate(extra):
            path = self.work / f"table-{index}.bin"
            path.write_bytes(contents)
            arguments.extend((name, str(path)))
        builtin = "builtin" if builtin else "module"
        commands = ([str(self.c[bits]), endian, builtin] + arguments,
                    [str(self.rust), str(self.headers[bits]), endian, str(bits), builtin] + arguments)
        results = []
        for command in commands:
            result = subprocess.run(command, capture_output=True, timeout=15)
            results.append((result.returncode, result.stdout, result.stderr))
        self.assertEqual(results[0], results[1])
        return results[1]

    def populated(self, bus, bits, endian, variant):
        rng = random.Random(0xF12E + variant)
        data = self.entry(bus, bits)
        prefix = "OFF_" + self.buses[bus] + "_"
        fields = [name[len(prefix):] for name in self.layout[bits] if name.startswith(prefix)]
        string_fields = {
            "acpi": {"id"}, "pnp": {"id"}, "of": {"name", "type", "compatible"},
            "vio": {"type", "compat"}, "eisa": {"sig"}, "rpmsg": {"name"}, "i2c": {"name"},
            "spi": {"name"}, "platform": {"name"}, "mei": {"name"}, "fslmc": {"obj_type"},
            "tbsvc": {"protocol_key"}, "wmi": {"guid_string"}, "mhi": {"chan"}, "mhi_ep": {"chan"},
            "auxiliary": {"name"}, "vchiq": {"name"},
        }
        for field in fields:
            offset, width = self.field(bus, bits, field)
            if field in string_fields.get(bus, set()):
                text = "Abc_x9" if variant else ""
                if field == "guid_string":
                    text = "12345678-abcd-ef01-2345-6789abcdef01"
                self.set_string(data, bus, bits, field, text)
            elif field in ("guid", "uuid"):
                data[offset:offset + width] = bytes(range(16))
            elif field == "matches":
                for index, slot in enumerate((7, 1, 18, 6)):
                    position = offset + index * 80
                    data[position] = slot if endian == "little" else slot << 1
                    text = b" test:\tModel_X\x7f\xff\x01 "
                    data[position + 1:position + 1 + len(text)] = text
            elif field == "devs":
                for index, text in enumerate((b"pnp0C01", b"ABC1234", b"pnp0C01")):
                    data[offset + index * 8:offset + index * 8 + 7] = text
            elif field == "prod_id_hash":
                for index in range(4):
                    data[offset + index * 4:offset + index * 4 + 4] = rng.getrandbits(32).to_bytes(4, endian)
            elif field.endswith("bit"):
                for bit in (0, 3, 7):
                    index = offset + (bit // bits) * (bits // 8)
                    value = int.from_bytes(data[index:index + bits // 8], endian) | (1 << (bit % bits))
                    data[index:index + bits // 8] = value.to_bytes(bits // 8, endian)
            else:
                value = rng.getrandbits(width * 8) if variant else 0
                if variant == 3:
                    value = (1 << (width * 8)) - 1
                if field in ("flags", "match_flags"):
                    if variant < 2:
                        value = (1 << (width * 8)) - 1
                    elif variant == 2:
                        value = 0
                if field == "override_only":
                    value = variant % 2
                if field == "class_mask":
                    value = 0xffffff if variant else 0
                self.set_number(data, bus, bits, endian, field, value)
        if bus == "usb":
            self.set_number(data, bus, bits, endian, "bcdDevice_lo", 0x0123)
            self.set_number(data, bus, bits, endian, "bcdDevice_hi", 0x4567)
        if bus == "amba":
            self.set_number(data, bus, bits, endian, "id", 0x12340000)
            self.set_number(data, bus, bits, endian, "mask", 0xffffae00)
        return data

    def test_every_bus_in_both_classes_and_byte_orders(self):
        self.assertEqual(len(self.buses), 57)
        for bits in (32, 64):
            for endian in ("little", "big"):
                for bus in self.buses:
                    # C reads DMI bitfields using host, not target, allocation.
                    if bus == "dmi" and endian == "big":
                        continue
                    with self.subTest(bits=bits, endian=endian, bus=bus):
                        data = self.populated(bus, bits, endian, 0)
                        data += self.populated(bus, bits, endian, 1)
                        data += self.entry(bus, bits)
                        self.compare(bus, data, bits, endian)

    def test_empty_tables_terminators_sizes_and_unknown_types(self):
        for bus in self.buses:
            with self.subTest(bus=bus):
                zero = self.entry(bus, 64)
                self.compare(bus, zero)
                self.compare(bus, b"")
                self.compare(bus, zero[:-1])
                bad = zero.copy()
                bad[-1] = 1
                self.compare(bus, bad)
        self.compare("future_bus", b"unrecognized")
        for symbol in ("plain_symbol", "__mod_device_table__invalid", "__mod_device_table__kmod_no_type",
                       "__mod_device_table__kmod_name__usb"):
            self.compare("usb", b"", symbol=symbol)

    def test_randomized_numeric_fields_flags_and_wildcards(self):
        for bits, endian in ((32, "little"), (32, "big"), (64, "little"), (64, "big")):
            for bus in self.buses:
                if bus == "dmi" and endian == "big":
                    continue
                with self.subTest(bits=bits, endian=endian, bus=bus):
                    entries = bytearray()
                    for variant in range(32):
                        entries += self.populated(bus, bits, endian, variant)
                    entries += self.entry(bus, bits)
                    self.compare(bus, entries, bits, endian)

    def test_duplicate_aliases_and_builtin_module_ownership(self):
        bus = "platform"
        data = self.populated(bus, 64, "little", 1) + self.entry(bus, 64)
        other = self.entry(bus, 64)
        self.set_string(other, bus, 64, "name", "different")
        other += self.entry(bus, 64)
        for builtin in (False, True):
            result = self.compare(bus, data, builtin=builtin, extra=(
                ("__mod_device_table__kmod_module_two__platform__duplicate", data),
                ("__mod_device_table__kmod_module_three__platform__other", other),
            ))
            self.assertEqual(len(result[1].splitlines()), 2)
            if builtin:
                self.assertTrue(result[1].startswith(b"module_one\t"))
                self.assertIn(b"module_three\t", result[1])

    def test_usb_bcd_hex_ranges_and_flags(self):
        rng = random.Random(0xB0D)
        ranges = [(0, 0), (0, 0xffff), (0x199, 0x201), (0x9999, 0xffff),
                  (0x9, 0xa), (0xf, 0x10), (0x999, 0x1000), (0x1234, 0x1234), (9, 1)]
        ranges += [(rng.randrange(0x10000), rng.randrange(0x10000)) for _ in range(300)]
        for bits, endian in ((32, "big"), (64, "little")):
            data = bytearray()
            for index, (low, high) in enumerate(ranges):
                entry = self.populated("usb", bits, endian, 1)
                self.set_number(entry, "usb", bits, endian, "bcdDevice_lo", low)
                self.set_number(entry, "usb", bits, endian, "bcdDevice_hi", high)
                flags = 0x7ff if index % 4 == 0 else rng.randrange(0x800)
                self.set_number(entry, "usb", bits, endian, "match_flags", flags)
                data += entry
            data += self.entry("usb", bits)
            self.compare("usb", data, bits, endian)

    def test_warnings_masks_and_amba_fatal(self):
        for bus in ("pci", "cdx"):
            for mask in (0, 0xff, 0xffff00, 0xffffff, 0xf00f):
                data = self.populated(bus, 64, "little", 1)
                self.set_number(data, bus, 64, "little", "class_mask", mask)
                self.compare(bus, data + self.entry(bus, 64))
        for guid in ("short", "12345678xabcd-ef01-2345-6789abcdef01",
                     "12345678-abcd-ef01-2345-6789abcdeg01"):
            data = self.entry("wmi", 64)
            self.set_string(data, "wmi", 64, "guid_string", guid)
            self.compare("wmi", data + self.entry("wmi", 64))
        data = self.entry("amba", 64)
        self.set_number(data, "amba", 64, "little", "id", 1)
        self.compare("amba", data + self.entry("amba", 64))

    def test_input_bit_boundaries_and_buffer_overflow(self):
        for bits in (32, 64):
            for endian in ("little", "big"):
                data = self.populated("input", bits, endian, 1)
                for field, positions in (("keybit", (0x70, 0x71, 127, 128, 0x2ff)),
                                         ("evbit", (0, 31)), ("ffbit", (31, 32, 63, 64, 127)),
                                         ("swbit", (16, 17))):
                    offset, _ = self.field("input", bits, field)
                    for bit in positions:
                        at = offset + bit // bits * (bits // 8)
                        word = int.from_bytes(data[at:at + bits // 8], endian) | (1 << (bit % bits))
                        data[at:at + bits // 8] = word.to_bytes(bits // 8, endian)
                self.compare("input", data + self.entry("input", bits), bits, endian)
                offset, width = self.field("input", bits, "keybit")
                data[offset:offset + width] = b"\xff" * width
                self.compare("input", data + self.entry("input", bits), bits, endian)

    def test_dmi_filter_and_target_bitfield_order(self):
        for bits in (32, 64):
            little = self.populated("dmi", bits, "little", 1) + self.entry("dmi", bits)
            expected = self.compare("dmi", little, bits, "little")[1]
            big = self.populated("dmi", bits, "big", 1) + self.entry("dmi", bits)
            path = self.work / "dmi-big.bin"
            path.write_bytes(big)
            result = subprocess.run([str(self.rust), str(self.headers[bits]), "big", str(bits), "module",
                                     "__mod_device_table__kmod_test__dmi__ids", str(path)], capture_output=True)
            self.assertEqual((result.returncode, result.stdout, result.stderr), (0, expected, b""))

    def test_dmi_overflow_and_exact_match_flag(self):
        for bits in (32, 64):
            data = self.populated("dmi", bits, "little", 1)
            offset, _ = self.field("dmi", bits, "matches")
            for index in range(4):
                data[offset + index * 80] |= 0x80
            self.compare("dmi", data + self.entry("dmi", bits), bits)
            for index in range(4):
                start = offset + index * 80
                data[start:start + 80] = bytes([6]) + b"a" * 78 + b"\0"
            self.compare("dmi", data + self.entry("dmi", bits), bits)

    def test_of_and_vio_whitespace_replacement(self):
        for bus in ("of", "vio"):
            data = self.entry(bus, 64)
            self.set_string(data, bus, 64, "type", "some \t\n\r\v\fkind")
            if bus == "of":
                self.set_string(data, bus, 64, "name", "a name")
                self.set_string(data, bus, 64, "compatible", "vnd,a device")
            else:
                self.set_string(data, bus, 64, "compat", "vnd,a device")
            self.compare(bus, data + self.entry(bus, 64))

    def test_malformed_strings_and_undefined_pci_override_are_rejected(self):
        for bus, field in (("platform", "name"), ("acpi", "id"), ("of", "compatible")):
            data = self.entry(bus, 64)
            offset, width = self.field(bus, 64, field)
            data[offset:offset + width] = b"x" * width
            path = self.work / "malformed.bin"
            path.write_bytes(data + self.entry(bus, 64))
            result = subprocess.run([str(self.rust), str(self.headers[64]), "little", "64", "module",
                                     f"__mod_device_table__kmod_test__{bus}__ids", str(path)], capture_output=True)
            self.assertEqual(result.returncode, 1)
            self.assertEqual(result.stdout, b"")
            self.assertIn(b"unterminated device-table string", result.stderr)
        # C continues from an uninitialized alias buffer for this value.
        data = self.entry("pci", 64)
        self.set_number(data, "pci", 64, "little", "override_only", 0xffffffff)
        path = self.work / "invalid-pci.bin"
        path.write_bytes(data + self.entry("pci", 64))
        result = subprocess.run([str(self.rust), str(self.headers[64]), "little", "64", "module",
                                 "__mod_device_table__kmod_test__pci__ids", str(path)], capture_output=True)
        self.assertEqual((result.returncode, result.stdout, result.stderr),
                         (0, b"", b"Unknown PCI driver_override alias FFFFFFFF\n"))


if __name__ == "__main__":
    unittest.main()
