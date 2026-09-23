# SPDX-License-Identifier: GPL-2.0
"""Owned DIE-cache and symtypes/CRC differential tests against original C."""

import os
from pathlib import Path
import random
import re
import shlex
import struct
import subprocess
import tempfile
import unittest

from gendwarf_test_support import build_c, build_rust, cflags, require_c_headers

ROOT = Path(__file__).resolve().parents[2]

C_HARNESS = r'''
#define _GNU_SOURCE
#include <stdint.h>
#include <string.h>
#include "gendwarfksyms.h"
int debug, dump_dies, dump_die_map, dump_types, dump_versions, stable, symtypes;
#include "cache.c"
#include "die.c"
#include "symbols.c"
#include "kabi.c"
#include "types.c"
static uint64_t number(int count) {
    uint64_t value = 0;
    for (int i = 0; i < count; i++) {
        int c = getchar(); if (c == EOF) exit(99);
        value |= (uint64_t)c << (i * 8);
    }
    return value;
}
static char *blob(size_t *length) {
    size_t n = number(4); char *s = xmalloc(n + 1);
    if (fread(s, 1, n, stdin) != n) exit(99);
    s[n] = 0; if (length) *length = n; return s;
}
static void cache_test(void) {
    struct cache cache; cache_init(&cache);
    for (size_t rounds = number(4); rounds; rounds--) {
        for (size_t n = number(4); n; n--) {
            unsigned long key = number(8); int value = number(4);
            cache_set(&cache, key, value);
        }
        for (size_t n = number(4); n; n--) printf("%d\n", cache_get(&cache, number(8)));
        cache_free(&cache);
    }
}
static struct die *known[20000];
static size_t count_known;
static int known_id(struct die *d) {
    if (!d) return -1;
    for (size_t i = 0; i < count_known; i++) if (known[i] == d) return i;
    known[count_known] = d; return count_known++;
}
static void dump_entry(struct die *d, void *arg) {
    (void)arg;
    printf("%d %zu %u\n", known_id(d), d->addr, d->state);
}
static void die_test(void) {
    debug = 1;
    for (size_t n = number(4); n; n--) {
        unsigned op = number(1);
        if (op == 3) { die_map_free(); count_known = 0; }
        else if (op == 2) die_map_for_each(dump_entry, NULL);
        else {
            Dwarf_Die input = {.addr = (void *)(uintptr_t)number(8)};
            enum die_state want = number(4);
            struct die *d = NULL;
            if (op == 0) { d = die_map_get(&input, want); d->state = number(4); }
            else __die_map_get((uintptr_t)input.addr, want, &d);
            printf("%d\n", known_id(d));
        }
    }
}
int main(void) {
    unsigned flags = number(4);
    if (flags >> 31) { cache_test(); return 0; }
    if (flags & (1U << 30)) { die_test(); return 0; }
    debug = !!(flags & 1); dump_types = !!(flags & 2);
    dump_versions = !!(flags & 4); stable = !!(flags & 8); symtypes = !!(flags & 16);
    for (size_t n = number(4); n; n--) {
        Dwarf_Die input = {.addr = (void *)(uintptr_t)number(8)};
        enum die_state want = number(4), state = number(4);
        int tag = number(4); char *name = blob(NULL);
        struct die *d = die_map_get(&input, want);
        d->state = state; d->tag = tag; d->fqn = *name ? name : NULL;
        for (size_t fragments = number(4); fragments; fragments--) {
            int kind = number(1);
            if (kind == 0) { char *s = blob(NULL); die_map_add_string(d, s); free(s); }
            else if (kind == 1) die_map_add_linebreak(d, number(4));
            else { struct die child = {.addr = number(8)}; die_map_add_die(d, &child); }
        }
    }
    size_t length; char *exports = blob(&length);
    FILE *export_file = fmemopen(exports, length, "r");
    symbol_read_exports(export_file); fclose(export_file); free(exports);
    for (size_t n = number(4); n; n--) {
        char *name = blob(NULL); uintptr_t addr = number(8);
        struct symbol *s = symbol_get(name);
        if (addr != UINT64_MAX) { Dwarf_Die d = {.addr = (void *)addr}; symbol_set_die(s, &d); }
        free(name);
    }
    char *rules_blob = blob(&length);
    FILE *rules_file = tmpfile(); fwrite(rules_blob, 1, length, rules_file); fflush(rules_file);
    kabi_read_rules(fileno(rules_file)); fclose(rules_file); free(rules_blob);
    generate_symtypes_and_versions(flags & 32 ? stdout : NULL);
    symbol_print_versions();
    if (flags & 64) die_map_free();
    return 0;
}
'''

