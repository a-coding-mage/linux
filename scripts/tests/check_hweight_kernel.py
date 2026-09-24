#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit selected hweight ownership and exercise generic exports in a VM.

C and Rust Proprietary callers each execute their own complete workload and
invoke all four real exports through protected indirect calls. The only shared
algorithm oracle is unchanged lib/hweight.c under private names. Native x86
assembly is audited separately: it is not the generic Rust provider, exports
only 32/64 variants, and has a distinct register contract. --audit-only supports
that retained backend without pretending to have run the generic four-export
workload. UML generic ownership is recognized, but the current system-QEMU
runner cannot boot UML. No modules are loaded on the host.
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
from check_int_log_kernel import normalize_console_transport, kunit_runs, verify_kunit_warnings
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer
from check_prime_numbers_kernel import provider_type_ids, verify_guarded_calls, verify_module_import_versions, verify_rust_entrypoints
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports

ROOT = Path(__file__).resolve().parents[2]
WIDTHS = (8, 16, 32, 64)
EXPORTS = tuple('__sw_hweight' + str(n) for n in WIDTHS)
WRAPPERS = tuple(('hweight_call_' + str(n), '__sw_hweight' + str(n)) for n in WIDTHS)
COUNT = 65536 * 6 + 64 * 3 + 200000
DESCRIPTION = b'Non-GPL generic hweight public ABI check'
PRIVATE_C_FLAGS = 'CFLAGS_MODULE+=-D__DISABLE_EXPORTS'


def reference_source():
    return ('#include <linux/bitops.h>\n#include <linux/export.h>\n' +
            ''.join(f'{"unsigned long" if n == 64 else "unsigned int"} hweight_reference_{n}'
                    f'({"__u64" if n == 64 else "unsigned int"});\n' for n in WIDTHS) +
            '#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(symbol)\n' +
            ''.join(f'#define __sw_hweight{n} hweight_reference_{n}\n' for n in WIDTHS) +
            '#include "' + str(ROOT / 'lib/hweight.c') + '"\n')


C_SOURCE = r'''
#include <linux/bitops.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
@C_WRAPPERS@
static int check(u64 value)
{
    u32 word = value;
    @C_CHECKS@
    return 0;
}
static int __init hweight_abi_init(void)
{
    u64 state = 0x8e937abc1234d5efULL;
    unsigned int i, j, count = 0;
    int error;
    for (i = 0; i < 65536; ++i) {
        u64 values[] = {i, (u64)i << 16, ((u64)i << 32) | i,
            0xffff0000U | i, ~((u64)i), ((u64)i << 48) | 0x80000100U};
        for (j = 0; j < ARRAY_SIZE(values); ++j) {
            error = check(values[j]); if (error) return error; ++count;
        }
    }
    for (i = 0; i < 64; ++i) {
        u64 value = 1ULL << i;
        error = check(value); if (error) return error; ++count;
        error = check(~value); if (error) return error; ++count;
        error = check(value - 1); if (error) return error; ++count;
    }
    for (i = 0; i < 200000; ++i) {
        state ^= state << 13; state ^= state >> 7; state ^= state << 17;
        error = check(state); if (error) return error; ++count;
    }
    if (count != @COUNT@) return -EINVAL;
    pr_info("LUPOS_HWEIGHT_ABI_OK values=%u exports=4 checks=%u\n", count, count * 4);
    return 0;
}
static void __exit hweight_abi_exit(void) {}
module_init(hweight_abi_init);
module_exit(hweight_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Non-GPL generic hweight public ABI check");
'''

