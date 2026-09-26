# SPDX-License-Identifier: GPL-2.0-only
"""Original early-cpio parser, real header layout and original strscpy backend.

All fixtures stay outside the source tree. The kernel logging boundary is
captured, while original parser, alignment macros, name copying and word helpers
are compiled unchanged. Malformed matching names retain readable backing storage
for the original C string operations rather than invoking undefined reads.
"""
import os
from pathlib import Path
import re
import shlex
import shutil
import stat
import sys
import tempfile
import unittest

import boot_kernel
from kconfig_test_support import cached_conf_tools
from kernel_console import normalize_console_transport
from rbtree_native.transport import compiler_environment, NativeWriteWatch, outside
from rust_exports_test_support import rust_targets, read_exports
from test_argv_split import function
from test_errname import macro
from test_hexdump_abi import ElfRecords
from test_hexdump_kcfi import C_KCFI, RUST_KCFI, type_ids
from test_rational_build import environment, run


ROOT = Path(__file__).resolve().parents[2]
MICROCODE_PREFIX = b'kernel/x86/microcode/GenuineIntel.bin'
EMPTY_MEMBER = MICROCODE_PREFIX + b'.cpio-parser-proof'
EARLY_WARNING = b'File ' + EMPTY_MEMBER + b' exceeding MAX_CPIO_FILE_NAME [18]'


def verify_early_consumer(console, expected):
    """Require the exact original early warning, before the ordinary boot gate."""
    records = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip(), count=1)
               for line in normalize_console_transport(console).splitlines()]
    warnings = [line for line in records if b'exceeding MAX_CPIO_FILE_NAME' in line]
    if warnings != ([EARLY_WARNING] if expected else []):
        raise ValueError('missing, duplicate, malformed or unrequested early CPIO warning')
    boot_kernel.verify_module_events(console)
    if expected and records.index(EARLY_WARNING) >= records.index(boot_kernel.MARKER):
        raise ValueError('early CPIO warning did not precede successful userspace boot')


