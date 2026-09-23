# SPDX-License-Identifier: GPL-2.0
"""Independent libdw ABI, ownership, lifetime, and DWARF traversal checks."""

import os
from pathlib import Path
import random
import re
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

from gendwarf_test_support import ROOT, cflags, libraries, require_c_headers

SOURCE = ROOT / "scripts/gendwarfksyms"

C_ABI = r'''
#include <stddef.h>
#include <stdio.h>
#include <dwarf.h>
#include <elfutils/libdw.h>
#include <elfutils/libdwfl.h>
#define RECORD(T,F) printf("%zu ", offsetof(T,F))
int main(void) {
    printf("%zu %zu ", sizeof(Dwarf_Die), _Alignof(Dwarf_Die));
    RECORD(Dwarf_Die, addr); RECORD(Dwarf_Die, cu); RECORD(Dwarf_Die, abbrev); RECORD(Dwarf_Die, padding__);
    printf("%zu %zu ", sizeof(Dwarf_Attribute), _Alignof(Dwarf_Attribute));
    RECORD(Dwarf_Attribute, code); RECORD(Dwarf_Attribute, form); RECORD(Dwarf_Attribute, valp); RECORD(Dwarf_Attribute, cu);
    printf("%zu %zu ", sizeof(Dwfl_Callbacks), _Alignof(Dwfl_Callbacks));
    RECORD(Dwfl_Callbacks, find_elf); RECORD(Dwfl_Callbacks, find_debuginfo); RECORD(Dwfl_Callbacks, section_address); RECORD(Dwfl_Callbacks, debuginfo_path);
    printf("%zu %zu %zu %zu %zu %zu %zu %zu %zu\n", sizeof(Dwarf_Off), sizeof(Dwarf_Addr), sizeof(Dwarf_Word),
        sizeof(Dwarf_Half), sizeof(GElf_Word), sizeof(bool), _Alignof(bool), sizeof(ptrdiff_t), sizeof(long));
    CONSTANTS
    return 0;
}
'''

RUST_ABI = r'''
pub(crate) fn abi() {
    use std::mem::{size_of, align_of, offset_of};
    print!("{} {} ", size_of::<RawDie>(), align_of::<RawDie>());
    for n in [offset_of!(RawDie, addr), offset_of!(RawDie, cu), offset_of!(RawDie, abbrev), offset_of!(RawDie, padding)] { print!("{n} "); }
    print!("{} {} ", size_of::<Attribute>(), align_of::<Attribute>());
    for n in [offset_of!(Attribute, code), offset_of!(Attribute, form), offset_of!(Attribute, value), offset_of!(Attribute, cu)] { print!("{n} "); }
    print!("{} {} ", size_of::<Callbacks>(), align_of::<Callbacks>());
    for n in [offset_of!(Callbacks, find_elf), offset_of!(Callbacks, find_debug), offset_of!(Callbacks, section_address), offset_of!(Callbacks, debug_path)] { print!("{n} "); }
    println!("{} {} {} {} {} {} {} {} {}", size_of::<u64>(), size_of::<u64>(), size_of::<u64>(),
        size_of::<u16>(), size_of::<u32>(), size_of::<bool>(), align_of::<bool>(), size_of::<isize>(), size_of::<c_long>());
    CONSTANTS
}
'''

