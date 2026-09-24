# SPDX-License-Identifier: GPL-2.0
"""Windowed tracker differential and native object gates.

WIN_MINMAX_NATIVE: optional read-only native output (saved bcd_rust/scatterlist
commands, bindgen command, core, fixdep and gendwarfksyms). WIN_MINMAX_RUSTC:
matching rustc >=1.85. WIN_MINMAX_BINDGEN: real bindgen executable.
WIN_MINMAX_I686_SYSROOT: optional genuine i686 core sysroot. ELF32 nonzero exits
including SIGSYS fail. All supplied empty/invalid inputs fail. Fixtures use real
bindgen on the original header; the small kernel fixture crate is explicitly NOT
proof of integrated kernel metadata. Native integrated output is a root gate.
Commands and outputs print to stdout so callers can retain failed logs.
"""
import os
import hashlib
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / 'scripts/tests/win_minmax_fixtures'


def directory(name, files):
    if name not in os.environ:
        return None
    if not os.environ[name].strip():
        raise ValueError(name + ' supplied empty')
    path = Path(os.environ[name]).resolve()
    if not path.is_dir():
        raise ValueError(name + ' invalid directory')
    for file in files:
        if not (path / file).is_file():
            raise ValueError(name + ' missing ' + file)
    return path


class WinMinmax(unittest.TestCase):
    def setUp(self):
        self.native = directory('WIN_MINMAX_NATIVE', [
            'lib/.bcd_rust.o.cmd', 'lib/.scatterlist.o.cmd',
            'rust/bindings/.bindings_generated.rs.cmd', 'rust/libcore.rmeta',
            'scripts/target.json', 'include/generated/rustc_cfg',
            'scripts/basic/fixdep', 'scripts/kconfig/conf',
            'scripts/gendwarfksyms/gendwarfksyms'])
        self.sysroot = directory('WIN_MINMAX_I686_SYSROOT', [])
        if self.sysroot and not list((self.sysroot / 'lib/rustlib/i686-unknown-linux-gnu/lib').glob('libcore*.rlib')):
            raise ValueError('WIN_MINMAX_I686_SYSROOT has no core')
        self.rust = os.environ.get('WIN_MINMAX_RUSTC', 'rustc')
        self.bindgen = os.environ.get('WIN_MINMAX_BINDGEN')
        for name, value in [('WIN_MINMAX_RUSTC', self.rust), ('WIN_MINMAX_BINDGEN', self.bindgen)]:
            if name in os.environ and (not value or not shutil.which(value)):
                raise ValueError(name + ' invalid executable')
        self.temp = tempfile.TemporaryDirectory(prefix='win-minmax-')
        self.addCleanup(self.temp.cleanup)
        self.out = Path(self.temp.name)
        self.env = dict(os.environ, RUSTC_BOOTSTRAP='1', LC_ALL='C',
                        OBJTREE=str(self.native), RUST_MODFILE='lib/win_minmax_rust')
        for k in list(self.env):
            if k.startswith('KBUILD_') or k in ('MAKEFLAGS', 'MFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES'):
                del self.env[k]

    def run_cmd(self, args, cwd=None, expected=0, input=None):
        args = list(map(str, args))
        print('$ ' + shlex.join(args), flush=True)
        p = subprocess.run(args, cwd=cwd or self.out, env=self.env,
                           text=True, capture_output=True, input=input, timeout=50)
        print(p.stdout + p.stderr + '[exit %s]' % p.returncode, flush=True)
        if expected is not None:
            self.assertEqual(p.returncode, expected)
        return p

    def require_native(self):
        if self.native is None:
            self.skipTest('WIN_MINMAX_NATIVE not supplied')
        version = self.run_cmd([self.rust, '--version']).stdout
        match = re.search(r'rustc (\d+)\.(\d+)', version)
        self.assertIsNotNone(match)
        self.assertGreaterEqual(tuple(map(int, match.groups())), (1, 85))

    def c_flags(self):
        line = (self.native / 'lib/.scatterlist.o.cmd').read_text().splitlines()[0].split(' := ', 1)[1]
        args = shlex.split(line)[1:]
        args = args[:args.index('-c')]
        result = []
        for arg in args:
            if arg.startswith('-Wp,-MMD,') or arg.startswith('-O'):
                continue
            # Preserve genuine flags while moving only output paths/owner names.
            if arg.startswith('-DKBUILD_') or arg.startswith('-D__KBUILD_'):
                arg = arg.replace('scatterlist', 'win_minmax')
            result.append(arg)
        return result

    def rust_flags(self):
        line = (self.native / 'lib/.bcd_rust.o.cmd').read_text().splitlines()[0].split(' := ', 1)[1]
        args = shlex.split(line)
        while '=' in args[0] and not args[0].startswith('-'):
            args.pop(0)
        args = args[1:-1]
        result, skip = [], False
        for arg in args:
            if skip:
                skip = False
                continue
            if arg == '--out-dir':
                skip = True
                continue
            if arg.startswith('--emit=') or arg.startswith('-Copt-level='):
                continue
            if arg.startswith('--target=./'):
                arg = '--target=' + str(self.native / arg.split('=./')[1])
            elif arg.startswith('@./'):
                arg = '@' + str(self.native / arg[3:])
            elif arg == './rust/':
                arg = str(self.native / 'rust')
            result.append(arg)
        return result + ['-Dwarnings']

    def bindings(self):
        line = (self.native / 'rust/bindings/.bindings_generated.rs.cmd').read_text().splitlines()[0]
        bindgen = self.bindgen or shlex.split(line.split(' := ', 1)[1])[0]
        self.run_cmd([bindgen, ROOT / 'include/linux/win_minmax.h', '--use-core',
            '--rust-target', '1.85', '--no-layout-tests', '--allowlist-type', 'minmax.*',
            '--allowlist-function', 'minmax_running_.*', '-o', self.out / 'generated.rs',
            '--', *self.c_flags(), '-D__BINDGEN__'], cwd=self.native)
        generated = (self.out / 'generated.rs').read_text()
        print('Generated directly from original header:\n' + generated, flush=True)
        for source in ['lib/win_minmax.c', 'include/linux/win_minmax.h',
                       'lib/win_minmax.rs', 'include/linux/win_minmax_header.rs']:
            print('SHA256', source, hashlib.sha256((ROOT / source).read_bytes()).hexdigest(), flush=True)
        self.assertIn('pub struct minmax', generated)
        (self.out / 'kernel.rs').write_text('#![allow(missing_docs, non_camel_case_types)]\n'
            'pub mod bindings {\n' + generated + '\n}\n')
        (self.out / 'header.rs').write_text((FIXTURES / 'header.rs.txt').read_text().replace(
            'HEADER', str(ROOT / 'include/linux/win_minmax_header.rs')))

    def fixture_sources(self):
        header = re.sub(r'^#include.*$', '', (ROOT / 'include/linux/win_minmax.h').read_text(), flags=re.M)
        (self.out / 'fixture.h').write_text('typedef unsigned int u32;\n' + header +
            '\n_Static_assert(sizeof(struct minmax)==24,"size");\n'
            '_Static_assert(__builtin_offsetof(struct minmax_sample,v)==4,"offset");\n')
        oracle = re.sub(r'^#include.*$', '', (ROOT / 'lib/win_minmax.c').read_text(), flags=re.M)
        (self.out / 'oracle.c').write_text('#include "fixture.h"\n#define unlikely(x) (x)\n'
            '#define EXPORT_SYMBOL(x)\n' + oracle)
        (self.out / 'driver.c').write_text('#include "fixture.h"\n' + (FIXTURES / 'differential.c.txt').read_text())
        shutil.copyfile(FIXTURES / 'start32.c.txt', self.out / 'start32.c')

    def ordinary(self, bits):
        self.require_native()
        if bits == 32 and not self.sysroot:
            self.skipTest('WIN_MINMAX_I686_SYSROOT not supplied')
        self.bindings()
        self.fixture_sources()
        target = [] if bits == 64 else ['--target=i686-unknown-linux-gnu', '--sysroot=' + str(self.sysroot)]
        for opt in ['0', '2', 's']:
            with self.subTest(bits=bits, opt=opt):
                self.ordinary_opt(bits, opt, target)
        if bits == 64:
            self.negative_semantics(target)

    def ordinary_opt(self, bits, opt, target):
        rf = [self.rust, '--edition=2021', '--crate-type=rlib', '-Cpanic=abort',
              '-Dwarnings', '-Dunsafe-op-in-unsafe-fn', '-Coverflow-checks=y',
              '-Cdebug-assertions=n', '-Copt-level=' + opt, *target]
        self.run_cmd([*rf, '-Zcrate-attr=no_std', '--crate-name=kernel', self.out / 'kernel.rs', '-o', self.out / 'libkernel.rlib'])
        for name, source in [('rust', ROOT / 'lib/win_minmax.rs'), ('header', self.out / 'header.rs')]:
            self.run_cmd([*rf, '-Zcrate-attr=no_std', '--extern=kernel=' + str(self.out / 'libkernel.rlib'),
                source, '--emit=obj=' + str(self.out / (name + '.o'))])
        cf = ['clang', '-m' + str(bits), '-O' + opt, '-ffreestanding', '-fno-builtin',
              '-fno-pie', '-Wall', '-Wextra', '-Werror']
        self.run_cmd([*cf, '-Dminmax_running_min=c_minmax_running_min',
            '-Dminmax_running_max=c_minmax_running_max', '-c', self.out / 'oracle.c', '-o', self.out / 'c.o'])
        extra = ['-no-pie'] if bits == 64 else ['-nostdlib', '-static', '-Wl,-e,_start', self.out / 'start32.c']
        self.run_cmd([*cf, *extra, self.out / 'driver.c', self.out / 'c.o', self.out / 'rust.o',
                      self.out / 'header.o', '-o', self.out / 'differential'])
        self.assertEqual((self.out / 'differential').read_bytes()[4], 2 if bits == 64 else 1)
        self.run_cmd([self.out / 'differential'])
        print('PASS bits=%d opt=%s 20000 streams / 10000000 updates; partial initialization' % (bits, opt), flush=True)

    def negative_semantics(self, target):
        rf = [self.rust, '--edition=2021', '--crate-type=rlib', '-Cpanic=abort',
              '-Dwarnings', '-Dunsafe-op-in-unsafe-fn', '-Coverflow-checks=y',
              '-Cdebug-assertions=n', '-Copt-level=s', *target]
        cf = ['clang', '-m64', '-Os', '-ffreestanding', '-fno-builtin',
              '-fno-pie', '-Wall', '-Wextra', '-Werror']
        source = (ROOT / 'lib/win_minmax.rs').read_text().replace('../include/linux/win_minmax_header.rs',
            str(ROOT / 'include/linux/win_minmax_header.rs'))
        for name, old, new in [('ties', 'val.v <= (*m).s[0].v', 'val.v < (*m).s[0].v'),
                               ('edges', 'dt > win {', 'dt >= win {'),
                               ('reset', '(*m).s = [val; 3];', '(*m).s[0] = val;')]:
            mutant = source.replace(old, new)
            self.assertNotEqual(source, mutant)
            (self.out / 'mutant.rs').write_text(mutant)
            self.run_cmd([*rf, '-Zcrate-attr=no_std', '--extern=kernel=' + str(self.out / 'libkernel.rlib'),
                self.out / 'mutant.rs', '--emit=obj=' + str(self.out / 'mutant.o')])
            self.run_cmd([*cf, '-no-pie', self.out / 'driver.c', self.out / 'c.o', self.out / 'mutant.o',
                          self.out / 'header.o', '-o', self.out / 'negative'])
            self.assertNotEqual(self.run_cmd([self.out / 'negative'], expected=None).returncode, 0, name)

        # These controls must fail even though C and Rust still agree: otherwise
        # an accidental step-zero reset/tie can silently discard random states.
        driver = (self.out / 'driver.c').read_text()
        for name, old, new in [
                ('initial-reset', 'j != 0 && j % 97 == 0', 'j % 97 == 0'),
                ('initial-tie', 'j != 0 && j % 4 == 0', 'j % 4 == 0')]:
            mutant = driver.replace(old, new)
            self.assertNotEqual(driver, mutant)
            (self.out / 'corpus-control.c').write_text(mutant)
            self.run_cmd([*cf, '-no-pie', self.out / 'corpus-control.c', self.out / 'c.o',
                          self.out / 'rust.o', self.out / 'header.o', '-o', self.out / 'corpus-control'])
            self.assertEqual(self.run_cmd([self.out / 'corpus-control'], expected=None).returncode, 10, name)

    def test_ordinary64(self):
        self.ordinary(64)

    def test_ordinary32(self):
        self.ordinary(32)

    def native_fixture_flags(self):
        rf = self.rust_flags()
        # Real generated types in a clearly marked fixture crate until the root
        # regenerates the full kernel bindings. Keep every saved native flag.
        fixture_rf = []
        skip = False
        for arg in rf:
            if skip:
                skip = False
                continue
            if arg == '--extern':
                skip = True
                continue
            fixture_rf.append(arg)
        self.run_cmd([self.rust, *fixture_rf, '--crate-name=kernel', self.out / 'kernel.rs',
                      '--emit=link', '-o', self.out / 'libkernel.rlib'])
        rf = [arg if arg != 'kernel' else 'kernel=' + str(self.out / 'libkernel.rlib') for arg in rf]
        return rf

    def test_native_object(self):
        self.require_native()
        self.bindings()
        rf = self.native_fixture_flags()
        self.fixture_sources()
        # Kernel link scripts discard these header addressability anchors.
        (self.out / 'discard.lds').write_text('SECTIONS { /DISCARD/ : { *(.discard.addressable) } } INSERT AFTER .text;\n')
        for opt in ['0', '2', 's']:
            self.run_cmd([self.rust, *rf, '-Copt-level=' + opt, ROOT / 'lib/win_minmax_rust.rs',
                '--emit=obj=' + str(self.out / 'rust.o') + ',llvm-ir=' + str(self.out / 'rust.ll')])
            self.run_cmd(['clang', *self.c_flags(), '-O' + opt, '-S', '-emit-llvm',
                          ROOT / 'lib/win_minmax.c', '-o', self.out / 'c.ll'], cwd=self.native)
            def ids(path):
                s = path.read_text()
                values = {n: int(v) & 0xffffffff for n, v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}$', s, re.M)}
                return {name: values[n] for name, n in re.findall(r'^define [^\n]*?@([\w]+)\([^\n]*!kcfi_type !(\d+)', s, re.M)}
            for name in ['minmax_running_min', 'minmax_running_max']:
                self.assertEqual(ids(self.out / 'c.ll')[name], ids(self.out / 'rust.ll')[name])
            from rust_exports_test_support import read_exports
            records = read_exports(self.out / 'rust.o')
            self.assertEqual({r['name'] for r in records}, {'minmax_running_min', 'minmax_running_max'})
            for record in records:
                self.assertEqual(record['license'], '')
                self.assertEqual(record['namespace'], '')
                self.assertEqual(record['relocation_target'], record['name'])
            self.assertEqual(self.run_cmd(['nm', '-u', self.out / 'rust.o']).stdout.strip(), '')
            self.run_cmd([self.rust, *rf, '-Copt-level=' + opt, self.out / 'header.rs',
                          '--emit=obj=' + str(self.out / 'header.o')])
            self.run_cmd(['clang', *self.c_flags(), '-O' + opt,
                '-Dminmax_running_min=c_minmax_running_min', '-Dminmax_running_max=c_minmax_running_max',
                '-c', ROOT / 'lib/win_minmax.c', '-o', self.out / 'c.o'], cwd=self.native)
            driver_flags = ['clang', '-O' + opt, '-fno-pie', '-fsanitize=kcfi',
                            '-fsanitize-cfi-icall-experimental-normalize-integers']
            self.run_cmd([*driver_flags, '-no-pie', '-Wl,-T,' + str(self.out / 'discard.lds'), self.out / 'driver.c', self.out / 'c.o',
                          self.out / 'rust.o', self.out / 'header.o', '-o', self.out / 'native-diff'])
            self.run_cmd([self.out / 'native-diff'])
            (self.out / 'cfi.c').write_text('#include "fixture.h"\n'
                'typedef u32 (*good_t)(struct minmax *,u32,u32,u32);\n'
                'typedef u32 (*bad_t)(struct minmax *,u32,u32);\n'
                'int main(int argc,char **argv) { (void)argv; struct minmax m; minmax_reset(&m,0,0);\n'
                'good_t volatile good = minmax_running_min; if(argc==1) return good(&m,10,1,0);\n'
                'bad_t volatile bad = (bad_t)minmax_running_min; return bad(&m,10,1); }\n')
            self.run_cmd([*driver_flags, '-no-pie', self.out / 'cfi.c', self.out / 'rust.o', '-o', self.out / 'cfi'])
            self.run_cmd([self.out / 'cfi'])
            self.run_cmd([self.out / 'cfi', 'bad'], expected=-4)
        dwarf = self.run_cmd([self.native / 'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions',
            '-T', self.out / 'types', self.out / 'rust.o'], input='minmax_running_min\nminmax_running_max\n')
        self.assertIn('minmax_running_min', dwarf.stdout)
        self.assertIn('minmax_sample', (self.out / 'types').read_text())
        self.run_cmd(['clang', *self.c_flags(), '-Os', '-c', ROOT / 'lib/win_minmax.c',
                      '-o', self.out / 'original.o'], cwd=self.native)
        original = self.run_cmd([self.native / 'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions',
            '-T', self.out / 'original.types', self.out / 'original.o'],
            input='minmax_running_min\nminmax_running_max\n')
        self.assertIn('minmax_running_max', original.stdout)
        self.assertIn('minmax_sample', (self.out / 'original.types').read_text())

    def test_kconfig(self):
        self.require_native()
        stanza = re.search(r'^config RUST_WIN_MINMAX\n.*?(?=^config |\Z)',
            (ROOT / 'lib/Kconfig').read_text(), re.M | re.S).group()
        (self.out / 'Kconfig').write_text('config RUST\n\tbool "Rust"\n\n' + stanza)
        self.env['KCONFIG_CONFIG'] = str(self.out / '.config')
        for config, expected in [('', False), ('CONFIG_RUST=y\n', False),
                ('CONFIG_RUST=n\nCONFIG_RUST_WIN_MINMAX=y\n', False),
                ('CONFIG_RUST=y\nCONFIG_RUST_WIN_MINMAX=y\n', True)]:
            (self.out / '.config').write_text(config)
            self.run_cmd([self.native / 'scripts/kconfig/conf', '--olddefconfig', self.out / 'Kconfig'])
            self.assertEqual('CONFIG_RUST_WIN_MINMAX=y' in (self.out / '.config').read_text(), expected)

    def test_kbuild(self):
        self.require_native()
        self.bindings()
        rf = self.native_fixture_flags()
        source = self.out / 'source'
        dependencies = ['lib/win_minmax.rs', 'include/linux/win_minmax_header.rs',
                        'rust/ffi_export.rs', 'include/linux/export_header.rs']
        for name in ['lib/Makefile', 'lib/win_minmax_rust.rs', 'lib/win_minmax.c', *dependencies]:
            path = source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, path)
        (self.out / 'lib').mkdir()
        # Bound the real archive build to this owner and its genuine adjacent
        # objects. No kernel build and no fabricated neighbor definitions.
        for neighbor in ['vsprintf.o', 'xarray.o']:
            shutil.copyfile(self.native / 'lib' / neighbor, self.out / 'lib' / neighbor)
        (source / 'lib/Kbuild').write_text('include ' + str(source / 'lib/Makefile') + '\n'
            'lib-y := $(filter vsprintf.o win_minmax.o win_minmax_rust.o xarray.o,$(lib-y))\n')
        (self.out / 'scripts/basic').mkdir(parents=True)
        shutil.copyfile(self.native / 'scripts/basic/fixdep', self.out / 'scripts/basic/fixdep')
        (self.out / 'scripts/basic/fixdep').chmod(0o755)
        # C relative include paths belong to the read-only donor tree.
        cf = []
        for arg in self.c_flags():
            if arg.startswith('-I./'):
                arg = '-I' + str(self.native / arg[4:])
            elif arg == '-Ilib':
                arg = '-I' + str(self.native / 'lib')
            cf.append(arg)
        wrapper = self.out / 'rules.mk'
        wrapper.write_text('include ' + str(ROOT / 'scripts/Makefile.build') + '\n'
            '$(info WIN_MINMAX_ARCHIVE_ORDER=$(lib-y))\n'
            'rust_common_cmd = ' + shlex.join([self.rust, *rf, '-Copt-level=s']) +
            ' --out-dir $(dir $@) --emit=dep-info=$(depfile)\n'
            'c_flags = ' + shlex.join(cf) + ' -Os -Wp,-MMD,$(depfile)\n'
            'lib/vsprintf.o lib/xarray.o: ;\n'
            '.PHONY: selected\nselected: $(filter lib/win_minmax.o lib/win_minmax_rust.o,$(lib-y))\n')
        cmd = ['make', '--no-print-directory', '-f', wrapper, 'lib/lib.a', 'obj=lib', 'AR=ar',
            'srctree=' + str(ROOT), 'srcroot=' + str(source), 'objtree=' + str(self.out),
            'VPATH=' + str(source), 'CC=clang']
        baseline = None
        for setting in ['', 'y', 'n', 'y']:
            p = self.run_cmd([*cmd, 'CONFIG_RUST_WIN_MINMAX=' + setting])
            order = re.search(r'WIN_MINMAX_ARCHIVE_ORDER=(.*)', p.stdout).group(1).split()
            selected = 'lib/win_minmax_rust.o' if setting == 'y' else 'lib/win_minmax.o'
            other = 'lib/win_minmax.o' if setting == 'y' else 'lib/win_minmax_rust.o'
            self.assertEqual(order.count(selected), 1)
            self.assertNotIn(other, order)
            normalized = [s.replace('win_minmax_rust.o', 'win_minmax.o') for s in order]
            if baseline is None:
                baseline = normalized
            self.assertEqual(normalized, baseline, 'original sorted lib.a slot changed')
            archive = self.out / 'lib/lib.a'
            members = self.run_cmd(['ar', 't', archive]).stdout.splitlines()
            self.assertEqual([Path(m).name for m in members],
                             ['vsprintf.o', Path(selected).name, 'xarray.o'])
            obj = self.out / selected
            before = obj.stat().st_mtime_ns
            archive_before = archive.stat().st_mtime_ns
            self.run_cmd([*cmd, 'CONFIG_RUST_WIN_MINMAX=' + setting])
            self.assertEqual(before, obj.stat().st_mtime_ns, 'no-op recompiled')
            self.assertEqual(archive_before, archive.stat().st_mtime_ns, 'no-op rearchived')
        saved = (self.out / 'lib/.win_minmax_rust.o.cmd').read_text()
        for name in dependencies:
            self.assertIn(name, saved)
            obj = self.out / 'lib/win_minmax_rust.o'
            before = obj.stat().st_mtime_ns
            path = source / name
            path.write_text(path.read_text() + '\n')
            self.run_cmd([*cmd, 'CONFIG_RUST_WIN_MINMAX=y'])
            self.assertGreater(obj.stat().st_mtime_ns, before, name)
            before = obj.stat().st_mtime_ns
            self.run_cmd([*cmd, 'CONFIG_RUST_WIN_MINMAX=y'])
            self.assertEqual(before, obj.stat().st_mtime_ns)

    def audit_selected(self, variable, machine):
        build = directory(variable, ['.config', 'lib/lib.a', 'lib/win_minmax_rust.o',
            'lib/.win_minmax_rust.o.cmd', 'vmlinux', 'Module.symvers',
            'rust/libkernel.rmeta', 'rust/libbindings.rmeta',
            'rust/bindings/bindings_generated.rs', 'scripts/gendwarfksyms/gendwarfksyms'])
        if build is None:
            self.skipTest(variable + ' not supplied: integrated native gate unproven')
        config = (build / '.config').read_text()
        self.assertIn('CONFIG_RUST_WIN_MINMAX=y\n', config)
        self.assertIn('CONFIG_RUST=y\n', config)
        generated = (build / 'rust/bindings/bindings_generated.rs').read_text()
        self.assertIn('pub struct minmax {', generated)
        self.assertIn('pub struct minmax_sample {', generated)
        owner = build / 'lib/win_minmax_rust.o'
        members = self.run_cmd(['ar', 't', build / 'lib/lib.a']).stdout.splitlines()
        names = [Path(m).name for m in members]
        self.assertEqual(names.count('win_minmax_rust.o'), 1)
        self.assertNotIn('win_minmax.o', names)
        position = names.index('win_minmax_rust.o')
        self.assertEqual(names[position-1:position+2], ['vsprintf.o', 'win_minmax_rust.o', 'xarray.o'])
        saved = (build / 'lib/.win_minmax_rust.o.cmd').read_text()
        for token in ['--extern kernel', '-Dwarnings', '-Zsanitizer=kcfi',
                      'win_minmax.rs', 'win_minmax_header.rs', 'ffi_export.rs', 'export_header.rs',
                      'libkernel.rmeta', 'libbindings.rmeta']:
            self.assertIn(token, saved)
        self.assertIn(machine, self.run_cmd(['readelf', '-h', owner]).stdout)
        from rust_exports_test_support import read_exports
        records = read_exports(owner)
        self.assertEqual(len(records), 2)
        for symbol in ['minmax_running_min', 'minmax_running_max']:
            record = next(r for r in records if r['name'] == symbol)
            self.assertEqual(record['license'], '')
            self.assertEqual(record['namespace'], '')
            self.assertEqual(record['relocation_target'], symbol)
            definitions = self.run_cmd(['nm', '-g', '--defined-only', build / 'vmlinux']).stdout
            self.assertEqual(len(re.findall(r'\bT ' + symbol + '$', definitions, re.M)), 1)
            rows = [r.split() for r in (build / 'Module.symvers').read_text().splitlines()
                    if len(r.split()) > 1 and r.split()[1] == symbol]
            self.assertEqual(len(rows), 1)
            self.assertEqual(rows[0][1:4], [symbol, 'vmlinux', 'EXPORT_SYMBOL'])
        dwarf = self.run_cmd([build / 'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions',
            '-T', self.out / 'integrated.types', owner], input='minmax_running_min\nminmax_running_max\n')
        self.assertIn('minmax_running_max', dwarf.stdout)
        self.assertIn('minmax_sample', (self.out / 'integrated.types').read_text())

    def test_selected_x86(self):
        self.audit_selected('NATIVE_WIN_MINMAX_KERNEL_BUILD', 'Advanced Micro Devices X86-64')

    def test_selected_arm64(self):
        self.audit_selected('NATIVE_WIN_MINMAX_ARM64_KERNEL_BUILD', 'AArch64')


if __name__ == '__main__':
    unittest.main()
