#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit the genuine memweight owner and run independent allocated-buffer callers.

The original unchanged C is the only algorithm oracle. Success workloads never
enter the original BUG_ON domain. Crashing threshold VM validation is a separate
root-owned gate; private unit controls cover the source boundary. No host insmod.
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
EXPORTS = ('memweight',)
WRAPPERS = (('memweight_call', 'memweight'),)
COUNT = 12651
DESCRIPTION = b'Non-GPL memweight public ABI check'
PRIVATE_C_FLAGS = 'CFLAGS_MODULE+=-D__DISABLE_EXPORTS'


def nonempty_path(value):
    if not value.strip(): raise argparse.ArgumentTypeError('empty path input')
    return Path(value)


def provider(config):
    architecture(config)
    if config.get('RUST_MEMWEIGHT', 'n') not in ('n', 'y'):
        raise ValueError('invalid RUST_MEMWEIGHT selector')
    if config.get('RUST_MEMWEIGHT') == 'y':
        if config.get('RUST') != 'y': raise ValueError('RUST_MEMWEIGHT requires RUST')
        return 'Rust'
    return 'C'


def owner_path(build, selection):
    if selection not in ('C', 'Rust'): raise ValueError('unknown memweight provider')
    return build / ('lib/memweight_rust.o' if selection == 'Rust' else 'lib/memweight.o')


def verify_bindings(build):
    newer(build / 'rust/libbindings.rmeta', [ROOT / 'include/linux/bitmap.h',
        ROOT / 'include/linux/slab.h', ROOT / 'rust/bindings/bindings_helper.h', ROOT / 'rust/bindgen_parameters'])
    newer(build / 'rust/libkernel.rmeta', [build / 'rust/libbindings.rmeta'])


def verify_strict_lints(flags):
    """Interpret ordered warnings options, including the global lint cap.

    Per-lint kernel policy (for example -Astable_features) remains selectable.
    force-warn is different: it can make even -Dwarnings nonfatal.
    """
    levels = {'A': 'allow', 'W': 'warn', 'D': 'deny', 'F': 'forbid'}
    warnings, cap = None, None
    tokens = iter(flags)
    for token in tokens:
        level, value = None, None
        if token in ('--cap-lints', '--force-warn', '--allow', '--warn', '--deny', '--forbid'):
            level, value = token[2:], next(tokens, None)
        elif any(token.startswith('--' + name + '=') for name in
                 ('cap-lints', 'force-warn', 'allow', 'warn', 'deny', 'forbid')):
            level, value = token[2:].split('=', 1)
        elif len(token) >= 2 and token[0] == '-' and token[1] in levels:
            level = levels[token[1]]
            value = token[2:] or next(tokens, None)
        if level is None: continue
        if not value or value.startswith('='):
            raise ValueError('malformed Rust lint option')
        if level == 'cap-lints':
            if value not in ('allow', 'warn', 'deny', 'forbid'):
                raise ValueError('unsupported Rust lint cap')
            # rustc takes the first cap-lints value (unlike ordered lint levels).
            if cap is None: cap = value
        elif level == 'force-warn':
            raise ValueError('Rust force-warn bypasses -Dwarnings')
        # Rust 1.85 treats a comma-separated spelling as one unknown lint,
        # not a list. It must not accidentally establish strict warnings.
        elif value == 'warnings':
            if warnings != 'forbid': warnings = level
    if warnings not in ('deny', 'forbid') or cap not in (None, 'deny', 'forbid'):
        raise ValueError('Rust memweight requires unsuppressed -Dwarnings')


def response_flags(flags, base, active=()):
    expanded = []
    for flag in flags:
        if flag.startswith('@'):
            path = (base / flag[1:]).resolve()
            if path in active: raise ValueError('recursive Rust response file')
            expanded.extend(response_flags(path.read_text().splitlines(), base, (*active, path)))
        else: expanded.append(flag)
    return expanded