RUST_HARNESS = r'''
//! Owned graph fixture harness for the unmodified gendwarfksyms algorithms.
#[path="SOURCE/gendwarfksyms_header.rs"] #[allow(dead_code)] mod gendwarfksyms_header;
#[path="ELF"] #[allow(dead_code)] mod elf;
#[path="SOURCE/cache.rs"] mod cache;
#[path="SOURCE/die.rs"] #[allow(dead_code)] mod die;
#[path="SOURCE/symbols.rs"] #[allow(dead_code)] mod symbols;
#[path="SOURCE/kabi.rs"] #[allow(dead_code)] mod kabi;
#[path="SOURCE/types.rs"] mod types;
use gendwarfksyms_header::{Diagnostics, Options, Result};
use die::{DieMap, DieState, Fragment};
use std::io::{self, Read, Write};
struct Input { data: Vec<u8>, at: usize }
impl Input {
    fn number(&mut self, count: usize) -> u64 {
        let mut bytes = [0; 8]; bytes[..count].copy_from_slice(&self.data[self.at..self.at+count]);
        self.at += count; u64::from_le_bytes(bytes)
    }
    fn blob(&mut self) -> Vec<u8> {
        let n = self.number(4) as usize; let bytes = self.data[self.at..self.at+n].to_vec();
        self.at += n; bytes
    }
}
fn state(n: u64) -> DieState {
    match n { 0 => DieState::Incomplete, 1 => DieState::Fqn, 2 => DieState::Unexpanded,
        3 => DieState::Complete, 4 => DieState::Symbol, _ => panic!("bad fixture") }
}
fn run(input: &mut Input, flags: u64, diag: &mut Diagnostics, out: &mut Vec<u8>) -> Result<()> {
    let mut dies = DieMap::default();
    for _ in 0..input.number(4) {
        let addr = input.number(8) as usize;
        let want = state(input.number(4)); let actual = state(input.number(4));
        let tag = input.number(4) as i32; let name = input.blob();
        let id = dies.get_or_insert(addr, want);
        let d = &mut dies.entries[id]; d.state = actual; d.tag = tag;
        d.fqn = (!name.is_empty()).then_some(name);
        for _ in 0..input.number(4) {
            let fragment = match input.number(1) { 0 => Fragment::String(input.blob()),
                1 => Fragment::Linebreak(input.number(4) as i32), _ => Fragment::Die(input.number(8) as usize) };
            d.fragments.push(fragment);
        }
    }
    let mut symbols = symbols::Symbols::read_exports(input.blob().as_slice(), diag)?;
    for _ in 0..input.number(4) {
        let name = input.blob(); let addr = input.number(8);
        if addr != u64::MAX { let id = symbols.get(&name).unwrap(); symbols.set_die(id, addr as usize)?; }
    }
    let rules = kabi::Rules::read(&input.blob(), diag)?;
    types::generate(&mut dies, &mut symbols, &rules, diag,
        if flags & 32 != 0 { Some(out) } else { None })?;
    symbols.print_versions(out, diag)?;
    if flags & 64 != 0 { dies.clear(diag); }
    Ok(())
}
fn main() {
    let mut data = Vec::new(); io::stdin().read_to_end(&mut data).unwrap();
    let mut input = Input { data, at: 0 }; let flags = input.number(4);
    if flags >> 31 != 0 {
        let mut c = cache::Cache::default();
        for _ in 0..input.number(4) {
            for _ in 0..input.number(4) { let key = input.number(8) as usize; let value = input.number(4) as i32; c.set(key,value); }
            for _ in 0..input.number(4) { println!("{}", c.get(input.number(8) as usize)); }
            c.clear();
        }
        return;
    }
    if flags & (1 << 30) != 0 {
        let mut dies = DieMap::default();
        let mut diag = Diagnostics::new(Options { debug: true, ..Options::default() });
        for _ in 0..input.number(4) {
            match input.number(1) {
                3 => dies.clear(&mut diag),
                2 => for id in dies.ordered_indices() {
                    println!("{} {} {}", id, dies.entries[id].addr, dies.entries[id].state as u32);
                },
                op => {
                    let addr = input.number(8) as usize; let want = state(input.number(4));
                    let id = if op == 0 {
                        let id = dies.get_or_insert(addr, want); dies.entries[id].state = state(input.number(4)); Some(id)
                    } else { dies.find(addr, want) };
                    println!("{}", id.map_or(-1, |id| id as i32));
                }
            }
        }
        io::stderr().write_all(&diag.bytes).unwrap();
        return;
    }
    let mut diag = Diagnostics::new(Options { debug: flags & 1 != 0,
        dump_types: flags & 2 != 0, dump_versions: flags & 4 != 0,
        stable: flags & 8 != 0, symtypes: flags & 16 != 0, ..Options::default() });
    let mut out = Vec::new();
    let result = run(&mut input, flags, &mut diag, &mut out);
    io::stdout().write_all(&out).unwrap(); io::stderr().write_all(&diag.bytes).unwrap();
    if let Err(error) = result { io::stderr().write_all(&error.0).unwrap(); std::process::exit(1); }
}
'''


