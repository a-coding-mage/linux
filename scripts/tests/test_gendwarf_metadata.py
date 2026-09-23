# SPDX-License-Identifier: GPL-2.0
"""Byte-exact symbol-table and kABI record parity against original libelf code."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest

import gendwarf_test_support as support
from modpost_test_support import Elf


C_HARNESS = r'''
#include <fcntl.h>
#include <unistd.h>
#include "gendwarfksyms.h"
int debug = 1, dump_dies, dump_die_map, dump_types, dump_versions, stable, symtypes;
static const char *wanted;
static struct symbol *found;
static void find(struct symbol *s, void *unused) {
    if (!strcmp(s->name, wanted)) found = s;
}
static void show(struct symbol *s, void *unused) {
    printf("%s %u:%llu:%u:%zx:%zx:%lx\n", s->name, s->addr.section,
           (unsigned long long)s->addr.address, s->state,
           s->die_addr, s->ptr_die_addr, s->crc);
}
int main(int argc, char **argv) {
    stable = atoi(argv[3]);
    if (!strcmp(argv[1], "rules")) {
        int fd = open(argv[2], O_RDONLY);
        kabi_read_rules(fd); close(fd);
        for (int i = 4; i + 2 < argc; i += 3) {
            unsigned long value = 0;
            const char *text = NULL;
            if (!strcmp(argv[i], "d")) printf("%d\n", kabi_is_declonly(argv[i+1]));
            if (!strcmp(argv[i], "i")) printf("%d\n", kabi_is_enumerator_ignored(argv[i+1], argv[i+2]));
            if (!strcmp(argv[i], "e")) {
                bool ok = kabi_get_enumerator_value(argv[i+1], argv[i+2], &value);
                printf("%d %lu\n", ok, value);
            }
            if (!strcmp(argv[i], "b")) {
                bool ok = kabi_get_byte_size(argv[i+1], &value);
                printf("%d %lu\n", ok, value);
            }
            if (!strcmp(argv[i], "t")) {
                bool ok = kabi_get_type_string(argv[i+1], &text);
                printf("%d %s\n", ok, text ? text : "");
            }
        }
        kabi_free();
    } else {
        symbol_read_exports(stdin);
        if (strcmp(argv[2], "-")) {
            int fd = open(argv[2], O_RDONLY);
            symbol_read_symtab(fd); close(fd);
        }
        for (int i = 4; i + 2 < argc; i += 3) {
            if (!strcmp(argv[i], "g")) {
                struct symbol *s = symbol_get(argv[i+1]);
                printf("get %s\n", s ? s->name : "-");
                continue;
            }
            wanted = argv[i+1]; found = NULL; symbol_for_each(find, NULL);
            if (!found) return 2;
            unsigned long value = strtoul(argv[i+2], NULL, 0);
            Dwarf_Die d = { .addr = (void *)value };
            if (!strcmp(argv[i], "d")) symbol_set_die(found, &d);
            if (!strcmp(argv[i], "p")) symbol_set_ptr(found, &d);
            if (!strcmp(argv[i], "c")) symbol_set_crc(found, value);
        }
        symbol_for_each(show, NULL);
        symbol_print_versions(); symbol_free();
    }
    return 0;
}
'''

RUST_HARNESS = r'''
//! Focused original-metadata compatibility harness.
#[allow(dead_code)] #[path = "@ROOT@/scripts/elf-parse.rs"] mod elf;
#[allow(dead_code)] #[path = "@SOURCE@/gendwarfksyms_header.rs"] mod gendwarfksyms_header;
#[allow(dead_code)] #[path = "@SOURCE@/symbols.rs"] mod symbols;
#[allow(dead_code)] #[path = "@SOURCE@/kabi.rs"] mod kabi;
use gendwarfksyms_header::{Diagnostics, Options, Result};
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
fn run(args: &[Vec<u8>], diag: &mut Diagnostics, output: &mut Vec<u8>) -> Result<()> {
    if args[1] == b"parse" {
        let data = std::fs::read(std::ffi::OsStr::from_bytes(&args[2])).unwrap();
        writeln!(output, "{} {} {}", elf::ElfFile::parse_any(&data).is_ok(),
                 elf::ElfFile::parse(&data, 1 << 1).is_ok(),
                 elf::ElfFile::parse(&data, 1 << 2 | 1 << 3).is_ok()).unwrap();
    } else if args[1] == b"rules" {
        let data = std::fs::read(std::ffi::OsStr::from_bytes(&args[2])).unwrap();
        let rules = kabi::Rules::read(&data, diag)?;
        for command in args[4..].chunks_exact(3) {
            let (kind, name, field) = (&command[0][..], &command[1][..], &command[2][..]);
            match kind {
                b"d" => writeln!(output, "{}", u8::from(rules.is_declonly(name))).unwrap(),
                b"i" => writeln!(output, "{}", u8::from(rules.is_enumerator_ignored(name, field))).unwrap(),
                b"e" | b"b" => {
                    let value = if kind == b"e" { rules.enumerator_value(name, field)? }
                                else { rules.byte_size(name)? };
                    writeln!(output, "{} {}", u8::from(value.is_some()), value.unwrap_or(0)).unwrap();
                }
                b"t" => {
                    let value = rules.type_string(name);
                    write!(output, "{} ", u8::from(value.is_some())).unwrap();
                    output.extend_from_slice(value.unwrap_or_default()); output.push(b'\n');
                }
                _ => unreachable!(),
            }
        }
    } else {
        let mut symbols = symbols::Symbols::read_exports(std::io::stdin().lock(), diag)?;
        if args[2] != b"-" {
            let data = std::fs::read(std::ffi::OsStr::from_bytes(&args[2])).unwrap();
            symbols.read_symtab(&data, diag)?;
        }
        for command in args[4..].chunks_exact(3) {
            let (kind, name) = (&command[0][..], &command[1][..]);
            if kind == b"g" {
                output.extend_from_slice(b"get ");
                output.extend_from_slice(symbols.get(name).map(|id| &symbols.entries[id].name[..]).unwrap_or(b"-"));
                output.push(b'\n'); continue;
            }
            let id = symbols.entries.iter().position(|entry| entry.name == name).unwrap();
            let text = std::str::from_utf8(&command[2]).unwrap();
            let value = if let Some(hex) = text.strip_prefix("0x") { u64::from_str_radix(hex, 16).unwrap() }
                        else { text.parse::<u64>().unwrap() };
            match kind {
                b"d" => symbols.set_die(id, value as usize)?,
                b"p" => symbols.set_ptr(id, value as usize)?,
                b"c" => symbols.set_crc(id, value as u32, diag)?,
                _ => unreachable!(),
            }
        }
        for id in symbols.ordered_indices() {
            let entry = &symbols.entries[id]; output.extend_from_slice(&entry.name);
            writeln!(output, " {}:{}:{}:{:x}:{:x}:{:x}", entry.addr.0, entry.addr.1,
                     entry.state as u8, entry.die_addr.unwrap_or(0), entry.ptr_die_addr.unwrap_or(0), entry.crc).unwrap();
        }
        symbols.print_versions(output, diag)?;
    }
    Ok(())
}
fn main() {
    let args: Vec<_> = std::env::args_os().map(|arg| arg.as_bytes().to_vec()).collect();
    let mut diag = Diagnostics::new(Options { debug: true, stable: args[3] != b"0", ..Options::default() });
    let mut output = Vec::new();
    let result = run(&args, &mut diag, &mut output);
    if let Err(error) = &result { diag.bytes.extend_from_slice(&error.0); }
    std::io::stdout().write_all(&output).unwrap(); std::io::stderr().write_all(&diag.bytes).unwrap();
    if result.is_err() { std::process::exit(1); }
}
'''


class GendwarfMetadataTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        support.require_c_headers()
        cls.temp = tempfile.TemporaryDirectory(prefix="gendwarf-metadata-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.work = Path(cls.temp.name)
        c, rust = cls.work / "metadata.c", cls.work / "metadata.rs"
        c.write_text(C_HARNESS)
        rust.write_text(RUST_HARNESS.replace("@ROOT@", str(support.ROOT)).replace("@SOURCE@", str(support.SOURCE)))
        cls.tools = (cls.work / "metadata-c", cls.work / "metadata-rust")
        subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2", "-I" + str(support.SOURCE),
                        "-I" + str(support.ROOT / "scripts/include"), *support.cflags(), str(c),
                        str(support.SOURCE / "symbols.c"), str(support.SOURCE / "kabi.c"),
                        *support.libraries(), "-o", str(cls.tools[0])], check=True)
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O",
                        "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
                        str(rust), "-o", str(cls.tools[1])], check=True)

    def compare(self, mode="symbols", data=None, exports=b"", commands=(), stable=True):
        path = self.work / "fixture.o"
        if data is not None:
            path.write_bytes(data)
        args = [mode, str(path) if data is not None else "-", str(int(stable)), *commands]
        results = [subprocess.run([str(tool), *args], input=exports, capture_output=True) for tool in self.tools]
        for attr in ("returncode", "stdout", "stderr"):
            self.assertEqual(getattr(results[0], attr), getattr(results[1], attr),
                             (mode, args[3:], attr, results[0].stderr, results[1].stderr))
        return results[0]

    def test_export_lines_duplicates_whitespace_and_bytes(self):
        cases = [b"", b"foo", b"foo\nfoo\nbar\n", b" \t\v\f\rfoo ignored tail\nbar\0tail\n",
                 b"\n", b" \t\n", b"\0x\n", b"foo\n\nbar", b"a\xff\na\x80\n", b"foo\rbar\n",
                 b"foo\n__gendwarfksyms_ptr_foo\n", b"__gendwarfksyms_ptr_foo\nfoo\n"]
        for exports in cases:
            with self.subTest(exports=exports):
                self.compare(exports=exports)
        rng = random.Random(0x57D1)
        names = [bytes(rng.choice([*range(33, 127), *range(128, 256)]) for _ in range(rng.randrange(1, 40)))
                 for _ in range(3000)]
        self.compare(exports=b"\n".join(names + names[::3]))

    def test_symbol_aliases_states_and_crc_overrides(self):
        commands = ["g", "first", "0", "p", "first", "0x345", "d", "alias", "0x678",
                    "g", "alias", "0", "c", "first", "0x12345678", "c", "alias", "0x87654321",
                    "g", "other", "0", "d", "other", "0x999"]
        for bits in (32, 64):
            for endian in ("little", "big"):
                for extended in (False, True):
                    image = Elf(bits, endian)
                    text = image.section(".text", bytes(32))
                    for name, value in (("first", 0), ("alias", 0), ("other", 4), ("third", 0)):
                        image.symbol(name, text, value=value)
                    image.symbol("local", text, binding=0)
                    image.symbol("undefined")
                    image.symbol("weak", text, value=8, binding=2)
                    image.symbol("absolute", 0xfff1, value=0x1234)
                    image.symbol("common", 0xfff2, value=16)
                    exports = b"first\nalias\nthird\nother\nlocal\nundefined\nweak\nabsolute\ncommon\n"
                    with self.subTest(bits=bits, endian=endian, extended=extended):
                        self.compare(data=image.build(extended), exports=exports, commands=commands)

    def test_malformed_export_fails_before_stdin_eof(self):
        results = []
        for tool in self.tools:
            process = subprocess.Popen([str(tool), "symbols", "-", "0"], stdin=subprocess.PIPE,
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            try:
                process.stdin.write(b"valid\n\n")
                process.stdin.flush()
                process.wait(timeout=5)
                results.append((process.returncode, process.stdout.read(), process.stderr.read()))
            finally:
                if process.poll() is None:
                    process.kill()
                    process.wait()
                process.stdin.close()
                process.stdout.close()
                process.stderr.close()
        self.assertEqual(results[0], results[1])
        self.assertEqual(results[0][0], 1)

    def test_multiple_addresses_pointer_names_and_random_alias_sets(self):
        for reverse in (False, True):
            image = Elf()
            names = [("a", 1), ("b", 1), ("a", 2), ("__gendwarfksyms_ptr_b", 3)]
            for name, value in reversed(names) if reverse else names:
                image.symbol(name, 0xfff1, value=value)
            self.compare(data=image.build(), exports=b"a\nb\n", commands=["c", "a", "5"])
        rng = random.Random(0xD1E)
        for _ in range(50):
            image = Elf(rng.choice((32, 64)), rng.choice(("little", "big")))
            names = [f"symbol_{i}" for i in range(30)]
            definitions = [(name, rng.randrange(8)) for name in names] * 2
            rng.shuffle(definitions)
            for name, value in definitions:
                image.symbol(name, 0xfff1, value=value)
            commands = sum((["g", name, "0", "c", name, str(rng.getrandbits(32))] for name in names), [])
            self.compare(data=image.build(), exports="\n".join(names).encode(), commands=commands)

    @staticmethod
    def rules(records, bits=64, endian="little"):
        image = Elf(bits, endian)
        image.section(".discard.gendwarfksyms.kabi_rules", records, flags=0)
        return image.build()

    def test_all_rule_kinds_duplicates_targets_and_byte_values(self):
        records = [(b"declonly", b"struct name", b"unused"), (b"enumerator_ignore", b"enum NAME", b""),
                   (b"enumerator_value", b"enum NAME", b"-1"), (b"byte_size", b"struct name", b"34"),
                   (b"byte_size", b"struct name", b"56"), (b"type_string", b"s#weird\xff", b"bytes\xff\x80"),
                   (b"declonly", b"", b""), (b"enumerator_ignore", b" one", b"")]
        data = b"".join(b"1\0" + b"\0".join(row) + b"\0" for row in records)
        commands = ["d", "struct name", "", "d", "missing", "", "d", "", "", "i", "enum", "NAME",
                    "i", "enum", "missing", "i", "", "one", "e", "enum", "NAME", "b", "struct name", "",
                    "b", "missing", "", "t", b"s#weird\xff", "", "t", "missing", ""]
        for bits in (32, 64):
            for endian in ("little", "big"):
                for stable in (False, True):
                    self.compare("rules", self.rules(data, bits, endian), commands=commands, stable=stable)

    def test_rule_unsigned_conversion(self):
        values = [b"", b"0", b"0001", b"-0", b"-1", b"+1", b"  \t\v\f\r\n42", b"+", b"-", b" ",
                  b"0x10", b"12 ", b"1\xff", b"18446744073709551615", b"18446744073709551616",
                  b"-18446744073709551615", b"-18446744073709551616", b"9" * 1000]
        values += [str(random.Random(seed).getrandbits(70)).encode() for seed in range(60)]
        for value in values:
            for tag, command, target, field in ((b"byte_size", "b", b"target", ""),
                                                 (b"enumerator_value", "e", b"target FIELD", "FIELD")):
                with self.subTest(value=value, tag=tag):
                    self.compare("rules", self.rules(b"1\0" + tag + b"\0" + target + b"\0" + value + b"\0"),
                                 commands=[command, "target", field])

    def test_rule_errors_and_trailing_records(self):
        records = [b"", b"x", b"12345", b"123456", b"12345\0", b"1\0x\0\0\0",
                   b"2\0declonly\0name\0\0", b"1\0bad\0target\0\0", b"1\0declonly\0name\0",
                   b"1\0declonly\0", b"1\0declonly\0name\0value", b"1\0declonly\0name\0value\0"]
        records += [b"1\0declonly\0name\0\0" + bytes(size) for size in range(1, 9)]
        for data in records:
            with self.subTest(data=data):
                self.compare("rules", self.rules(data))
        self.compare("rules", Elf().build())

    def test_non_elf_inputs_are_rejected_without_c_crashes(self):
        for data in (b"", b"not an elf\n", b"!<arch>\n"):
            with self.subTest(data=data):
                path = self.work / "invalid.o"
                path.write_bytes(data)
                # libelf's gelf_fsize on ELF_K_NONE/AR crashes in the original
                # symbol scanner. Rust must reject these inputs normally.
                result = subprocess.run([str(self.tools[1]), "symbols", str(path), "1"],
                                        input=b"exported\n", capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b"error: gendwarfksyms: elf_for_each_global:", result.stderr)
                self.assertNotIn(b"panicked", result.stderr)

    def test_all_elf_object_kinds(self):
        for kind in (0, 1, 2, 3, 4, 0xfe00, 0xff00, 0xffff):
            image = Elf()
            image.symbol("exported", 0xfff1, value=32)
            data = bytearray(image.build())
            struct.pack_into("<H", data, 16, kind)
            with self.subTest(kind=kind):
                self.compare(data=data, exports=b"exported\n")
                self.compare("rules", data)

    def test_shared_parser_still_enforces_existing_type_masks(self):
        for bits in (32, 64):
            for endian in ("little", "big"):
                image = Elf(bits, endian)
                data = bytearray(image.build())
                path = self.work / "type-mask.o"
                for kind in (0, 1, 2, 3, 4, 31, 32, 0xfe00, 0xffff):
                    struct.pack_into(image.order + "H", data, 16, kind)
                    path.write_bytes(data)
                    result = subprocess.run([str(self.tools[1]), "parse", str(path), "0"], capture_output=True)
                    self.assertEqual(result.returncode, 0, result.stderr)
                    expected = f"true {str(kind == 1).lower()} {str(kind in (2, 3)).lower()}\n".encode()
                    self.assertEqual(result.stdout, expected)


if __name__ == "__main__":
    unittest.main()