class EarlyCpio(unittest.TestCase):
    def setUp(self):
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT,))
        temporary = tempfile.TemporaryDirectory(prefix='earlycpio-', dir=parent)
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.env = dict(environment(), RUSTC_BOOTSTRAP='1')
        self.rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        self.bindgen = shlex.split(os.environ.get('BINDGEN', 'bindgen'))

    def headers(self):
        include = self.work / 'include'
        (include / 'linux').mkdir(parents=True, exist_ok=True)
        (include / 'asm').mkdir(exist_ok=True)
        (include / 'linux/types.h').write_text('#include <stddef.h>\ntypedef __PTRDIFF_TYPE__ ssize_t;\n')
        (include / 'linux/bitops.h').write_text('/* Generic little-endian word helpers need no bitops. */\n')
        (include / 'asm/byteorder.h').write_text('#undef __BIG_ENDIAN\n#define __LITTLE_ENDIAN 1234\n')
        printk = (ROOT / 'include/linux/printk.h').read_text()
        entry = re.search(r'struct pi_entry \{.*?\} __packed;', printk, re.S).group()
        # This macro contains multiline comments whose physical lines need no
        # continuation slash; retain its complete original conditional arm.
        index_macro = printk[printk.index('#define __printk_index_emit'):
                             printk.index('#else /* !CONFIG_PRINTK_INDEX */')]
        logging = ('#include <linux/compiler_attributes.h>\n#include <linux/kern_levels.h>\n' +
                   'int _printk(const char *format, ...);\n' + entry + '\n' +
                   '#define pr_fmt(fmt) fmt\n#ifdef CONFIG_PRINTK_INDEX\n' +
                   index_macro +
                   '#else\n#define __printk_index_emit(...) do {} while (0)\n#endif\n' +
                   macro(printk, 'printk_index_wrap') + macro(printk, 'printk') + macro(printk, 'pr_warn'))
        (include / 'linux/kernel.h').write_text('#ifndef EARLY_CPIO_KERNEL_H\n#define EARLY_CPIO_KERNEL_H\n'
            '#include <stddef.h>\n#include <linux/align.h>\n' + logging + '\n#endif\n')
        declaration = re.search(r'^ssize_t sized_strscpy\([^\n]*;',
                                (ROOT / 'include/linux/string.h').read_text(), re.M).group()
        (include / 'linux/string.h').write_text('#ifndef EARLY_CPIO_STRING_H\n#define EARLY_CPIO_STRING_H\n'
            '#include <linux/types.h>\n#ifndef EARLY_CPIO_BINDGEN\n#include <string.h>\n#endif\n' +
            'extern size_t strlen(const char *);\nextern int memcmp(const void *, const void *, size_t);\n' +
            declaration + '\n#define strscpy(dst, src, size) sized_strscpy(dst, src, size)\n#endif\n')
        source = ('#include <limits.h>\n#include <errno.h>\n#include <linux/string.h>\n'
                  '#include <linux/kernel.h>\n#include <asm-generic/word-at-a-time.h>\n'
                  '#define WARN_ON_ONCE(value) (value)\n#define IS_ENABLED(option) 0\n'
                  '#define unlikely(value) (value)\n#define ALLBUTLAST_BYTE_MASK (~0ul >> 8)\n'
                  '#define kasan_check_read(pointer, size) ((void)0)\n'
                  '#define kcsan_check_read(pointer, size) ((void)0)\n' +
                  function((ROOT / 'include/asm-generic/rwonce.h').read_text(), 'read_word_at_a_time') +
                  function((ROOT / 'lib/string.c').read_text(), 'sized_strscpy'))
        (self.work / 'strings.c').write_text(source)
        (self.work / 'bindings.h').write_text('#include <linux/earlycpio.h>\n'
                                             '#include <linux/kernel.h>\n#include <linux/string.h>\n')
        return ['-I' + str(include), '-idirafter', str(ROOT / 'include')]

    def build(self, bits, optimization, compiler, *, printk=True, index=True, protected=False):
        include = self.headers()
        config = ['CONFIG_PRINTK'] if printk else []
        if index:
            config += ['CONFIG_PRINTK_INDEX']
        if bits == 64:
            config += ['CONFIG_64BIT']
        cflags = [*include, '-O' + optimization, '-funsigned-char', '-fno-strict-overflow',
                  '-Wall', '-Wextra', '-Wno-unused-parameter', '-Werror', '-fno-strict-aliasing',
                  *['-D' + item for item in config], *([*C_KCFI] if protected else [])]
        binding = self.work / 'bindings.rs'
        run([*self.bindgen, self.work / 'bindings.h', '--use-core', '--rust-target', '1.85',
             '--ctypes-prefix', 'ffi', '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type', 'cpio_data|pi_entry', '--allowlist-var', 'MAX_CPIO_FILE_NAME|KERN_WARNING',
             '--allowlist-function', 'find_cpio_data|strlen|memcmp|sized_strscpy|_printk',
             '-o', binding, '--', '-m' + str(bits), '-ffreestanding', '-DEARLY_CPIO_BINDGEN',
             *include], env=self.env)
        targets = rust_targets()
        if bits not in targets:
            self.skipTest('the actual i686 Rust standard library is required')
        rflags = [*targets[bits], '--edition=2021', '-Dwarnings', '-Cpanic=abort',
                  '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                  *[flag for item in config for flag in ('--cfg', item)],
                  *([*RUST_KCFI] if protected else [])]
        ffi = self.work / 'libffi.rlib'
        run([*self.rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs',
             '-o', ffi], env=self.env)
        wrapper = self.work / 'owner.rs'
        wrapper.write_text('//! Execute the actual owner with the actual header declarations.\n'
            'extern crate self as kernel;\npub extern crate ffi;\n'
            '#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs)]\n'
            'pub mod bindings { include!("' + str(binding) + '"); }\n'
            '#[path="' + str(ROOT / 'lib/earlycpio_rust.rs') + '"] mod owner;\n'
            '#[path="' + str(ROOT / 'include/linux/earlycpio_header.rs') + '"] pub mod declarations;\n'
            'pub use owner::find_cpio_data;\n')
        obj, lib, ir = self.work / 'owner.o', self.work / 'owner.a', self.work / 'owner.ll'
        run([*self.rustc, *rflags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
             '-Wmissing-docs', '-Wunsafe-op-in-unsafe-fn', '-Wunreachable-pub',
             '--emit=obj=' + str(obj), '--emit=link=' + str(lib), '--emit=llvm-ir=' + str(ir),
             wrapper], env=self.env)
        self.assertEqual(read_exports(obj), [])
        imports = run(['nm', '-u', obj], env=self.env).stdout
        for forbidden in (b'pr_warn', b' malloc', b'kmalloc', b' strscpy'):
            self.assertNotIn(forbidden, imports)
        self.assertEqual(b'_printk' in imports, printk)
        image = ElfRecords(obj)
        self.assertEqual(b'.printk_index' in image.names, printk and index)
        if printk and index:
            section = image.names.index(b'.printk_index')
            self.assertEqual(len(image.section(section)), image.word)
            target, start = image.relocations[(section, 0)]
            self.assertEqual(image.pointer_string(target, start),
                             b'\x014File %s exceeding MAX_CPIO_FILE_NAME [%d]\n')
            self.assertEqual(image.pointer_string(target, start + image.word), b'find_cpio_data')
            self.assertTrue(image.pointer_string(target, start + 2 * image.word).endswith(b'lib/earlycpio.rs'))
            self.assertIsNone(image.pointer_string(target, start + 3 * image.word + 4))
            self.assertIsNone(image.pointer_string(target, start + 4 * image.word + 4))
        original = self.work / 'original.o'
        renames = ['-Dfind_cpio_data=c_find_cpio_data']
        run([*compiler, *cflags, *renames, '-c', ROOT / 'lib/earlycpio.c', '-o', original], env=self.env)
        if protected:
            cir = self.work / 'original.ll'
            run([*compiler, *cflags, *renames, '-S', '-emit-llvm', ROOT / 'lib/earlycpio.c', '-o', cir], env=self.env)
            self.assertEqual(type_ids(cir)['c_find_cpio_data'], type_ids(ir)['find_cpio_data'])
        binary = self.work / 'compare'
        run([*compiler, *cflags, ROOT / 'scripts/tests/earlycpio_driver.c', self.work / 'strings.c',
             original, lib, '-ldl', '-lpthread', '-lm', '-no-pie', '-o', binary], env=self.env)
        self.assertEqual(binary.read_bytes()[4], 1 if bits == 32 else 2)
        self.assertEqual(run([binary], env=self.env).stdout,
                         b'earlycpio: headers, bounds, iteration, names and diagnostics passed\n')

    def test_original_parser_and_diagnostics(self):
        for compiler in (shlex.split(os.environ.get('HOSTCC', 'cc')), ['clang']):
            for optimization in ('0', '2', 's'):
                with self.subTest(compiler=compiler, optimization=optimization):
                    self.build(64, optimization, compiler)

    def test_real_i686_layout_long_output_and_bounds(self):
        compiler = shlex.split(os.environ.get('EARLYCPIO_I686_CC', 'cc -m32'))
        for optimization in ('0', '2', 's'):
            self.build(32, optimization, compiler)

    def test_printk_configuration_and_protected_struct_return(self):
        for printk, index in ((False, False), (True, False), (True, True)):
            with self.subTest(printk=printk, index=index):
                self.build(64, '2', ['clang'], printk=printk, index=index, protected=True)


