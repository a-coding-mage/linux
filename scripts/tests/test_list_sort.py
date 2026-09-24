# SPDX-License-Identifier: GPL-2.0-only
"""List-sort differential proof, discoverable with unittest.

LIST_SORT_NATIVE: optional genuine x86_64 KCFI kernel output tree containing
saved bcd_rust/list_sort commands, original list_sort.o, bindings, kernel rmeta,
gendwarfksyms, kconfig/conf and basic/fixdep. Never modified. Missing input skips native tests;
any explicitly supplied empty, incomplete or unusable input is an error.
LIST_SORT_RUSTC: compiler (default rustc; use matching native compiler >=1.85).
LIST_SORT_I686_SYSROOT: optional genuine i686 Rust core sysroot. Enables required
ELF32 execution: every nonzero exit, including SIGSYS, fails.
LIST_SORT_I686_RUNNER: optional shell-tokenized executable/arguments prefix;
requires the sysroot and native input. Executed without a shell.
All generated fixtures/objects live in TemporaryDirectory, never the source tree.
"""
import hashlib
import io
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

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / 'scripts/tests/list_sort_fixtures'


def input_directory(name, required_files):
    if name not in os.environ:
        return None
    value = os.environ[name]
    if not value.strip():
        raise ValueError(name + ' was supplied empty')
    path = Path(value).resolve()
    if not path.is_dir():
        raise ValueError(name + ' is not a directory: ' + str(path))
    for item in required_files:
        if not (path / item).is_file():
            raise ValueError(name + ' missing ' + item)
    return path