def verify_rust_compiler(obj, build=None):
    flags = compilation_flags(obj)
    verify_strict_lints(response_flags(flags, build or obj.parent))
    while flags and re.match(r'\w+=', flags[0]): flags.pop(0)
    if not flags: raise ValueError('missing Rust compiler command')
    version = subprocess.run([flags[0], '--version'], capture_output=True, check=True, text=True).stdout
    match = re.match(r'rustc (\d+)\.(\d+)\.', version)
    if not match or tuple(map(int, match.groups())) < (1, 85):
        raise ValueError('memweight requires Rust 1.85 or newer')


def verify_rust_workload(obj, module=None):
    imports = {line.split()[-1] for line in tool('nm', '-u', obj).splitlines() if line.split()}
    expected = {b'memweight', b'memweight_reference', b'__kvmalloc_node_noprof', b'kvfree', b'_printk'}
    if not expected <= imports or imports - expected - {b'memset', b'memcpy'}:
        raise ValueError('Rust memweight workload missing imports or delegates to an unexpected function: ' + repr(imports))
    for path in (obj, *([module] if module is not None else [])):
        definitions = {row.split()[-1] for row in tool('nm', '-g', '--defined-only', path).splitlines() if row.split()}
        if b'memweight_rust_exercise' not in definitions or b'memweight_exercise' in definitions:
            raise ValueError('Rust memweight caller must own its complete workload')


C_SOURCE = r'''
#include <linux/bitmap.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
#include <linux/slab.h>
size_t memweight_reference(const void *, size_t);
static noinline size_t memweight_call(const void *ptr, size_t bytes)
{
    size_t (*volatile selected)(const void *, size_t) = memweight;
    return selected(ptr, bytes);
}
static u64 next(u64 *state)
{ *state ^= *state << 13; *state ^= *state >> 7; *state ^= *state << 17; return *state; }
static unsigned char value(unsigned int kind, size_t i, u64 *state)
{
    switch (kind) {
    case 0: return 0;
    case 1: return 255;
    case 2: return 0xaa;
    case 3: return 1U << (i % 8);
    default: return next(state);
    }
}
static int one(const void *ptr, size_t bytes)
{
    size_t expected = memweight_reference(ptr, bytes);
    if (memweight_call(ptr, bytes) != expected) return -EINVAL;
    return 0;
}
static int exercise(unsigned char *buffer, unsigned int *count)
{
    static const size_t lengths[] = {257, 511, 512, 513, 4095, 4096, 4097, 65535};
    size_t i, offset, length;
    unsigned int kind;
    u64 state;
    int error;
    *count = 0;
    error = one(NULL, 0); if (error) return error; ++*count;
    for (kind = 0; kind < 5; ++kind) {
        state = 0x8e937abc1234d5efULL;
        for (i = 0; i < 65544; ++i) buffer[i] = value(kind, i, &state);
        for (offset = 0; offset < sizeof(long); ++offset) {
            for (length = 0; length <= 256; ++length) {
                error = one(buffer + offset, length); if (error) return error; ++*count;
            }
            for (i = 0; i < ARRAY_SIZE(lengths); ++i) {
                error = one(buffer + offset, lengths[i]); if (error) return error; ++*count;
            }
        }
        state = 0x8e937abc1234d5efULL;
        for (i = 0; i < 65544; ++i)
            if (buffer[i] != value(kind, i, &state)) return -EINVAL;
    }
    state = 0x8e937abc1234d5efULL;
    for (i = 0; i < 2048; ++i) {
        offset = next(&state) % sizeof(long); length = next(&state) % 65536;
        error = one(buffer + offset, length); if (error) return error; ++*count;
    }
    state = 0x8e937abc1234d5efULL;
    for (i = 0; i < 65544; ++i)
        if (buffer[i] != value(4, i, &state)) return -EINVAL;
    /* 128 MiB: result bit 30 is set, strictly below the BUG threshold. */
    for (i = 0; i < 134217736; ++i) buffer[i] = 255;
    error = one(buffer, 134217728); if (error) return error; ++*count;
    error = one(buffer + 1, 134217735); if (error) return error; ++*count;
    for (i = 0; i < 134217736; ++i) if (buffer[i] != 255) return -EINVAL;
    return 0;
}
static int __init memweight_abi_init(void)
{
    unsigned char *buffer = __kvmalloc_node_noprof(134217736, sizeof(long), GFP_KERNEL, NUMA_NO_NODE);
    unsigned int count = 0;
    int error;
    if (!buffer) return -ENOMEM;
    error = exercise(buffer, &count);
    kvfree(buffer);
    if (error || count != @COUNT@) return error ? error : -EINVAL;
    pr_info("LUPOS_MEMWEIGHT_ABI_OK values=%u exports=1 checks=%u\n", count, count);
    return 0;
}
static void __exit memweight_abi_exit(void) {}
module_init(memweight_abi_init);
module_exit(memweight_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Non-GPL memweight public ABI check");
'''


