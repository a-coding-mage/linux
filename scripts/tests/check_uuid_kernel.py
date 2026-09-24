#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check native UUID/GUID ownership and disposable C/Rust module callers.

Only the explicit CLI builds modules and boots a disposable guest. Reference C
is unchanged source included under private names, never a replacement provider.
Random draws are checked for bounded writes, masks and changing state, not
compared with independently random reference draws.
"""
import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys
import tempfile

from boot_kernel import module_name, verify_module_events
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_int_log_kernel import normalize_console_transport, kunit_runs as log_kunit_runs, verify_kunit_warnings as log_kunit_warnings
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_prime_numbers_kernel import provider_type_ids, verify_guarded_calls, verify_module_import_versions, verify_rust_entrypoints
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_reciprocal_kernel import module_elf
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports

ROOT = Path(__file__).resolve().parents[2]
FUNCTIONS = ("generate_random_uuid", "generate_random_guid", "guid_gen", "uuid_gen",
             "uuid_is_valid", "guid_parse", "uuid_parse")
PUBLIC = (*FUNCTIONS, "guid_null", "uuid_null")
GPL = ("guid_gen", "uuid_gen")
CASES = (b"uuid_test_guid_valid", b"uuid_test_uuid_valid", b"uuid_test_guid_invalid",
         b"uuid_test_uuid_invalid", b"uuid_test_uuid_gen", b"uuid_test_guid_gen",
         b"uuid_test_generate_random_uuid", b"uuid_test_generate_random_guid")
COUNT, RANDOM = 9217, 1024
DESCRIPTION = b"UUID and GUID native ABI check"
PRIVATE_C_FLAGS = "CFLAGS_MODULE+=-D__DISABLE_EXPORTS"
WRAPPERS = tuple(("uuid_call_" + name, name) for name in FUNCTIONS)


def reference_source():
    declarations = "\n".join("#define " + name + " uuid_reference_" + name
                             for name in (*PUBLIC, "guid_index", "uuid_index"))
    return ('#include <linux/export.h>\n#undef EXPORT_SYMBOL\n#undef EXPORT_SYMBOL_GPL\n'
            '#define EXPORT_SYMBOL(symbol)\n#define EXPORT_SYMBOL_GPL(symbol)\n' +
            declarations + '\n#include "' + str(ROOT / "lib/uuid.c") + '"\n')


WORKLOAD = r'''
#include <linux/errno.h>
#include <linux/kernel.h>
#include <linux/printk.h>
#include <linux/uuid.h>

bool uuid_reference_uuid_is_valid(const char *);
int uuid_reference_guid_parse(const char *, guid_t *);
int uuid_reference_uuid_parse(const char *, uuid_t *);
bool uuid_call_uuid_is_valid(const char *);
int uuid_call_guid_parse(const char *, guid_t *);
int uuid_call_uuid_parse(const char *, uuid_t *);
void uuid_call_generate_random_uuid(unsigned char *);
void uuid_call_generate_random_guid(unsigned char *);
void uuid_call_guid_gen(guid_t *);
void uuid_call_uuid_gen(uuid_t *);
int uuid_nulls_check(void);
int uuid_exercise(void);

static int failure(unsigned int stage, unsigned int record)
{
    pr_err("LUPOS_UUID_FAILURE stage=%u record=%u\n", stage, record);
    return -EINVAL;
}
static void fill(unsigned char *b, unsigned int n, unsigned char v)
{ unsigned int i; for (i = 0; i < n; i++) b[i] = v; }
static void copy(unsigned char *a, const unsigned char *b, unsigned int n)
{ unsigned int i; for (i = 0; i < n; i++) a[i] = b[i]; }
static bool equal(const unsigned char *a, const unsigned char *b, unsigned int n)
{ unsigned int i; for (i = 0; i < n; i++) if (a[i] != b[i]) return false; return true; }

static int one(const unsigned char text[37], unsigned int record)
{
    unsigned char a[128], b[128];
    int offset, kind, x, y;
    if (uuid_reference_uuid_is_valid((const char *)text) != uuid_call_uuid_is_valid((const char *)text))
        return failure(1, record);
    for (kind = 0; kind != 2; kind++) {
        fill(a, sizeof(a), 0xa5); copy(b, a, sizeof(a));
        x = kind ? uuid_reference_guid_parse((const char *)text, (guid_t *)(a+48)) :
                   uuid_reference_uuid_parse((const char *)text, (uuid_t *)(a+48));
        y = kind ? uuid_call_guid_parse((const char *)text, (guid_t *)(b+48)) :
                   uuid_call_uuid_parse((const char *)text, (uuid_t *)(b+48));
        if (x != y || !equal(a,b,sizeof(a))) return failure(2, record);
        for (offset = -20; offset <= 40; offset++) {
            fill(a, sizeof(a), 0xa5); copy(a+40,text,37); copy(b,a,sizeof(a));
            x = kind ? uuid_reference_guid_parse((char *)a+40, (guid_t *)(a+40+offset)) :
                       uuid_reference_uuid_parse((char *)a+40, (uuid_t *)(a+40+offset));
            y = kind ? uuid_call_guid_parse((char *)b+40, (guid_t *)(b+40+offset)) :
                       uuid_call_uuid_parse((char *)b+40, (uuid_t *)(b+40+offset));
            if (x != y || !equal(a,b,sizeof(a))) return failure(3, record);
        }
    }
    return 0;
}

int uuid_exercise(void)
{
    static const unsigned char valid[36] = "01234567-89Ab-CdEf-0123-456789abcdef";
    unsigned char text[37], bytes[48], previous[16];
    unsigned int i, v, kind, round, record = 0, changed;
    int error;
    if (!uuid_nulls_check()) return failure(4, 0);
    copy(text, valid, 36); text[36] = 0xff;
    error = one(text, record++); if (error) return error;
    for (i = 0; i < 36; i++) for (v = 0; v < 256; v++) {
        copy(text, valid, 36); text[i] = v;
        error = one(text, record++); if (error) return error;
    }
    for (kind = 0; kind < 4; kind++) {
        changed = 0; fill(previous,16,0);
        for (round = 0; round < 256; round++) {
            fill(bytes,sizeof(bytes),0xa5);
            switch (kind) {
            case 0: uuid_call_generate_random_uuid(bytes+16); break;
            case 1: uuid_call_generate_random_guid(bytes+16); break;
            case 2: uuid_call_uuid_gen((uuid_t *)(bytes+16)); break;
            default: uuid_call_guid_gen((guid_t *)(bytes+16)); break;
            }
            for (i = 0; i < 16; i++) if (bytes[i] != 0xa5 || bytes[32+i] != 0xa5)
                return failure(5, kind*256+round);
            if ((bytes[16+(kind%2 ? 7 : 6)] & 0xf0) != 0x40 || (bytes[24] & 0xc0) != 0x80)
                return failure(6, kind*256+round);
            if (round && !equal(previous,bytes+16,16)) changed++;
            copy(previous,bytes+16,16);
        }
        /* Sanity, not an entropy estimate or comparison of unrelated draws. */
        if (!changed) return failure(7, kind);
    }
    return record;
}
'''

RUST_WORKLOAD = r'''
unsafe extern "C" {
    fn uuid_reference_uuid_is_valid(p: *const ffi::c_char) -> bool;
    fn uuid_reference_guid_parse(p: *const ffi::c_char, out: *mut declarations::guid_t) -> ffi::c_int;
    fn uuid_reference_uuid_parse(p: *const ffi::c_char, out: *mut declarations::uuid_t) -> ffi::c_int;
}

fn failure(stage: u32, record: u32) -> ffi::c_int {
    // SAFETY: Static format and matching C unsigned integer arguments.
    unsafe { bindings::_printk(c"\x013LUPOS_UUID_FAILURE stage=%u record=%u\n".as_ptr().cast(), stage, record); }
    -22
}

fn same(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] { return false; }
        i += 1;
    }
    true
}

fn one(text: &[u8; 37], record: u32) -> ffi::c_int {
    // SAFETY: All 36 inspected bytes exist, including every invalid prefix.
    if unsafe { uuid_reference_uuid_is_valid(text.as_ptr().cast()) !=
                uuid_call_uuid_is_valid(text.as_ptr().cast()) } {
        return failure(1, record);
    }
    let mut kind = 0;
    while kind < 2 {
        let mut a = [0xa5u8; 128];
        let mut b = [0xa5u8; 128];
        // SAFETY: Disjoint buffers contain full 16-byte destinations.
        let (x, y) = unsafe {
            if kind == 1 {
                (uuid_reference_guid_parse(text.as_ptr().cast(), a.as_mut_ptr().add(48).cast()),
                 uuid_call_guid_parse(text.as_ptr().cast(), b.as_mut_ptr().add(48).cast()))
            } else {
                (uuid_reference_uuid_parse(text.as_ptr().cast(), a.as_mut_ptr().add(48).cast()),
                 uuid_call_uuid_parse(text.as_ptr().cast(), b.as_mut_ptr().add(48).cast()))
            }
        };
        if x != y || !same(&a, &b) { return failure(2, record); }
        let mut offset = -20i32;
        while offset <= 40 {
            let mut i = 0;
            while i < 128 { a[i] = 0xa5; b[i] = 0xa5; i += 1; }
            i = 0;
            while i < 37 { a[40+i] = text[i]; b[40+i] = text[i]; i += 1; }
            let output = (40 + offset) as usize;
            // SAFETY: Each source and output lies within its full 128-byte
            // storage. Raw pointers deliberately permit their C-defined alias.
            let (x, y) = unsafe {
                if kind == 1 {
                    (uuid_reference_guid_parse(a.as_ptr().add(40).cast(), a.as_mut_ptr().add(output).cast()),
                     uuid_call_guid_parse(b.as_ptr().add(40).cast(), b.as_mut_ptr().add(output).cast()))
                } else {
                    (uuid_reference_uuid_parse(a.as_ptr().add(40).cast(), a.as_mut_ptr().add(output).cast()),
                     uuid_call_uuid_parse(b.as_ptr().add(40).cast(), b.as_mut_ptr().add(output).cast()))
                }
            };
            if x != y || !same(&a, &b) { return failure(3, record); }
            offset += 1;
        }
        kind += 1;
    }
    0
}

/// Independent Rust workload; the only C calls beyond native exports are the
/// three unchanged original-C validation/parsing oracle functions.
#[no_mangle]
pub extern "C" fn uuid_rust_exercise() -> ffi::c_int {
    const VALID: &[u8; 36] = b"01234567-89Ab-CdEf-0123-456789abcdef";
    if uuid_nulls_check() == 0 { return failure(4, 0); }
    let mut text = [0u8; 37];
    let mut i = 0;
    while i < 36 { text[i] = VALID[i]; i += 1; }
    text[36] = 0xff;
    let mut record = 0u32;
    let error = one(&text, record);
    if error != 0 { return error; }
    record += 1;
    i = 0;
    while i < 36 {
        let mut value = 0u32;
        while value < 256 {
            let mut j = 0;
            while j < 36 { text[j] = VALID[j]; j += 1; }
            text[i] = value as u8;
            let error = one(&text, record);
            if error != 0 { return error; }
            record += 1;
            value += 1;
        }
        i += 1;
    }
    let mut kind = 0u32;
    while kind < 4 {
        let mut changed = 0u32;
        let mut previous = [0u8; 16];
        let mut round = 0u32;
        while round < 256 {
            let mut bytes = [0xa5u8; 48];
            // SAFETY: These genuine nominal bindings point into a 16-byte
            // destination with 16-byte canaries on each side.
            unsafe {
                match kind {
                    0 => uuid_call_generate_random_uuid(bytes.as_mut_ptr().add(16)),
                    1 => uuid_call_generate_random_guid(bytes.as_mut_ptr().add(16)),
                    2 => uuid_call_uuid_gen(bytes.as_mut_ptr().add(16).cast()),
                    _ => uuid_call_guid_gen(bytes.as_mut_ptr().add(16).cast()),
                }
            }
            i = 0;
            while i < 16 {
                if bytes[i] != 0xa5 || bytes[32+i] != 0xa5 { return failure(5, kind*256+round); }
                i += 1;
            }
            let version = if kind & 1 == 1 { 7 } else { 6 };
            if bytes[16+version] & 0xf0 != 0x40 || bytes[24] & 0xc0 != 0x80 {
                return failure(6, kind*256+round);
            }
            let mut different = false;
            i = 0;
            while i < 16 {
                if previous[i] != bytes[16+i] { different = true; }
                previous[i] = bytes[16+i];
                i += 1;
            }
            if round != 0 && different { changed += 1; }
            round += 1;
        }
        if changed == 0 { return failure(7, kind); }
        kind += 1;
    }
    record as ffi::c_int
}
'''

SIGNATURES = (
    ("generate_random_uuid", "void", "unsigned char *p", "p", "()", "p: *mut ffi::c_uchar"),
    ("generate_random_guid", "void", "unsigned char *p", "p", "()", "p: *mut ffi::c_uchar"),
    ("guid_gen", "void", "guid_t *p", "p", "()", "p: *mut declarations::guid_t"),
    ("uuid_gen", "void", "uuid_t *p", "p", "()", "p: *mut declarations::uuid_t"),
    ("uuid_is_valid", "bool", "const char *p", "p", "bool", "p: *const ffi::c_char"),
    ("guid_parse", "int", "const char *p, guid_t *out", "p, out", "ffi::c_int", "p: *const ffi::c_char, out: *mut declarations::guid_t"),
    ("uuid_parse", "int", "const char *p, uuid_t *out", "p, out", "ffi::c_int", "p: *const ffi::c_char, out: *mut declarations::uuid_t"),
)


def sources(caller):
    if caller == "c":
        text = ('#include <linux/init.h>\n#include <linux/module.h>\n#include <linux/uuid.h>\n'
                'int uuid_exercise(void);\nint uuid_nulls_check(void);\n')
        for name, ret, args, values, _, _ in SIGNATURES:
            text += (f'{ret} uuid_call_{name}({args});\n'
                     f'noinline notrace {ret} uuid_call_{name}({args}) {{\n'
                     f'  {ret} (* volatile selected)({args}) = {name};\n'
                     f'  {"return " if ret != "void" else ""}selected({values});\n}}\n')
        text += ('int uuid_nulls_check(void) { unsigned int i; for (i=0;i<16;i++) '
                 'if (READ_ONCE(guid_null.b[i]) || READ_ONCE(uuid_null.b[i])) return 0; return 1; }\n'
                 'static int __init uuid_abi_init(void) { int n=uuid_exercise(); '
                 f'if (n != {COUNT}) return -EINVAL; pr_info("{result_marker(caller).decode()}\\n"); return 0; }}\n'
                 'static void __exit uuid_abi_exit(void) {}\nmodule_init(uuid_abi_init);\nmodule_exit(uuid_abi_exit);\n'
                 'MODULE_LICENSE("GPL");\nMODULE_DESCRIPTION("UUID and GUID native ABI check");\n')
        return text
    if caller != "rust": raise ValueError("unknown UUID caller")
    text = ('//! Disposable independent Rust native UUID/GUID caller.\n'
            'use kernel::{bindings, ffi};\n'
            '#[allow(dead_code)]\n#[path = "' + str(ROOT / "include/linux/uuid_header.rs") + '"]\n'
            'pub mod declarations;\n')
    for name, _, _, values, ret, args in SIGNATURES:
        types = ", ".join(arg.split(": ", 1)[1] for arg in args.split(", "))
        text += ('/// Call the selected native provider through its real typed pointer.\n'
                 '///\n/// # Safety\n/// All arguments obey the original UUID header contract.\n'
                 '#[inline(never)]\n#[no_mangle]\n'
                 f'pub unsafe extern "C" fn uuid_call_{name}({args}) -> {ret} {{\n'
                 f' let selected: unsafe extern "C" fn({types}) -> {ret} = declarations::{name};\n'
                 f' unsafe {{ core::ptr::read_volatile(&selected)({values}) }}\n}}\n')
    text += ('/// Read both actual exported null statics.\n#[no_mangle]\n'
             'pub extern "C" fn uuid_nulls_check() -> ffi::c_int { let mut i=0; while i<16 { '
             'let a=core::ptr::addr_of!(declarations::guid_null).cast::<u8>(); '
             'let b=core::ptr::addr_of!(declarations::uuid_null).cast::<u8>(); '
             'if unsafe { a.add(i).read_volatile() | b.add(i).read_volatile() } != 0 { return 0; } i+=1; } 1 }\n'
             '/// Execute the checked C-oracle workload.\n#[no_mangle]\n#[link_section=".init.text"]\n'
             'pub extern "C" fn init_module() -> ffi::c_int { let n=uuid_rust_exercise(); '
             f'if n!={COUNT} {{ return -22; }} unsafe {{ bindings::_printk(c"\\x016{result_marker(caller).decode()}\\n".as_ptr().cast()); }} 0 }}\n'
             '/// No persistent resources.\n#[no_mangle]\n#[link_section=".exit.text"]\npub extern "C" fn cleanup_module() {}\n'
             '#[used]\n#[link_section=".init.data"]\nstatic INIT: extern "C" fn()->ffi::c_int=init_module;\n'
             '#[used]\n#[link_section=".exit.data"]\nstatic EXIT: extern "C" fn()=cleanup_module;\n'
             '#[used]\n#[link_section=".modinfo"]\nstatic INFO: [u8; 61] = *b"license=GPL\\0description=UUID and GUID native ABI check\\0";\n'
             '#[used]\nstatic __IS_RUST_MODULE: ()=();\n')
    info = b"license=GPL\0description=UUID and GUID native ABI check\0"
    return text.replace("[u8; 61]", f"[u8; {len(info)}]") + RUST_WORKLOAD


def result_marker(caller):
    if caller not in ("c", "rust"): raise ValueError("unknown UUID caller")
    return (b"LUPOS_UUID_" + (b"RUST_" if caller == "rust" else b"") +
            f"ABI_OK parser={COUNT} overlaps=61 rng={RANDOM} nulls=2".encode())


def states(config):
    suite, framework = config.get("UUID_KUNIT_TEST", "n"), config.get("KUNIT", "n")
    if suite not in ("y", "m") or framework not in ("y", "m") or suite == "y" and framework != "y":
        raise ValueError("requires original UUID_KUNIT_TEST=y/m with compatible KUNIT")
    if config.get("RUST_UUID") == "y" and config.get("RUST") != "y":
        raise ValueError("Rust UUID provider requires RUST=y")
    if config.get("RUST_UUID_KUNIT_TEST", "n") not in ("n", "y"):
        raise ValueError("invalid UUID KUnit language selector")
    if config.get("RUST_UUID_KUNIT_TEST") == "y" and config.get("RUST") != "y":
        raise ValueError("Rust UUID suite requires RUST=y independently of provider")
    return suite, framework


def selected_versions(build):
    result = {}
    for row in (build/"Module.symvers").read_bytes().splitlines():
        fields = row.split()
        if len(fields) < 2 or fields[1].decode() not in PUBLIC: continue
        name = fields[1].decode()
        license = b"EXPORT_SYMBOL_GPL" if name in GPL else b"EXPORT_SYMBOL"
        if name.encode() in result or len(fields) != 4 or fields[2:] != [b"vmlinux", license] or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", fields[0]):
            raise ValueError("wrong UUID version owner/license/namespace/CRC")
        result[name.encode()] = fields[0].lower()
    if set(result) != {n.encode() for n in PUBLIC}: raise ValueError("missing UUID exports")
    if any(row.split()[1:2] in ([b"guid_index"], [b"uuid_index"])
           for row in (build/"Module.symvers").read_bytes().splitlines()):
        raise ValueError("private UUID indexes must not be exported")
    return result


def verify_exports(owner, arch):
    records = read_exports(owner)
    if len(records) != len(PUBLIC) or {r["name"] for r in records} != set(PUBLIC):
        raise ValueError("missing, duplicate or extra UUID exports")
    for record in records:
        name = record["name"]
        expected = dict(license="GPL" if name in GPL else "", namespace="", relocation_target=name,
                        relocation_addend=0, pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1,
                        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        if any(record[key] != value for key,value in expected.items()):
            raise ValueError("incorrect UUID native export record")


def verify_statics(owner):
    _,sections,symbols,_,_=module_elf(owner)
    expected={b"guid_null":bytes(16),b"uuid_null":bytes(16),
              b"guid_index":bytes((3,2,1,0,5,4,7,6,8,9,10,11,12,13,14,15)),
              b"uuid_index":bytes(range(16))}
    for name,payload in expected.items():
        rows=[s for s in symbols if s[0]==name]
        if len(rows)!=1 or rows[0][1]!=0x11 or rows[0][2]!=0 or rows[0][5]!=16:
            raise ValueError("wrong UUID static type, visibility or size")
        index,offset=rows[0][3:5]
        if not 0<index<len(sections): raise ValueError("undefined UUID static")
        section=sections[index]
        if section[1] not in (1,8) or not section[2]&2 or section[2]&4:
            raise ValueError("UUID static not allocated data")
        offset-=section[3]
        if offset<0 or offset+16>section[4]: raise ValueError("UUID static outside section")
        actual=bytes(16) if section[1]==8 else section[-1][offset:offset+16]
        if actual!=payload: raise ValueError("wrong UUID null/index bytes")


def verify_rng_calls(owner,arch):
    assembly=tool(os.environ.get("LLVM_OBJDUMP","llvm-objdump"),"-dr","--no-show-raw-insn",owner).decode()
    blocks=re.split(r"(?m)^[0-9a-f]+ <([^>]+)>:\n",assembly)
    functions=dict(zip(blocks[1::2],blocks[2::2]))
    for name in FUNCTIONS[:4]:
        body=functions.get(name,"")
        calls=list(re.finditer(r"R_\w+\s+get_random_bytes(?:-0x4)?\s*$",body,re.M))
        if len(calls)!=1: raise ValueError("UUID generator must call actual RNG exactly once")
        before=body[:calls[0].start()]
        lines=before.splitlines()
        writes=([line for line in lines if re.search(r", %(?:rsi|esi|si|sil)(?:\s|$)",line)]
                if arch=="x86_64" else [line for line in lines if re.search(r"\b\w+\s+[wx]1,",line)])
        pattern=(r"movl\s+\$(?:0x10|16), %esi" if arch=="x86_64" else r"mov\s+w1, #(?:0x10|16)\b")
        if not writes or not re.search(pattern,writes[-1]):
            raise ValueError("UUID generator lacks actual 16-byte RNG request")


def verify_dwarf(build, owner, versions):
    config = configuration(build)
    if config.get("MODVERSIONS") != "y": return
    if config.get("GENDWARFKSYMS") != "y": raise ValueError("UUID audit requires native DWARF versions")
    with tempfile.TemporaryDirectory(prefix="uuid-dwarf-") as work:
        types = Path(work)/"types"
        result = subprocess.run([str(build/"scripts/gendwarfksyms/gendwarfksyms"), "--symtypes", str(types), str(owner)],
            input=b"\n".join(sorted(versions))+b"\n", capture_output=True, check=True, timeout=180)
        expected = {b"#SYMVER "+name+b" "+crc for name,crc in versions.items()}
        if result.stderr or set(result.stdout.splitlines()) != expected or len(result.stdout.splitlines()) != len(expected):
            raise ValueError("UUID defining-object DWARF CRC mismatch")
        if any(name not in types.read_bytes() for name in (b"guid_t", b"uuid_t")):
            raise ValueError("UUID DWARF lacks actual nominal types")


def verify_suite_metadata(obj, builtin):
    prefix = b"uuid_kunit." if builtin else b""
    fields = metadata_fields(obj)
    for name,value in ((b"license",b"Dual BSD/GPL"), (b"author",b"Andy Shevchenko <andriy.shevchenko@linux.intel.com>"),
                       (b"description",b"Test cases for lib/uuid.c module")):
        require_metadata_field(fields,prefix+name,value)


def verify_suite_registration(obj, arch, *, rust_suite, builtin, cfi):
    """Check real registration/callback relocations, not merely section names."""
    _,sections,symbols,_,_=module_elf(obj)
    registration=[(i,s) for i,s in enumerate(sections) if s[0]==b".kunit_test_suites"]
    if (len(registration)!=1 or registration[0][1][1:3]!=(1,3) or
            registration[0][1][4]!=8 or registration[0][1][7]!=8):
        raise ValueError("UUID suite registration absent, duplicated or malformed")
    index=registration[0][0]
    relocations=[r for s in sections if s[1]==4 and s[6]==index for r in s[-1]]
    absolute=257 if arch=="aarch64" else 1
    if len(relocations)!=1 or relocations[0][0]!=0 or relocations[0][1]!=absolute:
        raise ValueError("UUID suite needs one actual registration relocation")
    _,_,target,addend=relocations[0]
    if (not 0<target[3]<len(sections) or not sections[target[3]][2]&2 or
            sections[target[3]][2]&4 or not 0<=target[4]+addend-sections[target[3]][3]<sections[target[3]][4]):
        raise ValueError("UUID registration points outside allocated suite data")
    defined=[s for s in symbols if 0<s[3]<len(sections)]
    if {s[0] for s in symbols} & {b"init_module",b"cleanup_module"} or any(s[0].startswith(b".initcall") for s in sections):
        raise ValueError("UUID suite must retain stateless KUnit registration")
    if sum(b"__IS_RUST_MODULE" in s[0] for s in defined)!=int(rust_suite and not builtin):
        raise ValueError("UUID KUnit Rust owner marker differs from selected lifecycle")
    if read_exports(obj): raise ValueError("UUID KUnit suite must not export symbols")
    callbacks=[]
    for case in CASES:
        matches=[s for s in defined if s[1]&15==2 and not s[0].startswith(b"__cfi_") and
                 (s[0]==case or re.search(rb"[0-9]"+re.escape(case)+rb"(?:17h[0-9a-f]+E)?$",s[0]))]
        if len(matches)!=1: raise ValueError("UUID suite missing original callback: "+case.decode())
        callbacks.append(matches[0])
    references=[]
    for section in sections:
        if section[1]!=4 or not 0<section[6]<len(sections): continue
        owner=sections[section[6]]
        if not owner[2]&2 or owner[2]&4: continue
        for offset,kind,symbol,addend in section[-1]:
            for i,callback in enumerate(callbacks):
                if (symbol[3],symbol[4]+addend)==(callback[3],callback[4]):
                    if kind!=absolute: raise ValueError("UUID callback registration is not a native pointer")
                    references.append((section[6],offset,i))
    if [r[2] for r in sorted(references)]!=list(range(len(CASES))):
        raise ValueError("UUID callback registration missing, duplicated or reordered")
    if cfi:
        types=provider_type_ids(obj,names=tuple(s[0].decode() for s in callbacks))
        if len(set(types.values()))!=1 or not next(iter(types.values())):
            raise ValueError("UUID callbacks lost their common native KUnit KCFI type")
    return callbacks


def verify_suite_object(build, test, arch, *, builtin, rust_suite, members):
    source=ROOT/("lib/tests/uuid_kunit.rs" if rust_suite else "lib/tests/uuid_kunit.c")
    dependencies=([ROOT/"include/linux/uuid_header.rs",build/"rust/libkernel.rmeta",
                   build/"rust/libbindings.rmeta"] if rust_suite else
                  [ROOT/"include/kunit/test.h",ROOT/"include/linux/uuid.h"])
    verify_build_command(build,test,source,dependencies)
    elf_target(test,arch)
    verify_references(test,("guid_parse","uuid_parse","guid_gen","uuid_gen",
        "generate_random_uuid","generate_random_guid","__kunit_do_failed_assertion",
        "kunit_binary_assert_format","kunit_unary_assert_format"))
    cfi=configuration(build).get("CFI")=="y"
    if cfi:
        flags=compilation_flags(test)
        required="-Zsanitizer=kcfi" if rust_suite else "-fsanitize=kcfi"
        if required not in flags or any(flag.startswith("-fno-sanitize=") and
                {"all","kcfi"}&set(flag.split("=",1)[1].split(",")) for flag in flags):
            raise ValueError("UUID KUnit suite lost actual KCFI compilation")
    verify_suite_registration(test,arch,rust_suite=rust_suite,builtin=builtin,cfi=cfi)
    verify_suite_metadata(test,builtin)
    if rust_suite:
        kernel=build/"rust/kernel.o"
        if kernel.resolve() not in members: raise ValueError("Rust UUID suite kernel helpers are not linked")
        verify_build_command(build,kernel,ROOT/"rust/kernel/lib.rs",[ROOT/"rust/kernel/kunit.rs"])
        newer(build/"rust/libkernel.rmeta",[ROOT/"rust/kernel/kunit.rs"])
        newer(build/"rust/libbindings.rmeta",[ROOT/"include/kunit/test.h",ROOT/"include/kunit/assert.h",
                                             ROOT/"include/linux/uuid.h"])


def verify_linked_implementation(build, selection):
    config = configuration(build)
    suite, framework = states(config)
    if selection != ("Rust" if config.get("RUST_UUID") == "y" else "C"):
        raise ValueError("UUID provider selection differs from configuration")
    arch = architecture(config)
    archive = build/"vmlinux.a"
    ordered = [(build/os.fsdecode(line)).resolve() for line in tool("ar","t",archive).splitlines()]
    members = set(ordered)
    owner = build/("lib/uuid_rust.o" if selection == "Rust" else "lib/uuid.o")
    other = build/("lib/uuid.o" if selection == "Rust" else "lib/uuid_rust.o")
    if ordered.count(owner.resolve()) != 1 or other.resolve() in members:
        raise ValueError("UUID archive owner missing, duplicated or mixed")
    source = ROOT/("lib/uuid_rust.rs" if selection == "Rust" else "lib/uuid.c")
    deps = ([ROOT/p for p in ("lib/uuid.rs","rust/ffi_export.rs","include/linux/export_header.rs")] +
            [build/"rust/libkernel.rmeta",build/"rust/libbindings.rmeta"] if selection == "Rust" else
            [ROOT/"include/linux/uuid.h",ROOT/"include/linux/ctype.h"])
    verify_build_command(build,owner,source,deps)
    elf_target(owner,arch)
    verify_exports(owner,arch)
    verify_statics(owner)
    verify_rng_calls(owner,arch)
    symbols = tool("nm","-g","--defined-only",owner).splitlines()
    for name in PUBLIC:
        rows = [line.split() for line in symbols if line.split()[-1:] == [name.encode()]]
        if len(rows) != 1 or rows[0][-2] not in ((b"T",) if name in FUNCTIONS else (b"R",b"B")):
            raise ValueError("UUID symbol missing native function/static definition")
    verify_references(owner,("get_random_bytes",))
    versions = selected_versions(build)
    if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("."+owner.name+".cmd")) != versions:
        raise ValueError("UUID versions do not originate in selected defining object")
    verify_dwarf(build,owner,versions)
    if config.get("CFI") == "y":
        if ("-Zsanitizer=kcfi" if selection=="Rust" else "-fsanitize=kcfi") not in compilation_flags(owner):
            raise ValueError("UUID provider lost KCFI")
        provider_type_ids(owner,names=FUNCTIONS)
    newer(archive,[owner])
    test = build/"lib/tests/uuid_kunit.o"
    if ordered.count(test.resolve()) != int(suite=="y"):
        raise ValueError("UUID KUnit archive membership mismatch")
    modules = []
    if framework == "m":
        framework_module = verify_framework_module(build,members,arch)
        verify_module_metadata(build,framework_module)
        verify_module_import_versions(build,framework_module)
        modules.append(framework_module)
    else:
        for name in ("test","assert"):
            obj=build/("lib/kunit/"+name+".o")
            if obj.resolve() not in members: raise ValueError("original KUnit framework not linked")
            verify_build_command(build,obj,ROOT/("lib/kunit/"+name+".c"))
            newer(archive,[obj])
    verify_suite_object(build,test,arch,builtin=suite=="y",
                        rust_suite=config.get("RUST_UUID_KUNIT_TEST")=="y",members=members)
    if suite=="y": newer(archive,[test])
    else:
        module=test.with_suffix(".ko")
        verify_module(build,module,test,"uuid_kunit",arch)
        verify_module_metadata(build,module)
        verify_module_import_versions(build,module)
        verify_suite_metadata(module,False)
        verify_suite_registration(module,arch,rust_suite=config.get("RUST_UUID_KUNIT_TEST")=="y",
                                  builtin=False,cfi=config.get("CFI")=="y")
        modules.append(module)
    newer(build/"Module.symvers",[owner,test])
    newer(build/"vmlinux.o",[archive])
    newer(build/"vmlinux",[build/"vmlinux.o"])
    newer(build/("arch/arm64/boot/Image" if arch=="aarch64" else "arch/x86/boot/bzImage"),[build/"vmlinux"])
    return modules


def verify_rust_workload(obj,module=None):
    imports={row.split()[-1] for row in tool("nm","-u",obj).splitlines() if row.split()}
    expected={b"_printk",*(name.encode() for name in PUBLIC),
              b"uuid_reference_uuid_is_valid",b"uuid_reference_guid_parse",b"uuid_reference_uuid_parse"}
    # Native overflow checks can retain this real core guard in the bounded
    # loop arithmetic. The caller is GPL; ordinary module version/license
    # validation below still applies to this genuine import. No panic stub.
    overflow={name for name in imports if re.fullmatch(
        rb"_RNvNtNtC[A-Za-z0-9]+_4core9panicking11panic_const24panic_const_add_overflow",name)}
    extra=imports-expected-{b"memcpy",b"memset"}-overflow
    if not expected<=imports or extra:
        raise ValueError("Rust UUID caller unexpected runtime/core import or workload delegation: missing=" +
                         repr(sorted(expected-imports)) + " extra=" + repr(sorted(extra)))
    for path in (obj,*([module] if module is not None else [])):
        definitions={row.split()[-1] for row in tool("nm","-g","--defined-only",path).splitlines() if row.split()}
        if b"uuid_exercise" in definitions or b"uuid_rust_exercise" not in definitions:
            raise ValueError("Rust UUID caller must own its workload without a C dispatcher")


def verify_consumer(build,work,caller):
    config,arch=configuration(build),architecture(configuration(build))
    name="uuid_rust_abi" if caller=="rust" else "uuid_abi"
    obj=work/("uuid_rust_main.o" if caller=="rust" else "uuid_c_main.o")
    module=work/(name+".ko")
    for path in (obj,module):
        elf_target(path,arch)
        verify_references(path,PUBLIC)
    if module_name(module)!=name: raise ValueError("wrong UUID module identity")
    source=obj.with_suffix(".rs" if caller=="rust" else ".c")
    deps=([ROOT/"include/linux/uuid_header.rs",build/"rust/libkernel.rmeta",build/"rust/libbindings.rmeta"]
          if caller=="rust" else [ROOT/"include/linux/uuid.h"])
    verify_build_command(work,obj,source,deps)
    if source.read_text()!=sources(caller): raise ValueError("UUID caller differs from checked source")
    inputs=[]
    fixture_sources=[("uuid_reference",reference_source(),[ROOT/"lib/uuid.c",ROOT/"include/linux/uuid.h"])]
    if caller=="c": fixture_sources.append(("uuid_workload",WORKLOAD,[ROOT/"include/linux/uuid.h"]))
    for stem,expected,dependencies in fixture_sources:
        path=work/(stem+".o")
        if path.with_suffix(".c").read_text()!=expected: raise ValueError("UUID workload/oracle source differs")
        verify_build_command(work,path,path.with_suffix(".c"),dependencies)
        inputs.append(path)
    constituents=[(work/os.fsdecode(line)).resolve() for line in module.with_suffix(".mod").read_bytes().splitlines()]
    if constituents!=[path.resolve() for path in (obj,*inputs)]:
        raise ValueError("UUID caller contains unexpected workload/constituents")
    newer(module,[obj,*inputs,selected_metadata(build,module)])
    verify_module_metadata(build,module,work=work,require_c_suppression=True)
    verify_module_import_versions(build,module)
    require_metadata_field(metadata_fields(module),b"license",b"GPL")
    require_metadata_field(metadata_fields(module),b"description",DESCRIPTION)
    if read_exports(module): raise ValueError("UUID caller must not export symbols")
    undefined={row.split()[-1] for row in tool("nm","-u",module).splitlines() if row.split()}
    if undefined & {b"guid_index",b"uuid_index"}: raise ValueError("private UUID indexes imported")
    if caller=="rust":
        verify_rust_workload(obj,module)
        for path in (obj,module): verify_rust_entrypoints(path,arch)
    for path in [*inputs,*([obj] if caller=="c" else [])]:
        flags=compilation_flags(path)
        if "-D__DISABLE_EXPORTS" not in flags or "-U__DISABLE_EXPORTS" in flags or read_exports(path):
            raise ValueError("UUID private C fixture incidental exports")
    verify_common_metadata(build,work,flags=compilation_flags,exports=read_exports)
    if config.get("CFI")=="y":
        flags=compilation_flags(obj)
        required="-Zsanitizer=kcfi" if caller=="rust" else "-fsanitize=kcfi"
        if required not in flags or any(flag.startswith("-fno-sanitize=") and
                {"all","kcfi"}&set(flag.split("=",1)[1].split(",")) for flag in flags):
            raise ValueError("UUID caller lost KCFI flags")
        owner=build/("lib/uuid_rust.o" if config.get("RUST_UUID")=="y" else "lib/uuid.o")
        types=provider_type_ids(owner,names=FUNCTIONS)
        for path in (obj,module): verify_guarded_calls(path,arch,types,wrappers=WRAPPERS)


def license_source(forbidden=None):
    if forbidden not in (None,*GPL): raise ValueError("unknown UUID license probe")
    body = (f"{forbidden}(&value); return 0;" if forbidden else
            'generate_random_uuid(bytes); generate_random_guid(bytes); '
            'if (!uuid_is_valid("01234567-89ab-cdef-0123-456789abcdef")) return -EINVAL; '
            'if (guid_parse("01234567-89ab-cdef-0123-456789abcdef",&g) || '
            'uuid_parse("01234567-89ab-cdef-0123-456789abcdef",&u)) return -EINVAL; '
            'return READ_ONCE(guid_null.b[0]) | READ_ONCE(uuid_null.b[0]);')
    variables = (("guid_t" if forbidden=="guid_gen" else "uuid_t")+" value;" if forbidden else
                 "unsigned char bytes[16]; guid_t g; uuid_t u;")
    return ('#include <linux/module.h>\n#include <linux/uuid.h>\n'
            'static int __init uuid_license_init(void) {'+variables+body+'}\n'
            'static void __exit uuid_license_exit(void) {}\n'
            'module_init(uuid_license_init);\nmodule_exit(uuid_license_exit);\nMODULE_LICENSE("Proprietary");\n'
            'MODULE_DESCRIPTION("UUID export license gate");\n')


def verify_license_result(result,forbidden):
    if forbidden is None:
        if result.returncode: raise ValueError("unrestricted UUID proprietary build failed")
    else:
        message=(result.stdout+result.stderr).decode(errors="replace")
        expected=r"ERROR: modpost: (?:[^\n]*[/])?uuid_license\.ko: GPL-incompatible module uses GPL-only symbol '"+forbidden+r"'"
        matches=[line for line in message.splitlines() if re.fullmatch(expected,line)]
        if result.returncode==0 or len(matches)!=1:
            raise ValueError("expected actual proprietary GPL-only UUID modpost rejection")
        remaining="\n".join(line for line in message.splitlines() if line not in matches)
        if re.search(r"error:|undefined!",remaining,re.I):
            raise ValueError("license-negative fixture failed before its intended modpost check")


def build_license_probes(build,work,args,env):
    for forbidden in (None,*GPL):
        directory=work/("license-"+(forbidden or "public"))
        directory.mkdir(exist_ok=True)
        (directory/"uuid_license.c").write_text(license_source(forbidden))
        (directory/"Makefile").write_text("obj-m := uuid_license.o\n")
        result=subprocess.run([*shlex.split(os.environ.get("MAKE","make")),"-C",str(ROOT),
            "O="+str(build),"M="+str(directory),*args,PRIVATE_C_FLAGS,"modules"],env=env,capture_output=True)
        (directory/"build.log").write_bytes(result.stdout+result.stderr)
        verify_license_result(result,forbidden)
        if forbidden is None:
            module=directory/"uuid_license.ko"
            require_metadata_field(metadata_fields(module),b"license",b"Proprietary")
            verify_references(module,tuple(name for name in PUBLIC if name not in GPL))
            verify_module_metadata(build,module,work=directory,require_c_suppression=True)
            verify_module_import_versions(build,module)
            undefined={row.split()[-1] for row in tool("nm","-u",module).splitlines() if row.split()}
            rows=[row.split() for row in (build/"Module.symvers").read_bytes().splitlines()]
            if any(len(row)>=4 and row[1] in undefined and row[3]!=b"EXPORT_SYMBOL" for row in rows):
                raise ValueError("proprietary UUID positive control imported GPL symbol")


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*",b"",line.strip()).strip()
            for line in normalize_console_transport(console).splitlines()]


def kunit_runs(console,runs):
    lines=console_lines(console)
    starts=[i for i,line in enumerate(lines) if line==b"# Subtest: uuid"]
    ends=[i for i,line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+uuid(?:\s.*)?",line)]
    if runs not in (1,2) or len(starts)!=runs or len(ends)!=runs:
        raise ValueError("missing or duplicate UUID suites")
    intervals=[]
    for start,end in zip(starts,ends):
        if start>=end or intervals and start<=intervals[-1][1]: raise ValueError("overlapping UUID suites")
        plan=False
        cases=0
        summaries=set()
        for i in range(start+1,end+1):
            line=lines[i]
            if re.match(rb"\d+\.\.",line):
                if line!=b"1..8" or plan or cases: raise ValueError("wrong or misplaced UUID plan")
                plan=True
            elif line.startswith(b"# Subtest:"): raise ValueError("unexpected UUID parameter suite")
            elif re.match(rb"(?:not )?ok\s",line):
                if i==end:
                    if not re.fullmatch(rb"ok [1-9]\d* uuid",line) or cases!=8: raise ValueError("UUID suite incomplete")
                elif plan and cases<8 and line==b"ok "+str(cases+1).encode()+b" "+CASES[cases]:
                    cases+=1
                else: raise ValueError("wrong, duplicate, skipped or reordered UUID case")
            elif re.match(rb"# (?:uuid|Totals|uuid_test_\w+):",line):
                match=re.fullmatch(rb"# ([\w]+): pass:(\d+) fail:0 skip:0 total:(\d+)",line)
                if not match or match[1] in summaries: raise ValueError("contradictory UUID totals")
                if match[1] in CASES:
                    if cases>=8 or match[1]!=CASES[cases] or match.groups()[1:]!=(b"1",b"1"):
                        raise ValueError("misplaced UUID case totals")
                elif match[1] in (b"uuid",b"Totals"):
                    if cases!=8 or match.groups()[1:]!=(b"8",b"8"): raise ValueError("wrong UUID totals")
                else: raise ValueError("unknown UUID summary")
                summaries.add(match[1])
            elif line.startswith(b"# module:") and line!=b"# module: uuid_kunit":
                raise ValueError("wrong UUID suite module")
        if not plan or cases!=8: raise ValueError("incomplete UUID suite")
        intervals.append((start,end))
    for i,line in enumerate(lines):
        if re.match(rb"(?:not )?ok\s+\d+\s+uuid_test_",line) and not any(a<i<b for a,b in intervals):
            raise ValueError("UUID case outside suite")
    return intervals


def verify_console(console,caller,*,suite="y",framework_module=False,reload=False,config=None):
    if suite not in ("y","m") or framework_module and suite!="m": raise ValueError("incompatible UUID lifecycle")
    if re.search(rb"CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version",console,re.I):
        raise ValueError("unexpected UUID kernel fault or KUnit failure")
    lines=console_lines(console)
    allowed=set()
    if config and config.get("INT_LOG_KUNIT_TEST")=="y":
        log_ranges=log_kunit_runs(console,1)
        allowed=set(log_kunit_warnings(lines,log_ranges,"Rust" if config.get("RUST_INT_LOG")=="y" else "C",config.get("BUG")=="y"))
    if {i for i,line in enumerate(lines) if b"WARNING:" in line}!=allowed:
        raise ValueError("warning outside configured original integer-log zero cases")
    ranges=kunit_runs(console,2 if suite=="m" and reload else 1)
    preloads=int(framework_module)+int(suite=="m")
    marker=result_marker(caller)
    expected=[b"KUNIT"] if suite=="y" else []
    for index in range(preloads):
        if index==preloads-1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [marker,b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads+1))]
        for index in range(preloads):
            if index==preloads-1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [marker,f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(b"LUPOS_RUST_BUILD_BOOT_OK")
    actual=[]
    for i,line in enumerate(lines):
        if any(i==end for _,end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start<=i<=end for start,end in ranges): raise ValueError("event inside unfinished UUID suite")
            actual.append(re.sub(rb"^uuid_(?:rust_)?abi: ",b"",line))
    if actual!=expected: raise ValueError("UUID caller/suite events missing, duplicate or out of order")
    verify_module_events(console,module=True,preloads=preloads,reload=reload)
    return len(ranges)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build",type=Path)
    parser.add_argument("--caller",choices=("c","rust"),default="c")
    parser.add_argument("--allow-c-baseline",action="store_true")
    parser.add_argument("--reload-modules",action="store_true")
    parser.add_argument("--make-arg",action="append",default=[])
    parser.add_argument("--qemu",default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data",type=Path)
    args=parser.parse_args()
    build=args.build.resolve()
    try:
        config=configuration(build)
        suite,framework=states(config)
        selection="Rust" if config.get("RUST_UUID")=="y" else "C"
        if selection=="C" and not args.allow_c_baseline: raise ValueError("requires RUST_UUID=y or --allow-c-baseline")
        for option in ("MODULES","PRINTK","MULTIUSER"):
            if config.get(option)!="y": raise ValueError("requires CONFIG_"+option+"=y")
        if args.caller=="rust" and config.get("RUST")!="y": raise ValueError("Rust caller requires RUST=y")
        if args.reload_modules and config.get("MODULE_UNLOAD")!="y": raise ValueError("reload requires MODULE_UNLOAD=y")
        if config.get("MODULE_SIG_FORCE")=="y": raise ValueError("disposable unsigned fixtures need MODULE_SIG_FORCE disabled")
        arch=architecture(config)
        preloads=verify_linked_implementation(build,selection)
    except (OSError,ValueError,IndexError,KeyError,struct.error,subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work=build/"rust-uuid-test"
    work.mkdir(exist_ok=True)
    name,source=("uuid_rust_abi","uuid_rust_main") if args.caller=="rust" else ("uuid_abi","uuid_c_main")
    (work/"uuid_reference.c").write_text(reference_source())
    if args.caller=="c": (work/"uuid_workload.c").write_text(WORKLOAD)
    (work/(source+(".rs" if args.caller=="rust" else ".c"))).write_text(sources(args.caller))
    workload=" uuid_workload.o" if args.caller=="c" else ""
    (work/"Makefile").write_text(f"obj-m := {name}.o\n{name}-y := {source}.o uuid_reference.o{workload}\n")
    env={key:value for key,value in os.environ.items() if not key.startswith(("CONFIG_","KCONFIG_","KBUILD_")) and key not in
         ("MAKEFLAGS","MFLAGS","CARGO_MAKEFLAGS","MAKELEVEL","MAKEOVERRIDES","srctree","srcroot","objtree","VPATH","sub_make_done")}
    build_license_probes(build,work,args.make_arg,env)
    subprocess.run([*shlex.split(os.environ.get("MAKE","make")),"-C",str(ROOT),"O="+str(build),"M="+str(work),
                    *args.make_arg,PRIVATE_C_FLAGS,"modules"],env=env,check=True)
    verify_consumer(build,work,args.caller)
    command=[sys.executable,str(ROOT/"scripts/tests/boot_kernel.py"),"--build",str(build),"--arch",arch,
             "--module",str(work/(name+".ko")),"--qemu",args.qemu or ("qemu-system-aarch64" if arch=="aarch64" else "qemu-system-x86_64")]
    for module in preloads: command += ["--preload-module",str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data",str(args.qemu_data)]
    subprocess.run(command,env=env,check=True)
    count=verify_console((build/"rust-boot-test/console.log").read_bytes(),args.caller,suite=suite,
        framework_module=framework=="m",reload=args.reload_modules,config=config)
    print(f"{selection} UUID/GUID: {args.caller} caller passed {COUNT} parsing records/load, all 61 alias offsets, "
          f"{RANDOM} real RNG checks, two null statics; {count} original eight-case KUnit runs on {arch}; mixed license gates passed.")


if __name__=="__main__":
    main()
