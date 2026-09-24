# SPDX-License-Identifier: GPL-2.0-only
"""Complete bsearch differential and native proof; all artifacts retained privately.

Run unittest discovery. BSEARCH_SOURCE is the read-only kernel repository;
BSEARCH_OVERLAY supplies candidate files before integration. BSEARCH_OUT is a
new directory beneath the cwd. Optional BSEARCH_NATIVE_X86, BSEARCH_NATIVE_ARM64,
and BSEARCH_I686_SYSROOT must be valid when explicitly supplied. Every invocation
compiles anew; SIGSYS is a failure. No kernel build or module is produced.
"""
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest
from unittest import mock

import glob_runtime_fixtures as transport
from check_glob_kernel import verify_flag_policy
from check_reciprocal_kernel import module_elf
from rust_exports_test_support import read_exports

HERE = Path(__file__).resolve().parent
ROOT = Path(os.environ.get('BSEARCH_SOURCE', str(HERE.parents[1]))).resolve()
OVERLAY = Path(os.environ.get('BSEARCH_OVERLAY', str(ROOT))).resolve()
RUST = str(Path(os.environ.get('BSEARCH_RUSTC', shutil.which('rustc') or 'missing-rustc')).absolute())
BINDGEN = str(Path(os.environ.get('BSEARCH_BINDGEN', shutil.which('bindgen') or shutil.which('bindgen-0.71') or 'missing-bindgen')).absolute())
INPUTS = {
    'BSEARCH_NATIVE_X86': ['lib/.list_sort_rust.o.cmd', 'lib/.scatterlist.o.cmd',
        'rust/libcore.rmeta', 'rust/libbindings.rmeta', 'include/generated/rustc_cfg',
        'scripts/gendwarfksyms/gendwarfksyms'],
    'BSEARCH_NATIVE_ARM64': ['lib/.list_sort_rust.o.cmd', 'lib/.scatterlist.o.cmd',
        'rust/libcore.rmeta', 'rust/libbindings.rmeta', 'include/generated/rustc_cfg',
        'scripts/gendwarfksyms/gendwarfksyms'],
    'BSEARCH_I686_SYSROOT': ['lib/rustlib/i686-unknown-linux-gnu/lib'],
}


def validate():
    result = {}
    # Validate all explicit inputs before creating outputs or skipping anything.
    for name, files in INPUTS.items():
        if name not in os.environ:
            result[name] = None
            continue
        if not os.environ[name].strip():
            raise ValueError(name + ' explicitly empty')
        path = Path(os.environ[name]).resolve()
        for filename in files:
            if not (path / filename).exists():
                raise ValueError(name + ' missing ' + filename)
        if name.endswith('SYSROOT'):
            for library in ('libcore', 'libcompiler_builtins'):
                matches = {item.resolve() for pattern in (library+'.rlib', library+'-*.rlib')
                           for item in (path / files[0]).glob(pattern) if item.is_file()}
                if len(matches) != 1: raise ValueError(name + ' missing or ambiguous ' + library)
        result[name] = path
    for name in ('BSEARCH_SOURCE', 'BSEARCH_OVERLAY', 'BSEARCH_OUT', 'BSEARCH_RUSTC', 'BSEARCH_BINDGEN'):
        if name in os.environ and not os.environ[name].strip():
            raise ValueError(name + ' explicitly empty')
    for base, names in ((ROOT, ('lib/bsearch.c', 'include/linux/bsearch.h', 'include/linux/types.h',
                               'rust/ffi.rs', 'rust/bindings/lib.rs', 'rust/bindgen_parameters', 'scripts/Makefile.build')),
                        (OVERLAY, ('lib/bsearch.rs', 'lib/bsearch_rust.rs', 'include/linux/bsearch_header.rs',
                                   'rust/bindings/bsearch.rs', 'lib/Makefile', 'lib/Kconfig'))):
        for name in names:
            if not (base / name).is_file(): raise ValueError('invalid source input: ' + str(base / name))
    for native_name in ('BSEARCH_NATIVE_X86', 'BSEARCH_NATIVE_ARM64'):
        native = result[native_name]
        if native is None: continue
        for name in ('rust/.bindings.o.cmd', 'rust/bindings/.bindings_generated.rs.cmd',
                     'rust/bindings/bindings_helpers_generated.rs', 'include/config/auto.conf',
                     'scripts/basic/fixdep', 'scripts/kconfig/conf', '.config'):
            if not (native / name).is_file(): raise ValueError('invalid native input: ' + str(native / name))
        cfg = (native / '.config').read_text()
        for option in ('RUST', 'CFI', 'MODVERSIONS', 'GENDWARFKSYMS', 'X86_64' if native_name.endswith('X86') else 'ARM64'):
            if 'CONFIG_' + option + '=y\n' not in cfg:
                raise ValueError('wrong native configuration: ' + option)
        # Expand and validate all responses/policy before any output creation.
        validate_paths(rust_native(native, 'lib/.list_sort_rust.o.cmd', native)[1:])
        validate_paths(rust_native(native, 'rust/.bindings.o.cmd', native)[1:])
        validate_paths(c_native(native, native)[1:])
        command = saved(native, 'rust/bindings/.bindings_generated.rs.cmd')
        if executable(command[0], native) != str(Path(BINDGEN).resolve()):
            raise ValueError('native bindgen differs from selected compiler')
        validate_paths(relocate(command[command.index('--')+1:], native, native))
    for tool in (RUST, BINDGEN, 'clang', 'ld.lld', 'llvm-ar', 'readelf', 'nm', 'make', 'llvm-objcopy'):
        if shutil.which(tool) is None: raise ValueError('missing executable: ' + tool)
    return result


def executable(value, build):
    path = str(build / value) if '/' in value and not Path(value).is_absolute() else value
    found = shutil.which(path)
    if found is None: raise ValueError('missing native executable: ' + path)
    return str(Path(found).resolve())


def validate_paths(flags):
    """Expanded native paths must exist before allocating compiler outputs."""
    options = ('--target', '--extern', '--sysroot', '-include', '-imacros', '-include-pch',
               '-isysroot', '-isystem', '-iquote', '-idirafter', '-I', '-L')
    iterator = iter(flags)
    for flag in iterator:
        key, value = None, None
        if flag in options:
            key, value = flag, next(iterator)
        elif flag.startswith(('--target=', '--extern=', '--sysroot=')):
            key, value = flag.split('=', 1)
        else:
            for option in sorted(options[3:], key=len, reverse=True):
                if flag.startswith(option) and flag != option:
                    key, value = option, flag[len(option):]; break
        if key is None: continue
        if key == '--target' and '/' not in value and not value.endswith('.json'): continue
        if key == '--extern':
            if '=' not in value: continue
            value = value.split('=', 1)[1]
        if key == '-L' and '=' in value: value = value.split('=', 1)[1]
        if value.startswith(('=', '$SYSROOT')): continue
        if not Path(value).is_absolute() or not Path(value).exists():
            raise ValueError('absent or unrelocated native input: ' + value)


