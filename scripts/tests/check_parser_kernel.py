#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit the selected parser and run independent disposable callers in a guest.

No host module loading. The original lib/parser.c is the private oracle.
This file generates temporary test sources, never production C.
"""
import argparse
import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import sys
import tempfile

from boot_kernel import module_name, verify_module_events
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_int_log_kernel import normalize_console_transport, kunit_runs, verify_kunit_warnings
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer
from check_prime_numbers_kernel import provider_type_ids, verify_guarded_calls, verify_module_import_versions, verify_rust_entrypoints
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports

ROOT = Path(__file__).resolve().parents[2]
EXPORTS = ('match_token', 'match_int', 'match_uint', 'match_u64', 'match_octal',
           'match_hex', 'match_wildcard', 'match_strlcpy', 'match_strdup')
WRAPPERS = tuple(('parser_call_' + name, name) for name in EXPORTS)
DESCRIPTION = b'Parser native public ABI check'
PRIVATE_C_FLAGS = 'CFLAGS_MODULE+=-D__DISABLE_EXPORTS'
NUMBERS = ('', '-', '+', '0', '1', '-1', '+1', '07', '08', '0x10', '0Xff',
           '2147483647', '2147483648', '-2147483648', '-2147483649',
           '4294967295', '4294967296', '18446744073709551615',
           '18446744073709551616', '123456789012345678901234', '12z', '12\n', ' 12')
PATTERNS = ('literal', '%%', '%s', '%1s', '%0s', '%d', '%u', '%o', '%x',
            'a=%2s,%d', '%1s%1s%1s%1s', '%s!', '%q')
STRINGS = ('', 'literal', '%', 'a=ab,7', 'abcdef', '-7', '0x10', '08', '0', 'xyz')
WILDCARDS = ('', '*', '?', '**', '*a*b', 'a?c', '*?*', 'ab*cd', 'a**b')
COUNT = len(NUMBERS) * 5 + len(PATTERNS) * len(STRINGS) + len(WILDCARDS) * len(STRINGS) + 33 * 34 + 33 + 5

# Native declarations: Rust uses the actual generated parser.h layouts.
SIGNATURES = {
    'match_token': ('int', [('char *', 's'), ('const struct match_token *', 't'), ('substring_t *', 'a')], 'ffi::c_int', ['*mut ffi::c_char', '*const bindings::match_token', '*mut bindings::substring_t']),
    **{name: ('int', [('substring_t *', 's'), (typ + ' *', 'v')], 'ffi::c_int', ['*mut bindings::substring_t', '*mut ' + rust])
       for name, typ, rust in [('match_int', 'int', 'ffi::c_int'), ('match_uint', 'unsigned int', 'ffi::c_uint'),
                               ('match_u64', 'u64', 'u64'), ('match_octal', 'int', 'ffi::c_int'), ('match_hex', 'int', 'ffi::c_int')]},
    'match_wildcard': ('bool', [('const char *', 'p'), ('const char *', 's')], 'bool', ['*const ffi::c_char', '*const ffi::c_char']),
    'match_strlcpy': ('size_t', [('char *', 'd'), ('const substring_t *', 's'), ('size_t', 'n')], 'usize', ['*mut ffi::c_char', '*const bindings::substring_t', 'usize']),
    'match_strdup': ('char *', [('const substring_t *', 's')], '*mut ffi::c_char', ['*const bindings::substring_t']),
}


def reference_source():
    return ('#include <linux/export.h>\n#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(x)\n' +
            ''.join('#define ' + name + ' parser_reference_' + name + '\n' for name in EXPORTS) +
            '#include "' + str(ROOT / 'lib/parser.c') + '"\n')


def wrappers(caller):
    result = []
    for name, (ret, args, rret, rargs) in SIGNATURES.items():
        values = ', '.join(value for _, value in args)
        if caller == 'c':
            decl = ', '.join(typ + ' ' + value for typ, value in args)
            types = ', '.join(typ for typ, _ in args)
            result.append(f'{ret} parser_reference_{name}({decl});\n'
                          f'static noinline {ret} parser_call_{name}({decl}) {{\n'
                          f' {ret} (*volatile selected)({types}) = {name}; return selected({values});\n}}')
        else:
            decl = ', '.join(value + ': ' + typ for (_, value), typ in zip(args, rargs))
            types = ', '.join(rargs)
            result.append(f'unsafe extern "C" {{ fn parser_reference_{name}({decl}) -> {rret}; }}\n'
                          '/// Invoke the real binding through a protected indirect call.\n'
                          '#[no_mangle]\n#[inline(never)]\n'
                          f'pub unsafe extern "C" fn parser_call_{name}({decl}) -> {rret} {{\n'
                          f' let selected: unsafe extern "C" fn({types}) -> {rret} = bindings::{name};\n'
                          f' unsafe {{ core::ptr::read_volatile(&selected)({values}) }}\n}}')
    return '\n'.join(result)


C_BODY = r'''
#include <linux/module.h>
#include <linux/parser.h>
#include <linux/slab.h>
#include <linux/string.h>
@WRAPPERS@
static int parser_exercise(void)
{
    const char *numbers[] = {@NUMBERS@};
    const char *patterns[] = {@PATTERNS@};
    const char *strings[] = {@STRINGS@};
    const char *wildcards[] = {@WILDCARDS@};
    unsigned int i, j, k, count = 0;
    for (i = 0; i < ARRAY_SIZE(numbers); ++i) {
        substring_t s = {.from = (char *)numbers[i], .to = (char *)numbers[i] + strlen(numbers[i])};
        @CONVERSIONS@
    }
    for (i = 0; i < ARRAY_SIZE(patterns); ++i) {
        struct match_token table[] = {{17, patterns[i]}, {29, NULL}};
        for (j = 0; j < ARRAY_SIZE(strings); ++j) {
            substring_t a[3], b[3];
            char *s = (char *)strings[j];
            for (k = 0; k < 3; ++k) a[k] = b[k] = (substring_t){s, s};
            if (parser_call_match_token(s, table, a) != parser_reference_match_token(s, table, b)) return -EINVAL;
            for (k = 0; k < 3; ++k) if (a[k].from != b[k].from || a[k].to != b[k].to) return -EINVAL;
            ++count;
        }
    }
    {
        struct match_token sentinel[] = {{73, NULL}};
        struct match_token literal[] = {{17, "literal"}, {29, NULL}};
        struct match_token single[] = {{17, "%1s!"}, {19, "%s"}, {29, NULL}};
        char text[] = "abc";
        substring_t a, b;
        if (parser_call_match_token(NULL, sentinel, NULL) != parser_reference_match_token(NULL, sentinel, NULL)) return -EINVAL;
        if (parser_call_match_token("literal", literal, NULL) != parser_reference_match_token("literal", literal, NULL)) return -EINVAL;
        if (parser_call_match_token(text, single, &a) != parser_reference_match_token(text, single, &b) || a.from != b.from || a.to != b.to) return -EINVAL;
        count += 3;
    }
    {
        union { struct match_token table[3]; substring_t args[3]; } a, b;
        char text[] = "overlap";
        memset(&a, 0, sizeof(a)); memset(&b, 0, sizeof(b));
        a.table[0].token = b.table[0].token = 17;
        a.table[0].pattern = b.table[0].pattern = "%s";
        if (parser_call_match_token(text, a.table, a.args) != parser_reference_match_token(text, b.table, b.args) || memcmp(&a, &b, sizeof(a))) return -EINVAL;
        ++count;
    }
    for (i = 0; i < ARRAY_SIZE(wildcards); ++i)
        for (j = 0; j < ARRAY_SIZE(strings); ++j) {
            if (parser_call_match_wildcard(wildcards[i], strings[j]) != parser_reference_match_wildcard(wildcards[i], strings[j])) return -EINVAL;
            ++count;
        }
    for (i = 0; i <= 32; ++i) {
        char text[33], a[36], b[36], *x, *y;
        substring_t s = {text, text + i};
        for (j = 0; j < sizeof(text); ++j) text[j] = j % 7 ? 'a' + j % 26 : 0;
        for (j = 0; j <= 33; ++j) {
            size_t xlen, ylen;
            memset(a, 0x55, sizeof(a)); memset(b, 0x55, sizeof(b));
            xlen = parser_call_match_strlcpy(j ? a + 1 : NULL, &s, j);
            ylen = parser_reference_match_strlcpy(j ? b + 1 : NULL, &s, j);
            if (xlen != ylen || memcmp(a, b, sizeof(a))) return -EINVAL;
            ++count;
        }
        x = parser_call_match_strdup(&s); y = parser_reference_match_strdup(&s);
        /* Independent GFP_KERNEL allocations can fail independently. Check
         * every successful allocation bytewise, and always free both results. */
        k = (x && (memcmp(x, text, i) || x[i])) || (y && (memcmp(y, text, i) || y[i]));
        kfree(x); kfree(y);
        if (k) return -EINVAL;
        ++count;
    }
    {
        /* Descriptor overlap is defined: length is read before memcpy, and
         * source bytes live separately from the overwritten descriptor. */
        char text[] = "descriptor";
        substring_t a = {text, text + 10}, b = a;
        if (parser_call_match_strlcpy((char *)&a, &a, sizeof(a)) != parser_reference_match_strlcpy((char *)&b, &b, sizeof(b)) || memcmp(&a, &b, sizeof(a))) return -EINVAL;
        ++count;
    }
    return count == @COUNT@ ? 0 : -EINVAL;
}
static int __init parser_abi_init(void)
{
    int error = parser_exercise();
    if (error) { pr_err("LUPOS_PARSER_FAIL\n"); return error; }
    pr_info("@MARKER@\n"); return 0;
}
static void __exit parser_abi_exit(void) {}
module_init(parser_abi_init);
module_exit(parser_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Parser native public ABI check");
'''

RUST_BODY = r'''// SPDX-License-Identifier: GPL-2.0-only
//! Independent parser workload using generated native bindings.
use kernel::{bindings, ffi};
@WRAPPERS@
/// Execute the entire workload in Rust; C only supplies scalar oracles.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn parser_rust_exercise() -> ffi::c_int {
    // SAFETY: Each pointer below stays within its allocated test storage;
    // nulls are passed only on paths where original C never accesses them.
    unsafe {
    let numbers: &[&[u8]] = &[@NUMBERS@];
    let patterns: &[&[u8]] = &[@PATTERNS@];
    let strings: &[&[u8]] = &[@STRINGS@];
    let wildcards: &[&[u8]] = &[@WILDCARDS@];
    let mut count = 0usize;
    for number in numbers {
        let p = number.as_ptr().cast::<ffi::c_char>().cast_mut();
        let mut s = bindings::substring_t { from: p, to: p.add(number.len() - 1) };
        @CONVERSIONS@
    }
    for pattern in patterns {
        let table = [bindings::match_token { token: 17, pattern: pattern.as_ptr().cast() },
                     bindings::match_token { token: 29, pattern: core::ptr::null() }];
        for string in strings {
            let s = string.as_ptr().cast::<ffi::c_char>().cast_mut();
            let mut a = core::array::from_fn::<_, 3, _>(|_| bindings::substring_t { from: s, to: s });
            let mut b = core::array::from_fn::<_, 3, _>(|_| bindings::substring_t { from: s, to: s });
            if parser_call_match_token(s, table.as_ptr(), a.as_mut_ptr()) != parser_reference_match_token(s, table.as_ptr(), b.as_mut_ptr()) { return -22; }
            for k in 0..3 { if a[k].from != b[k].from || a[k].to != b[k].to { return -22; } }
            count += 1;
        }
    }
    let sentinel = [bindings::match_token { token: 73, pattern: core::ptr::null() }];
    let literal = [bindings::match_token { token: 17, pattern: c"literal".as_ptr().cast() },
                   bindings::match_token { token: 29, pattern: core::ptr::null() }];
    let single = [bindings::match_token { token: 17, pattern: c"%1s!".as_ptr().cast() },
                  bindings::match_token { token: 19, pattern: c"%s".as_ptr().cast() },
                  bindings::match_token { token: 29, pattern: core::ptr::null() }];
    if parser_call_match_token(core::ptr::null_mut(), sentinel.as_ptr(), core::ptr::null_mut()) != parser_reference_match_token(core::ptr::null_mut(), sentinel.as_ptr(), core::ptr::null_mut()) { return -22; }
    if parser_call_match_token(c"literal".as_ptr().cast_mut().cast(), literal.as_ptr(), core::ptr::null_mut()) != parser_reference_match_token(c"literal".as_ptr().cast_mut().cast(), literal.as_ptr(), core::ptr::null_mut()) { return -22; }
    let mut text = *b"abc\0";
    let mut a = core::mem::MaybeUninit::<bindings::substring_t>::uninit();
    let mut b = core::mem::MaybeUninit::<bindings::substring_t>::uninit();
    if parser_call_match_token(text.as_mut_ptr().cast(), single.as_ptr(), a.as_mut_ptr()) != parser_reference_match_token(text.as_mut_ptr().cast(), single.as_ptr(), b.as_mut_ptr()) { return -22; }
    let a = a.assume_init(); let b = b.assume_init();
    if a.from != b.from || a.to != b.to { return -22; }
    count += 3;
    // C permits table/capture overlap and reloads the overwritten token.
    union TableCapture {
        table: core::mem::ManuallyDrop<[bindings::match_token; 3]>,
        args: core::mem::ManuallyDrop<[bindings::substring_t; 3]>,
    }
    let mut a: TableCapture = core::mem::zeroed(); let mut b: TableCapture = core::mem::zeroed();
    let at = core::ptr::addr_of_mut!(a.table).cast::<bindings::match_token>();
    let bt = core::ptr::addr_of_mut!(b.table).cast::<bindings::match_token>();
    core::ptr::addr_of_mut!((*at).token).write(17);
    core::ptr::addr_of_mut!((*bt).token).write(17);
    core::ptr::addr_of_mut!((*at).pattern).write(c"%s".as_ptr().cast());
    core::ptr::addr_of_mut!((*bt).pattern).write(c"%s".as_ptr().cast());
    let mut text = *b"overlap\0";
    if parser_call_match_token(text.as_mut_ptr().cast(), at, core::ptr::addr_of_mut!(a.args).cast()) != parser_reference_match_token(text.as_mut_ptr().cast(), bt, core::ptr::addr_of_mut!(b.args).cast()) { return -22; }
    for j in 0..core::mem::size_of::<TableCapture>() {
        if *core::ptr::addr_of!(a).cast::<u8>().add(j) != *core::ptr::addr_of!(b).cast::<u8>().add(j) { return -22; }
    }
    count += 1;
    for pattern in wildcards {
        for string in strings {
            if parser_call_match_wildcard(pattern.as_ptr().cast(), string.as_ptr().cast()) != parser_reference_match_wildcard(pattern.as_ptr().cast(), string.as_ptr().cast()) { return -22; }
            count += 1;
        }
    }
    for len in 0..=32usize {
        let mut text = [0u8; 33];
        for (j, byte) in text.iter_mut().enumerate() { *byte = if j % 7 == 0 { 0 } else { b'a' + (j % 26) as u8 }; }
        let s = bindings::substring_t { from: text.as_mut_ptr().cast(), to: text.as_mut_ptr().add(len).cast() };
        for size in 0..=33usize {
            let mut a = [0x55u8; 36]; let mut b = [0x55u8; 36];
            let x = if size == 0 { core::ptr::null_mut() } else { a.as_mut_ptr().add(1).cast() };
            let y = if size == 0 { core::ptr::null_mut() } else { b.as_mut_ptr().add(1).cast() };
            if parser_call_match_strlcpy(x, &s, size) != parser_reference_match_strlcpy(y, &s, size) || a != b { return -22; }
            count += 1;
        }
        let x = parser_call_match_strdup(&s); let y = parser_reference_match_strdup(&s);
        let mut wrong = false;
        for &p in &[x, y] { if !p.is_null() {
            for (j, byte) in text.iter().take(len).enumerate() { if *p.add(j).cast::<u8>() != *byte { wrong = true; } }
            if *p.add(len) != 0 { wrong = true; }
        } }
        bindings::kfree(x.cast()); bindings::kfree(y.cast());
        if wrong { return -22; }
        count += 1;
    }
    let mut text = *b"descriptor\0";
    let mut a = bindings::substring_t { from: text.as_mut_ptr().cast(), to: text.as_mut_ptr().add(10).cast() };
    let mut b = bindings::substring_t { from: a.from, to: a.to };
    let ap = core::ptr::addr_of_mut!(a); let bp = core::ptr::addr_of_mut!(b);
    let size = core::mem::size_of::<bindings::substring_t>();
    if parser_call_match_strlcpy(ap.cast(), ap, size) != parser_reference_match_strlcpy(bp.cast(), bp, size) { return -22; }
    for j in 0..size { if *ap.cast::<u8>().add(j) != *bp.cast::<u8>().add(j) { return -22; } }
    count += 1;
    if count != @COUNT@ { return -22; }
    0
    }
}
/// Execute the Rust workload before publishing success.
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    // SAFETY: The workload owns its storage and retains no resources.
    let error = unsafe { parser_rust_exercise() };
    if error != 0 { return error; }
    // SAFETY: Static NUL terminated format, no arguments.
    unsafe { bindings::_printk(c"\x016@MARKER@\n".as_ptr().cast()); }
    0
}
/// All allocations were freed by the workload.
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn() -> ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=GPL\0description=Parser native public ABI check\0";
#[used]
#[link_section = ".modinfo"]
static MODINFO: [u8; INFO.len()] = {
    let mut bytes = [0; INFO.len()]; let mut i = 0;
    while i < bytes.len() { bytes[i] = INFO.as_bytes()[i]; i += 1; } bytes
};
#[used]
static __IS_RUST_MODULE: () = ();
'''


def result_marker(caller):
    if caller not in ('c', 'rust'): raise ValueError('unknown parser caller')
    return f'LUPOS_PARSER_{"RUST_" if caller == "rust" else ""}ABI_OK cases={COUNT} exports=9'.encode()


def sources(caller):
    result_marker(caller)
    source = C_BODY if caller == 'c' else RUST_BODY
    conversions = []
    for name in EXPORTS[1:6]:
        ret, args, _, rargs = SIGNATURES[name]
        if caller == 'c':
            typ = args[1][0].removesuffix(' *')
            conversions.append(f'{{ {typ} a = 85, b = 85; int x, y;\n'
                f'x = parser_call_{name}(&s, &a); y = parser_reference_{name}(&s, &b);\n'
                'if (x != y || a != b) return -EINVAL;\n'
                f'if (y && (parser_call_{name}(&s, NULL) != y || parser_reference_{name}(&s, NULL) != y)) return -EINVAL;\n'
                f'{{ union {{ char text[32]; {typ} output; }} u, v; substring_t us, vs;\n'
                'memset(&u, 0, sizeof(u)); memset(&v, 0, sizeof(v));\n'
                'memcpy(u.text, numbers[i], strlen(numbers[i])); memcpy(v.text, numbers[i], strlen(numbers[i]));\n'
                'us = (substring_t){u.text, u.text + strlen(numbers[i])}; vs = (substring_t){v.text, v.text + strlen(numbers[i])};\n'
                f'if (parser_call_{name}(&us, &u.output) != parser_reference_{name}(&vs, &v.output) || memcmp(&u, &v, sizeof(u))) return -EINVAL; }} ++count; }}')
        else:
            typ = rargs[1].removeprefix('*mut ')
            conversions.append(f'{{ let mut a: {typ} = 85; let mut b: {typ} = 85;\n'
                f'let x = parser_call_{name}(&mut s, &mut a); let y = parser_reference_{name}(&mut s, &mut b);\n'
                'if x != y || a != b { return -22; }\n'
                f'if y != 0 && (parser_call_{name}(&mut s, core::ptr::null_mut()) != y || parser_reference_{name}(&mut s, core::ptr::null_mut()) != y) {{ return -22; }}\n'
                f'union Input {{ text: [u8; 32], output: {typ} }}\n'
                'let mut u = Input { text: [0; 32] }; let mut v = Input { text: [0; 32] };\n'
                'let up = core::ptr::addr_of_mut!(u.text).cast::<u8>(); let vp = core::ptr::addr_of_mut!(v.text).cast::<u8>();\n'
                'core::ptr::copy_nonoverlapping(number.as_ptr(), up, number.len()); core::ptr::copy_nonoverlapping(number.as_ptr(), vp, number.len());\n'
                'let mut us = bindings::substring_t { from: up.cast(), to: up.add(number.len() - 1).cast() };\n'
                'let mut vs = bindings::substring_t { from: vp.cast(), to: vp.add(number.len() - 1).cast() };\n'
                f'if parser_call_{name}(&mut us, core::ptr::addr_of_mut!(u.output)) != parser_reference_{name}(&mut vs, core::ptr::addr_of_mut!(v.output)) {{ return -22; }}\n'
                'if u.text != v.text { return -22; } count += 1; }')
    import json
    for key, values in [('NUMBERS', NUMBERS), ('PATTERNS', PATTERNS), ('STRINGS', STRINGS), ('WILDCARDS', WILDCARDS)]:
        literals = [json.dumps(value) if caller == 'c' else 'b' + json.dumps(value + '\0').replace('\\u0000', '\\0') for value in values]
        source = source.replace('@' + key + '@', ', '.join(literals))
    return source.replace('@WRAPPERS@', wrappers(caller)).replace('@CONVERSIONS@', '\n'.join(conversions)).replace('@COUNT@', str(COUNT)).replace('@MARKER@', result_marker(caller).decode())


def provider(config):
    architecture(config)
    if config.get('UML') == 'y': raise ValueError('system-QEMU runner cannot boot UML')
    if config.get('RUST_PARSER', 'n') not in ('n', 'y'): raise ValueError('invalid parser selector')
    if config.get('RUST_PARSER') == 'y' and config.get('RUST') != 'y':
        raise ValueError('RUST_PARSER requires RUST')
    return 'Rust' if config.get('RUST_PARSER') == 'y' else 'C'


def owner_path(build, selection):
    if selection not in ('C', 'Rust'): raise ValueError('unknown parser provider')
    return build / ('lib/parser_rust.o' if selection == 'Rust' else 'lib/parser.o')


def selected_versions(build):
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()
            if len(line.split()) > 1 and line.split()[1].decode() in EXPORTS]
    if len(rows) != 9 or {row[1] for row in rows} != {name.encode() for name in EXPORTS}:
        raise ValueError('parser exported names missing or duplicated')
    if any(len(row) != 4 or row[2:] != [b'vmlinux', b'EXPORT_SYMBOL'] or
           not re.fullmatch(rb'0x[0-9a-fA-F]{8}', row[0]) for row in rows):
        raise ValueError('wrong parser export owner/license/namespace/CRC')
    return {row[1]: row[0].lower() for row in rows}


def verify_dwarf(build, owner, versions):
    config = configuration(build)
    if config.get('MODVERSIONS') != 'y': return
    if config.get('GENDWARFKSYMS') != 'y': raise ValueError('parser requires defining-object GENDWARFKSYMS proof')
    with tempfile.TemporaryDirectory(prefix='parser-dwarf-') as temporary:
        types = Path(temporary) / 'types'
        result = subprocess.run([str(build / 'scripts/gendwarfksyms/gendwarfksyms'), '--symtypes', str(types), str(owner)],
            input=b''.join(name + b'\n' for name in versions), check=True, capture_output=True, timeout=180)
        rows = [line.split() for line in result.stdout.splitlines()]
        if result.stderr or len(rows) != 9 or any(len(row) != 3 or row[0] != b'#SYMVER' for row in rows):
            raise ValueError('malformed parser defining-object version proof')
        if {row[1]: row[2] for row in rows} != versions: raise ValueError('parser defining-object DWARF CRC differs')
        if not types.read_bytes(): raise ValueError('empty parser DWARF type proof')


def verify_linked_implementation(build, selection):
    config = configuration(build)
    if selection != provider(config): raise ValueError('parser provider differs from configuration')
    arch, owner = architecture(config), owner_path(build, selection)
    candidates = {owner_path(build, kind).resolve() for kind in ('C', 'Rust')}
    # Parser is a direct built-in lib member, not a lib-y archive member.
    archives = [build / 'lib/built-in.a', build / 'vmlinux.a']
    for archive in archives:
        ordered = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', archive).splitlines()]
        if [member for member in ordered if member in candidates] != [owner.resolve()]:
            raise ValueError('parser archive owner absent, duplicated or mixed')
        newer(archive, [owner])
    source = ROOT / ('lib/parser_rust.rs' if selection == 'Rust' else 'lib/parser.c')
    deps = [ROOT / 'include/linux/parser.h'] if selection == 'C' else [ROOT / name for name in
        ('lib/parser.rs', 'include/linux/parser_header.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs')]
    if selection == 'Rust': deps += [build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta']
    verify_build_command(build, owner, source, deps)
    flags = compilation_flags(owner)
    if selection == 'Rust':
        if [flag for flag in flags if flag.startswith('RUST_MODFILE=')] != ['RUST_MODFILE=lib/parser']:
            raise ValueError('parser wrapper lost original __modname/modfile')
    elf_target(owner, arch)
    verify_references(owner, ('kmemdup_nul',))
    records = read_exports(owner)
    if len(records) != 9 or {row['name'] for row in records} != set(EXPORTS): raise ValueError('wrong parser export set')
    for row in records:
        expected = dict(license='', namespace='', relocation_target=row['name'], relocation_addend=0,
                        pointer_width=8, relocation_kind=257 if arch == 'aarch64' else 1,
                        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        if any(row[key] != value for key, value in expected.items()): raise ValueError('incorrect unrestricted parser export metadata')
    for path in (owner, build / 'vmlinux.o', build / 'vmlinux'):
        definitions = [row.split() for row in tool('nm', '-g', '--defined-only', path).splitlines()
                       if row.split() and row.split()[-1].decode() in EXPORTS]
        if len(definitions) != 9 or {row[-1] for row in definitions} != {name.encode() for name in EXPORTS} or any(row[-2] != b'T' for row in definitions):
            raise ValueError('parser final definitions absent, duplicated or not functions')
    versions = selected_versions(build)
    if config.get('MODVERSIONS') == 'y' and version_records(owner.with_name('.' + owner.name + '.cmd')) != versions:
        raise ValueError('parser CRC provenance differs from defining unit')
    verify_dwarf(build, owner, versions)
    if config.get('CFI') == 'y':
        if ('-Zsanitizer=kcfi' if selection == 'Rust' else '-fsanitize=kcfi') not in flags:
            raise ValueError('parser owner missing KCFI')
        provider_type_ids(owner, names=EXPORTS)
    newer(build / 'Module.symvers', [owner])
    newer(build / 'vmlinux.o', archives)
    newer(build / 'vmlinux', [build / 'vmlinux.o'])
    newer(build / ('arch/arm64/boot/Image' if arch == 'aarch64' else 'arch/x86/boot/bzImage'), [build / 'vmlinux'])
    return owner


def verify_rust_workload(obj, module=None):
    imports = {row.split()[-1] for row in tool('nm', '-u', obj).splitlines() if row.split()}
    required = {name.encode() for name in EXPORTS} | {('parser_reference_' + name).encode() for name in EXPORTS} | {b'kfree', b'_printk'}
    panics = {name for name in imports if re.fullmatch(rb'_R\w+4core9panicking\w+', name)}
    if not required <= imports or imports - required - panics - {b'memcpy', b'memset', b'memcmp', b'bcmp'}:
        raise ValueError('Rust parser imports indicate missing workload or C dispatch delegation: ' + repr(sorted(imports - required)))
    for path in (obj, *([module] if module is not None else [])):
        definitions = {row.split()[-1] for row in tool('nm', '-g', '--defined-only', path).splitlines() if row.split()}
        if b'parser_rust_exercise' not in definitions or b'parser_exercise' in definitions:
            raise ValueError('Rust parser does not own its workload')


def verify_consumer(build, work, caller):
    result_marker(caller)
    name, stem = ('parser_rust_abi', 'parser_rust_main') if caller == 'rust' else ('parser_abi', 'parser_c_main')
    obj, module, oracle = work / (stem + '.o'), work / (name + '.ko'), work / 'parser_reference.o'
    config = configuration(build)
    arch = architecture(config)
    source = obj.with_suffix('.rs' if caller == 'rust' else '.c')
    if source.read_text() != sources(caller): raise ValueError('parser caller source changed')
    deps = [build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta'] if caller == 'rust' else [ROOT / 'include/linux/parser.h']
    verify_build_command(work, obj, source, deps)
    if oracle.with_suffix('.c').read_text() != reference_source(): raise ValueError('parser original oracle changed')
    verify_build_command(work, oracle, oracle.with_suffix('.c'), [ROOT / 'lib/parser.c', ROOT / 'include/linux/parser.h'])
    members = [(work / os.fsdecode(row)).resolve() for row in module.with_suffix('.mod').read_bytes().splitlines()]
    if members != [obj.resolve(), oracle.resolve()]: raise ValueError('unexpected parser module constituents')
    for path in (obj, module):
        elf_target(path, arch)
        verify_references(path, EXPORTS)
    if module_name(module) != name: raise ValueError('wrong parser module identity')
    newer(module, [obj, oracle, selected_metadata(build, module)])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_module_import_versions(build, module)
    require_metadata_field(metadata_fields(module), b'license', b'GPL' if caller == 'rust' else b'Proprietary')
    require_metadata_field(metadata_fields(module), b'description', DESCRIPTION)
    if read_exports(module): raise ValueError('private parser caller exports symbols')
    undefined = {row.split()[-1] for row in tool('nm', '-u', module).splitlines() if row.split()}
    rows = [row.split() for row in (build / 'Module.symvers').read_bytes().splitlines()]
    if caller == 'c' and any(len(row) >= 4 and row[1] in undefined and row[3] != b'EXPORT_SYMBOL' for row in rows):
        raise ValueError('Proprietary parser caller imports GPL-only symbol')
    if caller == 'rust':
        flags = compilation_flags(obj)
        if not {'-Dwarnings', '-Dunsafe_op_in_unsafe_fn'} <= set(flags):
            raise ValueError('Rust parser caller missing strict warning/unsafe flags')
        verify_rust_workload(obj, module)
        for path in (obj, module): verify_rust_entrypoints(path, arch)
    for path in (oracle, *([obj] if caller == 'c' else [])):
        flags = compilation_flags(path)
        if '-D__DISABLE_EXPORTS' not in flags or '-U__DISABLE_EXPORTS' in flags or read_exports(path):
            raise ValueError('private parser C retains incidental exports')
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    if config.get('CFI') == 'y':
        flags = compilation_flags(obj)
        required = '-Zsanitizer=kcfi' if caller == 'rust' else '-fsanitize=kcfi'
        if required not in flags or any(flag.startswith('-fno-sanitize=') and {'all', 'kcfi'} & set(flag.split('=', 1)[1].split(',')) for flag in flags):
            raise ValueError('parser caller missing native KCFI')
        types = provider_type_ids(owner_path(build, provider(config)), names=EXPORTS)
        for path in (obj, module): verify_guarded_calls(path, arch, types, wrappers=WRAPPERS)


def verify_console(console, caller, *, reload=False, config=None):
    if re.search(rb'CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version', console, re.I):
        raise ValueError('unexpected parser kernel fault or failed test')
    lines = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip()).strip()
             for line in normalize_console_transport(console).splitlines()]
    allowed = set()
    if config and config.get('INT_LOG_KUNIT_TEST') == 'y':
        ranges = kunit_runs(console, 1)
        allowed = set(verify_kunit_warnings(lines, ranges, 'Rust' if config.get('RUST_INT_LOG') == 'y' else 'C', config.get('BUG') == 'y'))
    if {i for i, line in enumerate(lines) if b'WARNING:' in line} != allowed:
        raise ValueError('warning outside configured original integer-log zero cases')
    marker = result_marker(caller)
    expected = [marker, b'LUPOS_RUST_MODULE_LOAD_OK']
    if reload: expected += [b'LUPOS_RUST_MODULE_UNLOAD_OK 0', marker, b'LUPOS_RUST_MODULE_RELOAD_OK 0']
    expected.append(b'LUPOS_RUST_BUILD_BOOT_OK')
    actual = [re.sub(rb'^parser_(?:rust_)?abi: ', b'', line) for line in lines if b'LUPOS_' in line]
    if actual != expected: raise ValueError('parser workload/caller/load events differ')
    verify_module_events(console, module=True, preloads=0, reload=reload)
    return 2 if reload else 1


def nonempty(value):
    if not value.strip(): raise argparse.ArgumentTypeError('explicit value must not be empty')
    return value


def command_words(value, name):
    """Validate shell-style argv without executing a shell or the command."""
    try:
        words = shlex.split(value)
    except ValueError as error:
        raise ValueError(name + ' command is malformed: ' + str(error)) from error
    if not words or not words[0]: raise ValueError(name + ' command must name an executable')
    if any('\0' in word for word in words): raise ValueError(name + ' command contains NUL')
    executable = shutil.which(words[0])
    if executable is None: raise ValueError(name + ' executable does not exist: ' + words[0])
    # Preserve a symlink's basename while fixing relative PATH/command entries
    # before a child can change its working directory.
    return [os.path.abspath(executable), *words[1:]]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build', type=nonempty)
    parser.add_argument('--caller', choices=('c', 'rust'), default='c')
    parser.add_argument('--allow-c-baseline', action='store_true')
    parser.add_argument('--audit-only', action='store_true')
    parser.add_argument('--reload-modules', action='store_true')
    parser.add_argument('--make-arg', action='append', default=[], type=nonempty)
    parser.add_argument('--qemu', type=nonempty)
    parser.add_argument('--qemu-data', type=nonempty)
    args = parser.parse_args()
    build = Path(args.build).resolve()
    try:
        if not build.is_dir(): raise ValueError('build directory does not exist')
        if args.qemu_data and not Path(args.qemu_data).is_dir(): raise ValueError('qemu-data directory does not exist')
        # Validate every explicit command before artifact audit or fixture writes,
        # including environment values overridden by command-line selection.
        make = command_words(os.environ['MAKE'], 'MAKE') if 'MAKE' in os.environ else None
        emulator = command_words(os.environ['QEMU'], 'QEMU') if 'QEMU' in os.environ else None
        if args.qemu is not None: emulator = command_words(args.qemu, '--qemu')
        config = configuration(build)
        arch = architecture(config)
        selection = provider(config)
        if (args.caller == 'rust' or selection == 'Rust') and int(config.get('RUSTC_VERSION', '0')) < 108500:
            raise ValueError('native Rust metadata requires Rust >=1.85')
        if selection == 'C' and not args.allow_c_baseline: raise ValueError('requires RUST_PARSER=y or --allow-c-baseline')
        if not args.audit_only:
            if make is None: make = command_words('make', 'MAKE')
            if emulator is None:
                emulator = command_words('qemu-system-aarch64' if arch == 'aarch64' else 'qemu-system-x86_64', 'QEMU')
            for option in ('MODULES', 'PRINTK', 'MULTIUSER', 'CFI'):
                if config.get(option) != 'y': raise ValueError('requires CONFIG_' + option + '=y')
            if args.caller == 'rust' and config.get('RUST') != 'y': raise ValueError('Rust caller requires RUST=y')
            if args.reload_modules and config.get('MODULE_UNLOAD') != 'y': raise ValueError('reload requires MODULE_UNLOAD=y')
            if config.get('MODULE_SIG_FORCE') == 'y': raise ValueError('unsigned fixture requires MODULE_SIG_FORCE disabled')
        owner = verify_linked_implementation(build, selection)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError, argparse.ArgumentTypeError) as error:
        parser.error(str(error))
    if args.audit_only:
        print(f'{selection} parser: selected {owner}; artifact audit only, no runtime claim.')
        return
    work = build / 'rust-parser-test'
    work.mkdir(exist_ok=True)
    name, stem = ('parser_rust_abi', 'parser_rust_main') if args.caller == 'rust' else ('parser_abi', 'parser_c_main')
    (work / 'parser_reference.c').write_text(reference_source())
    (work / (stem + ('.rs' if args.caller == 'rust' else '.c'))).write_text(sources(args.caller))
    (work / 'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o parser_reference.o\n')
    env = {key: value for key, value in os.environ.items() if not key.startswith(('CONFIG_', 'KCONFIG_', 'KBUILD_')) and key not in
        ('MAKEFLAGS', 'MFLAGS', 'CARGO_MAKEFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES', 'srctree', 'srcroot', 'objtree', 'VPATH', 'sub_make_done')}
    subprocess.run([*make, '-C', str(ROOT), 'O=' + str(build), 'M=' + str(work),
                    *args.make_arg, PRIVATE_C_FLAGS, 'KRUSTFLAGS+=-Dwarnings -Dunsafe_op_in_unsafe_fn', 'modules'], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / 'scripts/tests/boot_kernel.py'), '--build', str(build), '--arch', arch,
               '--module', str(work / (name + '.ko')), '--qemu', shlex.join(emulator)]
    if args.reload_modules: command += ['--reload-modules']
    if args.qemu_data: command += ['--qemu-data', str(Path(args.qemu_data).resolve())]
    subprocess.run(command, env=env, check=True)
    runs = verify_console((build / 'rust-boot-test/console.log').read_bytes(), args.caller, reload=args.reload_modules, config=config)
    print(f'{selection} parser: independent {args.caller} caller, {COUNT} cases/load, nine exports, {runs} loads on {arch}.')


if __name__ == '__main__': main()