class NativeListSortArtifacts(unittest.TestCase):
    """Read-only audit of the actually linked native provider, not a test copy."""

    def audit(self, variable):
        from check_cordic_kernel import tool
        from check_div64_kernel import architecture, configuration, verify_build_command
        from check_polynomial_kernel import elf_target, newer
        from check_rational_kernel import compilation_flags
        from check_rust_exports_bridge import version_records
        from rust_exports_test_support import read_exports

        build = input_directory(variable, ['.config', 'vmlinux.a', 'vmlinux', 'Module.symvers',
                                'lib/list_sort_rust.o', 'lib/.list_sort_rust.o.cmd'])
        if build is None:
            self.skipTest(variable + ' not supplied: selected native artifact gate not run')
        config = configuration(build)
        self.assertEqual(config.get('RUST_LIST_SORT'), 'y')
        self.assertEqual(config.get('RUST'), 'y')
        arch = architecture(config)
        owner = build / 'lib/list_sort_rust.o'
        archive = build / 'vmlinux.a'
        members = [(build / os.fsdecode(line)).resolve()
                   for line in tool('ar', 't', archive).splitlines()]
        self.assertEqual(members.count(owner.resolve()), 1)
        self.assertNotIn((build / 'lib/list_sort.o').resolve(), members)
        position = members.index(owner.resolve())
        self.assertEqual(members[position - 1:position + 2],
                         [(build / name).resolve() for name in
                          ('lib/scatterlist.o', 'lib/list_sort_rust.o', 'lib/uuid.o')])
        dependencies = [ROOT / name for name in
                        ('lib/list_sort.rs', 'include/linux/list_sort_header.rs',
                         'rust/ffi_export.rs', 'include/linux/export_header.rs')]
        dependencies += [build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta']
        verify_build_command(build, owner, ROOT / 'lib/list_sort_rust.rs', dependencies)
        elf_target(owner, arch)
        self.assertEqual(tool('nm', '-u', owner).strip(), b'')
        self.assertEqual(len(re.findall(rb'\bT list_sort$',
                                      tool('nm', '--defined-only', '-g', owner), re.M)), 1)
        records = read_exports(owner)
        self.assertEqual(len(records), 1)
        expected = dict(name='list_sort', license='', namespace='',
                        relocation_target='list_sort', relocation_addend=0,
                        pointer_width=8, relocation_kind=257 if arch == 'aarch64' else 1,
                        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        for key, value in expected.items():
            self.assertEqual(records[0][key], value, key)
        if config.get('CFI') == 'y':
            self.assertIn('-Zsanitizer=kcfi', compilation_flags(owner))
        if config.get('MODVERSIONS') == 'y':
            rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()
                    if len(line.split()) > 1 and line.split()[1] == b'list_sort']
            self.assertEqual(len(rows), 1)
            self.assertEqual(rows[0][1:], [b'list_sort', b'vmlinux', b'EXPORT_SYMBOL'])
            self.assertEqual(version_records(owner.with_name('.' + owner.name + '.cmd')),
                             {b'list_sort': rows[0][0].lower()})
        newer(archive, [owner])
        newer(build / 'vmlinux', [archive])
        image = 'arch/arm64/boot/Image' if arch == 'aarch64' else 'arch/x86/boot/bzImage'
        newer(build / image, [build / 'vmlinux'])

    def test_selected_x86_kernel(self):
        self.audit('NATIVE_LIST_SORT_KERNEL_BUILD')

    def test_selected_arm64_kernel(self):
        self.audit('NATIVE_LIST_SORT_ARM64_KERNEL_BUILD')


class ListSortTest(unittest.TestCase):
    def setUp(self):
        self.native = input_directory('LIST_SORT_NATIVE', [
            'lib/.bcd_rust.o.cmd', 'lib/.list_sort.o.cmd', 'lib/list_sort.o',
            'rust/bindings/bindings_generated.rs', 'rust/libkernel.rmeta',
            'scripts/target.json', 'include/generated/rustc_cfg',
            'scripts/gendwarfksyms/gendwarfksyms', 'scripts/kconfig/conf',
            'scripts/basic/fixdep'])
        self.sysroot = input_directory('LIST_SORT_I686_SYSROOT', [])
        if self.sysroot:
            libdir = self.sysroot / 'lib/rustlib/i686-unknown-linux-gnu/lib'
            if not (libdir / 'libcore.rlib').is_file() and not list(libdir.glob('libcore-*.rlib')):
                raise ValueError('LIST_SORT_I686_SYSROOT has no genuine i686 core rlib')
        self.runner = []
        if 'LIST_SORT_I686_RUNNER' in os.environ:
            self.runner = shlex.split(os.environ['LIST_SORT_I686_RUNNER'])
            if not self.runner or not shutil.which(self.runner[0]):
                raise ValueError('LIST_SORT_I686_RUNNER must name an executable')
            if not self.sysroot:
                raise ValueError('LIST_SORT_I686_RUNNER requires LIST_SORT_I686_SYSROOT')
        if self.sysroot and not self.native:
            raise ValueError('LIST_SORT_I686_SYSROOT requires LIST_SORT_NATIVE for genuine bindings')
        self.rust = os.environ.get('LIST_SORT_RUSTC', 'rustc')
        if 'LIST_SORT_RUSTC' in os.environ and not shutil.which(self.rust):
            raise ValueError('LIST_SORT_RUSTC must name an executable')
        self.temp = tempfile.TemporaryDirectory(prefix='list-sort-test-')
        self.addCleanup(self.temp.cleanup)
        self.out = Path(self.temp.name)
        self.log = io.StringIO()
        self.env = {k: v for k, v in os.environ.items() if not k.startswith('KBUILD_')
                    and k not in ('MAKEFLAGS', 'MFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES',
                                  'srctree', 'srcroot', 'objtree', 'VPATH')}
        self.env.update(RUSTC_BOOTSTRAP='1', OBJTREE=str(self.native),
                        RUST_MODFILE='lib/list_sort_rust', LC_ALL='C')
        if self.native:
            caller = (FIXTURES / 'header_caller.rs.txt').read_text().replace(
                '../../include/linux/list_sort_header.rs', str(ROOT / 'include/linux/list_sort_header.rs'))
            (self.out / 'list_sort_header_caller.rs').write_text(caller)
            shutil.copyfile(FIXTURES / 'freestanding32.c.txt', self.out / 'list_sort_freestanding32.c')

    def require_native(self):
        if self.native is None:
            self.skipTest('LIST_SORT_NATIVE not supplied: genuine native gate not run')
        version = self.command([self.rust, '--version']).stdout
        match = re.search(r'rustc (\d+)\.(\d+)', version)
        self.assertIsNotNone(match, version)
        self.assertGreaterEqual(tuple(map(int, match.groups())), (1, 85))

    def test_native_differential_kcfi_semantics_and_versions(self):
        self.require_native()
        self.native_proof()

    def test_ordinary_elf64(self):
        self.require_native()
        self.ordinary_widths(64)

    def test_ordinary_elf32(self):
        self.require_native()
        if self.sysroot is None:
            self.skipTest('LIST_SORT_I686_SYSROOT not supplied: ELF32 gate not run')
        self.ordinary_widths(32)

    def test_kconfig_default_and_dependency(self):
        self.require_native()
        # Exercise the real component stanza with the real Kconfig evaluator.
        # Full architecture/configuration integration remains a kernel-build gate.
        stanza = re.search(r'^config RUST_LIST_SORT\n.*?(?=^config |\Z)',
                           (ROOT / 'lib/Kconfig').read_text(), re.M | re.S).group()
        kconfig = self.out / 'Kconfig'
        kconfig.write_text('config RUST\n\tbool "Rust"\n\n' + stanza)
        self.env['KCONFIG_CONFIG'] = str(self.out / '.config')
        for config, enabled in [('', False), ('CONFIG_RUST=y\n', False),
                                ('CONFIG_RUST=n\nCONFIG_RUST_LIST_SORT=y\n', False),
                                ('CONFIG_RUST=y\nCONFIG_RUST_LIST_SORT=y\n', True)]:
            (self.out / '.config').write_text(config)
            self.command([self.native / 'scripts/kconfig/conf', '--olddefconfig', kconfig])
            self.assertEqual('CONFIG_RUST_LIST_SORT=y' in
                             (self.out / '.config').read_text(), enabled)

    def test_kbuild_source_selection(self):
        # Evaluate the complete component Makefile through actual Kbuild rules.
        # Rust's recipe has '+', so -n alone still executes it. Suppress recipe
        # execution with a no-op shell while inspecting real rule expansion.
        # This proves rule selection, not compilation or incremental rebuilding.
        wrapper = self.out / 'rules.mk'
        wrapper.write_text('include ' + str(ROOT / 'scripts/Makefile.build') +
                           '\n$(info LIST_SORT_ORDER=$(real-obj-y))\n'
                           '.PHONY: selected\nselected: $(filter lib/list_sort.o lib/list_sort_rust.o,$(real-obj-y))\n')
        (self.out / 'lib').mkdir()
        for setting, owner, other in [('', 'list_sort.c', 'list_sort_rust.rs'),
                                       ('n', 'list_sort.c', 'list_sort_rust.rs'),
                                       ('y', 'list_sort_rust.rs', 'list_sort.c')]:
            result = self.command(['make', '--no-print-directory', '-n', '-f', wrapper,
                'selected', 'obj=lib', 'srctree=' + str(ROOT), 'srcroot=' + str(ROOT),
                'objtree=' + str(self.out), 'VPATH=' + str(ROOT),
                'CONFIG_RUST_LIST_SORT=' + setting, 'CC=clang',
                'RUSTC_OR_CLIPPY=' + self.rust, 'SHELL=' + shutil.which('true')])
            self.assertIn(str(ROOT / 'lib' / owner), result.stdout)
            self.assertNotIn(str(ROOT / 'lib' / other), result.stdout)
            order = re.search(r'^LIST_SORT_ORDER=(.*)$', result.stdout, re.M).group(1).split()
            selected = 'lib/list_sort_rust.o' if setting == 'y' else 'lib/list_sort.o'
            position = order.index(selected)
            self.assertEqual(order[position - 1:position + 2],
                             ['lib/scatterlist.o', selected, 'lib/uuid.o'])
            if setting == 'y':
                self.assertIn('--emit=dep-info=', result.stdout)
                self.assertIn('fixdep', result.stdout)

    def test_kbuild_incremental_dependencies(self):
        self.require_native()
        # A bounded owner-object build through unchanged Makefile.build rules.
        # Replay genuine native compiler flags, avoiding a synthetic kernel crate
        # or a full kernel configuration/build. Only copied sources are touched.
        source = self.out / 'source'
        dependencies = ['lib/list_sort.rs', 'include/linux/list_sort_header.rs',
                        'rust/ffi_export.rs', 'include/linux/export_header.rs']
        for name in ['lib/Makefile', 'lib/list_sort_rust.rs', *dependencies]:
            dest = source / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, dest)
        (self.out / 'lib').mkdir()
        (self.out / 'scripts/basic').mkdir(parents=True)
        shutil.copy2(self.native / 'scripts/basic/fixdep', self.out / 'scripts/basic/fixdep')
        compiler = shlex.join([self.rust, *self.native_rust_flags()])
        wrapper = self.out / 'dependencies.mk'
        wrapper.write_text('include ' + str(ROOT / 'scripts/Makefile.build') +
            '\nrust_common_cmd = ' + compiler +
            ' --out-dir $(dir $@) --emit=dep-info=$(depfile)\n')
        command = ['make', '--no-print-directory', '-f', wrapper, 'lib/list_sort_rust.o',
                   'obj=lib', 'srctree=' + str(ROOT), 'srcroot=' + str(source),
                   'objtree=' + str(self.out), 'VPATH=' + str(source),
                   'CONFIG_RUST_LIST_SORT=y']
        obj = self.out / 'lib/list_sort_rust.o'
        self.command(command)
        previous = obj.stat().st_mtime_ns
        self.command(command)
        self.assertEqual(obj.stat().st_mtime_ns, previous, 'unchanged build recompiled')
        saved = (self.out / 'lib/.list_sort_rust.o.cmd').read_text()
        for name in dependencies:
            recorded_paths = [Path(token).resolve() for token in saved.split()
                              if token.startswith(str(source))]
            self.assertIn((source / name).resolve(), recorded_paths)
            path = source / name
            path.write_text(path.read_text() + '\n')
            self.command(command)
            current = obj.stat().st_mtime_ns
            self.assertGreater(current, previous, 'dependency not rebuilt: ' + name)
            previous = current
            self.command(command)
            self.assertEqual(obj.stat().st_mtime_ns, previous)

    def command(self, args, expected=0, cwd=None, input_text=None):
        args = list(map(str, args))
        self.log.write('$ ' + shlex.join(args) + '\n')
        self.log.flush()
        p = subprocess.run(args, cwd=cwd or self.out, env=self.env, input=input_text,
                           text=True, capture_output=True, timeout=50,
                           preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        self.log.write(p.stdout + p.stderr + f'[exit {p.returncode}]\n')
        self.log.flush()
        if expected is not None:
            self.assertEqual(p.returncode, expected, self.log.getvalue())
        return p

    def ids(self, path):
        s = path.read_text()
        values = {n: int(v) & 0xffffffff for n, v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}$', s, re.M)}
        return {name: values[n] for name, n in re.findall(r'^define [^\n]*?@([\w]+)\([^\n]*!kcfi_type !(\d+)', s, re.M)}

    def native_rust_flags(self):
        line = (self.native / 'lib/.bcd_rust.o.cmd').read_text().splitlines()[0].split(' := ', 1)[1]
        args = shlex.split(line)
        # Skip saved environment assignments and compiler; caller chooses rustc.
        while '=' in args[0] and not args[0].startswith('-'):
            args.pop(0)
        args = args[1:-1]
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
                a = '--target=' + str(self.native / a.split('=./')[1])
            elif a.startswith('@./'):
                a = '@' + str(self.native / a[3:])
            elif a == './rust/':
                a = str(self.native / 'rust')
            result.append(a)
        return [*result, "-Dwarnings"]

    def c_flags(self):
        line = (self.native / 'lib/.list_sort.o.cmd').read_text().splitlines()[0].split(' := ', 1)[1]
        args = shlex.split(line)[1:]
        args = args[:args.index('-c')]
        saved_source = shlex.split(line)[-1]
        saved_root = str(Path(saved_source).parents[1])
        return [a.replace(saved_root, str(ROOT)) for a in args
                if not a.startswith('-Wp,-MMD,') and a not in ['-Os']]

    def ordinary_widths(self, bits):
        report = {"runs": []}
        # This reduced fixture is explicitly secondary to the full native proof.
        # Copy the exact real generated binding declaration, never a hand-written
        # replacement layout. It contains only pointers and is target-independent.
        generated = self.native/'rust/bindings/bindings_generated.rs'
        binding = re.search(r'pub struct list_head \{\n.*?\n\}', generated.read_text(), re.S).group()
        ctype = re.search(r'struct list_head \{\n.*?\n\};', (ROOT/'include/linux/types.h').read_text(), re.S).group()
        report['binding_source'] = str(generated)
        report['binding_sha256'] = hashlib.sha256(generated.read_bytes()).hexdigest()
        (self.out/'kernel_fixture.rs').write_text('#![no_std]\npub mod bindings {\n#[repr(C)]\n'+binding+'\n}\n')
        header = re.sub(r'^#include.*$', '', (ROOT/'include/linux/list_sort.h').read_text(), flags=re.M)
        (self.out/'fixture.h').write_text('typedef __SIZE_TYPE__ size_t;\n#define NULL ((void *)0)\n'+ctype+'\n'+header+
            '\n_Static_assert(sizeof(struct list_head) == 2*sizeof(void *), "size");\n'
            '_Static_assert(__builtin_offsetof(struct list_head, prev) == sizeof(void *), "prev");\n')
        oracle = re.sub(r'^#include.*$', '', (ROOT/'lib/list_sort.c').read_text(), flags=re.M)
        (self.out/'oracle.c').write_text('#include "fixture.h"\n#define likely(x) __builtin_expect(!!(x), 1)\n#define EXPORT_SYMBOL(x)\n'+oracle)
        driver = re.sub(r'^#include.*$', '', (ROOT/'scripts/tests/list_sort_fixtures/differential.c.txt').read_text(), flags=re.M)
        (self.out/'driver.c').write_text('#include "fixture.h"\n'+driver)
        if bits == 32 and self.runner:
            # Reject launchers that merely report success without running their
            # argument. A real runner must propagate this deliberate exit code.
            probe = self.out / 'runner_probe.c'
            probe.write_text('int main(int argc, char **argv) { (void)argc; (void)argv; return 73; }\n')
            self.command(['clang', '-m32', '-ffreestanding', '-fno-builtin', '-fno-pie',
                          '-nostdlib', '-static', '-Wl,-e,_start', probe,
                          self.out/'list_sort_freestanding32.c', '-o', self.out/'runner_probe'])
            self.command([*self.runner, self.out/'runner_probe'], expected=73)
        for bits in [bits]:
            target = [] if bits == 64 else ['--target=i686-unknown-linux-gnu', '--sysroot='+str(self.sysroot)]
            for opt in ['0', '2', 's']:

                d = self.out/f'ordinary{bits}-O{opt}'
                d.mkdir(exist_ok=True)
                flags = [self.rust, '--edition=2021', '--crate-type=rlib', '-Cpanic=abort', '-Dwarnings',
                         '-Dunsafe-op-in-unsafe-fn', '-Cdebug-assertions=n', '-Copt-level='+opt, *target]
                self.command([*flags, '--crate-name=kernel', self.out/'kernel_fixture.rs', '-o', d/'libkernel.rlib'])
                self.command([*flags, '-Zcrate-attr=no_std', '--extern=kernel='+str(d/'libkernel.rlib'),
                     ROOT/'lib/list_sort.rs', '--emit=obj='+str(d/'rust.o')+',llvm-ir='+str(d/'rust.ll')+',dep-info='+str(d/'rust.d')])
                cflags = ['clang', '-m'+str(bits), '-O'+opt, '-fno-pie', '-ffreestanding', '-fno-builtin', '-Wall', '-Wextra', '-Werror']
                self.command([*cflags, '-Dlist_sort=c_list_sort', '-c', self.out/'oracle.c', '-o', d/'c.o'])
                self.command([*cflags, '-c', self.out/'driver.c', '-o', d/'driver.o'])
                extra = ['-no-pie'] if bits == 64 else ['-nostdlib', '-static', '-Wl,-e,_start', self.out/'list_sort_freestanding32.c']
                self.command([*cflags, *extra, d/'driver.o', d/'rust.o', d/'c.o', '-o', d/'differential'])
                self.assertEqual((d/'rust.o').read_bytes()[4], 1 if bits == 32 else 2)
                self.assertEqual((d/'differential').read_bytes()[4], 1 if bits == 32 else 2)
                p = self.command([*(self.runner if bits == 32 else []), d/'differential'])
                report['runs'].append({'bits': bits, 'opt': opt, 'kind': 'ordinary ABI differential',
                                       'exit': p.returncode, 'stdout': p.stdout,
                                       'runtime_proven': p.returncode == 0})
        print(json.dumps(report, indent=2))

    def native_proof(self):

        report = {'rustc': self.command([self.rust, '-vV']).stdout, 'runs': [], 'ids': {}, 'sha256': {}}
        for f in ['lib/list_sort.c', 'lib/list_sort.rs', 'include/linux/list_sort.h', 'include/linux/list_sort_header.rs']:
            report['sha256'][f] = hashlib.sha256((ROOT/f).read_bytes()).hexdigest()
        rf, cf = self.native_rust_flags(), self.c_flags()
        self.assertIn('-Zsanitizer=kcfi', rf)
        self.assertIn('-fsanitize=kcfi', cf)
        for opt in ['0', '2', 's']:

            d = self.out / ('O' + opt)
            d.mkdir(exist_ok=True)
            self.command([self.rust, *rf, '-Copt-level='+opt, ROOT/'lib/list_sort_rust.rs',
                 '--emit=obj='+str(d/'rust.o')+',llvm-ir='+str(d/'rust.ll')+',dep-info='+str(d/'rust.d')])
            self.command([self.rust, *rf, '-Copt-level='+opt, self.out/'list_sort_header_caller.rs',
                 '--emit=obj='+str(d/'header_caller.o')])
            self.command(['clang', *cf, '-O'+opt, '-Dlist_sort=c_list_sort', '-c', ROOT/'lib/list_sort.c', '-o', d/'c.o'], cwd=self.native)
            self.command(['clang', *cf, '-O'+opt, '-Dlist_sort=c_list_sort', '-S', '-emit-llvm', ROOT/'lib/list_sort.c', '-o', d/'c.ll'], cwd=self.native)
            # The user-space driver calls libc: kernel-only varargs/stack shortcuts
            # do not satisfy libc's ABI. The two implementation objects retain them.
            driver_flags = [a for a in cf if a not in ['-mskip-rax-setup', '-mstack-alignment=8']]
            self.command(['clang', *driver_flags, '-O'+opt, '-DWITH_RUST_HEADER', '-x', 'c', '-c', ROOT/'scripts/tests/list_sort_fixtures/differential.c.txt', '-o', d/'driver.o'], cwd=self.native)
            self.command(['clang', '-no-pie', d/'driver.o', d/'header_caller.o', d/'rust.o', d/'c.o', '-o', d/'differential'])
            report['runs'].append({'opt': opt, 'kind': 'native differential', 'stdout': self.command([d/'differential']).stdout})
            for mode in ['inner', 'outer']:
                p = self.command([d/'differential', mode], -signal.SIGILL)
                report['runs'].append({'opt': opt, 'kind': mode+' bad KCFI control', 'exit': p.returncode})
            ci, ri = self.ids(d/'c.ll'), self.ids(d/'rust.ll')
            self.assertEqual(ci['c_list_sort'], ri['list_sort'], (ci, ri))
            report['ids'][opt] = {'C': ci['c_list_sort'], 'Rust': ri['list_sort']}
            (d/'symbols.txt').write_text(self.command(['readelf', '-Ws', d/'rust.o']).stdout)
            (d/'exports.txt').write_text(self.command(['readelf', '-x', '.export_symbol', '-r', d/'rust.o']).stdout)
            self.assertIn('list_sort', (d/'exports.txt').read_text())
            self.assertEqual(self.command(['nm', '-u', d/'rust.o']).stdout.strip(), '')
            definitions = self.command(['nm', '--defined-only', '-g', d/'rust.o']).stdout
            self.assertEqual(len(re.findall(r'\bT list_sort$', definitions, re.M)), 1)
        # Negative semantic controls prove that stable tie-taking and exact callback
        # traces are observed rather than only the final sorted keys.
        d = self.out/'controls'
        d.mkdir(exist_ok=True)
        source = (ROOT/'lib/list_sort.rs').read_text().replace(
            '../include/linux/list_sort_header.rs', str(ROOT/'include/linux/list_sort_header.rs'))
        mutants = {
            'unstable_ties': source.replace('cmp(priv_, a, b) <= 0', 'cmp(priv_, a, b) < 0'),
            'extra_self_compare': source.replace('            (*b).prev = tail;\n            tail = b;\n            b = (*b).next;\n            if b.is_null()',
                '            cmp(priv_, b, b);\n            (*b).prev = tail;\n            tail = b;\n            b = (*b).next;\n            if b.is_null()'),
        }
        for name, mutant in mutants.items():
            self.assertNotEqual(mutant, source)
            (d/(name+'.rs')).write_text(mutant)
            self.command([self.rust, *rf, '-Copt-level=2', d/(name+'.rs'), '--emit=obj='+str(d/(name+'.o'))])
            self.command(['clang', '-no-pie', self.out/'O2/driver.o', self.out/'O2/header_caller.o', self.out/'O2/c.o', d/(name+'.o'), '-o', d/name])
            self.command([d/name], -signal.SIGABRT)
            report['runs'].append({'kind': 'semantic negative control', 'name': name, 'exit': -signal.SIGABRT})

        dwarf = self.command([self.native/'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions',
                     '-T', self.out/'list_sort.symtypes', self.out/'Os/rust.o'], input_text='list_sort\n')
        (self.out/'gendwarfksyms.log').write_text(dwarf.stdout+dwarf.stderr)
        self.assertIn('list_sort', dwarf.stdout)
        self.assertIn('list_head', (self.out/'list_sort.symtypes').read_text())
        report['version'] = dwarf.stdout
        original = self.command([self.native/'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions',
                        '-T', self.out/'original.symtypes', self.native/'lib/list_sort.o'], input_text='list_sort\n')
        report['original_version'] = original.stdout
        self.assertIn('list_sort', original.stdout)
        self.assertIn('list_head', (self.out/'original.symtypes').read_text())
        # Cross-check the supplied artifact against a fresh unrenamed original-C
        # object so a stale native input cannot silently stand in for this source.
        self.command(['clang', *cf, '-Os', '-c', ROOT/'lib/list_sort.c',
                      '-o', self.out/'original-current.o'], cwd=self.native)
        rebuilt = self.command([self.native/'scripts/gendwarfksyms/gendwarfksyms',
            '--dump-versions', '-T', self.out/'original-current.symtypes',
            self.out/'original-current.o'], input_text='list_sort\n')
        self.assertEqual(rebuilt.stdout, original.stdout)
        self.assertEqual((self.out/'original-current.symtypes').read_text(),
                         (self.out/'original.symtypes').read_text())
        for dep in ['list_sort.rs', 'list_sort_header.rs', 'ffi_export.rs', 'export_header.rs']:
            self.assertIn(dep, (self.out/'O2/rust.d').read_text())
        report['rust_symtypes'] = (self.out/'list_sort.symtypes').read_text()
        report['original_symtypes'] = (self.out/'original.symtypes').read_text()
        print(json.dumps(report, indent=2))


if __name__ == '__main__':
    unittest.main()