RUST_SOURCE = r'''// SPDX-License-Identifier: GPL-2.0-only
//! Independent allocated-buffer memweight workload; original C only as oracle.
use kernel::{bindings, ffi};
unsafe extern "C" { fn memweight_reference(ptr: *const ffi::c_void, bytes: usize) -> usize; }
/// Invoke the native public binding through a protected indirect call.
/// # Safety
/// The region must be readable and below the original BUG threshold.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn memweight_call(ptr: *const ffi::c_void, bytes: usize) -> usize {
    let pointer: unsafe extern "C" fn(*const ffi::c_void, usize) -> usize = bindings::memweight;
    // SAFETY: The caller supplies the original memweight region contract.
    unsafe { core::ptr::read_volatile(&pointer)(ptr, bytes) }
}
fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13; *state ^= *state >> 7; *state ^= *state << 17; *state
}
fn value(kind: u32, i: usize, state: &mut u64) -> u8 {
    match kind { 0 => 0, 1 => 255, 2 => 0xaa, 3 => 1u8 << (i % 8), _ => next(state) as u8 }
}
unsafe fn one(ptr: *const u8, bytes: usize) -> Result<(), ffi::c_int> {
    // SAFETY: The workload supplies valid regions, including null only at zero.
    let expected = unsafe { memweight_reference(ptr.cast(), bytes) };
    // SAFETY: Same region, unchanged original C oracle.
    if unsafe { memweight_call(ptr.cast(), bytes) } != expected { return Err(-22); }
    Ok(())
}
/// Execute the complete Rust workload against the unchanged private C oracle.
/// # Safety
/// Buffer is an exclusive kernel allocation of at least 134217736 bytes.
#[no_mangle]
#[inline(never)]
pub unsafe fn memweight_rust_exercise(buffer: *mut u8) -> Result<u32, ffi::c_int> {
    let mut count = 0u32;
    // SAFETY: All offsets/lengths below stay in the actual allocation. NULL is
    // tested with length zero only. No input reaches the original BUG threshold.
    unsafe {
        one(core::ptr::null(), 0)?; count = count.wrapping_add(1);
        for kind in 0..5u32 {
            let mut state = 0x8e937abc1234d5efu64;
            for i in 0..65544usize { *buffer.add(i) = value(kind, i, &mut state); }
            for offset in 0..core::mem::size_of::<ffi::c_long>() {
                for length in 0..=256usize {
                    one(buffer.add(offset), length)?; count = count.wrapping_add(1);
                }
                for index in 0..8 {
                    let length = match index { 0 => 257, 1 => 511, 2 => 512, 3 => 513,
                        4 => 4095, 5 => 4096, 6 => 4097, _ => 65535 };
                    one(buffer.add(offset), length)?; count = count.wrapping_add(1);
                }
            }
            state = 0x8e937abc1234d5efu64;
            for i in 0..65544usize {
                if *buffer.add(i) != value(kind, i, &mut state) { return Err(-22); }
            }
        }
        let mut state = 0x8e937abc1234d5efu64;
        for _ in 0..2048 {
            let offset = next(&mut state) as usize % core::mem::size_of::<ffi::c_long>();
            let length = next(&mut state) as usize % 65536;
            one(buffer.add(offset), length)?; count = count.wrapping_add(1);
        }
        state = 0x8e937abc1234d5efu64;
        for i in 0..65544usize {
            if *buffer.add(i) != value(4, i, &mut state) { return Err(-22); }
        }
        for i in 0..134217736usize { *buffer.add(i) = 255; }
        one(buffer, 134217728)?; count = count.wrapping_add(1);
        one(buffer.add(1), 134217735)?; count = count.wrapping_add(1);
        for i in 0..134217736usize { if *buffer.add(i) != 255 { return Err(-22); } }
    }
    Ok(count)
}
/// Allocate real kernel memory, execute this caller's workload, then free once.
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    // SAFETY: Genuine kernel allocator, native alignment and generated constants.
    let buffer = unsafe { bindings::__kvmalloc_node_noprof(134217736,
        core::mem::size_of::<ffi::c_long>() as ffi::c_ulong, bindings::GFP_KERNEL,
        bindings::NUMA_NO_NODE) }.cast::<u8>();
    if buffer.is_null() { return -12; }
    // SAFETY: Allocation succeeds and covers the full workload region.
    let result = unsafe { memweight_rust_exercise(buffer) };
    // SAFETY: Exactly one free of the original allocation, including failure.
    unsafe { bindings::kvfree(buffer.cast()) };
    let count = match result { Ok(count) => count, Err(error) => return error };
    if count != @COUNT@ { return -22; }
    // SAFETY: Static format and matching unsigned-int arguments.
    unsafe { bindings::_printk(c"\x016LUPOS_MEMWEIGHT_RUST_ABI_OK values=%u exports=1 checks=%u\n".as_ptr().cast(), count, count); }
    0
}
/// All input memory is released before init returns, also on error.
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn() -> ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=Proprietary\0description=Non-GPL memweight public ABI check\0";
#[used]
#[link_section = ".modinfo"]
static MODINFO: [u8; INFO.len()] = {
    let mut bytes = [0; INFO.len()];
    let mut i = 0;
    while i < bytes.len() { bytes[i] = INFO.as_bytes()[i]; i += 1; }
    bytes
};
#[used]
static __IS_RUST_MODULE: () = ();
'''


