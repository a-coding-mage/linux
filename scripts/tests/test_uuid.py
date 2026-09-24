# SPDX-License-Identifier: GPL-2.0-only
"""UUID original-C differential, ABI and bounded Kbuild tests.

UUID_NATIVE_X86 / UUID_NATIVE_ARM64: optional read-only KCFI build inputs.
UUID_I686_SYSROOT: optional genuine Rust i686 core sysroot.
UUID_RUSTC: matching compiler >= 1.85 (default rustc).
Absent optional inputs skip their gates; explicit empty/invalid inputs fail.
All generated sources, commands and objects use TemporaryDirectory. Nonzero
runtime exits (including sandbox SIGSYS) fail. No native tree is modified.
"""
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / 'scripts/tests/uuid_fixtures'


def directory(name, files=()):
    if name not in os.environ:
        return None
    value = os.environ[name]
    if not value.strip():
        raise ValueError(name + ' is explicitly empty')
    path = Path(value).resolve()
    if not path.is_dir():
        raise ValueError(name + ' is not a directory')
    for item in files:
        if not (path / item).is_file():
            raise ValueError(name + ' missing ' + item)
    return path


def module(name):
    spec = importlib.util.spec_from_file_location('uuid_' + name, FIXTURES / (name + '.py'))
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