C_READER = r'''
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>
#include <dwarf.h>
#include <elfutils/libdw.h>
#include <elfutils/libdwfl.h>
static void hex(const char *s) { for (; s && *s; s++) printf("%02x", (unsigned char)*s); }
static void visit(Dwarf_Die *die, int depth) {
    if (depth > 1000) exit(90);
    printf("D %d %d\n", depth, dwarf_tag(die));
    Dwarf_Die copy;
    if (!dwarf_die_addr_die(dwarf_cu_getdwarf(die->cu), die->addr, &copy) || dwarf_tag(&copy) != dwarf_tag(die)) exit(91);
    for (unsigned id = 0; id < 256; id++) {
        Dwarf_Attribute a; Dwarf_Word value; bool flag;
        if (!dwarf_attr(die, id, &a)) continue;
        const char *s = dwarf_formstring(&a);
        if (s) { printf("S %u ", id); hex(s); puts(""); }
        if (!dwarf_formudata(&a, &value)) printf("U %u %llu\n", id, (unsigned long long)value);
        if (!dwarf_formflag(&a, &flag)) printf("F %u %u\n", id, flag);
        Dwarf_Die reference;
        if (dwarf_formref_die(&a, &reference)) {
            printf("R %u %d ", id, dwarf_tag(&reference)); hex(dwarf_diename(&reference)); puts("");
        }
    }
    Dwarf_Attribute attr; Dwarf_Word index;
    if (dwarf_attr(die, DW_AT_decl_file, &attr) && !dwarf_formudata(&attr, &index)) {
        Dwarf_Die cu; Dwarf_Files *files;
        const char *name = NULL;
        if (dwarf_cu_die(die->cu, &cu, NULL, NULL, NULL, NULL, NULL, NULL) &&
            !dwarf_getsrcfiles(&cu, &files, NULL)) name = dwarf_filesrc(files, index, NULL, NULL);
        printf("P %llu %d ", (unsigned long long)index, name != NULL); hex(name); puts("");
    }
    Dwarf_Die child; int status = dwarf_child(die, &child);
    if (status < 0) exit(92);
    for (int count = 0; !status; count++) {
        if (count > 100000) exit(93);
        visit(&child, depth + 1); status = dwarf_siblingof(&child, &child);
        if (status < 0) exit(94);
    }
}
static int module(Dwfl_Module *mod, void **userdata, const char *name, Dwarf_Addr base, void *arg) {
    (void)userdata; (void)base; (void)arg;
    printf("M "); hex(name); puts("");
    Dwarf_Addr bias; Dwarf *dw = dwfl_module_getdwarf(mod, &bias);
    Dwarf_CU *cu = NULL; Dwarf_Die die; int status;
    while (!(status = dwarf_get_units(dw, cu, &cu, NULL, NULL, &die, NULL))) visit(&die, 0);
    if (status < 0) { fprintf(stderr, "DWFL: %s; DWARF: %s\n", dwfl_errmsg(-1), dwarf_errmsg(-1)); exit(95); }
    return DWARF_CB_OK;
}
int main(int argc, char **argv) {
    if (argc != 2) return 99;
    Dwfl_Callbacks cb = {.find_debuginfo = dwfl_standard_find_debuginfo, .section_address = dwfl_offline_section_address};
    Dwfl *dwfl = dwfl_begin(&cb);
    int fd = open(argv[1], O_RDONLY);
    if (!dwfl || fd < 0 || !dwfl_report_offline(dwfl, argv[1], argv[1], fd)) return 96;
    if (dwfl_report_end(dwfl, NULL, NULL) || dwfl_getmodules(dwfl, module, NULL, 0)) return 97;
    dwfl_end(dwfl); return 0;
}
'''

RUST_READER = r'''
use std::fs::File;
use std::path::Path;
fn hex(value: &[u8]) { for byte in value { print!("{byte:02x}"); } }
fn visit(die: reader::Die<'_>) -> Result<(), gendwarfksyms_header::Error> {
    let mut stack = vec![(die, 0)];
    let mut count = 0;
    while let Some((die, depth)) = stack.pop() {
        count += 1; if count > 100000 { panic!("malformed graph exceeds traversal bound"); }
        println!("D {depth} {}", die.tag());
        assert_eq!(die.from_address(die.addr())?.tag(), die.tag());
        for id in 0..256 {
            if let Some(value) = die.string(id) { print!("S {id} "); hex(value); println!(); }
            if let Some(value) = die.udata(id) { println!("U {id} {value}"); }
            if let Some(value) = die.flag(id) { println!("F {id} {}", value as u8); }
            if let Some(value) = die.reference(id) {
                print!("R {id} {} ", value.tag()); hex(value.string(3).unwrap_or_default()); println!();
            }
        }
        if let Some(index) = die.udata(0x3a) {
            let path = die.source_file(index);
            print!("P {index} {} ", path.is_ok() as u8); if let Ok(path) = path { hex(path); } println!();
        }
        for child in die.children()?.into_iter().rev() { stack.push((child, depth + 1)); }
    }
    Ok(())
}
fn run(path: &Path) -> Result<(), gendwarfksyms_header::Error> {
    let file = File::open(path).unwrap(); let session = reader::Session::open(file, path)?;
    for module in session.modules()? {
        print!("M "); hex(module.name()); println!();
        for die in module.units()? { visit(die)?; }
    }
    Ok(())
}
fn main() {
    use std::io::Write;
    if let Err(error) = run(Path::new(&std::env::args_os().nth(1).unwrap())) {
        std::io::stderr().write_all(&error.0).unwrap(); std::process::exit(1);
    }
}
'''