class EarlyCpioSelection(unittest.TestCase):
    def setUp(self):
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT,))
        temporary = tempfile.TemporaryDirectory(prefix='earlycpio-selection-', dir=parent)
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def test_actual_kconfig_requires_native_rust_and_explicit_opt_in(self):
        found = re.search(r'(?ms)^config RUST_EARLYCPIO\n.*?(?=^config |\Z)',
                          (ROOT / 'lib/Kconfig').read_text())
        self.assertIsNotNone(found)
        source = self.work / 'Kconfig'
        source.write_text('config RUST\n\tbool "Rust"\n\n' + found.group())
        env = dict(environment(), KCONFIG_CONFIG=str(self.work / '.config'))
        for tool in cached_conf_tools():
            for rust in ('n', 'y'):
                for requested in (None, 'n', 'y'):
                    with self.subTest(tool=tool.name, rust=rust, requested=requested):
                        config = 'CONFIG_RUST=' + rust + '\n'
                        if requested is not None:
                            config += 'CONFIG_RUST_EARLYCPIO=' + requested + '\n'
                        (self.work / '.config').write_text(config)
                        run([tool, '--olddefconfig', source], cwd=self.work, env=env)
                        self.assertEqual('CONFIG_RUST_EARLYCPIO=y' in
                                         (self.work / '.config').read_text().splitlines(),
                                         rust == requested == 'y')

    def test_actual_makefile_c_rust_c_preserves_archive_position_and_owner(self):
        makefile = self.work / 'Makefile'
        makefile.write_text(f'''srctree := {ROOT}
src := {ROOT}/lib
obj := lib
objtree := {self.work}
include {ROOT}/lib/Makefile
.PHONY: earlycpio-selection lib/earlycpio_rust.o
earlycpio-selection:
\t@printf '%s\\n' '$(lib-y)' '$(filter earlycpio%.o,$(obj-y) $(obj-m))'
lib/earlycpio_rust.o:
\t@printf '%s\\n' '$(modfile)'
''')
        for host in ('c', 'rust'):
            for native in ('n', 'y', 'n'):
                command = [*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory',
                           '-rR', '-f', makefile, 'HOST_TOOLS_LANG=' + host,
                           'CONFIG_RUST=y', 'CONFIG_RUST_EARLYCPIO=' + native]
                result = run([*command, 'earlycpio-selection'], cwd=self.work, env=environment())
                archive, objects = result.stdout.splitlines()
                names = archive.split()
                expected = b'earlycpio_rust.o' if native == 'y' else b'earlycpio.o'
                self.assertEqual([name for name in names if name.startswith(b'earlycpio')], [expected])
                position = names.index(expected)
                self.assertEqual(names[position - 1:position + 2],
                                 [b'kobject_uevent.o', expected, b'seq_buf.o'])
                self.assertEqual(objects, b'')
                self.assertEqual(result.stderr, b'')
                if native == 'y':
                    owner = run([*command, 'lib/earlycpio_rust.o'], cwd=self.work, env=environment())
                    self.assertEqual(owner.stdout, b'lib/earlycpio\n')


