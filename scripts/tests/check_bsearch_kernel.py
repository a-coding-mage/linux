#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit selected binary search and run independent C/Rust callers in QEMU.

All builds, modules, logs and compiler output are private. Native/source inputs
are read-only. There is no dedicated original bsearch KUnit suite: both callers
execute the existing original-C differential corpus and compare pivot traces.
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

from bsearch_runtime_fixtures import caller_source, reference_source, marker, EXPORTS, CASES, DESCRIPTION
from boot_kernel import MARKER, architecture as boot_architecture, init_command, module_name, qemu_command, verify_module_events
from check_cmdline_kernel import verify_external_command, verify_strict_lints
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_int_log_kernel import normalize_console_transport, kunit_runs, verify_kunit_warnings
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer
from check_prime_numbers_kernel import provider_type_ids, verify_module_import_versions, verify_rust_entrypoints, verify_guarded_calls
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_reciprocal_kernel import module_elf
from check_rust_exports_bridge import version_records
from check_rbtree_kernel import (observed_build, linked_type_ids, command_words, run_logged,
    clean_environment, make_arguments, private_copy, execute_guest)
from rbtree_native.transport import explicit_path, native_flags, outside, require_rustc, compiler_environment, validate_compiler_outputs, verify_flag_policy
from rust_exports_test_support import read_exports

ROOT = Path(os.environ.get('BSEARCH_RUNTIME_SOURCE_ROOT', Path(__file__).resolve().parents[2])).resolve()
WRAPPERS = (('bsearch_call_bsearch', 'bsearch'),)


def verify_source(root):
    original = (root / 'lib/bsearch.c').read_text()
    if re.findall(r'^EXPORT_SYMBOL(_GPL)?\((\w+)\);', original, re.M) != [('', 'bsearch')]:
        raise ValueError('original bsearch export or license changed')
    if 'return __inline_bsearch(key, base, num, size, cmp);' not in original or 'NOKPROBE_SYMBOL(bsearch);' not in original:
        raise ValueError('original bsearch algorithm/kprobe contract changed')
    # Fixture generation consumes the existing original differential corpus.
    caller_source(root, 'c')


def provider(config):
    architecture(config)
    if config.get('UML') == 'y': raise ValueError('system QEMU cannot boot UML')
    if config.get('RUST_BSEARCH', 'n') not in ('n', 'y'): raise ValueError('invalid bsearch selection')
    if config.get('RUST_BSEARCH') == 'y' and config.get('RUST') != 'y': raise ValueError('Rust bsearch requires RUST')
    return 'Rust' if config.get('RUST_BSEARCH') == 'y' else 'C'


def owner_path(build, selection):
    if selection not in ('Rust', 'C'): raise ValueError('unknown bsearch provider')
    return build / ('lib/bsearch_rust.o' if selection == 'Rust' else 'lib/bsearch.o')


def selected_versions(build):
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()]
    found = [row for row in rows if len(row) > 1 and row[1] == b'bsearch']
    if (len(found) != 1 or len(found[0]) != 4 or found[0][2:] != [b'vmlinux', b'EXPORT_SYMBOL'] or
            not re.fullmatch(rb'0x[0-9a-fA-F]{8}', found[0][0])):
        raise ValueError('wrong bsearch owner/license/namespace/CRC')
    return {b'bsearch': found[0][0].lower()}


def verify_blacklist(owner, config, arch):
    _, sections, _, _, _ = module_elf(owner)
    matches = [(i, s) for i, s in enumerate(sections) if s[0] == b'_kprobe_blacklist']
    if config.get('KPROBES') != 'y':
        if matches: raise ValueError('unexpected disabled bsearch kprobe record')
        return
    if len(matches) != 1: raise ValueError('missing/duplicate bsearch kprobe record')
    index, section = matches[0]
    if (section[1], section[2], section[4], section[7], section[9]) != (1, 3, 8, 8, b'\0' * 8):
        raise ValueError('wrong bsearch kprobe section shape')
    relocations = [r for s in sections if s[1] == 4 and s[6] == index for r in s[9]]
    if len(relocations) != 1: raise ValueError('wrong bsearch kprobe relocation count')
    offset, kind, target, addend = relocations[0]
    if (offset, kind, target[0], addend, target[1] & 15) != (0, 257 if arch == 'aarch64' else 1, b'bsearch', 0, 2):
        raise ValueError('wrong bsearch kprobe relocation target/type')