RUST_SOURCE = r'''// SPDX-License-Identifier: GPL-2.0-only
//! Independent native Rust hweight workload; no C workload dispatcher.
use kernel::{bindings, ffi};
@RUST_WRAPPERS@
fn check(value: u64) -> Result<(), ffi::c_int> {
    let word = value as ffi::c_uint;
    @RUST_CHECKS@
    Ok(())
}
fn exercise() -> Result<u32, ffi::c_int> {
    let mut state = 0x8e937abc1234d5efu64;
    let mut count = 0u32;
    for i in 0..65536u32 {
        check(i as u64)?;
        check((i as u64) << 16)?;
        check(((i as u64) << 32) | i as u64)?;
        check((0xffff0000u32 | i) as u64)?;
        check(!(i as u64))?;
        check(((i as u64) << 48) | 0x80000100)?;
        count = count.wrapping_add(6);
    }
    for i in 0..64u32 {
        let value = 1u64 << i;
        check(value)?; check(!value)?; check(value.wrapping_sub(1))?;
        count = count.wrapping_add(3);
    }
    for _ in 0..200000 {
        state ^= state << 13; state ^= state >> 7; state ^= state << 17;
        check(state)?; count = count.wrapping_add(1);
    }
    Ok(count)
}
/// Execute all comparisons against the original private C scalar functions.
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    let count = match exercise() { Ok(count) => count, Err(error) => return error };
    if count != @COUNT@ { return -22; }
    // SAFETY: Static format and matching unsigned-int arguments.
    unsafe { bindings::_printk(c"\x016LUPOS_HWEIGHT_RUST_ABI_OK values=%u exports=4 checks=%u\n".as_ptr().cast(), count, count.wrapping_mul(4)); }
    0
}
/// No allocations or retained resources survive the stateless workload.
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn() -> ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=Proprietary\0description=Non-GPL generic hweight public ABI check\0";
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
    if caller not in ('c', 'rust'): raise ValueError('unknown hweight caller')
    wrappers, checks = [], []
    for n in WIDTHS:
        argument = 'value' if n == 64 else 'word'
        if caller == 'c':
            typ, result = ('__u64', 'unsigned long') if n == 64 else ('unsigned int', 'unsigned int')
            wrappers.append(f'{result} hweight_reference_{n}({typ});\n'
                f'static noinline {result} hweight_call_{n}({typ} value) {{\n'
                f'    {result} (*volatile selected)({typ}) = __sw_hweight{n};\n'
                '    return selected(value);\n}')
            checks.append(f'if (hweight_call_{n}({argument}) != hweight_reference_{n}({argument})) {{\n'
                f'        pr_err("LUPOS_HWEIGHT_FAIL function={n} input=%016llx\\n", value); return -EINVAL;\n    }}')
        else:
            typ, result = ('u64', 'ffi::c_ulong') if n == 64 else ('ffi::c_uint', 'ffi::c_uint')
            wrappers.append(f'unsafe extern "C" {{ fn hweight_reference_{n}(value: {typ}) -> {result}; }}\n'
                f'/// Invoke the genuine {n}-bit public export through its native binding.\n'
                '#[no_mangle]\n#[inline(never)]\n'
                f'pub extern "C" fn hweight_call_{n}(value: {typ}) -> {result} {{\n'
                f'    let pointer: unsafe extern "C" fn({typ}) -> {result} = bindings::__sw_hweight{n};\n'
                '    // SAFETY: Every integer is in this scalar function\'s domain.\n'
                '    unsafe { core::ptr::read_volatile(&pointer)(value) }\n}')
            checks.append('    // SAFETY: Original C reference accepts every scalar input.\n'
                f'    if hweight_call_{n}({argument}) != unsafe {{ hweight_reference_{n}({argument}) }} {{\n'
                '        // SAFETY: Static format and matching unsigned-long-long argument.\n'
                f'        unsafe {{ bindings::_printk(c"\\x013LUPOS_HWEIGHT_FAIL function={n} input=%016llx\\n".as_ptr().cast(), value as ffi::c_ulonglong); }}\n'
                '        return Err(-22);\n    }')
    return (C_SOURCE if caller == 'c' else RUST_SOURCE).replace('@COUNT@', str(COUNT)).replace(
        '@' + caller.upper() + '_WRAPPERS@', '\n'.join(wrappers)).replace(
        '@' + caller.upper() + '_CHECKS@', '\n    '.join(checks))


def provider(config):
    arch = architecture(config)
    for name in ('RUST_HWEIGHT', 'GENERIC_HWEIGHT'):
        if config.get(name, 'n') not in ('n', 'y'): raise ValueError('invalid hweight selector')
    rust, generic = config.get('RUST_HWEIGHT') == 'y', config.get('GENERIC_HWEIGHT') == 'y'
    if rust and (config.get('RUST') != 'y' or not generic):
        raise ValueError('RUST_HWEIGHT requires RUST and GENERIC_HWEIGHT')
    if arch == 'x86_64' and config.get('UML') != 'y':
        if generic or rust: raise ValueError('native x86 must retain its special-ABI assembly owner')
        return 'x86-assembly'
    if not generic: raise ValueError('generic hweight provider is not enabled')
    return 'Rust' if rust else 'C'


def owner_path(build, selection):
    if selection not in ('C', 'Rust', 'x86-assembly'): raise ValueError('unknown hweight provider')
    return build / {'C': 'lib/hweight.o', 'Rust': 'lib/hweight_rust.o',
                    'x86-assembly': 'arch/x86/lib/hweight.o'}[selection]


def export_names(selection):
    return EXPORTS[2:] if selection == 'x86-assembly' else EXPORTS


def selected_versions(build, names):
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()
            if len(line.split()) > 1 and line.split()[1].startswith(b'__sw_hweight')]
    if len(rows) != len(names) or {row[1] for row in rows} != {name.encode() for name in names}:
        raise ValueError('hweight exported names missing, duplicated or incorrect for backend')
    result = {}
    for row in rows:
        if (len(row) != 4 or row[2:] != [b'vmlinux', b'EXPORT_SYMBOL'] or
                not re.fullmatch(rb'0x[0-9a-fA-F]{8}', row[0])):
            raise ValueError('wrong hweight export owner/license/namespace/CRC')
        result[row[1]] = row[0].lower()
    return result


def verify_dwarf(build, owner, selection, versions):
    config = configuration(build)
    if config.get('MODVERSIONS') != 'y': return
    if config.get('GENDWARFKSYMS') != 'y': raise ValueError('hweight defining-type proof requires GENDWARFKSYMS')
    # Assembly uses the actual Kbuild type companion, not a checker declaration.
    source = owner.with_suffix('.gendwarfksyms.o') if selection == 'x86-assembly' else owner
    with tempfile.TemporaryDirectory(prefix='hweight-dwarf-') as temporary:
        symtypes = Path(temporary) / 'types'
        result = subprocess.run([str(build / 'scripts/gendwarfksyms/gendwarfksyms'), '--symtypes', str(symtypes), str(source)],
            input=b''.join(name + b'\n' for name in versions), check=True, capture_output=True, timeout=180)
        rows = [line.split() for line in result.stdout.splitlines()]
        if result.stderr or len(rows) != len(versions) or any(len(row) != 3 or row[0] != b'#SYMVER' for row in rows):
            raise ValueError('malformed hweight defining-object version proof')
        if {row[1]: row[2] for row in rows} != versions:
            raise ValueError('hweight defining-object DWARF CRC differs')
        text = symtypes.read_text()
        for alias, body in re.findall(r'^(t#\w+) typedef_type \w+ \{ (.*?) \}$', text, re.M): text = text.replace(alias, body)
        types = {line.split()[0]: line for line in text.splitlines() if line.startswith('__sw_hweight')}
        if set(types) != {name.decode() for name in versions}: raise ValueError('hweight type rows missing')
        for name, row in types.items():
            wanted = [('8', '7')] * 2 if name.endswith('64') else [('4', '7')] * 2
            if re.findall(r'byte_size\((\d+)\) encoding\((\d+)\)', row) != wanted:
                raise ValueError('wrong original hweight unsigned parameter/result widths')
    newer(build / 'vmlinux.o', [source])


def verify_linked_implementation(build, selection):
    config = configuration(build)
    if selection != provider(config): raise ValueError('hweight provider selection differs from configuration')
    arch, owner = architecture(config), owner_path(build, selection)
    archive = build / 'vmlinux.a'
    ordered = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', archive).splitlines()]
    candidates = {owner_path(build, language).resolve() for language in ('C', 'Rust', 'x86-assembly')}
    if [member for member in ordered if member in candidates] != [owner.resolve()]:
        raise ValueError('hweight archive owner absent, duplicated or mixed')
    source = ROOT / {'C': 'lib/hweight.c', 'Rust': 'lib/hweight_rust.rs', 'x86-assembly': 'arch/x86/lib/hweight.S'}[selection]
    dependencies = [ROOT / name for name in ({'C': ['include/linux/bitops.h', 'include/linux/export.h'],
        'Rust': ['lib/hweight.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs'],
        'x86-assembly': ['include/linux/export.h', 'arch/x86/include/asm/asm.h']}[selection])]
    verify_build_command(build, owner, source, dependencies)
    elf_target(owner, arch)
    names = export_names(selection)
    records = read_exports(owner)
    if len(records) != len(names) or {row['name'] for row in records} != set(names):
        raise ValueError('wrong selected hweight export set')
    for row in records:
        expected = dict(license='', namespace='', relocation_target=row['name'], relocation_addend=0,
                        pointer_width=8, relocation_kind=257 if arch == 'aarch64' else 1,
                        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        if any(row[key] != value for key, value in expected.items()):
            raise ValueError('incorrect unrestricted hweight export metadata/relocation')
    definitions = [line.split() for line in tool('nm', '-g', '--defined-only', owner).splitlines()
                   if line.split()[-1:] and line.split()[-1].startswith(b'__sw_hweight')]
    if len(definitions) != len(names) or {row[-1] for row in definitions} != {name.encode() for name in names} or any(row[-2] != b'T' for row in definitions):
        raise ValueError('selected hweight symbols are not exactly the defining global functions')
    versions = selected_versions(build, names)
    if config.get('MODVERSIONS') == 'y' and version_records(owner.with_name('.' + owner.name + '.cmd')) != versions:
        raise ValueError('hweight CRC provenance does not match selected defining unit')
    verify_dwarf(build, owner, selection, versions)
    if selection != 'x86-assembly' and config.get('CFI') == 'y':
        flag = '-Zsanitizer=kcfi' if selection == 'Rust' else '-fsanitize=kcfi'
        if flag not in compilation_flags(owner): raise ValueError('hweight owner missing KCFI')
        provider_type_ids(owner, names=names)
    newer(archive, [owner])
    newer(build / 'Module.symvers', [owner])
    newer(build / 'vmlinux.o', [archive])
    newer(build / 'vmlinux', [build / 'vmlinux.o'])
    if config.get('UML') != 'y':
        newer(build / ('arch/arm64/boot/Image' if arch == 'aarch64' else 'arch/x86/boot/bzImage'), [build / 'vmlinux'])
    return owner


def verify_consumer(build, work, caller):
    name, stem = ('hweight_rust_abi', 'hweight_rust_main') if caller == 'rust' else ('hweight_abi', 'hweight_c_main')
    obj, module = work / (stem + '.o'), work / (name + '.ko')
    config = configuration(build)
    arch, selection = architecture(config), provider(config)
    if selection == 'x86-assembly': raise ValueError('four-export caller cannot prove native x86 assembly')
    deps = ([build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta'] if caller == 'rust' else [ROOT / 'include/linux/bitops.h'])
    source = obj.with_suffix('.rs' if caller == 'rust' else '.c')
    if source.read_text() != sources(caller): raise ValueError('hweight caller source changed')
    verify_build_command(work, obj, source, deps)
    oracle = work / 'hweight_reference.o'
    if oracle.with_suffix('.c').read_text() != reference_source(): raise ValueError('hweight original C oracle changed')
    verify_build_command(work, oracle, oracle.with_suffix('.c'), [ROOT / 'lib/hweight.c', ROOT / 'include/linux/bitops.h'])
    for path in (obj, module):
        elf_target(path, arch)
        verify_references(path, EXPORTS)
    if module_name(module) != name: raise ValueError('wrong hweight module identity')
    newer(module, [obj, oracle, selected_metadata(build, module)])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_module_import_versions(build, module)
    require_metadata_field(metadata_fields(module), b'license', b'Proprietary')
    require_metadata_field(metadata_fields(module), b'description', DESCRIPTION)
    if read_exports(module): raise ValueError('private hweight caller unexpectedly exports symbols')
    undefined = {line.split()[-1] for line in tool('nm', '-u', module).splitlines() if line.split()}
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()]
    if any(len(row) >= 4 and row[1] in undefined and row[3] != b'EXPORT_SYMBOL' for row in rows):
        raise ValueError('Proprietary hweight caller imports a GPL-only symbol')
    if caller == 'rust':
        imports = {line.split()[-1] for line in tool('nm', '-u', obj).splitlines() if line.split()}
        expected = {name.encode() for name in EXPORTS} | {f'hweight_reference_{n}'.encode() for n in WIDTHS} | {b'_printk'}
        if imports != expected: raise ValueError('Rust hweight caller is not independently executing the complete scalar workload')
        for path in (obj, module): verify_rust_entrypoints(path, arch)
    for path in (oracle, *([obj] if caller == 'c' else [])):
        flags = compilation_flags(path)
        if '-D__DISABLE_EXPORTS' not in flags or '-U__DISABLE_EXPORTS' in flags or read_exports(path):
            raise ValueError('private hweight C fixture retains incidental export metadata')
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    if config.get('CFI') == 'y':
        flags = compilation_flags(obj)
        required = '-Zsanitizer=kcfi' if caller == 'rust' else '-fsanitize=kcfi'
        if required not in flags or any(flag.startswith('-fno-sanitize=') and {'all', 'kcfi'} & set(flag.split('=', 1)[1].split(',')) for flag in flags):
            raise ValueError('hweight caller missing native KCFI compilation')
        types = provider_type_ids(owner_path(build, selection), names=EXPORTS)
        for path in (obj, module): verify_guarded_calls(path, arch, types, wrappers=WRAPPERS)


def result_marker(caller):
    if caller not in ('c', 'rust'): raise ValueError('unknown hweight caller')
    return (b'LUPOS_HWEIGHT_' + (b'RUST_ABI' if caller == 'rust' else b'ABI') +
            f'_OK values={COUNT} exports=4 checks={COUNT * 4}'.encode())


def verify_console(console, caller, *, reload=False, config=None):
    if re.search(rb'CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version', console, re.I):
        raise ValueError('unexpected hweight kernel fault or failed test')
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
    actual = [re.sub(rb'^hweight_(?:rust_)?abi: ', b'', line) for line in lines if b'LUPOS_' in line]
    if actual != expected: raise ValueError('hweight workload counts, caller or load/reload events differ')
    verify_module_events(console, module=True, preloads=0, reload=reload)
    return 2 if reload else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build', type=Path)
    parser.add_argument('--caller', choices=('c', 'rust'), default='c')
    parser.add_argument('--allow-c-baseline', action='store_true')
    parser.add_argument('--audit-only', action='store_true')
    parser.add_argument('--reload-modules', action='store_true')
    parser.add_argument('--make-arg', action='append', default=[])
    parser.add_argument('--qemu', default=os.environ.get('QEMU'))
    parser.add_argument('--qemu-data', type=Path)
    args = parser.parse_args()
    build = args.build.resolve()
    try:
        config = configuration(build)
        selection = provider(config)
        if selection != 'Rust' and not args.allow_c_baseline: raise ValueError('requires RUST_HWEIGHT=y or explicit --allow-c-baseline')
        if not args.audit_only:
            if selection == 'x86-assembly': raise ValueError('native x86 retains special assembly; use --audit-only, not the four-export generic workload')
            if config.get('UML') == 'y': raise ValueError('generic UML recognized, but the system-QEMU runner cannot boot UML')
            for name in ('MODULES', 'PRINTK', 'MULTIUSER', 'CFI'):
                if config.get(name) != 'y': raise ValueError('requires CONFIG_' + name + '=y')
            if args.caller == 'rust' and config.get('RUST') != 'y': raise ValueError('Rust caller requires CONFIG_RUST=y')
            if args.reload_modules and config.get('MODULE_UNLOAD') != 'y': raise ValueError('reload requires MODULE_UNLOAD=y')
            if config.get('MODULE_SIG_FORCE') == 'y': raise ValueError('disposable unsigned modules cannot satisfy forced signing')
        owner = verify_linked_implementation(build, selection)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    if args.audit_only:
        print(f'{selection} hweight: selected owner {owner}; artifact-only audit, no runtime claim.')
        return
    arch = architecture(config)
    work = build / 'rust-hweight-test'
    work.mkdir(exist_ok=True)
    name, stem = ('hweight_rust_abi', 'hweight_rust_main') if args.caller == 'rust' else ('hweight_abi', 'hweight_c_main')
    (work / 'hweight_reference.c').write_text(reference_source())
    (work / (stem + ('.rs' if args.caller == 'rust' else '.c'))).write_text(sources(args.caller))
    (work / 'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o hweight_reference.o\n')
    env = {key: value for key, value in os.environ.items() if not key.startswith(('CONFIG_', 'KCONFIG_', 'KBUILD_')) and key not in
           ('MAKEFLAGS', 'MFLAGS', 'CARGO_MAKEFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES', 'srctree', 'srcroot', 'objtree', 'VPATH', 'sub_make_done')}
    subprocess.run([*shlex.split(os.environ.get('MAKE', 'make')), '-C', str(ROOT), 'O=' + str(build), 'M=' + str(work),
                    *args.make_arg, PRIVATE_C_FLAGS, 'modules'], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / 'scripts/tests/boot_kernel.py'), '--build', str(build), '--arch', arch,
               '--module', str(work / (name + '.ko')), '--qemu', args.qemu or ('qemu-system-aarch64' if arch == 'aarch64' else 'qemu-system-x86_64')]
    if args.reload_modules: command += ['--reload-modules']
    if args.qemu_data: command += ['--qemu-data', str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    runs = verify_console((build / 'rust-boot-test/console.log').read_bytes(), args.caller, reload=args.reload_modules, config=config)
    print(f'{selection} generic hweight: Proprietary {args.caller} caller passed {COUNT} values and {COUNT * 4} protected comparisons/load; {runs} loads on {arch}.')


if __name__ == '__main__': main()