class EarlyCpioNative(unittest.TestCase):
    """Use the original early Intel consumer without supplying microcode data.

    A zero-length regular member matches the consumer's filename prefix and
    emits the original long-name warning. Intel get_microcode_blob rejects its
    zero size before scanning or applying an update. The hypervisor CPUID bit
    supplies an independent negative control for reaching that early consumer.
    """

    def test_early_console_requires_exact_once_before_boot(self):
        boot = boot_kernel.MARKER + b'\n'
        verify_early_consumer(boot, False)
        verify_early_consumer(b'[    0.000000] ' + EARLY_WARNING + b'\r\n' + boot, True)
        for console in (boot, EARLY_WARNING + b'\n' + EARLY_WARNING + b'\n' + boot,
                        b'prefix ' + EARLY_WARNING + b'\n' + boot,
                        boot + EARLY_WARNING + b'\n', EARLY_WARNING + b'\n'):
            with self.subTest(console=console), self.assertRaises(ValueError):
                verify_early_consumer(console, True)
        with self.assertRaises(ValueError):
            verify_early_consumer(EARLY_WARNING + b'\n' + boot, False)

    def audit_provider(self, build, selected, arch, command):
        from check_div64_kernel import verify_build_command
        from check_glob_kernel import verify_saved_flags
        from check_polynomial_kernel import elf_target, newer

        archive = build / 'lib/lib.a'
        owner = build / ('lib/earlycpio_rust.o' if selected else 'lib/earlycpio.o')
        members = {(build / os.fsdecode(line)).resolve() for line in command(['ar', 't', archive]).stdout.splitlines()}
        candidates = {(build / name).resolve() for name in ('lib/earlycpio.o', 'lib/earlycpio_rust.o')}
        self.assertEqual(members & candidates, {owner.resolve()})
        source = ROOT / ('lib/earlycpio_rust.rs' if selected else 'lib/earlycpio.c')
        dependencies = ([ROOT / 'lib/earlycpio.rs'] if selected else [ROOT / 'include/linux/earlycpio.h'])
        verify_build_command(build, owner, source, dependencies)
        verify_saved_flags(owner, 'rust' if selected else 'c', build)
        elf_target(owner, arch)
        self.assertEqual(read_exports(owner), [])
        symbols = command(['nm', '--defined-only', build / 'vmlinux']).stdout.splitlines()
        self.assertEqual(len([line for line in symbols if line.split()[-1] == b'find_cpio_data']), 1)
        self.assertFalse(any(len(line.split()) >= 2 and line.split()[1] == b'find_cpio_data'
                             for line in (build / 'Module.symvers').read_bytes().splitlines()))
        for new, old in ((archive, [owner]), (build / 'vmlinux.o', [archive]),
                         (build / 'vmlinux', [build / 'vmlinux.o']),
                         (build / ('arch/x86/boot/bzImage' if arch == 'x86_64' else 'arch/arm64/boot/Image'),
                          [build / 'vmlinux'])):
            newer(new, old)

    def test_selected_arm64_provider_archive_and_linked_image(self):
        from check_div64_kernel import architecture, configuration

        value = os.environ.get('EARLYCPIO_SELECTED_ARM64_BUILD')
        if not value:
            self.skipTest('EARLYCPIO_SELECTED_ARM64_BUILD supplies a completed selected image')
        build = Path(value).resolve()
        config = configuration(build)
        self.assertEqual(architecture(config), 'aarch64')
        for name in ('RUST', 'RUST_EARLYCPIO', 'CFI', 'MODVERSIONS', 'GENDWARFKSYMS'):
            self.assertEqual(config.get(name), 'y')
        with tempfile.TemporaryDirectory(prefix='earlycpio-arm64-') as temporary:
            work = outside(Path(temporary), (ROOT, build))
            env = compiler_environment(work, environment())
            watch = NativeWriteWatch(build)
            with watch:
                self.audit_provider(build, True, 'aarch64',
                    lambda arguments: run(list(map(str, arguments)), cwd=work, env=env))
            self.assertEqual(watch.events, [], 'native donor must remain unmodified')

    def native(self, variable, selected):
        from check_div64_kernel import architecture, configuration
        from check_int_math_kernel import verify_references
        from test_gen_init_cpio import entries

        value = os.environ.get(variable)
        if not value:
            self.skipTest(variable + ' supplies a completed x86 native image')
        build = Path(value).resolve()
        config = configuration(build)
        self.assertEqual(architecture(config), 'x86_64')
        for name in ('MICROCODE', 'BLK_DEV_INITRD', 'CPU_SUP_INTEL', 'PRINTK'):
            self.assertEqual(config.get(name), 'y', 'early consumer requires ' + name)
        self.assertNotEqual(config.get('MICROCODE_DBG'), 'y')
        # This fixture intentionally reaches the initrd lookup, without an
        # embedded firmware candidate taking precedence over that lookup.
        self.assertNotEqual(config.get('FW_LOADER'), 'y')
        self.assertEqual(config.get('RUST_EARLYCPIO') == 'y', selected)
        parent = outside(Path(os.environ.get('EARLYCPIO_NATIVE_ARTIFACTS', tempfile.gettempdir())).resolve(),
                         (ROOT, build))
        work = Path(tempfile.mkdtemp(prefix='earlycpio-' + ('rust-' if selected else 'c-'), dir=parent))
        print('earlycpio native boot evidence: ' + str(work), flush=True)
        env = compiler_environment(work, environment())
        qemu = shlex.split(os.environ.get('EARLYCPIO_QEMU', os.environ.get('QEMU', 'qemu-system-x86_64')))
        self.assertNotIn('-cpu', qemu, 'the fixture supplies its explicit CPU and CPUID control')
        watch = NativeWriteWatch(build)
        self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native donor must remain unmodified'))
        with watch:
            def command(arguments):
                return run(list(map(str, arguments)), cwd=work, env=env)

            self.audit_provider(build, selected, 'x86_64', command)
            verify_references(build / 'arch/x86/kernel/cpu/microcode/core.o', ('find_cpio_data',))
            verify_references(build / 'arch/x86/kernel/cpu/microcode/intel.o', ('find_microcode_in_initrd',))
            empty = work / 'empty'
            empty.write_bytes(b'')
            manifest = work / 'earlycpio.manifest'
            manifest.write_text('dir /kernel 0755 0 0\n'
                                'dir /kernel/x86 0755 0 0\n'
                                'dir /kernel/x86/microcode 0755 0 0\n' +
                                f'file /{EMPTY_MEMBER.decode()} {empty} 0644 0 0\n')
            for hypervisor in (True, False):
                # Only the immutable artifacts needed by the existing runner
                # are linked; all PID 1, archive and console outputs stay here.
                view = work / ('hypervisor-on' if hypervisor else 'hypervisor-off')
                for name in ('arch/x86/boot/bzImage', 'usr/gen_init_cpio'):
                    target = view / name
                    target.parent.mkdir(parents=True, exist_ok=True)
                    target.symlink_to(build / name)
                shutil.copyfile(build / '.config', view / '.config')
                cpu = 'Nehalem-v1,hypervisor=' + ('on' if hypervisor else 'off')
                args = [sys.executable, ROOT / 'scripts/tests/boot_kernel.py', '--build', view,
                        '--qemu', shlex.join([*qemu, '-cpu', cpu]), '--timeout', '90',
                        '--extra-initramfs-manifest', manifest]
                if env.get('QEMU_DATA'):
                    args += ['--qemu-data', env['QEMU_DATA']]
                command(args)
                generated = entries((view / 'rust-boot-test/initramfs.cpio').read_bytes())
                matching = [entry for entry in generated if entry[0].startswith(MICROCODE_PREFIX)]
                self.assertEqual(len(matching), 1)
                name, fields, data, _ = matching[0]
                self.assertEqual(name, EMPTY_MEMBER)
                self.assertTrue(stat.S_ISREG(fields[1]))
                self.assertEqual(fields[6], 0)
                self.assertEqual(data, b'')
                verify_early_consumer((view / 'rust-boot-test/console.log').read_bytes(), not hypervisor)

    def test_original_c_early_microcode_consumer(self):
        self.native('EARLYCPIO_ORIGINAL_X86_BUILD', False)

    def test_selected_rust_early_microcode_consumer(self):
        self.native('EARLYCPIO_SELECTED_X86_BUILD', True)


if __name__ == '__main__':
    unittest.main()