RUST_PROBES = r'''
use std::fs::File;
use std::path::Path;
fn main() {
    let args: Vec<_> = std::env::args_os().collect(); let path = Path::new(&args[2]);
    if args[1] == "fds" || args[1] == "nul" {
        let count = || std::fs::read_dir("/proc/self/fd").unwrap().count();
        let before = count();
        for _ in 0..100 {
            let file = File::open(path).unwrap();
            let malformed;
            let path = if args[1] == "nul" {
                use std::os::unix::ffi::OsStringExt;
                malformed = std::ffi::OsString::from_vec(b"invalid\0path".to_vec());
                Path::new(&malformed)
            } else { path };
            if let Ok(session) = reader::Session::open(file, path) {
                if let Ok(modules) = session.modules() { for module in modules { let _ = module.units(); } }
            }
            assert_eq!(before, count(), "descriptor leaked after dropping session");
        }
    } else {
        let session = reader::Session::open(File::open(path).unwrap(), path).unwrap();
        let unit = session.modules().unwrap()[0].units().unwrap()[0];
        for address in [0, 1, 2, 4096, usize::MAX] { assert!(unit.from_address(address).is_err()); }
        assert!(unit.source_file(u64::MAX).is_err());
        assert_eq!(unit.from_address(unit.addr()).unwrap().tag(), unit.tag());
    }
}
'''


def elf_debug(info, abbrev, bits=64, little=True):
    endian = "<" if little else ">"
    ident = b"\x7fELF" + bytes((2 if bits == 64 else 1, 1 if little else 2, 1)) + bytes(9)
    names = b"\0.shstrtab\0.debug_abbrev\0.debug_info\0.strtab\0.symtab\0"
    ehsize, shsize = (64, 64) if bits == 64 else (52, 40)
    shfmt = endian + ("IIQQQQIIQQ" if bits == 64 else "IIIIIIIIII")
    sym_size = 24 if bits == 64 else 16
    data_end = ehsize + len(names) + len(abbrev) + len(info)
    section_at = data_end + 1 + sym_size
    machine = 62 if bits == 64 else 3
    header = ident + struct.pack(endian + ("HHIQQQIHHHHHH" if bits == 64 else "HHIIIIIHHHHHH"),
                                 1, machine, 1, 0, 0, section_at, 0, ehsize, 0, 0, shsize, 6, 1)
    sections = bytes(shsize)
    for name, kind, offset, size in ((1, 3, ehsize, len(names)), (11, 1, ehsize + len(names), len(abbrev)),
                                     (25, 1, ehsize + len(names) + len(abbrev), len(info))):
        sections += struct.pack(shfmt, name, kind, 0, 0, offset, size, 0, 0, 1, 0)
    sections += struct.pack(shfmt, names.index(b".strtab"), 3, 0, 0, data_end, 1, 0, 0, 1, 0)
    sections += struct.pack(shfmt, names.index(b".symtab"), 2, 0, 0, data_end + 1, sym_size, 4, 1, 1, sym_size)
    return header + names + abbrev + info + bytes(1 + sym_size) + sections


