# SPDX-License-Identifier: GPL-2.0
"""Canonical initrd globals, boot records and original parser/load behavior."""
import json
import os
import shlex
import unittest

from rbtree_native import transport
from rust_exports_test_support import read_exports
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import run
from test_sort_native import ids
import test_init_main_integration as integration
import test_init_main_print as printing
import test_init_main_command_line as support

ROOT = support.ROOT
SOURCE = ROOT / 'init/do_mounts_initrd.rs'
HEADER = ROOT / 'rust/bindings/init_mounts.h'
OPTIONS = ('BLK_DEV_RAM', 'PRINTK', 'PRINTK_INDEX')
OWNED_GLOBALS = ('initrd_start', 'initrd_end', 'initrd_below_start_ok',
                 'phys_initrd_start', 'phys_initrd_size')


def compare_owners(case, rust, original, r_ids, c_ids, index, ram, printk, work, env):
    """Compare the complete owner's state, ABI and retained built-in records."""
    r_image, c_image = ElfRecords(rust), ElfRecords(original)
    for name in OWNED_GLOBALS:
        r = next(row for row in r_image.symbols if row[0] == name.encode())
        c = next(row for row in c_image.symbols if row[0] == name.encode())
        case.assertEqual(r[3:], c[3:], name)
        case.assertEqual(r_image.names[r[1]], c_image.names[c[1]], name)
        case.assertEqual(r_image.sections[r[1]][1], c_image.sections[c[1]][1], name)
        if r_image.sections[r[1]][1] != 8:
            case.assertEqual(r_image.section(r[1])[r[2]:r[2] + r[3]],
                             c_image.section(c[1])[c[2]:c[2] + c[3]], name)
    private = [row for row in r_image.symbols if b'MOUNT_INITRD' in row[0]]
    case.assertEqual(len(private), 1)
    case.assertEqual(r_image.names[private[0][1]], b'.init.data')
    case.assertEqual(r_ids['initrd_load'], c_ids['initrd_load'])
    load = next(row for row in r_image.symbols if row[0] == b'initrd_load')
    case.assertEqual(r_image.names[load[1]], b'.init.text')
    case.assertEqual(integration.InitMainIntegration.setup_records(case, r_image, r_ids),
                     integration.InitMainIntegration.setup_records(case, c_image, c_ids))
    case.assertEqual(printing.InitMainPrint.index(case, rust, index),
                     printing.InitMainPrint.index(case, original, index))
    case.assertEqual(read_exports(rust), [])
    case.assertEqual(read_exports(original), [])
    undefined = run(['llvm-nm', '-u', rust], cwd=work, env=env).stdout
    for name in (b'memparse', b'init_mknod', b'init_unlink'):
        case.assertIn(name, undefined)
    case.assertEqual(b'rd_load_image' in undefined, ram)
    case.assertEqual(b'_printk' in undefined, printk)
    for name in (b'create_dev', b'new_encode_dev', b'Root_RAM0', b'pr_warn'):
        case.assertNotIn(name, undefined)