def number(value, size=4):
    return (value & ((1 << (8 * size)) - 1)).to_bytes(size, "little")


def blob(value):
    return number(len(value)) + value


def rules_elf(rules):
    payload = b"".join(b"1\0type_string\0" + name + b"\0" + value + b"\0" for name, value in rules)
    names = b"\0.shstrtab\0.discard.gendwarfksyms.kabi_rules\0"
    offset = 64 + len(names) + len(payload)
    header = b"\x7fELF\x02\x01\x01" + b"\0" * 9
    header += struct.pack("<HHIQQQIHHHHHH", 1, 62, 1, 0, 0, offset, 0, 64, 0, 0, 64, 3 if rules else 2, 1)
    sections = bytes(64) + struct.pack("<IIQQQQIIQQ", 1, 3, 0, 0, 64, len(names), 0, 0, 1, 0)
    if rules:
        sections += struct.pack("<IIQQQQIIQQ", 11, 1, 0, 0, 64 + len(names), len(payload), 0, 0, 1, 0)
    return header + names + payload + sections


def die(addr, state=3, tag=0x13, name=b"T", fragments=(), want=None):
    return addr, state if want is None else want, state, tag, name, fragments


def graph(dies, symbols=((b"exported", 1),), rules=(), flags=127):
    data = bytearray(number(flags) + number(len(dies)))
    for addr, want, state, tag, name, fragments in dies:
        data += number(addr, 8) + number(want) + number(state) + number(tag) + blob(name)
        data += number(len(fragments))
        for fragment in fragments:
            if isinstance(fragment, bytes):
                data += b"\0" + blob(fragment)
            elif isinstance(fragment, tuple):
                data += b"\1" + number(fragment[0])
            else:
                data += b"\2" + number(fragment, 8)
    data += blob(b"".join(name + b"\n" for name, _ in symbols)) + number(len(symbols))
    for name, addr in symbols:
        data += blob(name) + number(addr, 8)
    return data + blob(rules_elf(rules))


class GendwarfTypesTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        require_c_headers()
        cls.temporary = tempfile.TemporaryDirectory(prefix="gendwarf-types-")
        cls.work = Path(cls.temporary.name)
        source = ROOT / "scripts/gendwarfksyms"
        c_source, rust_source = cls.work / "harness.c", cls.work / "harness.rs"
        c_source.write_text(C_HARNESS)
        rust_source.write_text(RUST_HARNESS.replace("SOURCE", str(source)).replace("ELF", str(ROOT / "scripts/elf-parse.rs")))
        cls.c, cls.rust = cls.work / "types-c", cls.work / "types-rust"
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        subprocess.run(cc + ["-O2", "-I" + str(source), "-I" + str(ROOT / "scripts/include")] + cflags() +
                       [str(c_source), "-lelf", "-lz", "-o", str(cls.c)], check=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + ["--edition=2021", "-O",
                       "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
                       str(rust_source), "-o", str(cls.rust)], check=True)

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def compare(self, data):
        results = [subprocess.run([tool], input=data, capture_output=True, timeout=20) for tool in (self.c, self.rust)]
        normalize = lambda value: re.sub(rb"cache entry: 0x[0-9a-f]+", b"cache entry: <pointer>", value)
        self.assertEqual((results[1].returncode, results[1].stdout, normalize(results[1].stderr)),
                         (results[0].returncode, results[0].stdout, normalize(results[0].stderr)))
        return results[1]

    def test_primitive_crc_boundaries_and_all_option_combinations(self):
        for size in (0, 1, 2, 15, 16, 31, 32, 63, 64, 65, 255, 256, 4095, 4096, 65537):
            code = bytes(33 + i % 90 for i in range(size))
            self.compare(graph([die(1, 4, name=b"", fragments=[code])]))
        fixture = [die(1, 4, name=b"", fragments=[b"pointer ", 2]), die(2, fragments=[b"structure X", (1,), b"{} "])]
        for flags in range(128):
            self.compare(graph(fixture, flags=flags))

    def test_type_tags_quotes_duplicate_names_and_longest_definition(self):
        records = [die(1, 4, name=b"", fragments=[2, b" ", 3, b" ", 4, b" ", 5, b" ", 6])]
        for addr, tag in ((2, 2), (3, 0x13), (4, 0x17), (5, 4), (6, 0x16)):
            records.append(die(addr, tag=tag, name=b"name with space", fragments=[f"type{addr}".encode()]))
        records += [die(7, name=b"name with space", fragments=[b"longer type expansion"]),
                    die(8, name=b"name with space", fragments=[b"different same length"])]
        self.compare(graph(records))
        self.compare(graph([*records, die(9, name=b"byte_\xff", fragments=[b"a"])]))

    def test_named_cycles_repeated_references_and_anonymous_expansion(self):
        records = [die(1, 4, name=b"", fragments=[2, b" ", 2, b" ", 3]),
                   die(2, name=b"A", fragments=[b"struct A {", 3, b";}"]),
                   die(3, name=b"B", fragments=[b"union B {", 2, b";}"])]
        self.compare(graph(records))
        records += [die(4, tag=0x0f, name=b"", fragments=[b"const ", 2, (1,), (2,), (-1,), b"done"])]
        records[0] = die(1, 4, name=b"", fragments=[4, 4])
        self.compare(graph(records))

    def test_complete_unexpanded_lookup_and_fixed_insertion_bucket(self):
        for order in (False, True):
            definitions = [die(2, 2, fragments=[b"incomplete"]), die(2, 3, fragments=[b"complete type"])]
            if order:
                definitions.reverse()
            self.compare(graph([die(1, 4, name=b"", fragments=[2]), *definitions]))
        self.compare(graph([die(1, 4, name=b"", fragments=[2]), die(2, 3, want=0, fragments=[b"mismatched bucket"])]))
        self.compare(graph([die(1, 4, name=b"", fragments=[b"int"]), die(2, 0), die(2, 0), die(4, 1)]))

    def test_override_parser_and_rule_only_references(self):
        records = [die(1, 4, name=b"", fragments=[2]), die(2, fragments=[b"original"])]
        for override in (b"plain text", b"pointer s#Other", b"s#'Other type'", b"pre s#Other post",
                         b"s#Other s#Other", b"s#'Other type's#Other", b"s#Other\todd"):
            rules = [(b"s#T", override), (b"s#Other", b"other"), (b"s#'Other type'", b"quoted"),
                     (b"s#Other\todd", b"tabs are not delimiters")]
            self.compare(graph(records, rules=rules))
        self.compare(graph(records, rules=[(b"exported", b"symbol override")]))
        self.compare(graph(records, rules=[(b"s#T", b"first"), (b"s#T", b"last")]))
        self.compare(graph(records, rules=[(b"s#T", b"s#A"), (b"s#A", b"s#B"), (b"s#B", b"s#A")]))

    def test_errors_for_missing_children_and_invalid_type_overrides(self):
        records = [die(1, 4, name=b"", fragments=[2]), die(2, fragments=[b"original"])]
        for override in (b"", b"s#", b"s# ", b"s#''", b"s#'unterminated", b"u#'", b"t#Missing"):
            self.compare(graph(records, rules=[(b"s#T", override)]))
            self.compare(graph(records, rules=[(b"exported", override)]))
        self.compare(graph([die(1, 4, name=b"", fragments=[0xdeadbeef])]))
        self.compare(graph([die(1, 4, name=b"", fragments=[b"s#Missing"])]))
        self.compare(graph([die(1, 4, name=b"", fragments=[b"int"])], symbols=[(b"missing", -1)]))

    def test_large_random_dags_duplicate_type_names_and_hash_collisions(self):
        rng = random.Random(0xcac4e)
        for _ in range(20):
            records = [die(1, 4, name=b"", fragments=[2, 3, 4])]
            for addr in range(2, 100):
                fragments = [b"{" + str(addr).encode(), (rng.randrange(-4, 4),)]
                fragments += [rng.randrange(addr + 1, 101) if addr < 100 else b"int"]
                fragments += [b"}"]
                records.append(die(addr, name=f"T{rng.randrange(20)}".encode(), fragments=fragments))
            records.append(die(100, name=b"Last", fragments=[b"int"]))
            rng.shuffle(records)
            self.compare(graph(records))
        records = [die(1, 4, name=b"", fragments=[0x10002])]
        records += [die((i << 16) + 2, name=f"Type{i}".encode(), fragments=[str(i).encode()]) for i in range(5000)]
        self.compare(graph(records))

    def test_integer_cache_last_write_missing_and_clear(self):
        rng = random.Random(0xca5e)
        data = bytearray(number(1 << 31) + number(10))
        for _ in range(10):
            entries = [(rng.randrange(200) * 1024, rng.randrange(-100, 100)) for _ in range(2000)]
            data += number(len(entries))
            for key, value in entries:
                data += number(key, 8) + number(value)
            queries = [rng.randrange(300) * 1024 for _ in range(2000)]
            data += number(len(queries))
            for key in queries:
                data += number(key, 8)
        self.compare(data)

    def test_die_map_bucket_order_state_changes_hits_and_cumulative_clears(self):
        rng = random.Random(0xd1e)
        commands = []
        for cycle in range(5):
            for _ in range(2000):
                addr = rng.choice((rng.randrange(50), rng.randrange(100) * 65536 + 2, 0x100000001))
                want = rng.randrange(5)
                op = rng.randrange(2)
                command = bytes([op]) + number(addr, 8) + number(want)
                if op == 0:
                    command += number(want if rng.randrange(3) else rng.randrange(5))
                commands.append(command)
            commands += [b"\2", b"\3", b"\3"]
        self.compare(number(1 << 30) + number(len(commands)) + b"".join(commands))

    def test_safe_deep_anonymous_chain_and_malformed_cycle(self):
        # C recursively expands an anonymous chain and may overflow its stack;
        # the Rust implementation uses explicit traversal frames.
        records = [die(1, 4, name=b"", fragments=[2])]
        records += [die(i, tag=0x0f, name=b"", fragments=[i + 1]) for i in range(2, 20000)]
        records.append(die(20000, tag=0x0f, name=b"", fragments=[b"int"]))
        result = subprocess.run([self.rust], input=graph(records, flags=48), capture_output=True, timeout=20)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(b"exported int\n", result.stdout)
        records = [die(1, 4, name=b"", fragments=[2]), die(2, tag=0x0f, name=b"", fragments=[2])]
        result = subprocess.run([self.rust], input=graph(records), capture_output=True, timeout=20)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b"recursive anonymous type", result.stderr)


