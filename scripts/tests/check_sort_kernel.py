#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit selected four-API sort and run independent C/Rust callers in QEMU.

All builds, modules, logs and compiler output are private. Native/source inputs
are read-only. Both independent callers compare original-C callback traces and
check configured scheduling state; the original lib_sort KUnit module reloads.
Full/lazy modes disable cond_resched and are never reported as active scheduling.
"""
import argparse
import json
import math
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import tempfile
import types

from sort_runtime_fixtures import caller_source, reference_source, marker, EXPORTS, CASES, DESCRIPTION
from boot_kernel import MARKER, architecture as boot_architecture, init_command, module_name, qemu_command, verify_module_events
from check_cmdline_kernel import verify_external_command, verify_strict_lints
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_int_log_kernel import normalize_console_transport, kunit_runs, verify_kunit_warnings
from check_polynomial_kernel import elf_target, newer
from check_prime_numbers_kernel import provider_type_ids, verify_module_import_versions, verify_rust_entrypoints, verify_guarded_calls
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_reciprocal_kernel import module_elf
from check_rust_exports_bridge import version_records
from check_rbtree_kernel import (observed_build, linked_type_ids, command_words, run_logged,
    clean_environment, make_arguments, private_copy, execute_guest)
from rbtree_native.transport import explicit_path, native_flags, outside, require_rustc, compiler_environment, validate_compiler_outputs, verify_flag_policy
from rust_exports_test_support import read_exports

ROOT = Path(os.environ.get('SORT_RUNTIME_SOURCE_ROOT', Path(__file__).resolve().parents[2])).resolve()
WRAPPERS = tuple(('sort_call_' + name, name) for name in EXPORTS)


def verify_source(root):
    original = (root / 'lib/sort.c').read_text()
    if sorted(re.findall(r'^EXPORT_SYMBOL\((\w+)\);', original, re.M)) != sorted(EXPORTS):
        raise ValueError('original sort export surface changed')
    suite = (root / 'lib/tests/test_sort.c').read_text()
    if re.findall(r'KUNIT_CASE\((\w+)\)', suite) != ['test_sort'] or '.name = "lib_sort"' not in suite:
        raise ValueError('original lib_sort KUnit surface changed')

def provider(config):
    architecture(config)
    if config.get('UML') == 'y': raise ValueError('system QEMU cannot boot UML')
    if config.get('RUST_SORT', 'n') not in ('n', 'y'): raise ValueError('invalid sort selection')
    if config.get('RUST_SORT') == 'y' and config.get('RUST') != 'y': raise ValueError('Rust sort requires RUST')
    return 'Rust' if config.get('RUST_SORT') == 'y' else 'C'


def owner_path(build, selection):
    if selection not in ('Rust', 'C'): raise ValueError('unknown sort provider')
    return build / ('lib/sort_rust.o' if selection == 'Rust' else 'lib/sort.o')


def selected_versions(build):
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()]
    result = {}
    for name in EXPORTS:
        found = [row for row in rows if len(row) > 1 and row[1] == name.encode()]
        if (len(found) != 1 or len(found[0]) != 4 or found[0][2:] != [b'vmlinux', b'EXPORT_SYMBOL'] or
                not re.fullmatch(rb'0x[0-9a-fA-F]{8}', found[0][0])):
            raise ValueError('wrong sort owner/license/namespace/CRC')
        result[name.encode()] = found[0][0].lower()
    return result

def verify_scheduler(owner, config, arch, aggregate=None):
    if config.get('PREEMPT_DYNAMIC') != 'y': raise ValueError('runtime scheduling proof requires PREEMPT_DYNAMIC=y')
    imports = {line.split()[-1] for line in tool('nm', '-u', owner).splitlines() if line.split()}
    expected = b'__SCT__cond_resched' if config.get('HAVE_PREEMPT_DYNAMIC_CALL') == 'y' else b'dynamic_cond_resched'
    if expected not in imports: raise ValueError('selected sort lacks genuine configured scheduling call')
    if config.get('HAVE_PREEMPT_DYNAMIC_CALL') == 'y' and config.get('HAVE_STATIC_CALL_INLINE') == 'y':
        _, sections, symbols, _, _ = module_elf(owner)
        if b'__SCK__cond_resched' not in imports:
            raise ValueError('selected sort lacks addressable static-call key')
        if aggregate is None: aggregate = owner.parents[1] / 'vmlinux.o'
        _, linked_sections, linked_symbols, _, _ = module_elf(aggregate)
        relocs = [r for section in linked_sections if section[0] == b'.rela.static_call_sites' for r in section[-1]]
        if not relocs: raise ValueError('missing completed aggregate objtool static-call sites')
        calls = [(section[6], r) for section in sections if section[1] == 4
                 for r in section[-1] if r[2][0] == b'__SCT__cond_resched']
        if not calls: raise ValueError('missing original sort scheduling call relocation')
        for section_index, (address, kind, _, addend) in calls:
            functions = [sym for sym in symbols if sym[1] & 15 == 2 and sym[3] == section_index and sym[4] <= address < sym[4] + sym[5]]
            if len(functions) != 1 or kind != 4 or addend != -4 or sections[section_index][-1][address-1] != 0xe8:
                raise ValueError('unexpected sort static-call instruction shape')
            function = functions[0]
            linked = [sym for sym in linked_symbols if sym[0] == function[0] and sym[1] & 15 == 2 and sym[5] == function[5]]
            if len(linked) != 1: raise ValueError('ambiguous selected sort aggregate function')
            target = linked[0][4] + address - function[4] - 1
            matches = [r for r in relocs if r[1] == 2 and r[2][3] == linked[0][3] and r[2][4] + r[3] == target]
            if len(matches) != 1: raise ValueError('missing/duplicate selected sort static-call site')
            keys = [r for r in relocs if r[0] == matches[0][0] + 4]
            if len(keys) != 1 or keys[0][1] != 2 or keys[0][2][0] != b'__SCK__cond_resched' or keys[0][3] != 0:
                raise ValueError('wrong selected sort static-call site key')
        command = aggregate.with_name('.' + aggregate.name + '.cmd').read_text().splitlines()[0]
        if config.get('X86_KERNEL_IBT') == 'y' and not all(word in command for word in ('objtool', '--static-call', '--link')):
            raise ValueError('missing actual aggregate objtool static-call processing')


def verify_original_suite(build, root):
    config = configuration(build)
    if config.get('KUNIT') != 'm' or config.get('TEST_SORT') != 'm':
        raise ValueError('requires original modular KUNIT=m and TEST_SORT=m')
    result = [build / 'lib/kunit/kunit.ko', build / 'lib/tests/test_sort.ko']
    for module in result:
        if not module.is_file(): raise ValueError('missing original KUnit module ' + str(module))
        verify_module_import_versions(build, module)
    obj = build / 'lib/tests/test_sort.o'
    verify_build_command(build, obj, root / 'lib/tests/test_sort.c', [root / 'include/linux/sort.h'])
    verify_references(obj, ('sort',))
    symbols = tool('nm', '--defined-only', obj)
    for name in (b'test_sort', b'sort_test_cases', b'sort_test_suite'):
        if not any(row.split()[-1] == name for row in symbols.splitlines() if row.split()):
            raise ValueError('missing original sort KUnit graph member')
    newer(result[1], [obj, root / 'lib/tests/test_sort.c'])
    return result

def verify_bindings(build, root):
    generated = build / 'rust/bindings/bindings_generated.rs'
    deps = verify_external_command(build, generated, root / 'rust/bindings/bindings_helper.h', [root / 'include/linux/sort.h'])
    newer(generated, [*deps, root / 'rust/bindgen_parameters'])
    if re.search(r'pub fn sort(?:_r)?(?:_nonatomic)?\(', generated.read_text()):
        raise ValueError('incorrect raw Option sort binding remains generated')
    obj = build / 'rust/bindings.o'; metadata = build / 'rust/libbindings.rmeta'
    deps = verify_external_command(build, obj, root / 'rust/bindings/lib.rs', [generated, root / 'rust/bindings/sort.rs'])
    outputs = [p.split('=', 1)[1] for flag in compilation_flags(obj) if flag.startswith('--emit=')
               for p in flag[7:].split(',') if p.startswith('metadata=')]
    if len(outputs) != 1 or (build / outputs[0]).resolve() != metadata.resolve():
        raise ValueError('bindings command emits different metadata')
    newer(metadata, [*deps, build / 'include/generated/rustc_cfg'])
    newer(build / 'rust/libkernel.rmeta', [metadata])


def verify_selected(build, *, source=None, work=None, allow_c=False):
    build = explicit_path(build, 'build'); root = explicit_path(source or ROOT, 'source')
    verify_source(root); config = configuration(build); arch = architecture(config); selection = provider(config)
    if selection == 'C' and not allow_c: raise ValueError('requires CONFIG_RUST_SORT=y')
    for name in ('CFI', 'MODVERSIONS', 'GENDWARFKSYMS', 'MODULES'):
        if config.get(name) != 'y': raise ValueError('requires CONFIG_' + name + '=y')
    if config.get('CFI_PERMISSIVE') == 'y': raise ValueError('strict non-permissive CFI required')
    owner = owner_path(build, selection)
    image = build / boot_architecture(arch)[0]
    for path in (owner, build / 'lib/built-in.a', build / 'vmlinux.a', build / 'vmlinux.o', build / 'vmlinux', build / 'Module.symvers', image):
        if not path.is_file(): raise ValueError('missing selected artifact: ' + str(path))
    if work is None:
        parent = outside(explicit_path(os.environ.get('SORT_RUNTIME_LOG_DIR', tempfile.gettempdir()), 'log parent'), (build, root))
        if not parent.is_dir(): raise ValueError('private log parent must exist')
        work = Path(tempfile.mkdtemp(prefix='sort-selected-', dir=parent))
    work = outside(explicit_path(work, 'work'), (build, root))
    if not work.is_dir(): raise ValueError('private audit directory must exist')
    with observed_build(build, work):
        candidates = {owner_path(build, kind).resolve() for kind in ('C', 'Rust')}
        for archive in (build / 'lib/built-in.a', build / 'vmlinux.a'):
            members = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', archive).splitlines()]
            if [path for path in members if path in candidates] != [owner.resolve()]:
                raise ValueError('selected sort archive owner absent/mixed/duplicated')
            newer(archive, [owner])
        source_file = root / ('lib/sort_rust.rs' if selection == 'Rust' else 'lib/sort.c')
        deps = [root / 'include/linux/sort.h'] if selection == 'C' else [root / p for p in
            ('lib/sort.rs', 'lib/sort_sched.rs', 'include/linux/sort_header.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs')]
        if selection == 'Rust': deps += [build / 'rust/libbindings.rmeta']
        verify_build_command(build, owner, source_file, deps)
        command = compilation_flags(owner)
        flags = command.copy()
        while flags and '=' in flags[0] and not flags[0].startswith('-'): flags.pop(0)
        real = native_flags(flags[1:], build, 'rust' if selection == 'Rust' else 'c')
        verify_flag_policy(real, 'rust' if selection == 'Rust' else 'c')
        if selection == 'Rust':
            verify_strict_lints(real)
            if [f for f in command if f.startswith('RUST_MODFILE=')] != ['RUST_MODFILE=lib/sort']:
                raise ValueError('selected sort lost original modfile identity')
            require_rustc(flags[0], work, compiler_environment(work, dict(os.environ)))
            verify_bindings(build, root)
        elf_target(owner, arch); verify_scheduler(owner, config, arch)
        exports = read_exports(owner)
        if sorted(entry['name'] for entry in exports) != sorted(EXPORTS): raise ValueError('wrong sort defining exports')
        for entry in exports:
            expected = dict(license='', namespace='', relocation_target=entry['name'], relocation_addend=0,
                pointer_width=8, relocation_kind=257 if arch == 'aarch64' else 1,
                label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
            if any(entry[key] != value for key, value in expected.items()): raise ValueError('wrong actual sort export metadata')
        types = provider_type_ids(owner, names=EXPORTS)
        for path in (owner, build / 'vmlinux.o', build / 'vmlinux'):
            definitions = [row.split() for row in tool('nm', '-g', '--defined-only', path).splitlines()
                           if row.split() and row.split()[-1].decode() in EXPORTS]
            if len(definitions) != 4 or any(row[-2] != b'T' for row in definitions): raise ValueError('missing/duplicate final sort function')
            actual = linked_type_ids(path, EXPORTS, arch) if path.name == 'vmlinux' else provider_type_ids(path, names=EXPORTS)
            if actual != types: raise ValueError('final sort KCFI identity differs')
        versions = selected_versions(build)
        if version_records(owner.with_name('.' + owner.name + '.cmd')) != versions: raise ValueError('selected sort versions differ')
        proof = run_logged([build / 'scripts/gendwarfksyms/gendwarfksyms', '--symtypes', work / 'sort.symtypes', owner],
                           work / 'defining-dwarf.log', input=('\n'.join(EXPORTS) + '\n').encode())
        rows = [line.split() for line in proof.stdout.splitlines()]
        if proof.stderr or len(rows) != 4 or any(len(row) != 3 or row[0] != b'#SYMVER' for row in rows) or {row[1]: row[2].lower() for row in rows} != versions:
            raise ValueError('real defining sort DWARF differs from selected versions')
        if not (work / 'sort.symtypes').read_bytes(): raise ValueError('missing actual sort DWARF types')
        verify_original_suite(build, root)
        newer(build / 'Module.symvers', [owner]); newer(build / 'vmlinux.o', [build / 'vmlinux.a'])
        newer(build / 'vmlinux', [build / 'vmlinux.o']); newer(image, [build / 'vmlinux'])
    result = dict(build=str(build), source=str(root), selection=selection, architecture=arch, owner=str(owner),
                  versions={k.decode(): v.decode() for k, v in versions.items()}, kcfi=types, work=str(work), runtime=False)
    (work / 'selected.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def caller_names(caller):
    marker(caller)
    return ('sort_rust_abi', 'sort_rust_main') if caller == 'rust' else ('sort_abi', 'sort_c_main')


def verify_private_metadata(build, module, work, root):
    # These existing validators derive canonical sources from their own file.
    # Execute the actual private-source copies without writing bytecode caches.
    validators = {}
    for name in ('check_module_metadata', 'check_module_common'):
        path = root / 'scripts/tests' / (name + '.py')
        validator = types.ModuleType(name + '_sort_private')
        validator.__file__ = str(path)
        exec(compile(path.read_bytes(), str(path), 'exec'), validator.__dict__)
        validators[name] = validator
    metadata = validators['check_module_metadata'].verify_module_metadata(build, module, work=work, require_c_suppression=True)
    validators['check_module_common'].verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    return metadata


def verify_consumer(build, work, caller, *, source=None):
    root = source or ROOT; name, stem = caller_names(caller)
    obj = work / (stem + '.o'); module = work / (name + '.ko'); oracle = work / 'sort_reference.o'
    config = configuration(build); arch = architecture(config)
    supplied = obj.with_suffix('.rs' if caller == 'rust' else '.c')
    if supplied.read_text() != caller_source(root, caller) or oracle.with_suffix('.c').read_text() != reference_source(root):
        raise ValueError('independent caller/original C oracle source changed')
    deps = [root / 'include/linux/sort.h'] if caller == 'c' else [build / 'rust/libkernel.rmeta', root / 'include/linux/sort_header.rs']
    verify_external_command(work, obj, supplied, deps)
    verify_external_command(work, oracle, oracle.with_suffix('.c'), [root / 'lib/sort.c', root / 'include/linux/sort.h'])
    members = [(work / os.fsdecode(row)).resolve() for row in module.with_suffix('.mod').read_bytes().splitlines()]
    if members != [obj.resolve(), oracle.resolve()]: raise ValueError('unexpected sort module constituents')
    if [(work / os.fsdecode(row)).resolve() for row in (work / 'modules.order').read_bytes().splitlines()] != [module.with_suffix('.o').resolve()]:
        raise ValueError('unexpected external module order')
    if module_name(module) != name: raise ValueError('wrong sort caller module name')
    types = provider_type_ids(owner_path(build, provider(config)), names=EXPORTS)
    for path in (obj, module):
        elf_target(path, arch); verify_references(path, EXPORTS)
        verify_guarded_calls(path, arch, types, wrappers=WRAPPERS)
        if read_exports(path): raise ValueError('private sort caller exports symbols')
        if caller == 'rust': verify_rust_entrypoints(path, arch)
    imports = {row.split()[-1] for row in tool('nm', '-u', obj).splitlines() if row.split()}
    required = {name.encode() for name in EXPORTS} | {('original_' + name).encode() for name in EXPORTS} | {b'_printk', b'sort_runtime_arm', b'sort_runtime_pending', b'sort_runtime_clear'}
    allowed = {b'memcpy', b'memset', b'memcmp', b'bcmp', b'__stack_chk_fail', b'__x86_return_thunk', b'__fentry__'}
    intrinsic = {name for name in imports if re.fullmatch(rb'_R\w*4core\w+', name)}
    if not required <= imports or imports - required - allowed - intrinsic:
        raise ValueError('sort caller missing workload or delegating to a C dispatcher: ' + repr(sorted(imports)))
    if caller == 'rust' and b'sort_rust_exercise' not in tool('nm', '-g', '--defined-only', obj):
        raise ValueError('Rust caller does not own independent workload')
    verify_module_import_versions(build, module)
    require_metadata_field(metadata_fields(module), b'license', b'GPL')
    require_metadata_field(metadata_fields(module), b'description', DESCRIPTION.encode())
    metadata = verify_private_metadata(build, module, work, root)
    newer(module, [obj, oracle, metadata])
    for path, language in ((obj, 'rust' if caller == 'rust' else 'c'), (oracle, 'c')):
        command = compilation_flags(path)
        while command and '=' in command[0] and not command[0].startswith('-'): command.pop(0)
        real = native_flags(command[1:], work, language); verify_flag_policy(real, language)
        if language == 'c' and ('-D__DISABLE_EXPORTS' not in real or '-U__DISABLE_EXPORTS' in real or read_exports(path)):
            raise ValueError('private original-C fixture retains incidental exports')
        if language == 'rust':
            verify_strict_lints(real)
            require_rustc(command[0], work, compiler_environment(work, dict(os.environ)))
    return module


KUNIT_CASES = (b'test_sort',)


def sort_kunit_runs(console, runs):
    lines = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip()).strip() for line in normalize_console_transport(console).splitlines()]
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: lib_sort"]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+lib_sort(?:\s.*)?", line)]
    if runs not in (0, 1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("missing or duplicate lib_sort KUnit suite")
    result = []
    for start, end in zip(starts, ends):
        if start >= end or result and start <= result[-1][1]: raise ValueError("reordered lib_sort suite")
        cases, plan, summaries = [], False, set()
        for at in range(start + 1, end + 1):
            line = lines[at]
            if re.match(rb"\d+\.\.", line):
                if line != b"1..1" or plan or cases: raise ValueError("wrong lib_sort plan")
                plan = True
            elif re.match(rb"(?:not )?ok\s", line):
                if at == end and re.fullmatch(rb"ok [1-9]\d* lib_sort", line):
                    if cases != list(KUNIT_CASES): raise ValueError("premature lib_sort completion")
                elif plan and len(cases) < len(KUNIT_CASES) and line == b"ok " + str(len(cases)+1).encode() + b" " + KUNIT_CASES[len(cases)]:
                    cases.append(KUNIT_CASES[len(cases)])
                else: raise ValueError("failed/skipped/duplicate/wrong lib_sort case")
            elif re.match(rb"# (?:lib_sort|Totals):", line):
                match = re.fullmatch(rb"# (lib_sort|Totals): pass:1 fail:0 skip:0 total:1", line)
                if not match or match[1] in summaries or cases != list(KUNIT_CASES): raise ValueError("wrong lib_sort summary")
                summaries.add(match[1])
            elif any(line.startswith(b"# " + case + b": pass:") for case in KUNIT_CASES):
                if len(cases) >= len(KUNIT_CASES): raise ValueError("misplaced case summary")
                name = KUNIT_CASES[len(cases)]
                if line != b"# " + name + b": pass:1 fail:0 skip:0 total:1" or name in summaries:
                    raise ValueError("wrong lib_sort case summary")
                summaries.add(name)
            elif line.startswith(b"# Subtest:"): raise ValueError("unexpected lib_sort parameterization")
        if not plan or cases != list(KUNIT_CASES):
            raise ValueError("incomplete original lib_sort suite")
        result.append((start, end))
    for i, line in enumerate(lines):
        if (any(case in line for case in KUNIT_CASES) or line.startswith(b"# lib_sort:")) and not any(a < i < b for a,b in result):
            raise ValueError("lib_sort case outside suite")
    return result


def verify_console(console, caller, *, reload=False, config=None):
    if re.search(rb'CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version', console, re.I):
        raise ValueError('sort kernel fault/test failure')
    lines = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip()).strip()
             for line in normalize_console_transport(console).splitlines()]
    allowed = set()
    if config and config.get('INT_LOG_KUNIT_TEST') == 'y':
        allowed = set(verify_kunit_warnings(lines, kunit_runs(console, 1),
            'Rust' if config.get('RUST_INT_LOG') == 'y' else 'C', config.get('BUG') == 'y'))
    if {i for i, line in enumerate(lines) if b'WARNING:' in line} != allowed:
        raise ValueError('warning outside configured original integer-log zero cases')
    ranges = sort_kunit_runs(console, 2 if reload else 1)
    expected = [b'LUPOS_RUST_PRELOAD_OK 0', b'KUNIT', b'LUPOS_RUST_PRELOAD_OK 1', marker(caller), b'LUPOS_RUST_MODULE_LOAD_OK']
    if reload:
        expected += [f'LUPOS_RUST_MODULE_UNLOAD_OK {i}'.encode() for i in (2, 1, 0)]
        expected += [b'LUPOS_RUST_MODULE_RELOAD_OK 0', b'KUNIT', b'LUPOS_RUST_MODULE_RELOAD_OK 1', marker(caller), b'LUPOS_RUST_MODULE_RELOAD_OK 2']
    expected += [MARKER]
    events = []
    for i, line in enumerate(lines):
        if any(i == end for _, end in ranges): events.append(b'KUNIT')
        if b'LUPOS_' in line:
            if any(start <= i <= end for start, end in ranges): raise ValueError('sort event inside KUnit suite')
            events.append(re.sub(rb'^sort_(?:rust_)?abi: ', b'', line))
    if events != expected: raise ValueError('sort workload/KUnit/lifecycle events differ')
    verify_module_events(console, module=True, preloads=2, reload=reload)
    return 2 if reload else 1


def prepare_boot(build, work, module, arch, host, root, *, reload=False):
    work.mkdir(); source = work / 'boot_init.rs'
    source.write_bytes((root / 'scripts/tests/boot_init.rs').read_bytes()); init = work / 'init'
    command = [str(source) if word.endswith('/scripts/tests/boot_init.rs') else word for word in init_command(host, arch, init)]
    command += ['--out-dir=' + str(work)]
    require_rustc(host[0], work, compiler_environment(work, clean_environment(os.environ)))
    validate_compiler_outputs(command, work, work, (build, root))
    run_logged(command, work / 'init-compile.log', env=clean_environment(os.environ))
    fixture = work / 'fixture'; fixture.write_bytes(b'Rust-generated initramfs fixture\n\0with binary data\xff')
    if any(c.isspace() for c in str(module)): raise ValueError('initramfs module path contains whitespace')
    entries = ('dir /dev 0755 0 0\nnod /dev/console 0600 0 0 c 5 1\nnod /dev/kmsg 0600 0 0 c 1 11\n' +
        f'file /init {init} 0755 0 0\nfile /fixture {fixture} 0640 123 456 /hardlink\n' +
        'slink /symlink /fixture 0777 0 0\n' + f'file /test-module.ko {module} 0600 0 0\n')
    preloads = verify_original_suite(build, root)
    guests = []
    for index, preload in enumerate(preloads):
        guest = '/preload-module.' + str(index); guests.append((guest, preload))
        entries += f'file {guest} {preload} 0600 0 0\n'
    guests.append(('/test-module.ko', module))
    if reload:
        plan = work / 'reload-plan'; plan.write_text(''.join(guest + '\t' + module_name(path) + '\n' for guest, path in guests))
        entries += f'file /reload-plan {plan} 0600 0 0\n'
    manifest = work / 'manifest'; manifest.write_text(entries); archive = work / 'initramfs.cpio'
    run_logged([build / 'usr/gen_init_cpio', '-t', '0', '-c', '-o', archive, manifest], work / 'initramfs.log')
    return archive


def validate_options(args, environment=None):
    env = os.environ if environment is None else environment
    build = explicit_path(args.build, 'build'); root = explicit_path(args.source, 'source')
    verify_source(root); config = configuration(build); arch = architecture(config); selection = provider(config)
    if selection == 'C' and not args.allow_c_baseline: raise ValueError('requires CONFIG_RUST_SORT=y')
    parent = outside(explicit_path(args.output_parent, 'output parent'), (root, build, Path(__file__).resolve().parents[2]))
    if not parent.is_dir() or not os.access(parent, os.W_OK): raise ValueError('output parent must exist and be writable')
    if any(c.isspace() for c in str(parent)): raise ValueError('initramfs paths cannot contain whitespace')
    make = command_words(env.get('MAKE', 'make'), 'MAKE')
    if any(not re.fullmatch(r'(?:-j[1-9]\d*|--jobs=[1-9]\d*|--no-print-directory)', part) for part in make[1:]):
        raise ValueError('MAKE argv may only select job count or suppress directory messages')
    for name in ('QEMU', 'HOSTRUSTC'):
        if name in env: command_words(env[name], name)
    qemu = command_words(args.qemu, '--qemu') if args.qemu is not None else command_words(env['QEMU'], 'QEMU') if 'QEMU' in env else None
    data = explicit_path(args.qemu_data, 'qemu-data') if args.qemu_data is not None else None
    if data is not None and not data.is_dir(): raise ValueError('qemu-data directory does not exist')
    make_args = make_arguments(args.make_arg, arch)
    rust = None
    if config.get('RUST') == 'y':
        command = compilation_flags(owner_path(build, selection) if selection == 'Rust' else build / 'rust/kernel.o')
        while command and '=' in command[0] and not command[0].startswith('-'): command.pop(0)
        if not command: raise ValueError('missing selected Rust command')
        rust = command_words(command[0], 'selected Rust compiler')[0]
    if not args.audit_only:
        defaults = {}
        if rust:
            defaults.update(RUSTC=rust, HOSTRUSTC=rust)
            defaults['BINDGEN'] = command_words(compilation_flags(build / 'rust/bindings/bindings_generated.rs')[0], 'selected bindgen')[0]
        present = {item.split('=', 1)[0] for item in make_args}
        make_args += [name + '=' + value for name, value in defaults.items() if name not in present]
    if not math.isfinite(args.timeout) or args.timeout <= 0: raise ValueError('timeout must be finite and positive')
    if not args.audit_only:
        for name in ('MULTIUSER', 'PRINTK', 'MODULES'):
            if config.get(name) != 'y': raise ValueError('runtime requires CONFIG_' + name + '=y')
        if args.reload_modules and config.get('MODULE_UNLOAD') != 'y': raise ValueError('reload requires MODULE_UNLOAD=y')
        if config.get('MODULE_SIG_FORCE') == 'y': raise ValueError('unsigned caller requires disabled signature enforcement')
        if args.caller == 'rust' and config.get('RUST') != 'y': raise ValueError('Rust caller requires RUST=y')
        if not args.build_only and qemu is None: qemu = command_words('qemu-system-' + arch, 'default QEMU')
        for relative in ('usr/gen_init_cpio', 'scripts/gendwarfksyms/gendwarfksyms'):
            if not (build / relative).is_file(): raise ValueError('missing completed native artifact ' + relative)
    host = command_words(env.get('HOSTRUSTC', rust or 'rustc'), 'HOSTRUSTC')
    if len(host) != 1: raise ValueError('HOSTRUSTC must name one compiler')
    return dict(build=build, root=root, config=config, arch=arch, selection=selection, parent=parent,
                make=make, make_args=make_args, qemu=qemu, data=data, host=host)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build'); parser.add_argument('--source', default=str(ROOT))
    parser.add_argument('--audit-only', action='store_true'); parser.add_argument('--allow-c-baseline', action='store_true')
    parser.add_argument('--caller', choices=('c', 'rust'), default='c')
    parser.add_argument('--reload-modules', action='store_true'); parser.add_argument('--build-only', action='store_true')
    parser.add_argument('--output-parent', default=os.environ.get('SORT_RUNTIME_LOG_DIR', tempfile.gettempdir()))
    parser.add_argument('--make-arg', action='append', default=[]); parser.add_argument('--qemu'); parser.add_argument('--qemu-data')
    parser.add_argument('--timeout', type=float, default=300)
    args = parser.parse_args(argv)
    try:
        values = validate_options(args); build, root = values['build'], values['root']
        work = Path(tempfile.mkdtemp(prefix='sort-runtime-', dir=values['parent']))
        print('Retained sort runtime evidence: ' + str(work), flush=True)
        audit = work / 'audit'; audit.mkdir()
        result = verify_selected(build, source=root, work=audit, allow_c=args.allow_c_baseline)
        if args.audit_only: print(json.dumps(result, indent=2)); return
        with observed_build(build, work):
            private = work / 'build'; private_copy(build, private, root)
            consumer = work / 'consumer'; consumer.mkdir(); name, stem = caller_names(args.caller)
            (consumer / (stem + ('.rs' if args.caller == 'rust' else '.c'))).write_text(caller_source(root, args.caller))
            (consumer / 'sort_reference.c').write_text(reference_source(root))
            (consumer / 'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o sort_reference.o\nRUSTFLAGS_{stem}.o += --extern bindings\n')
            run_logged([*values['make'], '-C', root, 'O=' + str(private), 'M=' + str(consumer), *values['make_args'],
                'CFLAGS_MODULE+=-D__DISABLE_EXPORTS', 'RUSTFLAGS_MODULE+=-Dwarnings -Dunsafe_op_in_unsafe_fn', 'modules'],
                work / 'external-module.log', env=clean_environment(os.environ))
            module = verify_consumer(private, consumer, args.caller, source=root)
            boot = work / 'boot'; archive = prepare_boot(private, boot, module, values['arch'], values['host'], root, reload=args.reload_modules)
            result.update(caller=args.caller, consumer=str(module), boot=str(boot), runtime=False)
            if not args.build_only:
                command = qemu_command(shlex.join(values['qemu']), values['arch'], build / boot_architecture(values['arch'])[0], archive, values['data'])
                mode = 'lazy' if values['config'].get('ARCH_HAS_PREEMPT_LAZY') == 'y' else 'none'
                command[command.index('-append') + 1] += ' preempt=' + mode
                console = execute_guest(command, boot, args.timeout)
                loads = verify_console(console, args.caller, reload=args.reload_modules, config=values['config'])
                result.update(runtime=True, loads=loads, cases_per_load=CASES, preempt_mode=mode,
                              reschedule_requests_enabled=mode == 'none', original_kunit_runs=loads)
        (work / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    except (OSError, ValueError, KeyError, IndexError, struct.error, subprocess.SubprocessError) as error:
        parser.error(str(error))
    print(json.dumps(result, indent=2))


if __name__ == '__main__': main()