class InitMountsInitrd(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def settings(self, work, enabled):
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + name + '\n' +
            ('#define CONFIG_' + name + ' 1\n' if name in enabled else '') for name in OPTIONS))
        return ['-include', str(config)]

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'rust/bindings/init_mounts_generated.rs'
        generated.parent.mkdir(parents=True, exist_ok=True)
        parameters = shlex.split((ROOT / 'rust/init_mounts_bindgen_parameters').read_text(), comments=True)
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=ffi',
             '--no-layout-tests', '--no-doc-comments', *parameters,
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            env = {**env, 'OBJTREE': str(work)}
            cflags = reader.native_flags(build, 'init/.do_mounts_initrd.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            for ram in (False, True):
                for printk, index in ((False, False), (True, False), (True, True)):
                    with self.subTest(ram=ram, printk=printk, index=index):
                        enabled = ({'BLK_DEV_RAM'} if ram else set()) | ({'PRINTK'} if printk else set()) | ({'PRINTK_INDEX'} if index else set())
                        extra = self.settings(work, enabled)
                        self.bindings(build, work, env, reader, extra)
                        flags = [flag for flag in rflags if not any(flag == '--cfg=CONFIG_' + name or
                            flag.startswith('--cfg=CONFIG_' + name + '=') for name in OPTIONS)]
                        flags += ['--cfg=CONFIG_' + name for name in enabled]
                        rust, original = work / 'rust.o', work / 'c.o'
                        rir, cir = work / 'rust.ll', work / 'c.ll'
                        run([*flags, '--crate-name=init_mounts_initrd', '--emit=obj=' + str(rust) + ',llvm-ir=' + str(rir),
                             SOURCE], cwd=work, env=env)
                        run([*cflags, *extra, '-c', ROOT / 'init/do_mounts_initrd.c', '-o', original], cwd=work, env=env)
                        run([*cflags, *extra, '-S', '-emit-llvm', ROOT / 'init/do_mounts_initrd.c', '-o', cir], cwd=work, env=env)
                        compare_owners(self, rust, original, ids(rir), ids(cir), index, ram, printk, work, env)

    def test_native_x86_state_records_and_abi(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_state_records_and_abi(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_parser_and_load_sequence(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            env = {**env, 'OBJTREE': str(work)}
            cflags = reader.native_flags(build, 'init/.do_mounts_initrd.o.cmd', False)
            integers = (ROOT / 'lib/kstrtox.c').read_text()
            printf = (ROOT / 'lib/vsprintf.c').read_text()
            parser = work / 'parser.c'
            parser.write_text('#include <linux/kernel.h>\n#include <linux/ctype.h>\n'
                '#include <linux/overflow.h>\n#include <linux/limits.h>\n#include ' +
                json.dumps(str(ROOT / 'lib/kstrtox.h')) + '\n' +
                function(integers, '_parse_integer_fixup_radix') + function(integers, '_parse_integer_limit') +
                function(printf, 'simple_strntoull') + function(printf, 'simple_strtoull') +
                function((ROOT / 'lib/cmdline.c').read_text(), 'memparse'))
            linker = work / 'records.lds'
            linker.write_text('''SECTIONS {
  .init.setup : { setup_begin = .; KEEP(*(.init.setup)) setup_end = .; }
  /DISCARD/ : { *(.discard.*) }
} INSERT AFTER .data;
''')
            for ram in (False, True):
                for printk in (False, True):
                    enabled = ({'BLK_DEV_RAM'} if ram else set()) | ({'PRINTK'} if printk else set())
                    extra = self.settings(work, enabled)
                    self.bindings(build, work, env, reader, extra)
                    for optimization in ('0', '2'):
                        with self.subTest(ram=ram, printk=printk, optimization=optimization):
                            objects = []
                            for source in (parser, ROOT / 'lib/ctype.c'):
                                obj = work / (source.stem + '.o')
                                run([*cflags, *extra, '-O' + optimization, '-c', source, '-o', obj], cwd=work, env=env)
                                objects.append(obj)
                            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                            flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Coverflow-checks=yes',
                                     '-Copt-level=' + optimization, '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                            ffi, kernel = work / 'libffi.rlib', work / 'libkernel.rlib'
                            run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                            (work / 'kernel.rs').write_text('#![no_std]\npub extern crate ffi;\n')
                            run([*rustc, *flags, '--crate-name=kernel', '--crate-type=rlib', work / 'kernel.rs',
                                 '--extern', 'ffi=' + str(ffi), '-o', kernel], cwd=work, env=env)
                            archive = work / 'rust.a'
                            run([*rustc, *flags, '--crate-name=init_mounts_initrd', '--crate-type=staticlib',
                                 *['--cfg=CONFIG_' + name for name in enabled], '--extern', 'kernel=' + str(kernel),
                                 '-Ldependency=' + str(work), SOURCE, '-o', archive], cwd=work, env=env)
                            original = work / 'original.o'
                            run([*cflags, *extra, '-O' + optimization, '-c', ROOT / 'init/do_mounts_initrd.c', '-o', original], cwd=work, env=env)
                            binaries = []
                            for owner in (original, archive):
                                binary = work / (owner.stem + '-compare')
                                service = [flag for flag in cflags if not flag.startswith('-m')]
                                run([*service, *extra, '-O' + optimization,
                                     ROOT / 'scripts/tests/init_mounts_initrd_driver.c', owner, *objects, '-no-pie',
                                     '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                                binaries.append(binary)
                            for disabled in (0, 1):
                                for result in (-17, 0, 1):
                                    for unlink in (0, -2):
                                        for mknod in (0, -13):
                                            arguments = list(map(str, (disabled, result, unlink, mknod)))
                                            expected = run([binaries[0], *arguments], cwd=work, env=env).stdout
                                            self.assertTrue(expected.startswith(b'INIT_MOUNTS_INITRD_OK '))
                                            self.assertEqual(run([binaries[1], *arguments], cwd=work, env=env).stdout, expected)


if __name__ == '__main__':
    unittest.main()