def verify_bindings(build, root):
    generated = build / 'rust/bindings/bindings_generated.rs'
    deps = verify_external_command(build, generated, root / 'rust/bindings/bindings_helper.h', [root / 'include/linux/bsearch.h'])
    newer(generated, [*deps, root / 'rust/bindgen_parameters'])
    if re.search(r'pub fn bsearch\(', generated.read_text()):
        raise ValueError('incorrect raw Option bsearch binding remains generated')
    obj = build / 'rust/bindings.o'; metadata = build / 'rust/libbindings.rmeta'
    deps = verify_external_command(build, obj, root / 'rust/bindings/lib.rs', [generated, root / 'rust/bindings/bsearch.rs'])
    outputs = [p.split('=', 1)[1] for flag in compilation_flags(obj) if flag.startswith('--emit=')
               for p in flag[7:].split(',') if p.startswith('metadata=')]
    if len(outputs) != 1 or (build / outputs[0]).resolve() != metadata.resolve():
        raise ValueError('bindings command emits different metadata')
    newer(metadata, [*deps, build / 'include/generated/rustc_cfg'])
    newer(build / 'rust/libkernel.rmeta', [metadata])


def verify_selected(build, *, source=None, work=None, allow_c=False):
    build = explicit_path(build, 'build'); root = explicit_path(source or ROOT, 'source')
    verify_source(root); config = configuration(build); arch = architecture(config); selection = provider(config)
    if selection == 'C' and not allow_c: raise ValueError('requires CONFIG_RUST_BSEARCH=y')
    for name in ('CFI', 'MODVERSIONS', 'GENDWARFKSYMS', 'MODULES'):
        if config.get(name) != 'y': raise ValueError('requires CONFIG_' + name + '=y')
    if config.get('CFI_PERMISSIVE') == 'y': raise ValueError('strict non-permissive CFI required')
    owner = owner_path(build, selection)
    image = build / boot_architecture(arch)[0]
    for path in (owner, build / 'lib/built-in.a', build / 'vmlinux.a', build / 'vmlinux.o', build / 'vmlinux', build / 'Module.symvers', image):
        if not path.is_file(): raise ValueError('missing selected artifact: ' + str(path))
    if work is None:
        parent = outside(explicit_path(os.environ.get('BSEARCH_RUNTIME_LOG_DIR', tempfile.gettempdir()), 'log parent'), (build, root))
        if not parent.is_dir(): raise ValueError('private log parent must exist')
        work = Path(tempfile.mkdtemp(prefix='bsearch-selected-', dir=parent))
    work = outside(explicit_path(work, 'work'), (build, root))
    if not work.is_dir(): raise ValueError('private audit directory must exist')
    with observed_build(build, work):
        candidates = {owner_path(build, kind).resolve() for kind in ('C', 'Rust')}
        for archive in (build / 'lib/built-in.a', build / 'vmlinux.a'):
            members = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', archive).splitlines()]
            if [path for path in members if path in candidates] != [owner.resolve()]:
                raise ValueError('selected bsearch archive owner absent/mixed/duplicated')
            newer(archive, [owner])
        source_file = root / ('lib/bsearch_rust.rs' if selection == 'Rust' else 'lib/bsearch.c')
        deps = [root / 'include/linux/bsearch.h'] if selection == 'C' else [root / p for p in
            ('lib/bsearch.rs', 'include/linux/bsearch_header.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs')]
        if selection == 'Rust': deps += [build / 'rust/libbindings.rmeta']
        verify_build_command(build, owner, source_file, deps)
        command = compilation_flags(owner)
        flags = command.copy()
        while flags and '=' in flags[0] and not flags[0].startswith('-'): flags.pop(0)
        real = native_flags(flags[1:], build, 'rust' if selection == 'Rust' else 'c')
        verify_flag_policy(real, 'rust' if selection == 'Rust' else 'c')
        if selection == 'Rust':
            verify_strict_lints(real)
            if [f for f in command if f.startswith('RUST_MODFILE=')] != ['RUST_MODFILE=lib/bsearch']:
                raise ValueError('selected bsearch lost original modfile identity')
            require_rustc(flags[0], work, compiler_environment(work, dict(os.environ)))
            verify_bindings(build, root)
        elf_target(owner, arch); verify_blacklist(owner, config, arch)
        exports = read_exports(owner)
        if len(exports) != 1 or exports[0]['name'] != 'bsearch': raise ValueError('wrong bsearch defining exports')
        expected = dict(license='', namespace='', relocation_target='bsearch', relocation_addend=0,
            pointer_width=8, relocation_kind=257 if arch == 'aarch64' else 1,
            label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        if any(exports[0][key] != value for key, value in expected.items()): raise ValueError('wrong actual bsearch export metadata')
        types = provider_type_ids(owner, names=EXPORTS)
        for path in (owner, build / 'vmlinux.o', build / 'vmlinux'):
            definitions = [row.split() for row in tool('nm', '-g', '--defined-only', path).splitlines()
                           if row.split() and row.split()[-1] == b'bsearch']
            if len(definitions) != 1 or definitions[0][-2] != b'T': raise ValueError('missing/duplicate final bsearch function')
            actual = linked_type_ids(path, EXPORTS, arch) if path.name == 'vmlinux' else provider_type_ids(path, names=EXPORTS)
            if actual != types: raise ValueError('final bsearch KCFI identity differs')
        versions = selected_versions(build)
        if version_records(owner.with_name('.' + owner.name + '.cmd')) != versions: raise ValueError('selected bsearch versions differ')
        proof = run_logged([build / 'scripts/gendwarfksyms/gendwarfksyms', '--symtypes', work / 'bsearch.symtypes', owner],
                           work / 'defining-dwarf.log', input=b'bsearch\n')
        rows = [line.split() for line in proof.stdout.splitlines()]
        if proof.stderr or len(rows) != 1 or len(rows[0]) != 3 or rows[0][:2] != [b'#SYMVER', b'bsearch'] or rows[0][2].lower() != versions[b'bsearch']:
            raise ValueError('real defining bsearch DWARF differs from selected versions')
        if not (work / 'bsearch.symtypes').read_bytes(): raise ValueError('missing actual bsearch DWARF types')
        newer(build / 'Module.symvers', [owner]); newer(build / 'vmlinux.o', [build / 'vmlinux.a'])
        newer(build / 'vmlinux', [build / 'vmlinux.o']); newer(image, [build / 'vmlinux'])
    result = dict(build=str(build), source=str(root), selection=selection, architecture=arch, owner=str(owner),
                  versions={k.decode(): v.decode() for k, v in versions.items()}, kcfi=types, work=str(work), runtime=False)
    (work / 'selected.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def caller_names(caller):
    marker(caller)
    return ('bsearch_rust_abi', 'bsearch_rust_main') if caller == 'rust' else ('bsearch_abi', 'bsearch_c_main')


def verify_consumer(build, work, caller, *, source=None):
    root = source or ROOT; name, stem = caller_names(caller)
    obj = work / (stem + '.o'); module = work / (name + '.ko'); oracle = work / 'bsearch_reference.o'
    config = configuration(build); arch = architecture(config)
    supplied = obj.with_suffix('.rs' if caller == 'rust' else '.c')
    if supplied.read_text() != caller_source(root, caller) or oracle.with_suffix('.c').read_text() != reference_source(root):
        raise ValueError('independent caller/original C oracle source changed')
    deps = [root / 'include/linux/bsearch.h'] if caller == 'c' else [build / 'rust/libkernel.rmeta', root / 'include/linux/bsearch_header.rs']
    verify_external_command(work, obj, supplied, deps)
    verify_external_command(work, oracle, oracle.with_suffix('.c'), [root / 'lib/bsearch.c', root / 'include/linux/bsearch.h'])
    members = [(work / os.fsdecode(row)).resolve() for row in module.with_suffix('.mod').read_bytes().splitlines()]
    if members != [obj.resolve(), oracle.resolve()]: raise ValueError('unexpected bsearch module constituents')
    if [(work / os.fsdecode(row)).resolve() for row in (work / 'modules.order').read_bytes().splitlines()] != [module.with_suffix('.o').resolve()]:
        raise ValueError('unexpected external module order')
    if module_name(module) != name: raise ValueError('wrong bsearch caller module name')
    types = provider_type_ids(owner_path(build, provider(config)), names=EXPORTS)
    for path in (obj, module):
        elf_target(path, arch); verify_references(path, EXPORTS)
        verify_guarded_calls(path, arch, types, wrappers=WRAPPERS)
        if read_exports(path): raise ValueError('private bsearch caller exports symbols')
        if caller == 'rust': verify_rust_entrypoints(path, arch)
    imports = {row.split()[-1] for row in tool('nm', '-u', obj).splitlines() if row.split()}
    required = {b'bsearch', b'original_bsearch', b'_printk'}
    allowed = {b'memcpy', b'memset', b'memcmp', b'bcmp', b'__stack_chk_fail', b'__x86_return_thunk', b'__fentry__'}
    intrinsic = {name for name in imports if re.fullmatch(rb'_R\w*4core\w+', name)}
    if not required <= imports or imports - required - allowed - intrinsic:
        raise ValueError('bsearch caller missing workload or delegating to a C dispatcher: ' + repr(sorted(imports)))
    if caller == 'rust' and b'bsearch_rust_exercise' not in tool('nm', '-g', '--defined-only', obj):
        raise ValueError('Rust caller does not own independent workload')
    verify_module_import_versions(build, module)
    require_metadata_field(metadata_fields(module), b'license', b'GPL')
    require_metadata_field(metadata_fields(module), b'description', DESCRIPTION.encode())
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    newer(module, [obj, oracle, selected_metadata(build, module)])
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


def verify_console(console, caller, *, reload=False, config=None):
    if re.search(rb'CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version', console, re.I):
        raise ValueError('bsearch kernel fault/test failure')
    lines = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip()).strip()
             for line in normalize_console_transport(console).splitlines()]
    allowed = set()
    if config and config.get('INT_LOG_KUNIT_TEST') == 'y':
        allowed = set(verify_kunit_warnings(lines, kunit_runs(console, 1),
            'Rust' if config.get('RUST_INT_LOG') == 'y' else 'C', config.get('BUG') == 'y'))
    if {i for i, line in enumerate(lines) if b'WARNING:' in line} != allowed:
        raise ValueError('warning outside configured original integer-log zero cases')
    events = [re.sub(rb'^bsearch_(?:rust_)?abi: ', b'', line) for line in lines if b'LUPOS_' in line]
    expected = [marker(caller), b'LUPOS_RUST_MODULE_LOAD_OK']
    if reload: expected += [b'LUPOS_RUST_MODULE_UNLOAD_OK 0', marker(caller), b'LUPOS_RUST_MODULE_RELOAD_OK 0']
    expected += [MARKER]
    if events != expected: raise ValueError('bsearch workload/lifecycle events differ')
    verify_module_events(console, module=True, preloads=0, reload=reload)
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
    if reload:
        plan = work / 'reload-plan'; plan.write_text('/test-module.ko\t' + module_name(module) + '\n')
        entries += f'file /reload-plan {plan} 0600 0 0\n'
    manifest = work / 'manifest'; manifest.write_text(entries); archive = work / 'initramfs.cpio'
    run_logged([build / 'usr/gen_init_cpio', '-t', '0', '-c', '-o', archive, manifest], work / 'initramfs.log')
    return archive