def ids(path):
    source = path.read_text()
    meta = {k: int(v) & 0xffffffff for k, v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}', source, re.M)}
    return {n: meta[k] for n, k in re.findall(r'^define [^\n]*?@([^ (]+)\([^\n]*!kcfi_type !(\d+)', source, re.M)}


def saved(build, filename):
    line = (build / filename).read_text().splitlines()[0].split(' := ', 1)[1].split(' ; ', 1)[0]
    return shlex.split(line.replace('$(pound)', '#'))


def relocate(args, build, folder):
    """C/bindgen inputs retain the real donor cwd semantics, never its outputs."""
    return transport.native_flags(args, build, 'c')


def rust_native(build, filename, folder):
    args = saved(build, filename)
    while '=' in args[0] and not args[0].startswith('-'):
        args.pop(0)
    compiler = args.pop(0)
    flags = []
    skip = False
    for arg in args:
        if skip:
            skip = False
        elif arg in ('--out-dir', '--crate-name'):
            skip = True
        elif arg.startswith(('--emit=', '--crate-name=')) or arg.endswith('.rs'):
            continue
        else:
            flags.append(arg)
    flags = transport.native_flags(flags, build, 'rust')
    verify_flag_policy(flags, 'rust')
    if executable(compiler, build) != str(Path(RUST).resolve()):
        raise ValueError('native rustc differs from selected compiler')
    return [RUST, *flags, '--out-dir', str(folder)]


def c_native(build, folder):
    args = saved(build, 'lib/.scatterlist.o.cmd')
    flags = []
    skip = False
    for arg in args:
        if skip:
            skip = False
        elif arg == '-o':
            skip = True
        elif arg == '-c' or arg.endswith('/lib/scatterlist.c') or arg.startswith('-Wp,-MMD,'):
            continue
        elif arg.startswith(('-DKBUILD_', '-D__KBUILD_')):
            flags.append(arg.replace('scatterlist', 'bsearch'))
        else:
            flags.append(arg)
    flags = [executable(flags[0], build), *relocate(flags[1:], build, folder)]
    verify_flag_policy(flags[1:], 'c')
    return flags


class BsearchTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.inputs = validate()
        cls.protected = [ROOT, OVERLAY, *(path for path in cls.inputs.values() if path is not None)]
        parent = Path(os.environ.get('BSEARCH_OUT', os.environ.get('TMPDIR', '/tmp'))).resolve()
        if any(parent.is_relative_to(path) for path in cls.protected):
            raise ValueError('BSEARCH_OUT must be outside source/native/sysroot inputs')
        if not parent.is_dir(): raise ValueError('BSEARCH_OUT must be an existing private output directory')
        cls.work = Path(tempfile.mkdtemp(prefix='run-', dir=parent))
        cls.env = {**os.environ, 'RUSTC_BOOTSTRAP': '1', 'RUST_MODFILE': 'lib/bsearch'}
        cls.env = transport.compiler_environment(cls.work, cls.env)
        version = subprocess.run([RUST, '--version'], cwd=cls.work, env=cls.env, check=True, capture_output=True, text=True).stdout
        match = re.fullmatch(r'rustc (\d+)\.(\d+)\.(\d+) .*\n?', version)
        if not match or tuple(map(int, match.groups())) < (1, 85, 0): raise ValueError('Rust1.85 or newer required')
        (cls.work / 'inputs.json').write_text(json.dumps({'root': str(ROOT), 'overlay': str(OVERLAY),
            'inputs': {key: str(value) if value is not None else None for key, value in cls.inputs.items()},
            'rustc': RUST, 'bindgen': BINDGEN, 'version': version}, indent=2) + '\n')
        sources = [Path(__file__), HERE / 'fixtures/bsearch_consumer.rs', HERE / 'fixtures/bsearch_driver.c',
            *(OVERLAY / name for name in ('lib/bsearch.rs', 'lib/bsearch_rust.rs', 'include/linux/bsearch_header.rs',
                                          'rust/bindings/bsearch.rs', 'lib/Makefile', 'lib/Kconfig')),
            *(ROOT / name for name in ('lib/bsearch.c', 'include/linux/bsearch.h', 'include/linux/types.h',
                                      'rust/bindgen_parameters', 'rust/bindings/lib.rs', 'rust/bindings/bindings_helper.h'))]
        (cls.work / 'source-hashes.json').write_text(json.dumps({str(path): hashlib.sha256(path.read_bytes()).hexdigest()
                                                               for path in sources}, indent=2)+'\n')
        shutil.copyfile(__file__, cls.work / 'test_bsearch.executed.py')
        print('bsearch evidence:', cls.work, flush=True)

    def run_command(self, args, folder, expected=0, stdin=None, environment=None):
        args = list(map(str, args))
        folder = Path(folder).resolve()
        if not folder.is_relative_to(self.work) or any(folder.is_relative_to(path) for path in self.protected):
            raise ValueError('command cwd must be private, outside all inputs')
        scratch = Path(tempfile.mkdtemp(prefix='command-', dir=self.work))
        environment = transport.compiler_environment(scratch, {**self.env, **(environment or {})})
        with (self.work / 'commands.log').open('a') as index:
            index.write(str(scratch / 'command.log') + '\n')
        with (scratch / 'command.log').open('w') as log:
            log.write('$ ' + shlex.join(args) + '\ncwd=' + str(folder) + '\ntmp=' + environment['TMPDIR'] + '\n')
            log.flush()
            try:
                result = subprocess.run(args, cwd=folder, env=environment, input=stdin,
                    text=True, capture_output=True, timeout=180,
                    preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
            except (OSError, subprocess.TimeoutExpired) as error:
                log.write('LAUNCH/TIMEOUT ' + repr(error) + '\n')
                raise
            log.write(result.stdout + result.stderr + '\nexit=' + str(result.returncode) + '\n')
        if expected is not None:
            self.assertEqual(result.returncode, expected, (args, result.stderr, self.work))
        return result

    def fixture(self, folder, target):
        inc = folder / 'include/linux'
        inc.mkdir(parents=True)
        original = (ROOT / 'include/linux/types.h').read_text()
        typedef = re.search(r'^typedef .*\(\*cmp_func_t\).*?;', original, re.M)[0]
        (inc / 'types.h').write_text('#ifndef FIXTURE_TYPES\n#define FIXTURE_TYPES\n'
            'typedef __SIZE_TYPE__ size_t;\n#define NULL ((void *)0)\n'
            '#define __always_inline inline __attribute__((always_inline))\n' + typedef + '\n#endif\n')
        (inc / 'export.h').write_text('#define EXPORT_SYMBOL(x)\n')
        (inc / 'kprobes.h').write_text('#define NOKPROBE_SYMBOL(x)\n')
        self.run_command([BINDGEN, ROOT / 'include/linux/bsearch.h', '--use-core',
            '--rust-target=1.85', '--ctypes-prefix=ffi', '--no-layout-tests',
            '--blocklist-function=^bsearch$', '--allowlist-type=cmp_func_t',
            '-o', folder / 'generated.rs', '--', *target, '-I'+str(inc.parent)], folder)
        self.assertIn('Option<', (folder / 'generated.rs').read_text())
        (folder / 'bindings.rs').write_text('//! Actual bindgen alias and targeted public declaration.\n'
            '#![no_std]\n#![feature(cfi_encoding)]\n#![allow(non_camel_case_types,missing_docs)]\n'
            'include!("generated.rs");\n#[path='+json.dumps(str(OVERLAY / 'rust/bindings/bsearch.rs'))+
            '] mod api;\npub use api::*;\n')

    def matrix(self, bits, modes=None):
        execution_failures = []
        compilation_failures = []
        sysroot = self.inputs['BSEARCH_I686_SYSROOT']
        if bits == 32 and sysroot is None:
            self.skipTest('BSEARCH_I686_SYSROOT absent; genuine ELF32 untested')
        for opt in ('0', '2', 's'):
            for protected in (modes if modes is not None else ((False, True) if bits == 64 else (False,))):
                folder = self.work / f'elf{bits}-O{opt}-{protected}'
                folder.mkdir()
                ct = ['-m32'] if bits == 32 else []
                rt = ['--target=i686-unknown-linux-gnu', '--sysroot='+str(sysroot)] if bits == 32 else []
                self.fixture(folder, ct)
                c = ['clang', *ct, '-O'+opt, '-g', '-fno-builtin', '-fno-pie',
                     '-ffunction-sections', '-fdata-sections', '-I'+str(folder / 'include'),
                     '-I'+str(ROOT / 'include')]
                r = [RUST, '--edition=2021', '-Dwarnings', '-Dunsafe-op-in-unsafe-fn',
                     '-Wmissing-docs', '-Wrust-2018-idioms', '-Wunreachable-pub',
                     '-Cpanic=abort', '-Coverflow-checks=y', '-Copt-level='+opt, '-Cdebuginfo=2',
                     '-Crelocation-model=static', '--out-dir', folder, *rt]
                if protected:
                    c += ['-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers']
                    r += ['-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                result = self.run_command([*r, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs'], folder, None)
                if result.returncode:
                    compilation_failures.append((str(folder), result.stderr))
                    continue
                r += ['--extern', 'ffi='+str(folder / 'libffi.rlib'), '-Ldependency='+str(folder)]
                self.run_command([*r, '--crate-name=bindings', '--crate-type=rlib', folder / 'bindings.rs'], folder)
                r += ['--extern', 'bindings='+str(folder / 'libbindings.rlib')]
                # Compile the complete candidate, not an algorithm rewritten for testing.
                for name, source in [('owner', OVERLAY / 'lib/bsearch.rs'),
                                     ('consumer', HERE / 'fixtures/bsearch_consumer.rs')]:
                    self.run_command([*r, '-Zcrate-attr=no_std', '--crate-name='+name, '--crate-type=rlib',
                        '--emit=obj='+str(folder / (name+'.o'))+',llvm-ir='+str(folder / (name+'.ll')),
                        source], folder) if name == 'owner' else self.run_command([
                        *r, '--crate-name='+name, '--crate-type=rlib',
                        '--emit=obj='+str(folder / (name+'.o'))+',llvm-ir='+str(folder / (name+'.ll')), source], folder)
                self.run_command([*c, '-Dbsearch=original_bsearch', '-c', ROOT / 'lib/bsearch.c',
                    '-o', folder / 'original.o'], folder)
                (folder / 'discard.lds').write_text('SECTIONS { /DISCARD/ : { *(.eh_frame*) '
                    '*(.gcc_except_table*) *(.data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n')
                # Genuine core for any reachable panic; never a dummy personality or allocator.
                (folder / 'panic.rs').write_text('//! Abort transport.\n#![no_std]\n'
                    'unsafe extern "C" { fn abort() -> !; }\n'
                    '#[panic_handler] fn panic(_: &core::panic::PanicInfo<\'_>) -> ! { unsafe { abort() } }\n')
                self.run_command([*r, '--crate-name=panic', '--crate-type=staticlib', folder / 'panic.rs'], folder)
                extra = []
                if bits == 32:
                    (folder / 'start.c').write_text('void abort(void){__asm__ volatile("int $0x80"::"a"(1),"b"(94));__builtin_unreachable();}\n'
                        'extern int main(int,char**);\n__attribute__((naked)) void _start(void){__asm__("mov (%esp),%eax; lea 4(%esp),%ecx; push %ecx; push %eax; call main; mov %eax,%ebx; mov $1,%eax; int $0x80");}\n')
                    extra = ['-nostdlib', '-static', '-Wl,-e,_start', folder / 'start.c']
                binary = folder / 'run'
                self.run_command([*c, '-no-pie', '-Wl,--gc-sections', '-Wl,-T,'+str(folder / 'discard.lds'),
                    *extra, HERE / 'fixtures/bsearch_driver.c', folder / 'owner.o', folder / 'consumer.o',
                    folder / 'original.o', folder / 'libbindings.rlib', folder / 'libpanic.a', '-o', binary], folder)
                self.assertEqual(binary.read_bytes()[:5], b'\x7fELF'+bytes([1 if bits == 32 else 2]))
                for arguments, expected in [([], 95), (['good'], 0)] + [
                    ([mode], -signal.SIGILL if protected else 0) for mode in ('bad', 'context', 'signature')]:
                    result = self.run_command([binary, *arguments], folder, None)
                    if result.returncode != expected:
                        execution_failures.append((str(binary), arguments, result.returncode, expected))
                if protected:
                    self.assertIn('"kcfi"(i32 ', (folder / 'owner.ll').read_text())
                    self.assertIn('"kcfi"(i32 ', (folder / 'consumer.ll').read_text())
        (self.work / f'elf{bits}-failures.json').write_text(json.dumps({
            'compile': compilation_failures, 'execute': execution_failures}, indent=2)+'\n')
        self.assertEqual((compilation_failures, execution_failures), ([], []),
            'Retained binaries require real execution; unsupported KCFI/SIGSYS never passes')

    def test_original_c_encoding_grammar(self):
        report = {}
        encoding = re.search(r'#\[cfi_encoding\s*=\s*"([^"]+)"\]',
                             (OVERLAY / 'rust/bindings/bsearch.rs').read_text())[1]
        for target in ('x86_64-linux-gnu', 'i686-linux-gnu', 'aarch64-linux-gnu',
                       'arm-linux-gnueabi', 'riscv32-linux-gnu', 'riscv64-linux-gnu',
                       'loongarch64-linux-gnu', 's390x-linux-gnu', 'powerpc-linux-gnu',
                       'powerpc64-linux-gnu'):
            folder = self.work / target
            folder.mkdir()
            self.fixture(folder, ['--target='+target])
            for normalized in (False, True):
                out = folder / ('normalized.ll' if normalized else 'ordinary.ll')
                compiled = self.run_command(['clang', '--target='+target, '-flto', '-fsanitize=cfi-icall',
                    *(['-fsanitize-cfi-icall-experimental-normalize-integers'] if normalized else []),
                    '-I'+str(folder / 'include'), '-I'+str(ROOT / 'include'), '-S', '-emit-llvm',
                    ROOT / 'lib/bsearch.c', '-o', out], folder, None)
                if compiled.returncode:
                    self.assertIn("unsupported option '-fsanitize=cfi-icall' for target", compiled.stderr)
                    report[target+':'+str(normalized)] = {'unsupported': compiled.stderr}
                    continue  # explicit compiler capability, not positive ABI proof
                grammars = re.findall(r'!"(_ZTSFPvPKvS1_[^"]+)"', out.read_text())
                self.assertTrue(grammars, (target, normalized))
                suffix = encoding+'E.normalized' if normalized else 'PFiS1_S1_EE'
                self.assertTrue(any(value.endswith(suffix) for value in grammars), grammars)
                report[target+':'+str(normalized)] = grammars
        (self.work / 'c-grammar.json').write_text(json.dumps(report, indent=2)+'\n')

    def test_elf64_differential_and_protected_calls(self):
        self.matrix(64)

    def test_elf32_original_c_differential(self):
        self.matrix(32)

    def test_i686_kcfi_capability_is_explicit(self):
        sysroot = self.inputs['BSEARCH_I686_SYSROOT']
        if sysroot is None: self.skipTest('no BSEARCH_I686_SYSROOT; i686 KCFI capability unproven')
        folder = self.work / 'i686-cfi-capability'; folder.mkdir()
        result = self.run_command([RUST, '--edition=2021', '-Dwarnings', '--crate-type=rlib',
            '--target=i686-unknown-linux-gnu', '--sysroot=' + str(sysroot), '-Zsanitizer=kcfi',
            '-Zsanitizer-cfi-normalize-integers', '--out-dir=' + str(folder),
            '--emit=obj=' + str(folder / 'ffi.o'), ROOT / 'rust/ffi.rs'], folder, None)
        if result.returncode == 0:
            self.matrix(32, modes=(True,))
        else:
            self.assertIn('kcfi sanitizer is not supported for this target', result.stderr)
            (folder / 'UNSUPPORTED.txt').write_text('Actual compiler rejection; no i686 KCFI positive proof.\n' + result.stderr)

    def native(self, name):
        build = self.inputs[name]
        if build is None:
            self.skipTest(name+' absent; native object checks not run')
        folder = Path(tempfile.mkdtemp(prefix=name + '-', dir=self.work))
        watch = transport.NativeWriteWatch(build)
        try:
            with watch:
                self._native(folder, build)
        finally:
            (folder / 'donor-observation.json').write_text(json.dumps({
                'root': str(build), 'directories': len(watch.paths), 'events': watch.events}, indent=2) + '\n')
        self.assertEqual(watch.events, [], 'native donor write detected')

    def _native(self, folder, build):
        r = rust_native(build, 'lib/.list_sort_rust.o.cmd', folder)
        br = rust_native(build, 'rust/.bindings.o.cmd', folder)
        c = c_native(build, folder)
        # Regenerate full actual bindings with only the requested function blocklist.
        command = saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        split = command.index('--')
        # Use current authoritative binding parameters/modules with saved native
        # compiler target flags. New unrelated manual bindings must not be paired
        # with stale generated public declarations from an older donor command.
        parameters = (ROOT / 'rust/bindgen_parameters').read_text()
        parameter_flags = shlex.split('\n'.join(line for line in parameters.splitlines()
                                               if line and not line.startswith('#')))
        common = command.index('--rust-target')
        options = [BINDGEN, str((build / command[1]).resolve()), *parameter_flags, *command[common:split]]
        index = options.index('-o')
        generated = folder / 'rust/bindings'
        generated.mkdir(parents=True)
        options[index+1] = str(generated / 'bindings_generated.rs')
        self.run_command([*options, '--blocklist-function=^bsearch$', '--',
            *relocate(command[split+1:], build, folder)], folder)
        path = generated / 'bindings_generated.rs'
        path.write_text(re.sub(r'pub const RUST_CONST_HELPER_([a-zA-Z0-9_]*)', r'pub const \1', path.read_text()))
        self.assertNotIn('pub fn bsearch(', path.read_text())
        self.assertIn('pub type cmp_func_t =', path.read_text())
        shutil.copyfile(build / 'rust/bindings/bindings_helpers_generated.rs', generated / 'bindings_helpers_generated.rs')
        source = folder / 'bindings'
        shutil.copytree(ROOT / 'rust/bindings', source, symlinks=False)
        shutil.copyfile(OVERLAY / 'rust/bindings/bsearch.rs', source / 'bsearch.rs')
        binding_source = (source / 'lib.rs').read_text()
        if 'mod bsearch;' not in binding_source:
            binding_source += '\nmod bsearch;\npub use bsearch::{bsearch, BsearchCmp};\n'
        (source / 'lib.rs').write_text(binding_source)
        (folder / 'bindgen_parameters').write_text(parameters)
        self.run_command([*br, '--crate-name=bindings', '--emit=metadata='+str(folder / 'libbindings.rmeta')+
            ',obj='+str(folder / 'bindings.o'), source / 'lib.rs'], folder,
            environment={'OBJTREE': str(folder)})
        # Native owner uses the actual rebuilt binding crate, not a copied nominal type.
        r += ['--extern', 'bindings='+str(folder / 'libbindings.rmeta')]
        tree = folder / 'source'
        for filename in ('lib/bsearch.rs', 'lib/bsearch_rust.rs', 'include/linux/bsearch_header.rs',
                         'rust/ffi_export.rs', 'include/linux/export_header.rs'):
            destination = tree / filename
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile((OVERLAY if (OVERLAY / filename).exists() else ROOT) / filename, destination)
        report = {}
        callback = folder / 'callback.c'
        callback.write_text('#include <linux/bsearch.h>\nint native_compare(const void *, const void *);\n'
                            'int native_compare(const void *a, const void *b) { return a == b; }\n')
        self.run_command([*c, '-S', '-emit-llvm', callback, '-o', folder / 'callback.ll'], folder)
        callback_id = ids(folder / 'callback.ll')['native_compare']
        for opt in ('0', '2', 's'):
            d = folder / ('O'+opt)
            d.mkdir()
            self.run_command([*r, '-Copt-level='+opt, '--crate-name=bsearch_rust',
                '--emit=obj='+str(d / 'rust.o')+',llvm-ir='+str(d / 'rust.ll')+',asm='+str(d / 'rust.s')+
                ',dep-info='+str(d / 'rust.d'), tree / 'lib/bsearch_rust.rs'], folder)
            self.run_command([*c, '-O'+opt, '-c', ROOT / 'lib/bsearch.c', '-o', d / 'c.o'], folder)
            self.run_command([*c, '-O'+opt, '-S', '-emit-llvm', ROOT / 'lib/bsearch.c', '-o', d / 'c.ll'], folder)
            self.assertEqual(ids(d / 'c.ll')['bsearch'], ids(d / 'rust.ll')['bsearch'])
            body = re.search(r'^define [^\n]*@bsearch\([^\n]*\n.*?^}',
                             (d / 'rust.ll').read_text(), re.M | re.S)[0]
            # At O0 the inline body is a separate function; inspect that exact
            # body as well, never accept any unrelated nonzero callback ID.
            if not re.search(r'"kcfi"\(i32 (-?\d+)\)', body):
                body = re.search(r'^define [^\n]*__inline_bsearch[^\n]*\n.*?^}',
                                 (d / 'rust.ll').read_text(), re.M | re.S)[0]
            self.assertEqual({int(value) & 0xffffffff for value in re.findall(r'"kcfi"\(i32 (-?\d+)\)', body)},
                             {callback_id})
            self.assertIn('bsearch_header.rs', (d / 'rust.d').read_text())
            self.assertEqual(read_exports(d / 'c.o'), read_exports(d / 'rust.o'))
            exports = read_exports(d / 'rust.o')
            self.assertEqual(len(exports), 1)
            self.assertEqual((exports[0]['name'], exports[0]['license'], exports[0]['namespace'],
                              exports[0]['relocation_target'], exports[0]['relocation_addend']),
                             ('bsearch', '', '', 'bsearch', 0))
            self.assertEqual(self.blacklist(d / 'c.o'), self.blacklist(d / 'rust.o'))
            expected_blacklist = 'CONFIG_KPROBES=y\n' in (build / '.config').read_text()
            self.assertEqual(self.blacklist(d / 'rust.o') is not None, expected_blacklist)
            for language in ('c', 'rust'):
                elf = (d / (language+'.o')).read_bytes()
                self.assertEqual(elf[:6], b'\x7fELF\x02\x01')
                self.assertEqual(int.from_bytes(elf[18:20], 'little'),
                                 62 if 'CONFIG_X86_64=y\n' in (build / '.config').read_text() else 183)
                dump = self.run_command([build / 'scripts/gendwarfksyms/gendwarfksyms',
                    '--dump-versions', '-T', d / (language+'.symtypes'), d / (language+'.o')], folder,
                    stdin='bsearch\n')
                (d / (language+'-versions.log')).write_text(dump.stdout+dump.stderr)
                self.assertRegex(dump.stdout, r'^#SYMVER bsearch 0x[0-9a-f]{8}\n$')
                self.abi((d / (language+'.symtypes')).read_text(), language)
                self.run_command(['readelf', '-WSr', d / (language+'.o')], folder)
            report[opt] = {'kcfi': ids(d / 'rust.ll')['bsearch'], 'callback_kcfi': callback_id,
                'c_crc': (d / 'c-versions.log').read_text(),
                'rust_crc': (d / 'rust-versions.log').read_text()}
        (folder / 'summary.json').write_text(json.dumps(report, indent=2)+'\n')
        self.native_negatives(folder, build, r, br, c, source, tree, callback_id)
        self.kbuild(folder, build)

    def abi(self, text, language):
        first = text.splitlines()[0]
        self.assertNotIn('unspecified_parameters', text)
        if language == 'c':
            self.assertEqual(first, 'bsearch subprogram ( formal_parameter pointer_type { const_type { base_type void } } key , '
                'formal_parameter pointer_type { const_type { base_type void } } base , formal_parameter t#size_t num , '
                'formal_parameter t#size_t size , formal_parameter t#cmp_func_t cmp ) -> pointer_type { base_type void }')
            self.assertIn('base_type unsigned long byte_size(8) encoding(7)', text)
            self.assertIn('subroutine_type ( formal_parameter pointer_type { const_type { base_type void } } , '
                'formal_parameter pointer_type { const_type { base_type void } } ) -> base_type int byte_size(4) encoding(5)', text)
        else:
            self.assertEqual(first, 'bsearch subprogram ( formal_parameter pointer_type *const core::ffi::c_void { e#core::ffi::c_void } key , '
                'formal_parameter pointer_type *const core::ffi::c_void { e#core::ffi::c_void } base , '
                'formal_parameter base_type usize byte_size(8) encoding(7) num , '
                'formal_parameter base_type usize byte_size(8) encoding(7) size , '
                'formal_parameter s#bindings::bsearch::BsearchCmp cmp ) -> pointer_type *mut core::ffi::c_void { e#core::ffi::c_void }')
            self.assertIn('subroutine_type ( formal_parameter pointer_type *const core::ffi::c_void { e#core::ffi::c_void } , '
                'formal_parameter pointer_type *const core::ffi::c_void { e#core::ffi::c_void } ) -> base_type i32 byte_size(4) encoding(5)', text)
            self.assertRegex(text, r'None.*discr_value\(0\)')
            self.assertRegex(text, r's#bindings::bsearch::BsearchCmp structure_type .* byte_size\(8\) alignment\(8\)')

    def native_negatives(self, folder, build, r, br, c, source, tree, callback_id):
        negative = folder / 'negative'; negative.mkdir()
        wrong = negative / 'callback.c'
        wrong.write_text((folder / 'callback.c').read_text().replace('int native_compare', 'unsigned int native_compare'))
        self.run_command([*c, '-S', '-emit-llvm', wrong, '-o', negative / 'callback.ll'], folder)
        self.assertNotEqual(ids(negative / 'callback.ll')['native_compare'], callback_id)
        disabled = [flag for flag in r if flag not in ('-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers')]
        self.run_command([*disabled, '--crate-name=bsearch_rust', '--emit=llvm-ir='+str(negative / 'disabled.ll'),
                          tree / 'lib/bsearch_rust.rs'], folder)
        self.assertNotIn('bsearch', ids(negative / 'disabled.ll'))
        self.assertNotIn('"kcfi"(i32 ', (negative / 'disabled.ll').read_text())
        bad_source = negative / 'bindings'; shutil.copytree(source, bad_source)
        binding = bad_source / 'bsearch.rs'
        text = binding.read_text()
        self.assertEqual(text.count('PFu3i32S1_S1_E'), 1)
        binding.write_text(text.replace('PFu3i32S1_S1_E', 'PFu3i32PKvS0_E'))
        self.run_command([*br, '--crate-name=bindings', '--emit=metadata='+str(negative / 'libbindings.rmeta')+
                          ',obj='+str(negative / 'bindings.o'),
                          bad_source / 'lib.rs'], folder, environment={'OBJTREE': str(folder)})
        wrong_flags = [flag.replace(str(folder / 'libbindings.rmeta'), str(negative / 'libbindings.rmeta')) for flag in r]
        self.run_command([*wrong_flags, '--crate-name=bsearch_rust', '--emit=llvm-ir='+str(negative / 'wrong-context.ll'),
                          tree / 'lib/bsearch_rust.rs'], folder)
        self.assertNotEqual(ids(negative / 'wrong-context.ll')['bsearch'], ids(folder / 'O2/c.ll')['bsearch'])
        # Genuine configuration-off objects, not an invented blacklist record.
        no_kprobe = []; iterator = iter(r)
        for flag in iterator:
            if flag == '--cfg':
                value = next(iterator)
                if value != 'CONFIG_KPROBES': no_kprobe += [flag, value]
            elif flag != '--cfg=CONFIG_KPROBES': no_kprobe.append(flag)
        self.run_command([*no_kprobe, '--crate-name=bsearch_rust', '--emit=obj='+str(negative / 'rust-no-kprobe.o'),
                          tree / 'lib/bsearch_rust.rs'], folder)
        original = negative / 'c-no-kprobe.c'
        original.write_text('#include <linux/kconfig.h>\n#undef CONFIG_KPROBES\n#include '+
                            json.dumps(str(ROOT / 'lib/bsearch.c'))+'\n')
        self.run_command([*c, '-c', original, '-o', negative / 'c-no-kprobe.o'], folder)
        self.assertIsNone(self.blacklist(negative / 'c-no-kprobe.o'))
        self.assertIsNone(self.blacklist(negative / 'rust-no-kprobe.o'))
        self.run_command([*no_kprobe, '--cfg=CONFIG_KPROBES', '--crate-name=bsearch_rust',
                          '--emit=obj='+str(negative / 'rust-kprobe.o'), tree / 'lib/bsearch_rust.rs'], folder)
        original = negative / 'c-kprobe.c'
        original.write_text('#include <linux/kconfig.h>\n#undef CONFIG_KPROBES\n#define CONFIG_KPROBES 1\n#include '+
                            json.dumps(str(ROOT / 'lib/bsearch.c'))+'\n')
        self.run_command([*c, '-c', original, '-o', negative / 'c-kprobe.o'], folder)
        self.assertIsNotNone(self.blacklist(negative / 'rust-kprobe.o'))
        self.assertEqual(self.blacklist(negative / 'c-kprobe.o'), self.blacklist(negative / 'rust-kprobe.o'))
        mutated = negative / 'wrong-blacklist.o'
        self.run_command(['llvm-objcopy', '--redefine-sym', 'bsearch=wrong_target', negative / 'rust-kprobe.o', mutated], folder)
        with self.assertRaises(AssertionError): self.blacklist(mutated)

    def kbuild(self, folder, native):
        fixture = folder / 'kbuild-source'
        build = folder / 'kbuild-output'
        for name in ('lib/bsearch.rs', 'lib/bsearch_rust.rs', 'include/linux/bsearch_header.rs',
                     'rust/ffi_export.rs', 'include/linux/export_header.rs', 'lib/bsearch.c'):
            destination = fixture / name; destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile((OVERLAY if (OVERLAY / name).exists() else ROOT) / name, destination)
        makefile = (OVERLAY / 'lib/Makefile').read_text()
        self.assertIn('RUSTFLAGS_bsearch_rust.o += --extern bindings', makefile)
        (fixture / 'lib/Makefile').write_text(makefile + '\n'
            'bsearch-order := $(obj-y)\n'
            'obj-y := $(filter bsearch.o bsearch_rust.o,$(obj-y))\nobj-m :=\nlib-y :=\n'
            '$(file >$(objtree)/selected-order.txt,$(bsearch-order))\n')
        (build / 'lib').mkdir(parents=True)
        (build / 'rust').mkdir()
        shutil.copytree(native / 'include', build / 'include', symlinks=False)
        for name in ('scripts/basic/fixdep', 'scripts/gendwarfksyms/gendwarfksyms'):
            (build / name).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(native / name, build / name)
        words = saved(native, 'lib/.list_sort_rust.o.cmd')
        while '=' in words[0] and not words[0].startswith('-'): words.pop(0)
        first_generated = next(i for i, flag in enumerate(words) if flag.startswith('-Zallow-features='))
        flags = transport.native_flags(words[1:first_generated], native, 'rust')
        verify_flag_policy(flags, 'rust')
        flags += ['-L' + str(native / 'rust')]
        response = build / 'rust-flags.rsp'; response.write_text('\n'.join(flags) + '\n')
        driver = build / 'driver.mk'
        driver.write_text('include ' + str(ROOT / 'scripts/Makefile.build') + '\n')
        cflags = [flag for flag in c_native(native, build)[1:] if 'KBUILD_' not in flag]
        command = ['make', '-rR', '-f', driver, 'srctree='+str(ROOT), 'VPATH='+str(fixture),
            'srcroot='+str(fixture), 'objtree='+str(build), 'obj=lib', 'V=1', 'need-builtin=1',
            'CONFIG_RUST=y', 'RUSTC='+RUST, 'RUSTC_OR_CLIPPY='+RUST,
            'KBUILD_RUSTFLAGS=@'+str(response),
            # Only remap this genuine freshly rebuilt dependency; a full root
            # build regenerates bindings at its normal path instead.
            'RUSTFLAGS_bsearch_rust.o=--extern bindings='+str(folder / 'libbindings.rmeta'),
            'rust_crate_features=arbitrary_self_types,asm_goto,generic_arg_infer,used_with_arg',
            'c_flags=-Wp,-MMD,$(depfile) '+shlex.join(cflags)+
            ' -DKBUILD_MODFILE=\'"lib/bsearch"\' -DKBUILD_MODNAME=\'"bsearch"\' -DKBUILD_BASENAME=\'"bsearch"\'',
            'CC=clang', 'LD=ld.lld', 'AR=llvm-ar', 'NM=llvm-nm', 'OBJCOPY=llvm-objcopy',
            'AWK=awk', 'READELF=llvm-readelf', 'CONFIG_SHELL=/bin/sh',
            'CONFIG_OBJTOOL=', 'CONFIG_FTRACE_MCOUNT_USE_RECORDMCOUNT=', 'KBUILD_BUILTIN=1']
        for selection in ('', 'y', '', 'y'):
            selected = 'bsearch_rust' if selection else 'bsearch'
            invocation = [*command, 'CONFIG_RUST_BSEARCH='+selection, 'lib/built-in.a']
            self.run_command(invocation, build)
            order = (build / 'selected-order.txt').read_text().split()
            at = order.index(selected+'.o')
            self.assertEqual(order[at-2:at+2], ['iov_iter.o', 'clz_ctz.o', selected+'.o', 'find_bit.o'])
            self.assertNotIn(('bsearch' if selection else 'bsearch_rust')+'.o', order)
            archive = self.run_command(['llvm-ar', 't', build / 'lib/built-in.a'], build).stdout.splitlines()
            self.assertEqual([Path(value).name for value in archive], [selected+'.o'])
            obj = build / ('lib/'+selected+'.o'); stamp = obj.stat().st_mtime_ns
            self.run_command(invocation, build)
            self.assertEqual(obj.stat().st_mtime_ns, stamp)
            saved_command = (build / ('lib/.'+selected+'.o.cmd')).read_text()
            self.assertIn('#SYMVER bsearch ', saved_command)
            self.assertEqual(read_exports(obj), read_exports(folder / 'O2' / ('rust.o' if selection else 'c.o')))
            self.assertEqual(self.blacklist(obj), self.blacklist(folder / 'O2' / ('rust.o' if selection else 'c.o')))
            if selection:
                self.assertEqual([word for word in saved(build, 'lib/.'+selected+'.o.cmd')
                                  if word.startswith('RUST_MODFILE=')], ['RUST_MODFILE=lib/bsearch'])
                recorded = {Path(line.strip().removesuffix('\\').strip()).resolve()
                            for line in saved_command.splitlines() if line.startswith('  /')}
                for dependency in ('lib/bsearch.rs', 'include/linux/bsearch_header.rs', 'include/linux/export_header.rs'):
                    self.assertIn((fixture / dependency).resolve(), recorded)
                    (fixture / dependency).touch()
                    self.run_command(invocation, build)
                    self.assertNotEqual(obj.stat().st_mtime_ns, stamp)
                    stamp = obj.stat().st_mtime_ns
        for selection in ('', 'y'):
            selected = 'bsearch_rust' if selection else 'bsearch'
            for suffix in ('s', 'll'):
                output = build / ('lib/'+selected+'.'+suffix)
                invocation = [*command, 'CONFIG_RUST_BSEARCH='+selection, 'lib/'+selected+'.'+suffix]
                self.run_command(invocation, build)
                source = fixture / ('lib/'+selected+('.rs' if selection else '.c'))
                saved_command = (output.parent / ('.'+output.name+'.cmd')).read_text()
                self.assertIn(' := '+str(source)+'\n', saved_command)
                stamp = output.stat().st_mtime_ns
                self.run_command(invocation, build)
                self.assertEqual(output.stat().st_mtime_ns, stamp)
                if selection:
                    dependency = fixture / 'include/linux/bsearch_header.rs'
                    recorded = {Path(line.strip().removesuffix('\\').strip()).resolve()
                                for line in saved_command.splitlines() if line.startswith('  /')}
                    self.assertIn(dependency.resolve(), recorded)
                    dependency.touch()
                    self.run_command(invocation, build)
                    self.assertNotEqual(output.stat().st_mtime_ns, stamp)

    def blacklist(self, obj):
        _, sections, _, _, _ = module_elf(obj)
        matches = [(index, section) for index, section in enumerate(sections) if section[0] == b'_kprobe_blacklist']
        if not matches: return None
        self.assertEqual(len(matches), 1)
        index, section = matches[0]
        self.assertEqual((section[1], section[2], section[4], section[7], section[9]), (1, 3, 8, 8, b'\0' * 8))
        relocations = [record for relocation in sections if relocation[1] == 4 and relocation[6] == index
                       for record in relocation[9]]
        self.assertEqual(len(relocations), 1)
        offset, kind, target, addend = relocations[0]
        self.assertEqual((offset, target[0], addend), (0, b'bsearch', 0))
        self.assertEqual(target[1] & 15, 2)  # genuine function, not copied numeric address
        return section[1:5], section[7:10], kind, target[0], addend

    def test_native_x86(self):
        self.native('BSEARCH_NATIVE_X86')

    def test_native_arm64(self):
        self.native('BSEARCH_NATIVE_ARM64')

    def test_concurrent_native_transport(self):
        names = [name for name in ('BSEARCH_NATIVE_X86', 'BSEARCH_NATIVE_ARM64') if self.inputs[name] is not None]
        if not names: self.skipTest('no native donors; concurrent native transport unproven')
        # Two complete independent invocations against each same read-only donor.
        with ThreadPoolExecutor(max_workers=2) as pool:
            list(pool.map(self.native, [name for name in names for _ in range(2)]))

    def test_transport_inputs_and_deleted_intermediates(self):
        folder = self.work / 'transport-controls'; folder.mkdir()
        donor = folder / 'synthetic-input'; donor.mkdir()
        (donor / 'inner.rsp').write_text('--target=./scripts/target.json\n-Ldependency=rust\n--extern\nkernel=rust/libkernel.rmeta\n')
        (donor / 'outer.rsp').write_text('@inner.rsp\n--out-dir=lib\n--emit=obj=lib/old.o\n-Dwarnings\n-Zsanitizer=kcfi\n')
        flags = transport.native_flags(['@outer.rsp', 'lib/old.rs'], donor, 'rust')
        self.assertEqual(flags, ['--target='+str(donor / 'scripts/target.json'), '-Ldependency='+str(donor / 'rust'),
            '--extern', 'kernel='+str(donor / 'rust/libkernel.rmeta'), '-Dwarnings', '-Zsanitizer=kcfi'])
        (donor / 'c.rsp').write_text('-I"include with space" -include include/config.h -o lib/old.o -Wp,-MMD,lib/old.d -Werror')
        self.assertEqual(transport.native_flags(['@c.rsp'], donor, 'c'),
            ['-I'+str(donor / 'include with space'), '-include', str(donor / 'include/config.h'), '-Werror'])
        (donor / 'recursive.rsp').write_text('@recursive.rsp\n')
        for flags in (['@missing'], ['@recursive.rsp'], ['--extern'], ['-Cincremental=lib/cache'], ['-C', 'incremental=lib/cache']):
            with self.subTest(flags=flags), self.assertRaises((ValueError, FileNotFoundError)):
                transport.native_flags(flags, donor, 'rust')
        for flags in (['--target='+str(donor / 'missing.json')], ['--extern', 'kernel='+str(donor / 'absent.rmeta')],
                      ['-Iinclude'], ['-Ldependency='+str(donor / 'absent')]):
            with self.subTest(flags=flags), self.assertRaises(ValueError): validate_paths(flags)
        def deleted(index):
            path = donor / f'synthetic-{index}.rcgu.o'
            path.write_bytes(b'observer control, not an ABI object'); path.unlink()
        with transport.NativeWriteWatch(donor) as control:
            with ThreadPoolExecutor(max_workers=2) as pool: list(pool.map(deleted, range(2)))
        for mask in (0x100, 0x200):
            self.assertTrue(any(int(value, 16) & mask for _, value in control.events))
        probe = folder / 'probe.rs'
        probe.write_text('//! Actual compiler intermediate control.\n#![no_std]\n#[no_mangle]\npub extern "C" fn probe(value:u32)->u32{value.wrapping_add(1)}\n')
        with transport.NativeWriteWatch(folder) as observed:
            self.run_command([RUST, '--edition=2021', '-Dwarnings', '--crate-type=rlib', '-Ccodegen-units=1',
                '--emit=obj='+str(folder / 'probe.o'), '--out-dir='+str(folder), probe], folder)
        (folder / 'transient-observation.json').write_text(json.dumps({'synthetic': control.events, 'actual': observed.events}, indent=2)+'\n')
        actual = [(path, int(mask, 16)) for path, mask in observed.events if '.rcgu.' in path]
        self.assertTrue(any(mask & 0x100 for _, mask in actual))
        self.assertTrue(any(mask & (0x200 | 0x40) for _, mask in actual))
        self.assertFalse(list(folder.rglob('*.rcgu.o')))
        for name in INPUTS:
            for value in ('', str(donor / 'missing')):
                with self.subTest(name=name, value=value), mock.patch.dict(os.environ, {name: value}), \
                        mock.patch('subprocess.run', side_effect=AssertionError('invalid input launched compiler')), \
                        mock.patch('tempfile.mkdtemp', side_effect=AssertionError('invalid input allocated output')), \
                        self.assertRaises(ValueError): validate()
        with self.assertRaises(ValueError): self.run_command([RUST, '--version'], ROOT)

    def test_kconfig_c_default_and_dependency(self):
        native = next((value for key, value in self.inputs.items() if key.startswith('BSEARCH_NATIVE') and value is not None), None)
        if native is None: self.skipTest('native conf absent; real Kconfig selection unproven')
        folder = self.work / 'kconfig'; folder.mkdir()
        source = (OVERLAY / 'lib/Kconfig').read_text()
        stanza = re.search(r'^config RUST_BSEARCH\n.*?(?=^config |^endmenu)', source, re.M | re.S)[0]
        (folder / 'Kconfig').write_text('config RUST\n\tbool "Rust"\n\n'+stanza)
        for index, (initial, expected) in enumerate((('', False), ('CONFIG_RUST=y\n', False),
                ('CONFIG_RUST=y\nCONFIG_RUST_BSEARCH=y\n', True), ('CONFIG_RUST_BSEARCH=y\n', False))):
            config = folder / ('config-'+str(index)); config.write_text(initial)
            self.run_command([native / 'scripts/kconfig/conf', '--olddefconfig', folder / 'Kconfig'], folder,
                environment={'KCONFIG_CONFIG': str(config), 'KCONFIG_AUTOCONFIG': str(folder / ('auto-'+str(index))),
                             'KCONFIG_AUTOHEADER': str(folder / ('header-'+str(index))),
                             'KCONFIG_RUSTCCFG': str(folder / ('rustcfg-'+str(index)))})
            self.assertEqual('CONFIG_RUST_BSEARCH=y\n' in config.read_text(), expected)

    def test_original_source_commit_retained(self):
        for name in ('lib/bsearch.rs', 'include/linux/bsearch_header.rs'):
            expected = re.findall(r'^// SOURCE-COMMIT: (.+)$', (ROOT / name).read_text(), re.M)
            self.assertEqual(expected, ['d482bb509b7d065808de40ce78b5bca39f40b783'])
            self.assertEqual(re.findall(r'^// SOURCE-COMMIT: (.+)$', (OVERLAY / name).read_text(), re.M), expected)


if __name__ == '__main__':
    unittest.main()