def synthetic(bits=64, little=True, cyclic=False):
    endian = "little" if little else "big"
    # A CU with two base types, using inline names and byte-size attributes.
    abbrev = bytes((1, 0x11, 1, 3, 8, 0, 0, 2, 0x24, 0, 3, 8, 0x0b, 0x0b, 0, 0))
    if cyclic:
        # DW_AT_sibling ref4 points back at this leaf DIE.
        abbrev = abbrev[:-2] + bytes((1, 0x13, 0, 0))
    abbrev += b"\0"
    unit = (4).to_bytes(2, endian) + bytes(4) + bytes((bits // 8,)) + b"\1fixture\0"
    child_offset = len(unit) + 4
    unit += b"\2integer\0\4"
    if cyclic:
        unit += child_offset.to_bytes(4, endian)
    unit += b"\2word\0\10"
    if cyclic:
        unit += child_offset.to_bytes(4, endian)
    unit += b"\0"
    info = len(unit).to_bytes(4, endian) + unit
    return elf_debug(info, abbrev, bits, little)


class GendwarfReaderTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        require_c_headers()
        cls.temporary = tempfile.TemporaryDirectory(prefix="gendwarf-reader-")
        cls.work = Path(cls.temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.link = []
        for value in libraries():
            if value.startswith("-l:"):
                cls.link += ["-l", "dylib:+verbatim=" + value[3:]]
            elif value.startswith("-l"):
                cls.link += ["-l", value[2:]]
            else:
                cls.link += ["-C", "link-arg=" + value]
        cls.prefix = ('//! Independent reader boundary harness.\n'
                      '#[path="' + str(SOURCE / "gendwarfksyms_header.rs") + '"]\n'
                      '#[allow(dead_code)] mod gendwarfksyms_header;\n'
                      '#[path="' + str(SOURCE / "reader.rs") + '"]\n'
                      '#[allow(dead_code)] mod reader;\n')
        cls.c = cls.build_c("reader-c", C_READER)
        cls.rust = cls.build_rust("reader-rust", RUST_READER)
        cls.probes = cls.build_rust("reader-probes", RUST_PROBES)

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    @classmethod
    def build_c(cls, name, source):
        path = cls.work / (name + ".c")
        path.write_text(source)
        result = cls.work / name
        subprocess.run([*cls.cc, "-O2", "-Wall", "-Wextra", "-Werror", *cflags(), str(path),
                        *libraries(), "-o", str(result)], check=True)
        return result

    @classmethod
    def build_rust(cls, name, source, prefix=None):
        path = cls.work / (name + ".rs")
        path.write_text((cls.prefix if prefix is None else prefix) + source)
        result = cls.work / name
        subprocess.run([*cls.rustc, "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
                        "-Wunreachable-pub", "-Wrust-2018-idioms", str(path), *cls.link,
                        "-o", str(result)], check=True)
        return result

    def compare(self, path):
        result = [subprocess.run([tool, path], capture_output=True, timeout=15) for tool in (self.c, self.rust)]
        self.assertEqual(result[0].returncode, 0, result[0].stderr)
        self.assertEqual((result[1].returncode, result[1].stdout, result[1].stderr),
                         (result[0].returncode, result[0].stdout, result[0].stderr))
        self.assertIn(b"D 0 ", result[1].stdout)
        return result[1].stdout

    def test_abi_layouts_and_all_dwarf_constants(self):
        constants = re.findall(r"pub\(crate\) const (DW_[A-Za-z_]+):", (SOURCE / "reader.rs").read_text())
        self.assertGreater(len(constants), 40)
        c = C_ABI.replace("CONSTANTS", "\n".join(f'printf("%u\\n", (unsigned){name});' for name in constants))
        rust = RUST_ABI.replace("CONSTANTS", "\n".join(f'println!("{{}}", constants::{name});' for name in constants))
        copied = self.work / "reader-abi-source.rs"
        copied.write_text((SOURCE / "reader.rs").read_text() + "\n" + rust)
        prefix = self.prefix.replace(str(SOURCE / "reader.rs"), str(copied))
        oracle = self.build_c("abi-c", c)
        translated = self.build_rust("abi-rust", "fn main() { reader::abi(); }", prefix)
        self.assertEqual(subprocess.check_output([translated]), subprocess.check_output([oracle]))

    def test_all_ffi_function_and_callback_signatures(self):
        # Derive C function-pointer declarations from the actual Rust FFI, then
        # ask the C compiler to compare them with the official headers. The one
        # intentionally opaque GElf_Shdr pointer is restored to its C spelling.
        source = (SOURCE / "reader.rs").read_text()
        names = {"RawDie": "Dwarf_Die", "Attribute": "Dwarf_Attribute", "Callbacks": "Dwfl_Callbacks",
                 "DwflModule": "Dwfl_Module", "DwarfCu": "Dwarf_CU", "Abbrev": "Dwarf_Abbrev", "Files": "Dwarf_Files",
                 "c_char": "char", "c_uint": "unsigned int", "c_int": "int", "c_long": "long", "c_void": "void",
                 "u8": "uint8_t", "u16": "uint16_t", "u32": "uint32_t", "u64": "uint64_t", "isize": "ptrdiff_t", "usize": "size_t"}

        def ctype(value):
            value = value.strip()
            if value.startswith("*mut "):
                return ctype(value[5:]) + " *"
            if value.startswith("*const "):
                return "const " + ctype(value[7:]) + " *"
            if value.startswith("Option<"):
                return ctype(value[7:-1])
            return names.get(value, value)

        def arguments(values, section=False, named=False):
            output = []
            for value in values.split(","):
                value = value.strip()
                if not value:
                    continue
                name = ""
                if named:
                    name, value = value.split(":", 1)
                if section and (name.strip() == "header" or (not named and value == "*const c_void")):
                    output.append("const GElf_Shdr *")
                else:
                    output.append(ctype(value))
            return ", ".join(output) or "void"

        aliases = re.findall(r'type (\w+)\s*=\s*unsafe extern "C" fn\((.*?)\)\s*->\s*(\w+)\s*;', source, re.S)
        self.assertEqual(len(aliases), 5)
        text = '#include <stddef.h>\n#include <stdint.h>\n#include <elfutils/libdw.h>\n#include <elfutils/libdwfl.h>\n'
        for name, args, result in aliases:
            text += f'typedef {ctype(result)} (*{name})({arguments(args, name == "SectionAddress")});\n'
        for field, alias in (("find_elf", "FindElf"), ("find_debuginfo", "FindDebug"), ("section_address", "SectionAddress")):
            text += f'_Static_assert(__builtin_types_compatible_p(__typeof__(((Dwfl_Callbacks *)0)->{field}), {alias}), "{field}");\n'
        external = source.split('unsafe extern "C" {', 1)[1].split("\n}", 1)[0]
        functions = re.findall(r'fn (\w+)\((.*?)\)\s*(?:->\s*(.*?))?;', external, re.S)
        self.assertGreaterEqual(len(functions), 20)
        for name, args, result in functions:
            signature = f'{ctype(result or "void")} (*)({arguments(args, name == "dwfl_offline_section_address", True)})'
            text += f'_Static_assert(__builtin_types_compatible_p(__typeof__(&{name}), {signature}), "{name}");\n'
        text += 'int main(void) { return 0; }\n'
        self.build_c("abi-signatures", text)

    def test_real_compiler_dwarf_versions_32_64_and_compression(self):
        source = self.work / "fixture.c"
        source.write_text('enum E { NEG=-9, BIG=0x7fffffff };\n'
                          'struct R { const char *name; unsigned bits:5; struct R *next; };\n'
                          'volatile struct R exported;\n'
                          'long fn(struct R *arg, enum E n) { return arg->bits+n; }\n')
        for bits in (32, 64):
            for dwarf in (2, 3, 4, 5):
                for optimize in ("-O0", "-O2"):
                    obj = self.work / "compiler.o"
                    subprocess.run([*self.cc, f"-m{bits}", f"-gdwarf-{dwarf}", optimize,
                                    "-c", str(source), "-o", str(obj)], check=True)
                    self.compare(obj)
        objcopy = shutil.which("objcopy")
        if objcopy:
            for kind in ("zlib", "zlib-gnu", "zstd"):
                compressed = self.work / (kind + ".o")
                result = subprocess.run([objcopy, "--compress-debug-sections=" + kind, str(obj), str(compressed)], capture_output=True)
                if result.returncode == 0:
                    self.compare(compressed)

    def test_real_cross_target_dwarf_both_endianness(self):
        clang = shutil.which(os.environ.get("CLANG", "clang"))
        if clang is None:
            self.skipTest("clang required for cross-target DWARF fixtures")
        source = self.work / "cross.c"
        source.write_text('typedef struct { unsigned long count; const int *ptr; } Value;\n'
                          'Value value; int read_value(Value *v) { return v->count + *v->ptr; }\n')
        for target in ("powerpc-linux-gnu", "powerpc64-linux-gnu", "powerpc64le-linux-gnu", "s390x-linux-gnu", "arm-linux-gnueabi"):
            obj = self.work / (target + ".o")
            subprocess.run([clang, "--target=" + target, "-g", "-gdwarf-4", "-c", str(source), "-o", str(obj)], check=True)
            self.compare(obj)

    def test_archives_and_non_utf8_paths(self):
        objects = []
        for i in range(3):
            source = self.work / f"unit{i}.c"
            source.write_text(f"struct T{i} {{ int member; }}; struct T{i} exported{i};\n")
            obj = self.work / f"unit{i}.o"
            subprocess.run([*self.cc, "-g", "-c", str(source), "-o", str(obj)], check=True)
            objects.append(obj)
        archive = self.work / "archive.a"
        subprocess.run([os.environ.get("AR", "ar"), "rcs", str(archive), *map(str, objects)], check=True)
        output = self.compare(archive)
        self.assertEqual(output.count(b"M "), 3)
        raw = os.fsencode(self.work) + b"/unit-\xff.o"
        with open(raw, "wb") as stream:
            stream.write(objects[0].read_bytes())
        self.compare(raw)

    def test_synthetic_dwarf_both_classes_endianness_and_cyclic_siblings(self):
        for bits in (32, 64):
            for little in (False, True):
                path = self.work / "synthetic.o"
                path.write_bytes(synthetic(bits, little))
                self.compare(path)
        path.write_bytes(synthetic(cyclic=True))
        result = subprocess.run([self.rust, path], capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertTrue(b"invalid or cyclic DWARF sibling" in result.stderr or
                        b"`dwarf_siblingof(&current, &current)` failed: -1" in result.stderr, result.stderr)

    def test_descriptor_ownership_success_failure_and_invalid_addresses(self):
        good = self.work / "good.o"
        good.write_bytes(synthetic())
        bad = self.work / "bad.o"
        bad.write_bytes(b"not ELF")
        empty = self.work / "empty.o"
        empty.write_bytes(b"")
        for path in (good, bad, empty, self.work):
            result = subprocess.run([self.probes, "fds", path], capture_output=True, timeout=15)
            self.assertEqual(result.returncode, 0, result.stderr)
        result = subprocess.run([self.probes, "addresses", good], capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 0, result.stderr)
        result = subprocess.run([self.probes, "nul", good], capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_borrowed_results_cannot_outlive_session(self):
        prefix = self.prefix + 'use std::fs::File; use std::path::Path;\n'
        setup = 'let session = reader::Session::open(File::open(path).unwrap(), path).unwrap();'
        cases = [
            ('reader::Module<\'static>', setup + 'session.modules().unwrap()[0]'),
            ('reader::Die<\'static>', setup + 'session.modules().unwrap()[0].units().unwrap()[0]'),
            ('&\'static [u8]', setup + 'session.modules().unwrap()[0].units().unwrap()[0].string(3).unwrap()'),
            ('&\'static [u8]', setup + 'session.modules().unwrap()[0].units().unwrap()[0].source_file(1).unwrap()'),
            ('reader::Die<\'static>', setup + 'session.modules().unwrap()[0].units().unwrap()[0].children().unwrap()[0]'),
        ]
        for index, (ret, body) in enumerate(cases):
            source = self.work / f"lifetime{index}.rs"
            source.write_text(prefix + f'fn escape(path: &Path) -> {ret} {{ {body} }}\nfn main() {{}}\n')
            result = subprocess.run([*self.rustc, "--edition=2021", "--emit=metadata", str(source), "-o", str(self.work / "bad.rmeta")], capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"E0515", result.stderr)

    def test_session_cannot_be_dropped_or_transferred_while_borrowed(self):
        source = self.work / "drop-borrowed.rs"
        source.write_text(self.prefix + '''
fn main() {
    let path = std::path::Path::new("unused");
    let session = reader::Session::open(std::fs::File::open(path).unwrap(), path).unwrap();
    let die = session.modules().unwrap()[0].units().unwrap()[0];
    drop(session);
    println!("{}", die.tag());
}
''')
        result = subprocess.run([*self.rustc, "--edition=2021", "--emit=metadata", str(source),
                                 "-o", str(self.work / "bad.rmeta")], capture_output=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"E0505", result.stderr)
        for value in ("reader::Session", "reader::Die<'static>", "reader::Module<'static>"):
            source.write_text(self.prefix + f'fn send<T: Send>() {{}}\nfn main() {{send::<{value}>();}}\n')
            result = subprocess.run([*self.rustc, "--edition=2021", "--emit=metadata", str(source),
                                     "-o", str(self.work / "bad.rmeta")], capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"E0277", result.stderr)

    def test_malformed_inputs_fail_without_crashes_or_descriptor_leaks(self):
        valid = synthetic()
        path = self.work / "malformed.o"
        for content in (b"", b"\x7fELF", valid[:16], valid[:64], valid[:-1], bytes(128)):
            path.write_bytes(content)
            result = subprocess.run([self.rust, path], capture_output=True, timeout=5)
            self.assertEqual(result.returncode, 1, result.stderr)
            self.assertIn(b"error: gendwarfksyms:", result.stderr)
        rng = random.Random(0xd0a4f)
        for _ in range(100):
            content = bytearray(valid)
            for _ in range(rng.randrange(1, 5)):
                at = rng.randrange(len(content))
                content[at] ^= rng.randrange(1, 256)
            path.write_bytes(content)
            result = subprocess.run([self.rust, path], capture_output=True, timeout=5)
            self.assertIn(result.returncode, (0, 1), result.stderr)


if __name__ == "__main__":
    unittest.main()