def validate_options(args, environment=None):
    env = os.environ if environment is None else environment
    build = explicit_path(args.build, 'build'); root = explicit_path(args.source, 'source')
    verify_source(root); config = configuration(build); arch = architecture(config); selection = provider(config)
    if selection == 'C' and not args.allow_c_baseline: raise ValueError('requires CONFIG_RUST_BSEARCH=y')
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
    parser.add_argument('--output-parent', default=os.environ.get('BSEARCH_RUNTIME_LOG_DIR', tempfile.gettempdir()))
    parser.add_argument('--make-arg', action='append', default=[]); parser.add_argument('--qemu'); parser.add_argument('--qemu-data')
    parser.add_argument('--timeout', type=float, default=300)
    args = parser.parse_args(argv)
    try:
        values = validate_options(args); build, root = values['build'], values['root']
        work = Path(tempfile.mkdtemp(prefix='bsearch-runtime-', dir=values['parent']))
        print('Retained bsearch runtime evidence: ' + str(work), flush=True)
        audit = work / 'audit'; audit.mkdir()
        result = verify_selected(build, source=root, work=audit, allow_c=args.allow_c_baseline)
        if args.audit_only: print(json.dumps(result, indent=2)); return
        with observed_build(build, work):
            private = work / 'build'; private_copy(build, private, root)
            consumer = work / 'consumer'; consumer.mkdir(); name, stem = caller_names(args.caller)
            (consumer / (stem + ('.rs' if args.caller == 'rust' else '.c'))).write_text(caller_source(root, args.caller))
            (consumer / 'bsearch_reference.c').write_text(reference_source(root))
            (consumer / 'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o bsearch_reference.o\nRUSTFLAGS_{stem}.o += --extern bindings\n')
            run_logged([*values['make'], '-C', root, 'O=' + str(private), 'M=' + str(consumer), *values['make_args'],
                'CFLAGS_MODULE+=-D__DISABLE_EXPORTS', 'RUSTFLAGS_MODULE+=-Dwarnings -Dunsafe_op_in_unsafe_fn', 'modules'],
                work / 'external-module.log', env=clean_environment(os.environ))
            module = verify_consumer(private, consumer, args.caller, source=root)
            boot = work / 'boot'; archive = prepare_boot(private, boot, module, values['arch'], values['host'], root, reload=args.reload_modules)
            result.update(caller=args.caller, consumer=str(module), boot=str(boot), runtime=False)
            if not args.build_only:
                command = qemu_command(shlex.join(values['qemu']), values['arch'], build / boot_architecture(values['arch'])[0], archive, values['data'])
                console = execute_guest(command, boot, args.timeout)
                loads = verify_console(console, args.caller, reload=args.reload_modules, config=values['config'])
                result.update(runtime=True, loads=loads, cases_per_load=CASES)
        (work / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    except (OSError, ValueError, KeyError, IndexError, struct.error, subprocess.SubprocessError) as error:
        parser.error(str(error))
    print(json.dumps(result, indent=2))


if __name__ == '__main__': main()