class UUIDTest(unittest.TestCase):
    def setUp(self):
        self.builds = {}
        for arch in ('x86', 'arm64'):
            build = directory('UUID_NATIVE_' + arch.upper(), [
                'rust/libkernel.rmeta', 'rust/libbindings.rmeta',
                'rust/bindings/bindings_generated.rs', 'lib/.scatterlist.o.cmd',
                'scripts/gendwarfksyms/gendwarfksyms', 'scripts/basic/fixdep',
                'scripts/kconfig/conf',
                'lib/.list_sort_rust.o.cmd' if arch == 'x86' else 'lib/math/.cordic_rust.o.cmd'])
            if build:
                self.builds[arch] = build
        self.sysroot = directory('UUID_I686_SYSROOT')
        if self.sysroot and not list((self.sysroot / 'lib/rustlib/i686-unknown-linux-gnu/lib').glob('libcore*.rlib')):
            raise ValueError('UUID_I686_SYSROOT lacks genuine i686 core')
        rust = os.environ.get('UUID_RUSTC', 'rustc')
        if not rust.strip() or not shutil.which(rust):
            raise ValueError('UUID_RUSTC must name an executable')
        self.temp = tempfile.TemporaryDirectory(prefix='uuid-test-')
        self.addCleanup(self.temp.cleanup)
        self.out = Path(self.temp.name)
        n = self.n = module('native')
        n.ROOT = n.WORK = ROOT
        n.OUT = self.out
        n.RUST = Path(shutil.which(rust))
        n.BUILDS = self.builds
        n.FIXTURES = self.out / 'fixtures'
        shutil.copytree(FIXTURES, n.FIXTURES)
        n.env = {k: v for k, v in os.environ.items() if not k.startswith('KBUILD_')
                 and k not in ('MAKEFLAGS', 'MFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES',
                               'srctree', 'srcroot', 'objtree', 'VPATH')}
        n.env.update(RUSTC_BOOTSTRAP='1', TMPDIR=str(self.out), LC_ALL='C')
        self.addCleanup(self.logs)
        version = n.command([n.RUST, '--version'], 'version')
        match = re.search(r'rustc (\d+)\.(\d+)', version)
        self.assertIsNotNone(match)
        self.assertGreaterEqual(tuple(map(int, match.groups())), (1, 85))
        for name in ('header.rs', 'host.rs'):
            path = n.FIXTURES / name
            path.write_text(path.read_text().replace('../include/linux/uuid_header.rs',
                str(ROOT / 'include/linux/uuid_header.rs')).replace('../lib/uuid.rs', str(ROOT / 'lib/uuid.rs')))
        hex_body = re.search(r'int hex_to_bin\(.*?\n}', (ROOT / 'lib/hexdump.c').read_text(), re.S).group()
        (n.FIXTURES / 'hex.c').write_text('#include <linux/hex.h>\n' + hex_body + '\n')

    def logs(self):
        # Captured by unittest callers; logs survive failures without output files
        # in the repository or an implicit persistent /tmp build directory.
        for path in sorted(self.out.glob('*.log')):
            print(path.name + '\n' + path.read_text())
        print(json.dumps(self.n.report, default=str))

    def require(self, arch='x86'):
        if arch not in self.builds:
            self.skipTest('UUID_NATIVE_' + arch.upper() + ' absent: native gate not run')
        return self.builds[arch]

    def test_native_x86(self):
        self.require()
        self.n.BUILDS = {'x86': self.builds['x86']}
        self.n.main()

    def test_native_arm64(self):
        self.require('arm64')
        self.n.BUILDS = {'arm64': self.builds['arm64']}
        self.n.main()

    def selected_view(self, arch):
        build = self.require(arch)
        view = self.out / ('selected-' + arch)
        view.mkdir()
        for path in build.iterdir():
            if path.name != 'lib':
                (view/path.name).symlink_to(path, target_is_directory=path.is_dir())
        (view/'lib').mkdir()
        for path in (build/'lib').iterdir():
            if path.name not in ('.uuid.o.cmd', 'uuid.o', 'uuid.symtypes'):
                (view/'lib'/path.name).symlink_to(path, target_is_directory=path.is_dir())
        self.assertFalse((view/'lib/.uuid.o.cmd').exists())
        self.n.BUILDS = {arch: view}
        flags = self.n.flags(view, False)
        self.assertTrue(all('scatterlist' not in item for item in flags))
        for value in ('-DKBUILD_MODFILE="lib/uuid"', '-DKBUILD_BASENAME="uuid"',
                      '-DKBUILD_MODNAME="uuid"', '-D__KBUILD_MODNAME=uuid'):
            self.assertEqual(flags.count(value), 1)
        self.n.main()

    def test_clean_rust_selected_x86_without_original_command(self):
        self.selected_view('x86')

    def test_clean_rust_selected_arm64_without_original_command(self):
        self.selected_view('arm64')

    def test_donor_rejects_wrong_source_output_and_object_identity(self):
        build = self.require()
        n = self.n
        view = self.out / 'bad-donor'
        (view/'lib').mkdir(parents=True)
        path = view/'lib/.scatterlist.o.cmd'
        original = (build/'lib/.scatterlist.o.cmd').read_text()
        changes = [original.replace('source_lib/scatterlist.o := ', 'source_lib/wrong.o := '),
                   original.replace('-o lib/scatterlist.o ', '-o lib/uuid.o '),
                   original.replace('KBUILD_BASENAME=', 'WRONG_BASENAME=')]
        for text in changes:
            self.assertNotEqual(text, original)
            path.write_text(text)
            with self.assertRaisesRegex(ValueError, 'scatterlist donor'):
                n.flags(view, False)
        path.write_text(original)
        audit_root = self.out/'per-file-audit'
        (audit_root/'lib').mkdir(parents=True)
        (audit_root/'lib/scatterlist.c').symlink_to(ROOT/'lib/scatterlist.c')
        n.ROOT = audit_root
        try:
            for setting in ('CFLAGS_scatterlist.o += -DONLY_DONOR',
                            'CFLAGS_REMOVE_uuid.o += -fsanitize=kcfi',
                            'uuid.o: private ccflags-y += -DONLY_UUID'):
                (audit_root/'lib/Makefile').write_text(
                    (ROOT/'lib/Makefile').read_text() + '\n' + setting + '\n')
                with self.assertRaisesRegex(ValueError, 'per-file compiler settings'):
                    n.flags(view, False)
        finally:
            n.ROOT = ROOT

    def ordinary(self, bits):
        build = self.require()
        n = self.n
        # Strip only includes from current original C, never keep a stale copied
        # algorithm/header as the oracle. This is the secondary ordinary ABI gate.
        def stripped(path):
            return re.sub(r'^#include[^\n]*', '', (ROOT / path).read_text(), flags=re.M)
        prefix = '''typedef __SIZE_TYPE__ size_t;
typedef unsigned char __u8;
typedef unsigned char u8;
typedef _Bool bool;
#define true 1
#define false 0
#define __must_check
#define __always_inline inline __attribute__((always_inline))
void *memcpy(void *, const void *, size_t);
int memcmp(const void *, const void *, size_t);
void get_random_bytes(void *, size_t);
int hex_to_bin(unsigned char);
#define EINVAL 22
#define EXPORT_SYMBOL(x)
#define EXPORT_SYMBOL_GPL(x)
'''
        (n.FIXTURES / 'ordinary.h').write_text(prefix + stripped('include/linux/uuid.h') + stripped('include/linux/ctype.h'))
        for name, source in [('ordinary_oracle.c', 'lib/uuid.c'), ('ordinary_ctype.c', 'lib/ctype.c')]:
            (n.FIXTURES / name).write_text('#include "ordinary.h"\n' + stripped(source))
        hex_body = re.search(r'int hex_to_bin\(.*?\n}', (ROOT / 'lib/hexdump.c').read_text(), re.S).group()
        (n.FIXTURES / 'ordinary_hex.c').write_text('#include "ordinary.h"\n' + hex_body + '\n')
        # Verify fixture nominal declarations against actual bindgen output.
        generated = (build / 'rust/bindings/bindings_generated.rs').read_text()
        fixture = (n.FIXTURES / 'kernel_fixture.rs').read_text()
        for kind in ('guid', 'uuid'):
            declaration = re.search(r'pub struct ' + kind + r'_t \{[^}]+}', generated).group()
            self.assertIn(declaration, fixture)
        module('ordinary').main(n, bits, self.sysroot)

    def test_ordinary_elf64(self):
        self.ordinary(64)

    def test_ordinary_elf32(self):
        if self.sysroot is None:
            self.skipTest('UUID_I686_SYSROOT absent: ELF32 runtime gate not run')
        self.ordinary(32)

    def test_initializer_negative_control(self):
        build = self.require()
        n = self.n
        n.env.update(OBJTREE=str(build), RUST_MODFILE='uuid_initializer')
        source = (ROOT / 'include/linux/uuid_header.rs').read_text()
        mutant = source
        for arg in ('a', 'b', 'c'):
            mutant = mutant.replace('((${}) as u32)'.format(arg), '(${})'.format(arg))
        self.assertNotEqual(mutant, source)
        (self.out / 'bad_header.rs').write_text(mutant)
        caller = (n.FIXTURES / 'header.rs').read_text()
        (self.out / 'bad.rs').write_text(caller.replace(str(ROOT / 'include/linux/uuid_header.rs'), str(self.out / 'bad_header.rs')))
        flags = n.flags(build, True)
        n.command([n.RUST, *flags, n.FIXTURES / 'header.rs', '--emit=obj=' + str(self.out / 'good.o')], 'initializer-good')
        n.command([n.RUST, *flags, self.out / 'bad.rs', '--emit=obj=' + str(self.out / 'bad.o')], 'initializer-bad', expected=1)
        self.assertIn('E0080', (self.out / 'initializer-bad.log').read_text())

    def test_kconfig(self):
        build = self.require()
        n = self.n
        stanza = re.search(r'^config RUST_UUID\n.*?(?=^config |\Z)', (ROOT / 'lib/Kconfig').read_text(), re.M | re.S).group()
        config = self.out / 'Kconfig'
        config.write_text('config RUST\n\tbool "Rust"\n\n' + stanza)
        n.env.update(KCONFIG_CONFIG=str(self.out / '.config'), KCONFIG_AUTOCONFIG=str(self.out / 'auto.conf'),
                     KCONFIG_AUTOHEADER=str(self.out / 'autoconf.h'), KCONFIG_RUSTCCFG=str(self.out / 'rustc_cfg'))
        for index, (text, enabled) in enumerate([('', False), ('CONFIG_RUST=y\n', False),
                ('CONFIG_RUST=n\nCONFIG_RUST_UUID=y\n', False), ('CONFIG_RUST=y\nCONFIG_RUST_UUID=y\n', True)]):
            (self.out / '.config').write_text(text)
            n.command([build / 'scripts/kconfig/conf', '--olddefconfig', config], 'kconfig-' + str(index))
            self.assertEqual('CONFIG_RUST_UUID=y' in (self.out / '.config').read_text(), enabled)

    def test_kbuild_switch_dependencies_noop_slot(self):
        build = self.require()
        n = self.n
        source = self.out / 'source'
        dependencies = ['lib/uuid.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs']
        for name in ['lib/Makefile', 'lib/uuid.c', 'include/linux/uuid.h', 'lib/uuid_rust.rs', *dependencies]:
            path = source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, path)
        (self.out / 'lib').mkdir()
        (self.out / 'scripts/basic').mkdir(parents=True)
        shutil.copy2(build / 'scripts/basic/fixdep', self.out / 'scripts/basic/fixdep')
        n.env.update(OBJTREE=str(build), RUST_MODFILE='lib/uuid_rust')
        wrapper = self.out / 'rules.mk'
        cf = ['-I' + str(source / 'include'), *n.flags(build, False)]
        # Saved C includes are relative to the read-only native output tree.
        cf = [('-I' + str(build / a[2:])) if a.startswith('-I') and not a[2:].startswith('/') else a for a in cf]
        for i, arg in enumerate(cf[:-1]):
            if arg == '-include' and not cf[i+1].startswith('/'):
                cf[i+1] = str(build / cf[i+1])
        wrapper.write_text('include ' + str(ROOT / 'scripts/Makefile.build') + '\n'
            'rust_common_cmd = ' + shlex.join([str(n.RUST), *n.flags(build, True)]) +
            ' --out-dir $(dir $@) --emit=dep-info=$(depfile)\n'
            'c_flags = ' + shlex.join(cf) + ' -Wp,-MMD,$(depfile)\n'
            '$(info UUID_ORDER=$(real-obj-y))\n.PHONY: selected\n'
            'selected: $(filter lib/uuid.o lib/uuid_rust.o,$(real-obj-y))\n')
        base = ['make', '--no-print-directory', '-f', wrapper, 'selected', 'obj=lib',
                'srctree=' + str(ROOT), 'srcroot=' + str(source), 'objtree=' + str(self.out),
                'VPATH=' + str(source), 'CC=clang']
        for turn, setting in enumerate(['', 'y', 'n', 'y']):
            command = [*base, 'CONFIG_RUST_UUID=' + setting]
            log = n.command(command, 'kbuild-switch-' + str(turn))
            owner = 'lib/uuid_rust.o' if setting == 'y' else 'lib/uuid.o'
            other = 'lib/uuid.o' if setting == 'y' else 'lib/uuid_rust.o'
            order = re.search(r'^UUID_ORDER=(.*)$', log, re.M).group(1).split()
            self.assertEqual(order.count(owner), 1)
            self.assertNotIn(other, order)
            position = order.index(owner)
            self.assertEqual(order[position-1:position+2], ['lib/list_sort.o', owner, 'lib/iov_iter.o'])
            obj = self.out / owner
            before = obj.stat().st_mtime_ns
            n.command(command, 'kbuild-noop-' + str(turn))
            self.assertEqual(before, obj.stat().st_mtime_ns)
        obj = self.out / 'lib/uuid_rust.o'
        saved = (self.out / 'lib/.uuid_rust.o.cmd').read_text()
        for index, name in enumerate(dependencies):
            recorded = [Path(token).resolve() for token in saved.split() if token.startswith(str(source))]
            self.assertIn((source / name).resolve(), recorded)
            before = obj.stat().st_mtime_ns
            path = source / name
            path.write_text(path.read_text() + '\n')
            n.command(command, 'kbuild-dependency-' + str(index))
            self.assertGreater(obj.stat().st_mtime_ns, before)
            before = obj.stat().st_mtime_ns
            n.command(command, 'kbuild-dependency-noop-' + str(index))
            self.assertEqual(obj.stat().st_mtime_ns, before)
        command = [*base, 'CONFIG_RUST_UUID=n']
        obj = self.out / 'lib/uuid.o'
        for index, name in enumerate(['lib/uuid.c', 'include/linux/uuid.h']):
            saved = (self.out / 'lib/.uuid.o.cmd').read_text()
            self.assertIn(str(source / name), saved)
            before = obj.stat().st_mtime_ns
            path = source / name
            path.write_text(path.read_text() + '\n')
            n.command(command, 'kbuild-c-dependency-' + str(index))
            self.assertGreater(obj.stat().st_mtime_ns, before)
            before = obj.stat().st_mtime_ns
            n.command(command, 'kbuild-c-noop-' + str(index))
            self.assertEqual(obj.stat().st_mtime_ns, before)
        # Exercise the real archive recipe as well. Keep the original Makefile
        # selection/order, filtering unrelated owners only to bound this private
        # build. Neighbors are inert object markers, not replacement algorithms.
        component = source / 'lib/Makefile'
        component.write_text(component.read_text() + '\n'
            'obj-y := $(filter list_sort.o uuid.o uuid_rust.o iov_iter.o,$(obj-y))\nlib-y :=\n')
        for name in ('list_sort', 'iov_iter'):
            (source / ('lib/' + name + '.c')).write_text('int uuid_slot_' + name + ';\n')
        archive = self.out / 'lib/built-in.a'
        archive_base = [arg if arg != 'selected' else 'lib/built-in.a' for arg in base]
        for index, setting in enumerate(('n', 'y', 'n', 'y')):
            command = [*archive_base, 'need-builtin=1', 'AR=llvm-ar', 'CONFIG_RUST_UUID=' + setting]
            n.command(command, 'kbuild-archive-switch-' + str(index))
            members = n.command(['llvm-ar', 't', archive], 'kbuild-archive-members-' + str(index))
            owner = 'uuid_rust.o' if setting == 'y' else 'uuid.o'
            self.assertEqual([Path(item).name for item in members.splitlines()],
                             ['list_sort.o', owner, 'iov_iter.o'])
            before = archive.stat().st_mtime_ns
            n.command(command, 'kbuild-archive-noop-' + str(index))
            self.assertEqual(archive.stat().st_mtime_ns, before)


class UUIDInputContract(unittest.TestCase):
    def test_explicit_invalid_optional_inputs_fail(self):
        # A requested gate must never become a skip due to an unusable path.
        with tempfile.TemporaryDirectory(prefix='uuid-input-test-') as temp:
            env = {k: v for k, v in os.environ.items() if not k.startswith('UUID_')}
            for name in ('UUID_NATIVE_X86', 'UUID_NATIVE_ARM64', 'UUID_I686_SYSROOT'):
                for value in ('', str(Path(temp) / 'missing'), temp):
                    with self.subTest(variable=name, value=value):
                        result = subprocess.run([sys.executable, '-B', str(Path(__file__).resolve()),
                            'UUIDTest.test_native_x86'], env={**env, name: value}, cwd=temp,
                            text=True, capture_output=True, timeout=30)
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn('ValueError: ' + name, result.stderr)
                        self.assertNotIn('OK (skipped=', result.stderr)


if __name__ == '__main__':
    unittest.main()