def sources(caller):
    if caller not in ('c', 'rust'): raise ValueError('unknown memweight caller')
    return (C_SOURCE if caller == 'c' else RUST_SOURCE).replace('@COUNT@', str(COUNT))


def reference_source():
    return ('#include <linux/bitmap.h>\n#include <linux/export.h>\n'
            'size_t memweight_reference(const void *, size_t);\n'
            '#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(symbol)\n'
            '#define memweight memweight_reference\n'
            '#include "' + str(ROOT / 'lib/memweight.c') + '"\n')



def selected_versions(build, names=EXPORTS):
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()
            if len(line.split()) > 1 and line.split()[1]== b'memweight']
    if len(rows) != len(names) or {row[1] for row in rows} != {name.encode() for name in names}:
        raise ValueError('memweight exported names missing, duplicated or incorrect for backend')
    result = {}
    for row in rows:
        if (len(row) != 4 or row[2:] != [b'vmlinux', b'EXPORT_SYMBOL'] or
                not re.fullmatch(rb'0x[0-9a-fA-F]{8}', row[0])):
            raise ValueError('wrong memweight export owner/license/namespace/CRC')
        result[row[1]] = row[0].lower()
    return result


def dwarf_proof(build, owner, name):
    # Do not let TMPDIR silently place compiler-tool scratch inside source or
    # donor output trees. A private test parent remains usable when supplied.
    parents = [Path(os.environ[key]) for key in ('TMPDIR', 'TEMP', 'TMP') if os.environ.get(key)] + [Path('/tmp')]
    parent = next((path.resolve() for path in parents if path.is_dir() and not any(
        path.resolve().is_relative_to(protected.resolve()) for protected in (ROOT, build))), None)
    if parent is None: raise ValueError('no external directory for DWARF proof')
    with tempfile.TemporaryDirectory(prefix='memweight-dwarf-', dir=parent) as temporary:
        symtypes = Path(temporary) / 'types'
        result = subprocess.run([str(build / 'scripts/gendwarfksyms/gendwarfksyms'),
            '--symtypes', str(symtypes), str(owner)], input=(name + '\n').encode(),
            check=True, capture_output=True, timeout=180)
        rows = [line.split() for line in result.stdout.splitlines()]
        if result.stderr or len(rows) != 1 or len(rows[0]) != 3 or rows[0][:2] != [b'#SYMVER', name.encode()]:
            raise ValueError('malformed memweight defining-object version proof')
        if not re.fullmatch(rb'0x[0-9a-fA-F]{8}', rows[0][2]):
            raise ValueError('malformed memweight defining-object CRC')
        text = symtypes.read_text()
        aliases = dict(re.findall(r'^(t#\w+) typedef_type \w+ \{ (.*?) \}$', text, re.M))
        for _ in range(len(aliases) + 1):
            for alias, body in aliases.items(): text = text.replace(alias, body)
        types = [line for line in text.splitlines() if line.startswith(name + ' ')]
        if len(types) != 1: raise ValueError('missing memweight defining type')
        row = types[0]
        c_void = 'pointer_type { const_type { base_type void } }' in row
        rust_void = ('pointer_type *const core::ffi::c_void { e#core::ffi::c_void }' in row and
            'e#core::ffi::c_void enumeration_type core::ffi::c_void { enumerator __variant1 = 0 , enumerator __variant2 = 1 } byte_size(1) alignment(1)' in text)
        if (re.findall(r'byte_size\((\d+)\) encoding\((\d+)\)', row) != [('8', '7'), ('8', '7')]
                or row.count('pointer_type') != 1 or row.count('formal_parameter') != 2
                or not row.startswith(name + ' subprogram ( formal_parameter pointer_type ')
                or not (c_void or rust_void)):
            raise ValueError('memweight needs native size_t return/length and const void pointer')
        return rows[0][2].lower(), row