def dwarf_node(label, tag, name=None, target=None, children=(), attrs=()):
    attributes = list(attrs)
    if name is not None:
        attributes.append((3, 8, name))
    if target is not None:
        attributes.append((0x49, 0x13, target))
    return label, tag, attributes, list(children)


def dwarf_object(units):
    """Build explicit DWARF4 scopes/references that compilers rarely emit."""
    from test_gendwarf_reader import elf_debug

    def leb(value):
        result = bytearray()
        while value >= 128:
            result.append((value & 127) | 128)
            value >>= 7
        return result + bytes([value])

    abbreviations = bytearray()
    info = bytearray()
    next_abbreviation = 1
    for roots in units:
        root = dwarf_node("cu", 0x11, b"fixture.h", children=roots)
        body = bytearray(b"\4\0\0\0\0\0\10")
        positions, references = {}, []
        stack = [root]
        while stack:
            current = stack.pop()
            if current is None:
                body += b"\0"
                continue
            label, tag, attrs, children = current
            positions[label] = 4 + len(body)
            code = next_abbreviation
            next_abbreviation += 1
            abbreviations += leb(code) + leb(tag) + bytes([bool(children)])
            body += leb(code)
            for attr, form, value in attrs:
                abbreviations += leb(attr) + leb(form)
                if form == 8:
                    body += value + b"\0"
                elif form == 0x13:
                    references.append((len(body), value))
                    body += bytes(4)
                elif form in (0x0b, 0x0c):
                    body += bytes([value])
                elif form == 0x19:
                    pass
                elif form == 0x0f:
                    body += leb(value)
                else:
                    raise ValueError(form)
            abbreviations += b"\0\0"
            if children:
                stack.append(None)
                stack.extend(reversed(children))
        for offset, label in references:
            body[offset:offset + 4] = number(positions[label])
        info += number(len(body)) + body
    return elf_debug(info, abbreviations + b"\0")


