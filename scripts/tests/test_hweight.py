#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Generic hweight: original-C differential, native ABI and real Kbuild.

Run with unittest discovery: python3 -m unittest discover -s scripts/tests -p test_hweight.py -v
HWEIGHT_RUSTC selects a compiler executable (default: rustc on PATH).
HWEIGHT_I686_SYSROOT enables real ELF32 compilation AND execution using matching
lib/rustlib/i686-unknown-linux-gnu/lib/libcore.rlib. SIGSYS is a test failure.
HWEIGHT_NATIVE_X86 / HWEIGHT_NATIVE_ARM64 optionally supply read-only configured
native trees with genuine rmeta, saved commands and gendwarfksyms.
Unset optional inputs skip; explicitly empty, missing or incomplete inputs fail.
All generated files live in TemporaryDirectory; TMPDIR controls its parent.
HWEIGHT_VERBOSE_COMMANDS=1 emits full compiler commands and diagnostics to stderr.
No native tree is built or written. C/Rust DWARF CRCs may differ by type names;
unsigned parameter/result shapes, genuine exports and normalized KCFI must agree.
"""
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / 'scripts/tests/fixtures/hweight'
NATIVE_COMMANDS = {
    # An unselected original object's .cmd need not exist in a clean build.
    # scatterlist is always C and has no source-specific compile flags in
    # lib/Makefile; its KBUILD identity is replaced below for our C oracle.
    'x86': ('lib/.list_sort_rust.o.cmd', 'lib/.scatterlist.o.cmd'),
    'arm64': ('lib/math/.cordic_rust.o.cmd', 'lib/.scatterlist.o.cmd'),
}
NAMES = {'__sw_hweight'+str(n) for n in (8, 16, 32, 64)}


def optional_directory(var, required):
    if var not in os.environ:
        raise unittest.SkipTest(var + ' not supplied')
    value = os.environ[var]
    if not value.strip():
        raise ValueError(var + ' explicitly empty')
    path = Path(value).resolve()
    if not path.is_dir():
        raise ValueError(var + ' is not a directory: ' + str(path))
    for name in required:
        if not (path / name).is_file():
            raise ValueError(var + ' missing ' + name)
    return path


class Harness:
    def __init__(self, case, out):
        self.case, self.out = case, out
        self.report = []
        self.rust = os.environ.get('HWEIGHT_RUSTC', 'rustc')
        if not self.rust.strip() or not shutil.which(self.rust):
            raise ValueError('HWEIGHT_RUSTC must name a usable compiler executable')
        self.env = dict(os.environ, RUSTC_BOOTSTRAP='1', TMPDIR=str(out),
                        CARGO_TARGET_DIR=str(out/'cargo'))
        self.run([self.rust, '-vV'])

    def fixture(self, name):
        dest = self.out / name
        if not dest.exists():
            content = (FIXTURES / (name + '.txt')).read_text()
            if name == 'binding_probe.rs':
                content = content.replace('@OWNER@', str(ROOT/'lib/hweight_rust.rs'))
            dest.write_text(content)
        return dest

    def check_dwarf(self, directory):
        for filename in ('rust.symtypes', 'c.symtypes'):
            text = (directory/filename).read_text()
            # Resolve the actual C __u64 typedef before comparing ABI shapes.
            for alias, body in re.findall(r'^(t#\w+) typedef_type \w+ \{ (.*?) \}$', text, re.M):
                text = text.replace(alias, body)
            rows = {line.split()[0]: line for line in text.splitlines()
                    if line.startswith('__sw_hweight')}
            self.case.assertEqual(set(rows), NAMES)
            for name, row in rows.items():
                widths = re.findall(r'byte_size\((\d+)\) encoding\((\d+)\)', row)
                expected = [('8', '7'), ('8', '7')] if name.endswith('64') else [('4', '7'), ('4', '7')]
                self.case.assertEqual(widths, expected, row)

    def run(self, args, *, cwd=None, expected=0, input=None):
        cwd = cwd or self.out
        args = list(map(str, args))
        p = subprocess.run(args, cwd=cwd, env=self.env, text=True, capture_output=True,
                           input=input, timeout=55,
                           preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        log = '$ '+shlex.join(args)+'\n'+p.stdout+p.stderr+f'\n[exit {p.returncode}]\n'
        with (self.out/'commands.log').open('a') as f:
            f.write(log)
        if os.environ.get('HWEIGHT_VERBOSE_COMMANDS') == '1':
            print(log, file=sys.stderr, end='')
        if expected is not None and p.returncode != expected:
            raise AssertionError(f'{shlex.join(args)}\nexit {p.returncode}, expected {expected}\n{p.stdout}{p.stderr}')
        return p


    def ordinary(self, bits):
        sysroot = optional_directory('HWEIGHT_I686_SYSROOT',
            ['lib/rustlib/i686-unknown-linux-gnu/lib/libcore.rlib']) if bits == 32 else None
        source = re.sub(r'^#include.*$', '', (ROOT/'lib/hweight.c').read_text(), flags=re.M)
        source = '#define EXPORT_SYMBOL(x)\ntypedef unsigned long long __u64;\n'+source
        (self.out/'oracle.c').write_text(source)
        for fast in (False, True):
            for opt in ('0', '2', 's'):
                for debug in (True, False):
                    with self.case.subTest(bits=bits, fast=fast, opt=opt, debug=debug):
                        tag = f'host{bits}-fast{int(fast)}-O{opt}-debug{int(debug)}'
                        d = self.out/tag
                        d.mkdir(exist_ok=True)
                        rf = [self.rust, '--edition=2021', '--crate-type=rlib', '-Zcrate-attr=no_std',
                              '-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '-Cpanic=abort',
                              '-Copt-level='+opt, '-Cdebug-assertions='+str(debug).lower(),
                              '-Coverflow-checks='+str(debug).lower()]
                        if bits == 32:
                            rf += ['--target=i686-unknown-linux-gnu', '--sysroot='+str(sysroot)]
                        if fast:
                            rf += ['--cfg', 'CONFIG_ARCH_HAS_FAST_MULTIPLIER']
                        self.run([*rf, ROOT/'lib/hweight_rust.rs', '--emit=obj='+str(d/'rust.o')+',dep-info='+str(d/'rust.d')])
                        cf = ['clang', '-m'+str(bits), '-O'+opt, '-ffreestanding', '-fno-builtin',
                              '-fno-pie', '-fno-stack-protector', '-Wall', '-Wextra', '-Werror']
                        defines = ['-DBITS_PER_LONG='+str(bits)]
                        if fast:
                            defines += ['-DCONFIG_ARCH_HAS_FAST_MULTIPLIER']
                        defines += [f'-D__sw_hweight{n}=c_hweight{n}' for n in (8, 16, 32, 64)]
                        self.run([*cf, *defines, '-c', self.out/'oracle.c', '-o', d/'c.o'])
                        self.run([*cf, '-c', self.fixture('driver.c'), '-o', d/'driver.o'])
                        extra = ['-no-pie'] if bits == 64 else ['-nostdlib', '-static', '-Wl,-e,_start', self.fixture('start32.S')]
                        self.run([*cf, *extra, d/'rust.o', d/'c.o', d/'driver.o', '-o', d/'differential'])
                        assert (d/'differential').read_bytes()[4] == (1 if bits == 32 else 2)
                        p = self.run([d/'differential'], expected=None)
                        self.report.append(dict(test=tag, exit=p.returncode, passed=p.returncode == 0))
                        self.case.assertEqual(p.returncode, 0, f"{tag}: execution failed (SIGSYS is a failure)")


    def rust_flags(self, build, command):
        args = shlex.split((build/command).read_text().splitlines()[0].split(' := ', 1)[1])
        while '=' in args[0] and not args[0].startswith('-'):
            args.pop(0)
        args = args[1:-1]
        # The ARM donor is a module; generic hweight is a built-in bool owner.
        if '--cfg' in args:
            i = args.index('--cfg')
            if args[i + 1] == 'MODULE':
                del args[i:i + 2]
        result = []
        skip = False
        for a in args:
            if skip:
                skip = False
                continue
            if a == '--out-dir':
                skip = True
                continue
            if a.startswith('--emit=') or a.startswith('-Copt-level='):
                continue
            if a.startswith('--target=./'):
                a = '--target='+str(build/a.split('=./')[1])
            elif a.startswith('@./'):
                a = '@'+str(build/a[3:])
            elif a == './rust/':
                a = str(build/'rust')
            result.append(a)
        return result+['-Dwarnings', '-Dunsafe_op_in_unsafe_fn']


    def c_flags(self, build, command):
        args = shlex.split((build/command).read_text().splitlines()[0].split(' := ', 1)[1])[1:]
        identities = ('-DKBUILD_MODFILE=', '-DKBUILD_BASENAME=',
                      '-DKBUILD_MODNAME=', '-D__KBUILD_MODNAME=')
        flags = [a for a in args[:args.index('-c')]
                 if not a.startswith(('-Wp,-MMD,', *identities)) and a not in ('-O0', '-O1', '-O2', '-O3', '-Os', '-Oz')]
        return flags + ['-DKBUILD_MODFILE="lib/hweight"', '-DKBUILD_BASENAME="hweight"',
                        '-DKBUILD_MODNAME="hweight"', '-D__KBUILD_MODNAME=hweight']


    def ids(self, path):
        s = path.read_text()
        values = {n: int(v) & 0xffffffff for n, v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}$', s, re.M)}
        return {name: values[n] for name, n in re.findall(r'^define [^\n]*?@([\w]+)\([^\n]*!kcfi_type !(\d+)', s, re.M)}


    def native(self, arch):
        from rust_exports_test_support import read_exports
        rcmd, ccmd = NATIVE_COMMANDS[arch]
        build = optional_directory('HWEIGHT_NATIVE_'+arch.upper(),
            [rcmd, ccmd, 'rust/libkernel.rmeta', 'rust/libbindings.rmeta',
             'include/generated/rustc_cfg', 'scripts/gendwarfksyms/gendwarfksyms'])
        for fast in (False, True):
            rf = self.rust_flags(build, rcmd)
            cf = self.c_flags(build, ccmd)
            assert '-Zsanitizer=kcfi' in rf and '-fsanitize=kcfi' in cf
            cfg = self.out / (arch + '-fast' + str(int(fast)) + '.cfg')
            cfg.write_text('\n'.join(line for line in
                (build/'include/generated/rustc_cfg').read_text().splitlines()
                if 'CONFIG_ARCH_HAS_FAST_MULTIPLIER' not in line) + '\n' +
                ('--cfg=CONFIG_ARCH_HAS_FAST_MULTIPLIER\n' if fast else ''))
            rf = ['@'+str(cfg) if a.startswith('@') else a for a in rf]
            switch = self.out / (arch + '-fast' + str(int(fast)) + '.h')
            switch.write_text('#undef CONFIG_ARCH_HAS_FAST_MULTIPLIER\n' +
                              ('#define CONFIG_ARCH_HAS_FAST_MULTIPLIER 1\n' if fast else ''))
            cf += ['-include', str(switch)]
            for opt in ('0', '2', 's'):
                d = self.out/(arch+'-fast'+str(int(fast))+'-O'+opt)
                d.mkdir(exist_ok=True)
                self.run([self.rust, *rf, '-Copt-level='+opt, ROOT/'lib/hweight_rust.rs',
                     '--emit=obj='+str(d/'rust.o')+',llvm-ir='+str(d/'rust.ll')+',dep-info='+str(d/'rust.d')])
                self.run([self.rust, *rf, '-Copt-level='+opt, self.fixture('binding_probe.rs'), '--emit=obj='+str(d/'bindings.o')])
                self.run(['clang', *cf, '-O'+opt, '-S', '-emit-llvm', ROOT/'lib/hweight.c', '-o', d/'c.ll'], cwd=build)
                self.run(['clang', *cf, '-O'+opt, '-c', ROOT/'lib/hweight.c', '-o', d/'c.o'], cwd=build)
                ri, ci = self.ids(d/'rust.ll'), self.ids(d/'c.ll')
                for n in (8, 16, 32, 64):
                    name = '__sw_hweight'+str(n)
                    assert ri[name] == ci[name], (arch, name, ri, ci)
                exports = read_exports(d/'rust.o')
                original_exports = read_exports(d/'c.o')
                self.case.assertEqual({row['name'] for row in exports}, NAMES)
                self.case.assertEqual({row['name'] for row in original_exports}, NAMES)
                assert len(exports) == len(original_exports) == 4
                for row in exports + original_exports:
                    assert row['name'] in ['__sw_hweight'+str(n) for n in (8,16,32,64)]
                    assert row['license'] == '' and row['namespace'] == ''
                    assert row['relocation_target'] == row['name'] and row['relocation_addend'] == 0
                    assert row['pointer_width'] == 8
                undefined = self.run(['nm', '-u', d/'rust.o']).stdout.strip()
                assert not undefined, undefined
                dwarf = self.run([build/'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions', '-T', d/'rust.symtypes', d/'rust.o'],
                            input=''.join('__sw_hweight'+str(n)+'\n' for n in (8,16,32,64)))
                (d/'versions.log').write_text(dwarf.stdout+dwarf.stderr)
                original_versions = self.run([build/'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions',
                                         '-T', d/'c.symtypes', d/'c.o'],
                                        input=''.join('__sw_hweight'+str(n)+'\n' for n in (8,16,32,64)))
                (d/'c-versions.log').write_text(original_versions.stdout+original_versions.stderr)
                for n in (8,16,32,64):
                    assert '__sw_hweight'+str(n) in dwarf.stdout
                    assert '__sw_hweight'+str(n) in original_versions.stdout
                self.run(['llvm-objdump', '-dr', d/'rust.o'])
                self.report.append(dict(test=arch+'-O'+opt, passed=True, kcfi=ri, exports=exports))
                self.check_dwarf(d)
                if arch == 'x86':
                    self.native_controls(build, cf, d, opt)
        if arch == 'x86':
            self.architecture_control(build)
            self.binding_control(build)


    def native_controls(self, build, cf, d, opt):
        renames = [f'-D__sw_hweight{n}=c_hweight{n}' for n in (8,16,32,64)]
        self.run(['clang', *cf, '-O'+opt, *renames, '-c', ROOT/'lib/hweight.c', '-o', d/'renamed-c.o'], cwd=build)
        self.run(['clang', '-O'+opt, '-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers',
             '-c', self.fixture('driver.c'), '-o', d/'driver.o'])
        self.run(['clang', '-no-pie', d/'driver.o', d/'rust.o', d/'renamed-c.o', '-o', d/'differential'])
        self.run([d/'differential'])
        self.run(['clang', '-O'+opt, '-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers',
             '-c', self.fixture('kcfi_bad.c'), '-o', d/'bad.o'])
        self.run(['clang', '-no-pie', d/'bad.o', d/'rust.o', '-o', d/'bad'])
        self.run([d/'bad'], expected=-4)
        self.run(['clang', '-no-pie', self.fixture('abi_driver.c'), self.fixture('abi_x86.S'), d/'rust.o', '-o', d/'abi'])
        self.run([d/'abi'])
        self.run([d/'abi', 'corrupt'], expected=1)
        special = self.run([d/'abi', 'special', 'registers'], expected=1)
        self.report.append(dict(test='native-x86-controls-O'+opt, passed=True,
                           special_register_probe_exit=special.returncode,
                           note='special preservation is not the generic C ABI'))


    def architecture_control(self, build):
        # The retained actual architecture object must pass the stronger ABI probe.
        d = self.out/'abi-architecture'
        d.mkdir(exist_ok=True)
        (d/'main.c').write_text('extern int special_probe(void); int main(void) { return special_probe(); }\n')
        self.run(['clang', '-no-pie', d/'main.c', self.fixture('abi_x86.S'), build/'arch/x86/lib/hweight.o', '-o', d/'probe'])
        self.run([d/'probe'])
        self.report.append(dict(test='retained-x86-assembly-special-registers', passed=True))


    def semantic_controls(self):
        self.ordinary(64)
        # Compile semantic mutations of the real owner, preserving its export machinery.
        original = (ROOT/'lib/hweight.rs').read_text()
        mutants = {
            'full-word-count8': original.replace('(res.wrapping_add(res >> 4)) & 0x0f', 'w.count_ones().wrapping_add(res & 0)'),
            'mask16': original.replace('& 0x00ff', '& 0x000f'),
            'fast32': original.replace('0x0101_0101)', '0x0101_0100)'),
            'slow32': original.replace('& 0x0000_00ff', '& 0x0000_000f'),
            'high64': original.replace('pub fn __sw_hweight64(w: u64) -> usize {', 'pub fn __sw_hweight64(w: u64) -> usize {\n    let w = w & 0xffff_ffff;'),
        }
        for name, source in mutants.items():
            assert source != original
            d = self.out/('mutation-'+name)
            d.mkdir(exist_ok=True)
            (d/'hweight.rs').write_text(source)
            owner = (ROOT/'lib/hweight_rust.rs').read_text().replace('../rust/ffi_export.rs', str(ROOT/'rust/ffi_export.rs'))
            (d/'owner.rs').write_text(owner)
            fast = name == 'fast32'
            flags = ['--cfg', 'CONFIG_ARCH_HAS_FAST_MULTIPLIER'] if fast else []
            self.run([self.rust, '--edition=2021', '--crate-type=rlib', '-Zcrate-attr=no_std', '-Dwarnings',
                 '-Dunsafe_op_in_unsafe_fn', '-Cpanic=abort', '-Copt-level=2', *flags,
                 d/'owner.rs', '--emit=obj='+str(d/'rust.o')])
            base = self.out/f'host64-fast{int(fast)}-O2-debug1'
            self.run(['clang', '-no-pie', base/'c.o', base/'driver.o', d/'rust.o', '-o', d/'differential'])
            p = self.run([d/'differential'], expected=None)
            assert p.returncode in (8,16,32,64), (name, p.returncode)
            self.report.append(dict(test='mutation-'+name, passed=True, rejected_exit=p.returncode))


    def binding_control(self, build):
        # Return-type mutation must fail against the genuine generated binding.
        d = self.out/'mutation-type'
        d.mkdir(exist_ok=True)
        probe = self.fixture('binding_probe.rs').read_text()
        probe = probe.replace('unsafe extern "C" fn(u64) -> usize', 'unsafe extern "C" fn(u64) -> u32')
        (d/'probe.rs').write_text(probe)
        rf = self.rust_flags(build, 'lib/.list_sort_rust.o.cmd')
        p = self.run([self.rust, *rf, d/'probe.rs', '--emit=obj='+str(d/'bad.o')], expected=None)
        assert p.returncode != 0 and 'mismatched types' in p.stderr
        self.report.append(dict(test='binding-return-width-mutation', passed=True))


class HweightTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='hweight-')
        self.addCleanup(temporary.cleanup)
        self.h = Harness(self, Path(temporary.name))

    def test_host64_original_c_matrix(self):
        self.h.ordinary(64)

    def test_elf32_original_c_matrix_and_execution(self):
        self.h.ordinary(32)

    def test_semantic_negative_controls(self):
        self.h.semantic_controls()

    def test_native_x86_bindings_kcfi_exports_dwarf_register_abi(self):
        self.h.native('x86')

    def test_native_arm64_bindings_kcfi_exports_dwarf(self):
        self.h.native('arm64')

    def test_native_flags_without_unselected_c_commands(self):
        # Replay genuine native artifacts through a private read-only view.
        # Neither selected Rust owner requires a previous C-provider build.
        for arch in ('x86', 'arm64'):
            with self.subTest(arch=arch):
                variable = 'HWEIGHT_NATIVE_' + arch.upper()
                original = optional_directory(variable, [*NATIVE_COMMANDS[arch]])
                view = self.h.out / ('without-original-c-' + arch)
                view.mkdir()
                for entry in original.iterdir():
                    if entry.name != 'lib':
                        (view / entry.name).symlink_to(entry)
                (view / 'lib').mkdir()
                for entry in (original / 'lib').iterdir():
                    if entry.name not in ('.list_sort.o.cmd', '.hweight.o.cmd'):
                        (view / 'lib' / entry.name).symlink_to(entry)
                for name in ('.list_sort.o.cmd', '.hweight.o.cmd'):
                    self.assertFalse((view / 'lib' / name).exists())
                flags = self.h.c_flags(view, NATIVE_COMMANDS[arch][1])
                self.assertFalse(any('scatterlist' in flag for flag in flags), flags)
                self.assertIn('-D__KBUILD_MODNAME=hweight', flags)
                self.assertIn('-fsanitize=kcfi', flags)
                self.assertIn('-fsanitize-cfi-icall-experimental-normalize-integers', flags)
                with patch.dict(os.environ, {variable: str(view)}):
                    self.h.native(arch)

    def test_explicit_invalid_inputs_fail(self):
        for variable, required in (
            ('HWEIGHT_I686_SYSROOT', ['lib/rustlib/i686-unknown-linux-gnu/lib/libcore.rlib']),
            ('HWEIGHT_NATIVE_X86', [NATIVE_COMMANDS['x86'][0]]),
            ('HWEIGHT_NATIVE_ARM64', [NATIVE_COMMANDS['arm64'][0]]),
        ):
            for value in ('', str(self.h.out/'missing'), str(self.h.out)):
                with self.subTest(variable=variable, value=value), patch.dict(os.environ, {variable: value}):
                    with self.assertRaises(ValueError):
                        optional_directory(variable, required)
        for value in ('', str(self.h.out/'missing-rustc')):
            with patch.dict(os.environ, HWEIGHT_RUSTC=value), self.assertRaises(ValueError):
                Harness(self, self.h.out)
        source = self.h.out/'empty.rs'
        source.write_text('#![no_std]\npub fn test() {}\n')
        result = self.h.run([self.h.rust, '--crate-type=rlib', '--sysroot='+str(self.h.out),
                             source, '--emit=obj='+str(self.h.out/'invalid.o')], expected=None)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('core', result.stderr)

    def test_kconfig_default_dependencies_and_x86_owner(self):
        build = optional_directory('HWEIGHT_NATIVE_X86', ['scripts/kconfig/conf'])
        h, d = self.h, self.h.out
        stanza = (ROOT/'lib/Kconfig').read_text().split('config RUST_HWEIGHT\n', 1)[1].split('\nconfig ', 1)[0]
        config = d/'Kconfig'
        config.write_text('mainmenu "Hweight selection"\nconfig RUST\n\tbool "Rust"\n'
                          'config GENERIC_HWEIGHT\n\tbool "Generic"\nconfig RUST_HWEIGHT\n'+stanza+'\n')
        for rust, generic, selected, expected in (
            ('y', 'y', False, False), ('y', 'y', True, True),
            ('n', 'y', True, False), ('y', 'n', True, False),
        ):
            (d/'.config').write_text(f'CONFIG_RUST={rust}\nCONFIG_GENERIC_HWEIGHT={generic}\n'
                                    + ('CONFIG_RUST_HWEIGHT=y\n' if selected else ''))
            h.run([build/'scripts/kconfig/conf', '--olddefconfig', config])
            self.assertEqual('CONFIG_RUST_HWEIGHT=y' in (d/'.config').read_text(), expected)
        self.assertNotIn('GENERIC_HWEIGHT', (ROOT/'arch/x86/Kconfig').read_text())
        makefile = d/'architecture.mk'
        makefile.write_text('include '+str(ROOT/'arch/x86/lib/Makefile')+
                            '\n.PHONY: owners\nowners:\n\t@echo $(lib-y) $(obj-y)\n')
        for selected in ('', 'y'):
            objects = h.run(['make', '-s', '-f', makefile, 'CONFIG_RUST_HWEIGHT='+selected,
                             'CONFIG_X86_64=y', 'owners']).stdout.split()
            self.assertEqual(objects.count('hweight.o'), 1)
            self.assertNotIn('hweight_rust.o', objects)

    def test_real_kbuild_selection_archive_slot_dependencies_and_noop(self):
        # Exercise the complete component Makefile and real Makefile.build/fixdep.
        # Unrelated members are inert objects; only hweight C/Rust is compiled.
        h, d = self.h, self.h.out
        for directory in ('lib', 'scripts/basic', 'include/config', 'stubs/linux', 'stubs/asm'):
            (d/directory).mkdir(parents=True, exist_ok=True)
        h.run([h.rust, '--edition=2021', '-Dwarnings', '-O', ROOT/'scripts/basic/fixdep.rs',
               '-o', d/'scripts/basic/fixdep'])
        (d/'stubs/linux/export.h').write_text('#define EXPORT_SYMBOL(x)\n')
        (d/'stubs/linux/bitops.h').write_text('#define BITS_PER_LONG 64\n')
        (d/'stubs/asm/types.h').write_text('typedef unsigned long long __u64;\n')
        rust = shlex.join([h.rust, '--edition=2021', '--crate-type=rlib', '-Zcrate-attr=no_std',
                          '-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '-Cpanic=abort', '-Copt-level=2'])
        base = ['make', '--no-print-directory', '-rR', '-j4', '-f', str(ROOT/'scripts/Makefile.build'),
                'obj=lib', 'srcroot='+str(ROOT), 'srctree='+str(ROOT), 'objtree='+str(d),
                'VPATH='+str(ROOT), 'need-builtin=1', 'KBUILD_BUILTIN=1', 'AR=ar', 'NM=nm',
                'CONFIG_GENERIC_HWEIGHT=y', 'CONFIG_BTREE=y',
                'rust_common_cmd='+rust+' --emit=dep-info=$(depfile)',
                'cmd_cc_o_c=clang -O2 -Wall -Wextra -Werror -ffreestanding -I'+str(d/'stubs')+
                ' -MMD -MF $(depfile) -c $< -o $@']
        listing = d/'listing.mk'
        listing.write_text('.PHONY: members\nmembers:\n\t@echo $(real-obj-y)\n')
        original = h.run([*base, '-f', listing, 'CONFIG_RUST_HWEIGHT=', 'members']).stdout.split()
        self.assertEqual(original.count('lib/hweight.o'), 1, original)
        # BTREE is the next selected original slot, not an appended owner.
        self.assertEqual(original[original.index('lib/hweight.o') + 1], 'lib/btree.o')
        empty = d/'empty.c'
        empty.write_text('/* unrelated archive member */\n')
        ignored = []
        for member in original:
            if member == 'lib/hweight.o':
                continue
            path = d/member
            path.parent.mkdir(parents=True, exist_ok=True)
            if path.suffix == '.a':
                h.run(['ar', 'cr', path])
                ignored += ['-o', str(Path(member).parent)]
            else:
                h.run(['clang', '-c', empty, '-o', path])
            ignored += ['-o', member]
        expected_c = [member for member in original if not member.endswith('.a')]
        archive = d/'lib/built-in.a'

        def build(selected, *extra, generic='y'):
            h.run([*base, *ignored, 'CONFIG_RUST_HWEIGHT='+('y' if selected else ''),
                   'CONFIG_GENERIC_HWEIGHT='+generic, *extra, 'lib/built-in.a'])
            actual = [str(Path(member).relative_to(d)) if Path(member).is_absolute() else member
                      for member in h.run(['ar', 't', archive]).stdout.splitlines()]
            expected = [m.replace('hweight.o', 'hweight_rust.o') if selected else m for m in expected_c]
            if generic != 'y':
                expected = [m for m in expected if Path(m).name not in ('hweight.o', 'hweight_rust.o')]
            self.assertEqual(actual, expected)
            symbols = h.run(['nm', '--defined-only', archive]).stdout
            for name in NAMES:
                self.assertEqual(len(re.findall(r'\bT '+name+r'$', symbols, re.M)), int(generic == 'y'))
            return d/('lib/hweight_rust.o' if selected else 'lib/hweight.o')

        for selected in (False, True, False, True):
            obj = build(selected)
            stamps = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
            build(selected)
            self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamps)
            unselected = ROOT/('lib/hweight.c' if selected else 'lib/hweight_rust.rs')
            build(selected, '-W', str(unselected))
            self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamps)
        command = (d/'lib/.hweight_rust.o.cmd').read_text()
        for source in ('lib/hweight_rust.rs', 'lib/hweight.rs', 'rust/ffi_export.rs',
                       'include/linux/export_header.rs'):
            dependencies = [token for token in command.split() if token.startswith('/')
                            and Path(token).resolve() == (ROOT/source).resolve()]
            self.assertTrue(dependencies, source + ' absent from fixdep command')
            dependency = dependencies[-1]
            before = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
            build(True, '-W', dependency)
            self.assertTrue(all(a > b for a, b in zip(
                (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), before)))
        self.assertIn('$(wildcard include/config/ARCH_HAS_FAST_MULTIPLIER)', command)
        before = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
        (d/'include/config/ARCH_HAS_FAST_MULTIPLIER').touch()
        build(True)
        self.assertTrue(all(a > b for a, b in zip(
            (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), before)))
        stamps = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
        build(True)
        self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamps)
        cobj = build(False)
        before = cobj.stat().st_mtime_ns, archive.stat().st_mtime_ns
        build(False, '-W', str(ROOT/'lib/hweight.c'))
        self.assertTrue(all(a > b for a, b in zip(
            (cobj.stat().st_mtime_ns, archive.stat().st_mtime_ns), before)))
        build(True, generic='')
        build(False, generic='')


if __name__ == '__main__':
    unittest.main()