def verify_dwarf(build, owner, selection, versions):
    config = configuration(build)
    if config.get('MODVERSIONS') != 'y' or config.get('GENDWARFKSYMS') != 'y':
        raise ValueError('memweight defining-type proof requires MODVERSIONS and GENDWARFKSYMS')
    crc, _ = dwarf_proof(build, owner, 'memweight')
    if versions != {b'memweight': crc}: raise ValueError('memweight defining DWARF CRC differs')
    newer(build / 'vmlinux.o', [owner])


def verify_linked_implementation(build, selection):
    config = configuration(build)
    if selection != provider(config): raise ValueError('memweight provider selection differs from configuration')
    arch, owner = architecture(config), owner_path(build, selection)
    archive = build / 'vmlinux.a'
    ordered = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', archive).splitlines()]
    candidates = {owner_path(build, language).resolve() for language in ('C', 'Rust')}
    if [member for member in ordered if member in candidates] != [owner.resolve()]:
        raise ValueError('memweight archive owner absent, duplicated or mixed')
    library = build / 'lib/built-in.a'
    members = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', library).splitlines()]
    if [member for member in members if member in candidates] != [owner.resolve()]:
        raise ValueError('memweight lib/built-in.a owner absent, duplicated or mixed')
    newer(library, [owner])
    newer(archive, [library])
    bitmap = build / 'lib/bitmap.o'
    if ordered.count(bitmap.resolve()) != 1 or members.count(bitmap.resolve()) != 1:
        raise ValueError('genuine bitmap dependency missing or duplicated')
    verify_build_command(build, bitmap, ROOT / 'lib/bitmap.c', [ROOT / 'include/linux/bitmap.h'])
    elf_target(bitmap, arch)
    definitions = [row.split() for row in tool('nm', '-g', '--defined-only', bitmap).splitlines()]
    if len([row for row in definitions if row[-2:] == [b'T', b'__bitmap_weight']]) != 1:
        raise ValueError('genuine bitmap_weight definition missing')
    newer(library, [bitmap])
    source = ROOT / {'C': 'lib/memweight.c', 'Rust': 'lib/memweight_rust.rs'}[selection]
    dependencies = [ROOT / name for name in ({'C': ['include/linux/bitmap.h', 'include/linux/bitops.h', 'include/linux/export.h', 'include/linux/bug.h'],
        'Rust': ['lib/memweight.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs'],
        }[selection])]
    if selection == 'Rust':
        dependencies += [build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta']
        verify_bindings(build)
        verify_rust_compiler(owner, build)
    verify_build_command(build, owner, source, dependencies)
    verify_references(owner, ('__bitmap_weight',))
    elf_target(owner, arch)
    names = EXPORTS
    records = read_exports(owner)
    if len(records) != len(names) or {row['name'] for row in records} != set(names):
        raise ValueError('wrong selected memweight export set')
    for row in records:
        expected = dict(license='', namespace='', relocation_target=row['name'], relocation_addend=0,
                        pointer_width=8, relocation_kind=257 if arch == 'aarch64' else 1,
                        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        if any(row[key] != value for key, value in expected.items()):
            raise ValueError('incorrect unrestricted memweight export metadata/relocation')
    definitions = [line.split() for line in tool('nm', '-g', '--defined-only', owner).splitlines()
                   if line.split()[-1:] and line.split()[-1]== b'memweight']
    if len(definitions) != len(names) or {row[-1] for row in definitions} != {name.encode() for name in names} or any(row[-2] != b'T' for row in definitions):
        raise ValueError('selected memweight symbols are not exactly the defining global functions')
    versions = selected_versions(build, names)
    if config.get('MODVERSIONS') == 'y' and version_records(owner.with_name('.' + owner.name + '.cmd')) != versions:
        raise ValueError('memweight CRC provenance does not match selected defining unit')
    verify_dwarf(build, owner, selection, versions)
    if config.get('CFI') == 'y':
        flag = '-Zsanitizer=kcfi' if selection == 'Rust' else '-fsanitize=kcfi'
        if flag not in compilation_flags(owner): raise ValueError('memweight owner missing KCFI')
        provider_type_ids(owner, names=names)
    newer(archive, [owner])
    newer(build / 'Module.symvers', [owner])
    newer(build / 'vmlinux.o', [archive])
    newer(build / 'vmlinux', [build / 'vmlinux.o'])
    if config.get('UML') != 'y':
        newer(build / ('arch/arm64/boot/Image' if arch == 'aarch64' else 'arch/x86/boot/bzImage'), [build / 'vmlinux'])
    return owner


def verify_consumer(build, work, caller):
    if caller not in ('c', 'rust'): raise ValueError('unknown memweight caller')
    name, stem = ('memweight_rust_abi', 'memweight_rust_main') if caller == 'rust' else ('memweight_abi', 'memweight_c_main')
    obj, module = work / (stem + '.o'), work / (name + '.ko')
    config = configuration(build)
    arch, selection = architecture(config), provider(config)
    deps = ([build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta'] if caller == 'rust' else [ROOT / 'include/linux/bitmap.h'])
    source = obj.with_suffix('.rs' if caller == 'rust' else '.c')
    if source.read_text() != sources(caller): raise ValueError('memweight caller source changed')
    if caller == 'rust':
        verify_bindings(build)
        verify_rust_compiler(obj, build)
    verify_build_command(work, obj, source, deps)
    oracle = work / 'memweight_reference.o'
    if oracle.with_suffix('.c').read_text() != reference_source(): raise ValueError('memweight original C oracle changed')
    verify_build_command(work, oracle, oracle.with_suffix('.c'), [ROOT / 'lib/memweight.c', ROOT / 'include/linux/bitmap.h'])
    constituents = [(work / os.fsdecode(line)).resolve() for line in module.with_suffix('.mod').read_bytes().splitlines()]
    if constituents != [obj.resolve(), oracle.resolve()]:
        raise ValueError('memweight caller contains unexpected workload constituents')
    for path in (obj, module):
        elf_target(path, arch)
        verify_references(path, EXPORTS)
    if module_name(module) != name: raise ValueError('wrong memweight module identity')
    newer(module, [obj, oracle, selected_metadata(build, module)])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    imports = verify_module_import_versions(build, module)
    for symbol, crc in selected_versions(build).items():
        if imports.get(symbol) != int(crc, 16): raise ValueError('memweight import CRC differs')
    require_metadata_field(metadata_fields(module), b'license', b'Proprietary')
    require_metadata_field(metadata_fields(module), b'description', DESCRIPTION)
    if read_exports(module): raise ValueError('private memweight caller unexpectedly exports symbols')
    undefined = {line.split()[-1] for line in tool('nm', '-u', module).splitlines() if line.split()}
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()]
    if any(len(row) >= 4 and row[1] in undefined and row[3] != b'EXPORT_SYMBOL' for row in rows):
        raise ValueError('Proprietary memweight caller imports a GPL-only symbol')
    if caller == 'rust':
        verify_rust_workload(obj, module)
        for path in (obj, module): verify_rust_entrypoints(path, arch)
    for path in (oracle, *([obj] if caller == 'c' else [])):
        flags = compilation_flags(path)
        if '-D__DISABLE_EXPORTS' not in flags or '-U__DISABLE_EXPORTS' in flags or read_exports(path):
            raise ValueError('private memweight C fixture retains incidental export metadata')
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    if config.get('CFI') == 'y':
        flags = compilation_flags(obj)
        required = '-Zsanitizer=kcfi' if caller == 'rust' else '-fsanitize=kcfi'
        if required not in flags or any(flag.startswith('-fno-sanitize=') and {'all', 'kcfi'} & set(flag.split('=', 1)[1].split(',')) for flag in flags):
            raise ValueError('memweight caller missing native KCFI compilation')
        types = provider_type_ids(owner_path(build, selection), names=EXPORTS)
        reference_types = provider_type_ids(oracle, names=('memweight_reference',))
        if types['memweight'] != reference_types['memweight_reference']:
            raise ValueError('memweight native KCFI identity differs from original C')
        dwarf_proof(build, oracle, 'memweight_reference')
        for path in (obj, module): verify_guarded_calls(path, arch, types, wrappers=WRAPPERS)


def result_marker(caller):
    if caller not in ('c', 'rust'): raise ValueError('unknown memweight caller')
    return (b'LUPOS_MEMWEIGHT_' + (b'RUST_ABI' if caller == 'rust' else b'ABI') +
            f'_OK values={COUNT} exports=1 checks={COUNT}'.encode())


def verify_console(console, caller, *, reload=False, config=None):
    if re.search(rb'CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version', console, re.I):
        raise ValueError('unexpected memweight kernel fault or failed test')
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
    actual = [re.sub(rb'^memweight_(?:rust_)?abi: ', b'', line) for line in lines if b'LUPOS_' in line]
    if actual != expected: raise ValueError('memweight workload counts, caller or load/reload events differ')
    verify_module_events(console, module=True, preloads=0, reload=reload)
    return 2 if reload else 1


def executable_command(value, name):
    try:
        command = shlex.split(value)
    except ValueError as error:
        raise ValueError('malformed explicit ' + name + ' command') from error
    if not command or not command[0]: raise ValueError('empty explicit ' + name + ' input')
    if any('\0' in word for word in command): raise ValueError(name + ' command contains NUL')
    executable = shutil.which(command[0])
    if executable is None: raise ValueError('invalid ' + name + ' executable: ' + command[0])
    command[0] = os.path.abspath(executable)
    return command


def validate_options(args):
    if 'modular' in str(args.build.resolve()).lower(): raise ValueError('MODULAR outputs forbidden')
    if not (args.build / '.config').is_file(): raise ValueError('missing build configuration')
    if args.qemu is not None:
        args.qemu = shlex.join(executable_command(args.qemu, 'QEMU'))
    if args.qemu_data is not None:
        if not args.qemu_data.is_dir(): raise ValueError('invalid QEMU data directory')
        args.qemu_data = args.qemu_data.resolve()
    args.make_command = executable_command(os.environ.get('MAKE', 'make'), 'MAKE')
    if any(not value.strip() for value in args.make_arg): raise ValueError('empty make argument')


def run_logged(command, log, env):
    command = list(map(str, command))
    scratch = Path(tempfile.mkdtemp(prefix='command-', dir=log.parent))
    env = dict(env, TMPDIR=str(scratch), TEMP=str(scratch), TMP=str(scratch))
    log.write_text(shlex.join(command) + '\ncwd=' + str(scratch) + '\n')
    try:
        result = subprocess.run(command, cwd=scratch, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    except OSError as error:
        with log.open('ab') as output: output.write(('launch-error=' + repr(error) + '\n').encode())
        raise RuntimeError('command failed to start; preserved log: ' + str(log)) from error
    with log.open('ab') as output:
        output.write(result.stdout + f'\nexit={result.returncode}\n'.encode())
    if result.returncode: raise RuntimeError('command failed; preserved log: ' + str(log))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build', type=nonempty_path)
    parser.add_argument('--caller', choices=('c', 'rust'), default='c')
    parser.add_argument('--allow-c-baseline', action='store_true')
    parser.add_argument('--audit-only', action='store_true')
    parser.add_argument('--reload-modules', action='store_true')
    parser.add_argument('--make-arg', action='append', default=[])
    parser.add_argument('--qemu', default=os.environ.get('QEMU'))
    parser.add_argument('--qemu-data', type=nonempty_path)
    args = parser.parse_args()
    build = args.build.resolve()
    try:
        validate_options(args)
        config = configuration(build)
        selection = provider(config)
        if selection != 'Rust' and not args.allow_c_baseline: raise ValueError('requires RUST_MEMWEIGHT=y or explicit --allow-c-baseline')
        if not args.audit_only:
            if args.qemu is None:
                emulator = 'qemu-system-aarch64' if architecture(config) == 'aarch64' else 'qemu-system-x86_64'
                args.qemu = shlex.join(executable_command(emulator, 'QEMU'))
            if config.get('UML') == 'y': raise ValueError('generic UML recognized, but the system-QEMU runner cannot boot UML')
            for name in ('MODULES', 'PRINTK', 'MULTIUSER', 'CFI', 'MODVERSIONS', 'GENDWARFKSYMS'):
                if config.get(name) != 'y': raise ValueError('requires CONFIG_' + name + '=y')
            if args.caller == 'rust' and config.get('RUST') != 'y': raise ValueError('Rust caller requires CONFIG_RUST=y')
            if args.reload_modules and config.get('MODULE_UNLOAD') != 'y': raise ValueError('reload requires MODULE_UNLOAD=y')
            if config.get('MODULE_SIG_FORCE') == 'y': raise ValueError('disposable unsigned modules cannot satisfy forced signing')
        owner = verify_linked_implementation(build, selection)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    if args.audit_only:
        print(f'{selection} memweight: selected owner {owner}; artifact-only audit, no runtime claim.')
        return
    arch = architecture(config)
    parent = build / 'rust-memweight-test'
    parent.mkdir(exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix=args.caller + '-', dir=parent))
    print('memweight module evidence:', work, flush=True)
    name, stem = ('memweight_rust_abi', 'memweight_rust_main') if args.caller == 'rust' else ('memweight_abi', 'memweight_c_main')
    (work / 'memweight_reference.c').write_text(reference_source())
    (work / (stem + ('.rs' if args.caller == 'rust' else '.c'))).write_text(sources(args.caller))
    (work / 'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o memweight_reference.o\n')
    env = {key: value for key, value in os.environ.items() if not key.startswith(('CONFIG_', 'KCONFIG_', 'KBUILD_')) and key not in
           ('MAKEFLAGS', 'MFLAGS', 'CARGO_MAKEFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES', 'srctree', 'srcroot', 'objtree', 'VPATH', 'sub_make_done')}
    run_logged([*args.make_command, '-C', str(ROOT), 'O=' + str(build), 'M=' + str(work),
                    *args.make_arg, PRIVATE_C_FLAGS, 'modules'], work / 'build.log', env)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / 'scripts/tests/boot_kernel.py'), '--build', str(build), '--arch', arch,
               '--module', str(work / (name + '.ko')), '--qemu', args.qemu or ('qemu-system-aarch64' if arch == 'aarch64' else 'qemu-system-x86_64')]
    if args.reload_modules: command += ['--reload-modules']
    if args.qemu_data: command += ['--qemu-data', str(args.qemu_data)]
    run_logged(command, work / 'boot.log', env)
    runs = verify_console((build / 'rust-boot-test/console.log').read_bytes(), args.caller, reload=args.reload_modules, config=config)
    print(f'{selection} memweight: Proprietary {args.caller} caller passed {COUNT} protected comparisons/load; {runs} loads on {arch}. BUG threshold crashing VM gate remains separate.')


if __name__ == '__main__': main()