class GendwarfEngineAuditTests(unittest.TestCase):
    """Integrated edge semantics of the explicit-stack DWARF processor."""

    @classmethod
    def setUpClass(cls):
        require_c_headers()
        cls.temporary = tempfile.TemporaryDirectory(prefix="gendwarf-engine-audit-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.tools = build_c(cls.work), build_rust(cls.work)

    def compare(self, units, exports=b"exported\n", options=(), status=0, copies=1):
        obj = self.work / "explicit.o"
        obj.write_bytes(dwarf_object(units))
        types = self.work / "explicit.symtypes"
        results = []
        for tool in self.tools:
            result = subprocess.run([tool, *options, "-T", types, *([obj] * copies)], input=exports,
                                    capture_output=True, timeout=20)
            results.append((result.returncode, result.stdout, result.stderr, types.read_bytes()))
        self.assertEqual(results[1], results[0])
        self.assertEqual(results[1][0], status, results[1][2])
        return results[1]

    def test_anonymous_and_empty_scopes_and_scope_reset(self):
        for namespace in (None, b"", b"named", b"byte_\xff"):
            inner = dwarf_node("inner", 0x13, b"I", children=[dwarf_node("member", 0x0d, b"m", "int")])
            outer = dwarf_node("outer", 0x13, b"O", children=[inner, dwarf_node("field", 0x0d, b"f", "inner")])
            nested = dwarf_node("scope", 0x39, namespace, children=[outer])
            roots = [dwarf_node("parent", 0x39, b"N", children=[nested]),
                     dwarf_node("int", 0x24, b"int", attrs=[(0x0b, 0x0b, 4)]),
                     dwarf_node("export", 0x34, b"exported", "outer")]
            for options in ((), ("--dump-dies",), ("--stable", "--dump-versions")):
                self.compare([roots], options=options)
        # Union and enum names do not establish a scope in the original tool.
        roots = [dwarf_node("ns", 0x39, b"N", children=[
                    dwarf_node("union", 0x17, b"U", children=[dwarf_node("inner", 0x13, b"I")])]),
                 dwarf_node("export", 0x34, b"exported", "inner")]
        result = self.compare([roots])
        self.assertIn(b"s#I structure_type I", result[3])
        self.assertNotIn(b"N::U::I", result[3])

    def test_all_modifiers_and_numeric_attribute_errors(self):
        for tag in (0x0f, 0x10, 0x16, 0x26, 0x2d, 0x2f, 0x35, 0x37, 0x40, 0x42, 0x47, 0x4b):
            roots = [dwarf_node("int", 0x24, b"raw_\xff", attrs=[(0x0b, 0x0f, (1 << 64) - 1), (0x3e, 0x0b, 5)]),
                     dwarf_node("modifier", tag, b"modifier with spaces", "int", attrs=[(0x88, 0x0b, 16)]),
                     dwarf_node("export", 0x34, b"exported", "modifier")]
            self.compare([roots], options=("--dump-dies", "--dump-versions"))
        for unsupported in (0x0b, 0x1c, 0x46, 0x4080):
            self.compare([[dwarf_node("bad", unsupported), dwarf_node("export", 0x34, b"exported", "bad")]],
                         status=1, options=("--dump-dies",))
        self.compare([[dwarf_node("struct", 0x13, b"S", children=[dwarf_node("bad", 0x1c)]),
                       dwarf_node("export", 0x34, b"exported", "struct")]], status=1)

    def test_missing_member_types_declarations_and_abstract_origins(self):
        roots = [dwarf_node("struct", 0x13, b"S", children=[dwarf_node("member", 0x0d, b"missing")]),
                 dwarf_node("export", 0x34, b"exported", "struct")]
        self.compare([roots])
        self.compare([roots], options=("--stable",), status=1)
        for declaration in (0, 1, 2, 255):
            roots[0] = dwarf_node("struct", 0x13, b"S", attrs=[(0x3c, 0x0c, declaration)],
                                  children=[dwarf_node("member", 0x0d, b"missing")])
            self.compare([roots], options=("--stable",), status=int(declaration == 0))
        roots = [dwarf_node("base", 0x24, b"int"),
                 dwarf_node("origin", 0x34, b"exported", "base"),
                 dwarf_node("use", 0x34, None, attrs=[(0x31, 0x13, "origin")])]
        for order in (roots, list(reversed(roots))):
            self.compare([order], options=("--dump-dies", "--dump-versions"))

    def test_repeated_units_and_recursive_aggregate_state_transitions(self):
        units = []
        for index in range(4):
            fields = [dwarf_node("member", 0x0d, b"next", "pointer")]
            if index > 1:
                fields.append(dwarf_node("value", 0x0d, b"value", "int"))
            units.append([dwarf_node("type", 0x13, b"same", children=fields),
                          dwarf_node("pointer", 0x0f, target="alias"),
                          dwarf_node("alias", 0x16, b"alias", "type"),
                          dwarf_node("int", 0x24, b"int"),
                          dwarf_node("export", 0x34, f"exported{index}".encode(), "alias")])
        exports = b"exported0\nexported1\nexported2\nexported3\n"
        for order in (units, list(reversed(units))):
            self.compare(order, exports, options=("--dump-dies", "--dump-versions"), copies=2)

    def test_cycles_without_aggregate_boundary_are_rejected(self):
        for tag in (0x0f, 0x16, 0x26):
            obj = self.work / "cyclic.o"
            obj.write_bytes(dwarf_object([[dwarf_node("cycle", tag, b"cycle", "cycle"),
                                          dwarf_node("export", 0x34, b"exported", "cycle")]]))
            result = subprocess.run([self.tools[1], obj], input=b"exported\n", capture_output=True, timeout=5)
            self.assertEqual(result.returncode, 1, result.stderr)
            self.assertIn(b"cyclic DWARF type without an aggregate boundary", result.stderr)

    def test_source_file_privacy_cache_is_reset_between_compilation_units(self):
        objects = []
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        for name, extension in (("private", ".c"), ("public", ".h")):
            source = self.work / (name + extension)
            source.write_text(f"struct {name}_type {{ int member; }}; struct {name}_type {name}_value;\n")
            obj = self.work / (name + ".o")
            subprocess.run([*cc, "-x", "c", "-g", "-gdwarf-4", "-c", source, "-o", obj], check=True)
            objects.append(obj)
        for order in (objects, list(reversed(objects))):
            obj = self.work / "combined.o"
            subprocess.run([*shlex.split(os.environ.get("LD", "ld")), "-r", *order, "-o", obj], check=True)
            symtypes = self.work / "combined.symtypes"
            results = []
            for tool in self.tools:
                result = subprocess.run([tool, "--dump-versions", "-T", symtypes, obj],
                                        input=b"private_value\npublic_value\n", capture_output=True, timeout=20)
                results.append((result.returncode, result.stdout, result.stderr, symtypes.read_bytes()))
            self.assertEqual(results[1], results[0])
            self.assertEqual(results[1][0], 0, results[1][2])
            self.assertIn(b"s#private_type structure_type private_type { }\n", results[1][3])
            self.assertIn(b"s#public_type structure_type public_type { member", results[1][3])

    def test_deep_modifier_chain_uses_explicit_stacks(self):
        # This exceeds the original recursive processor's practical call-stack
        # depth, without depending on a C crash as the expected oracle result.
        roots = [dwarf_node("base", 0x24, b"int")]
        previous = "base"
        for index in range(20000):
            current = f"modifier{index}"
            roots.append(dwarf_node(current, 0x0f, target=previous))
            previous = current
        roots.append(dwarf_node("export", 0x34, b"exported", previous))
        obj = self.work / "deep.o"
        obj.write_bytes(dwarf_object([roots]))
        result = subprocess.run([self.tools[1], obj], input=b"exported\n", capture_output=True, timeout=20)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertRegex(result.stdout, rb"^#SYMVER exported 0x[0-9a-f]{8}\n$")


if __name__ == "__main__":
    unittest.main()
