# SPDX-License-Identifier: GPL-2.0-only
"""Native version ownership, final timestamp resolution and hostname behavior.

The original C owners and generated headers are the oracle. Kernel donors are
read-only; each build stage gets its own canonical UTS_VERSION binding input.
"""
import os
import shlex
import unittest

from rbtree_native import transport
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports
from test_hexdump_abi import ElfRecords
from test_sort_native import ids
from test_rational_build import run
import test_init_main_command_line as support
import test_init_main_bootconfig
import test_init_main_integration
import test_init_main_print

ROOT = support.ROOT
HEADER = ROOT / 'rust/bindings/init_version.h'


class InitVersion(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare
    rust_config = staticmethod(test_init_main_bootconfig.InitMainBootconfig.rust_config)

    def bindings(self, build, work, env, reader, stage, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'rust/bindings' / ('init_version_generated.rs' if stage == 'temporary'
                                             else 'init_version_timestamp_generated.rs')
        generated.parent.mkdir(parents=True, exist_ok=True)
        stamp = build / ('init/utsversion-tmp.h' if stage == 'temporary'
                         else 'include/generated/utsversion.h')
        parameters = shlex.split((ROOT / 'rust/init_version_bindgen_parameters').read_text(), comments=True)
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=ffi',
             '--no-layout-tests', '--no-doc-comments', '--no-debug=.*',
             *parameters, '-o', generated, '--', *flags, '-include', stamp, *extra],
            cwd=work, env=env)
        return generated

    def object_state(self, image, name):
        row, = [row for row in image.symbols if row[0] == name.encode()]
        _, section, start, size, info = row
        relocations = {}
        for record in image.sections:
            if record[1] != 4 or record[7] != section:
                continue
            for at in range(record[4], record[4] + record[5], record[9]):
                offset, details, addend = image.unpack('QQq', at)
                if start <= offset < start + size:
                    target = image.symbols[details >> 32]
                    relocations[offset - start] = (details & 0xffffffff, target[0], addend)
        return (size, info, image.sections[section][2] & 3,
                image.section(section)[start:start + size], relocations)

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=')
                     else flag[:-1] + ',linkage)' if flag.startswith('-Zcrate-attr=feature(')
                     else flag for flag in flags]
            for number, settings in enumerate(({}, {'UTS_NS': False},
                                               {'UTS_NS': True, 'PRINTK_INDEX': True},
                                               {'PRINTK': False, 'PRINTK_INDEX': False, 'LTO': True},
                                               {'SMP': False, 'QUEUED_SPINLOCKS': False, 'QUEUED_RWLOCKS': False},
                                               {'FORTIFY_SOURCE': True})):
                with self.subTest(settings=settings):
                    config = work / 'config.h'
                    config.write_text(''.join('#undef CONFIG_' + name + '\n' +
                        ('#define CONFIG_' + name + ' 1\n' if enabled else '')
                        for name, enabled in settings.items()) +
                        ('#undef CONFIG_BUILD_SALT\n#define CONFIG_BUILD_SALT "version\\0salt\\xff"\n'
                         '#undef CONFIG_DEFAULT_HOSTNAME\n#define CONFIG_DEFAULT_HOSTNAME "' + 'h' * 65 + '"\n'
                         if number else ''))
                    extra = ['-include', str(config)]
                    objects = {'c': [], 'rust': []}
                    for stage, basename in (('temporary', 'version'), ('final', 'version-timestamp')):
                        generated = self.bindings(build, work, env, reader, stage, extra)
                        self.assertTrue('pub ns_unified_node:' in generated.read_text(),
                                        'bindgen lost the canonical ns_tree arm')
                        rust, rllvm = work / (basename + '-rust.o'), work / (basename + '-rust.ll')
                        run([*self.rust_config(flags, settings), '--crate-name=' + basename.replace('-', '_'),
                             '--emit=obj=' + str(rust) + ',llvm-ir=' + str(rllvm),
                             ROOT / ('init/' + basename + '.rs')], cwd=work,
                            env={**env, 'OBJTREE': str(work)})
                        cflags = reader.native_flags(build, 'init/.' + basename + '.o.cmd', False)
                        original, cllvm = work / (basename + '-c.o'), work / (basename + '-c.ll')
                        run([*cflags, *extra, '-c', ROOT / ('init/' + basename + '.c'), '-o', original],
                            cwd=work, env=env)
                        run([*cflags, *extra, '-S', '-emit-llvm', ROOT / ('init/' + basename + '.c'),
                             '-o', cllvm], cwd=work, env=env)
                        c_image, r_image = ElfRecords(original), ElfRecords(rust)
                        for symbol in ('init_uts_ns', 'linux_banner'):
                            self.assertEqual(self.object_state(c_image, symbol),
                                             self.object_state(r_image, symbol), symbol)
                        if stage == 'temporary':
                            # The actual fortified C owner adds a bounded scan;
                            # the Rust owner must select the same primitive.
                            primitive_names = {b'sized_strscpy', b'strnlen'}
                            self.assertEqual({row[0] for row in c_image.symbols
                                              if row[1] == 0 and row[0] in primitive_names},
                                             {row[0] for row in r_image.symbols
                                              if row[1] == 0 and row[0] in primitive_names})
                            self.assertEqual(self.object_state(c_image, 'linux_proc_banner'),
                                             self.object_state(r_image, 'linux_proc_banner'))
                            self.assertEqual(read_exports(original), read_exports(rust))
                            if number == 0:
                                for owner in (original, rust):
                                    dwarf_versions(dwarf_tools(), owner, ['init_uts_ns'], work)
                            c_notes, = [i for i, name in enumerate(c_image.names) if name == b'.note.Linux']
                            r_notes, = [i for i, name in enumerate(r_image.names) if name == b'.note.Linux']
                            self.assertEqual(c_image.sections[c_notes][1:4], r_image.sections[r_notes][1:4])
                            self.assertEqual(c_image.sections[c_notes][8], r_image.sections[r_notes][8])
                            self.assertEqual(c_image.section(c_notes), r_image.section(r_notes))
                            records = test_init_main_integration.InitMainIntegration.setup_records
                            self.assertEqual(records(self, c_image, ids(cllvm)), records(self, r_image, ids(rllvm)))
                            index = test_init_main_print.InitMainPrint.index
                            indexed = b'.printk_index' in c_image.names
                            self.assertEqual(index(self, original, indexed), index(self, rust, indexed))
                        objects['c'].append(original)
                        objects['rust'].append(rust)
                    # The final symbol must win in either link order, including
                    # each self relocation originating in its namespace record.
                    for order in (False, True):
                        linked = []
                        for language, inputs in objects.items():
                            output = work / (language + '-linked.o')
                            run(['ld.lld', '-r', *(reversed(inputs) if order else inputs), '-o', output],
                                cwd=work, env=env)
                            linked.append(ElfRecords(output))
                        for symbol in ('init_uts_ns', 'linux_banner', 'linux_proc_banner'):
                            self.assertEqual(self.object_state(linked[0], symbol),
                                             self.object_state(linked[1], symbol), symbol)

    def test_native_x86_state_metadata_and_two_stage_link(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_state_metadata_and_two_stage_link(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def hostname(self, fortify, no_fortify=False):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=')
                     else flag[:-1] + ',linkage)' if flag.startswith('-Zcrate-attr=feature(')
                     else flag for flag in flags]
            cflags = reader.native_flags(build, 'init/.version.o.cmd', False)
            for printk in (False, True):
                settings = {'PRINTK': printk, 'PRINTK_INDEX': printk, 'FORTIFY_SOURCE': fortify}
                config = work / 'config.h'
                config.write_text(''.join('#undef CONFIG_' + name + '\n' +
                    ('#define CONFIG_' + name + ' 1\n' if enabled else '')
                    for name, enabled in settings.items()) +
                    ('#define __NO_FORTIFY\n' if no_fortify else ''))
                extra = ['-include', str(config)]
                for optimization in ('0', '2'):
                    with self.subTest(printk=printk, optimization=optimization,
                                      fortify=fortify, no_fortify=no_fortify):
                        # The C header gate includes __OPTIMIZE__ and the local
                        # opt-out, not just CONFIG_FORTIFY_SOURCE.
                        self.bindings(build, work, env, reader, 'temporary',
                                      [*extra, '-O' + optimization])
                        rust = work / 'hostname.o'
                        run([*self.rust_config(flags, settings), '-Copt-level=' + optimization,
                             '-Coverflow-checks=yes',
                             '--crate-name=version', '--emit=obj', ROOT / 'init/version.rs', '-o', rust],
                            cwd=work, env={**env, 'OBJTREE': str(work)})
                        # The bounded C size_t addition must not introduce a
                        # Rust panic dependency, even with overflow checks.
                        self.assertNotIn(b'panic_const_add_overflow',
                            run(['llvm-nm', '-u', rust], cwd=work, env=env).stdout)
                        image = ElfRecords(rust)
                        record, = [row for row in image.symbols if row[4] & 15 == 1 and row[1]
                                   and image.names[row[1]] == b'.init.setup']
                        name = record[0].decode()
                        # Expose the real registry entry, leaving its original
                        # callback and KCFI identity intact for indirect calls.
                        run(['llvm-objcopy', '--globalize-symbol=' + name,
                             '--redefine-sym=' + name + '=rust_hostname_parameter', rust], cwd=work, env=env)
                        objects = []
                        for source in (ROOT / 'scripts/tests/init_version_driver.c',
                                       ROOT / 'lib/string.c', ROOT / 'lib/ctype.c'):
                            obj = work / (source.stem + '.o')
                            driver = source.name == 'init_version_driver.c'
                            # Preserve original optimized string assembly (its
                            # BUG immediates require optimization); vary both
                            # hostname owners independently at O0 and O2.
                            chosen = [flag for flag in cflags if not flag.startswith('-m')] if driver else cflags
                            run([*chosen, *extra, '-O' + (optimization if driver else '2'),
                                 '-ffunction-sections', '-fdata-sections',
                                 '-c', source, '-o', obj], cwd=work, env=env)
                            objects.append(obj)
                        executable = work / 'hostname'
                        run(['clang', '-no-pie', '-Wl,--gc-sections', rust, *objects, '-o', executable],
                            cwd=work, env=env)
                        self.assertEqual(run([executable], cwd=work, env=env).stdout,
                                         b'INIT_VERSION_HOSTNAME_OK cases=4112\n')

    def test_original_hostname_callback_copy_truncation_and_warning(self):
        self.hostname(False)

    def test_original_fortified_hostname_and_header_opt_out(self):
        for no_fortify in (False, True):
            self.hostname(True, no_fortify)


if __name__ == '__main__':
    unittest.main()
