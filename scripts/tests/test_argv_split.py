# SPDX-License-Identifier: GPL-2.0
"""Original-C argv semantics with explicit, instrumented allocator transport.

The original kmalloc_array inline and kstrndup implementation are retained.
Rust uses the actual slab helper and kernel ctype facade. The host allocation
transport checks sizes, GFP bits, failure order and hidden-owner lifetime; it
does not claim to execute kernel memcg/slab accounting. The real helper's
successful NULL-old path also traces kfree(NULL), which has no ownership effect.
"""
import os
from pathlib import Path
import re
import shlex
import shutil
import tempfile
import unittest

from rbtree_native.transport import compiler_environment, outside, NativeWriteWatch
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_rational_build import environment, run
from test_hexdump_kcfi import C_KCFI, RUST_KCFI
from kconfig_test_support import cached_conf_tools


ROOT = Path(__file__).resolve().parents[2]
NATIVE_MARKER = b'LUPOS_ARGV_SPLIT_ABI_OK cases=2352 callers=C,Rust flags=3 hidden-owner=stable'


def verify_argv_console(console, calls=1):
    from check_int_log_kernel import normalize_console_transport
    lines = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip()).strip()
             for line in normalize_console_transport(console).splitlines()]
    markers = [line for line in lines if line.startswith(b'LUPOS_ARGV_SPLIT_ABI_')]
    if markers != [NATIVE_MARKER] * calls:
        raise ValueError('native argument-vector comparison did not complete exactly once per load')


def define(source, name):
    match = re.search(r'^#define\s+' + re.escape(name) + r'\b[^\n]*(?:\\\n[^\n]*)*', source, re.M)
    if not match:
        raise AssertionError('missing original macro ' + name)
    return match.group() + '\n'


def function(source, name):
    match = re.search(r'(?m)^[^\n]*\b' + name + r'\([^;]*?\)\n\{.*?^\}', source, re.S)
    if not match:
        raise AssertionError('missing original function ' + name)
    return match.group() + '\n'


class ArgvSplit(unittest.TestCase):
    def setUp(self):
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT,))
        temporary = tempfile.TemporaryDirectory(prefix='argv-split-', dir=parent)
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.env = dict(environment(), RUSTC_BOOTSTRAP='1')
        self.rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        self.bindgen = shlex.split(os.environ.get('BINDGEN', 'bindgen'))

    def headers(self):
        include = self.work / 'include/linux'
        include.mkdir(parents=True, exist_ok=True)
        slab = (ROOT / 'include/linux/slab.h').read_text()
        strings = (ROOT / 'include/linux/string.h').read_text()
        types = (ROOT / 'include/linux/types.h').read_text()
        typedef = re.search(r'^typedef unsigned int __bitwise gfp_t;', types, re.M).group()
        (include / 'compiler.h').write_text('#ifndef ARGV_COMPILER_H\n#define ARGV_COMPILER_H\n'
            '#include <linux/compiler_attributes.h>\n#define __force\n#define __bitwise\n'
            '#define __rust_helper\n#define __realloc_size(...)\n#define __alloc_size(...)\n'
            '#define unlikely(x) (x)\n#endif\n')
        (include / 'kernel.h').write_text('#include <stddef.h>\n#include <stdbool.h>\n'
                                         '#include <linux/compiler.h>\n')
        (include / 'types.h').write_text('#ifndef ARGV_TYPES_H\n#define ARGV_TYPES_H\n'
            '#include <stddef.h>\n#include <linux/compiler.h>\n' + typedef + '\n#endif\n')
        (include / 'bits.h').write_text('#include <vdso/bits.h>\n')
        (include / 'export.h').write_text('#define EXPORT_SYMBOL(symbol)\n')
        declarations = '\n'.join(re.search(r'^extern [^\n]*\b' + name + r'\([^\n]*;', strings, re.M).group()
                                 for name in ('kstrndup', 'argv_split', 'argv_free'))
        (include / 'string.h').write_text('#ifndef ARGV_BINDGEN\n#include <string.h>\n#endif\n'
                                         '#include <linux/types.h>\n' + declarations + '\n')
        constants = ('#define CONFIG_PAGE_SHIFT 12\n#include <vdso/page.h>\n' +
            define((ROOT / 'include/linux/mmzone.h').read_text(), 'MAX_PAGE_ORDER') +
            define(slab, 'KMALLOC_SHIFT_MAX') + define(slab, 'KMALLOC_MAX_SIZE') +
            define((ROOT / 'include/linux/nodemask_types.h').read_text(), 'NUMA_NO_NODE'))
        # The actual no-partition token declaration and the original array
        # overflow check are compiled, with allocation policy at the boundary.
        token = re.search(r'typedef struct \{\} kmalloc_token_t;', slab).group()
        array = function(slab, '_kmalloc_array_noprof')
        (include / 'slab.h').write_text('''#ifndef ARGV_SLAB_H
#define ARGV_SLAB_H
#include <linux/kernel.h>
#include <linux/types.h>
#include <linux/gfp_types.h>
''' + constants + token + '''
#define __kmalloc_token(...) ((kmalloc_token_t){})
#define check_mul_overflow(a, b, out) __builtin_mul_overflow(a, b, out)
#define alloc_hooks(expression) (expression)
void *host_alloc(size_t size, gfp_t flags);
void *host_realloc(const void *old, size_t size, unsigned long align, gfp_t flags, int node);
void kfree(const void *pointer);
#define _kmalloc_noprof(size, flags, token) ((void)(token), host_alloc(size, flags))
#define kmalloc_track_caller(size, flags) host_alloc(size, flags)
#define krealloc_node_align host_realloc
#define kvrealloc_node_align host_realloc
''' + array + define(slab, 'kmalloc_array_noprof') + define(slab, 'kmalloc_array') + '\n#endif\n')
        # Keep the genuine snapshot implementation including its forced NUL.
        util = (ROOT / 'mm/util.c').read_text()
        (self.work / 'snapshot.c').write_text('#include <linux/string.h>\n#include <linux/slab.h>\n' +
            function(util, '__kmemdup_nul') + function(util, 'kstrndup'))
        (self.work / 'bindings.h').write_text('#include <linux/string.h>\n#include <linux/slab.h>\n'
            '#include <linux/ctype.h>\n#include "' + str(ROOT / 'rust/helpers/slab.c') + '"\n')
        return ['-I' + str(include.parent), '-idirafter', str(ROOT / 'include')]

    def build(self, bits, target, optimization, compiler, cfi=False):
        flags = self.headers()
        generated = self.work / 'bindings.rs'
        run([*self.bindgen, self.work / 'bindings.h', '--use-core', '--rust-target', '1.85',
             '--ctypes-prefix', 'ffi', '--no-layout-tests', '--no-doc-comments',
             '--allowlist-var', 'KMALLOC_MAX_SIZE|NUMA_NO_NODE|_ctype',
             '--allowlist-function', 'kstrndup|kfree|rust_helper_krealloc_node_align',
             '-o', generated, '--', '-DARGV_BINDGEN', '-m' + str(bits), *flags], env=self.env)
        ffi = self.work / 'libffi.rlib'
        rust_flags = [*target, '--edition=2021', '-Dwarnings', '-Cpanic=abort',
                      '-Coverflow-checks=yes', '-Copt-level=' + optimization]
        if cfi:
            rust_flags += RUST_KCFI
        run([*self.rustc, *rust_flags, '--crate-name=ffi', '--crate-type=rlib',
             ROOT / 'rust/ffi.rs', '-o', ffi], env=self.env)
        wrapper = self.work / 'owner.rs'
        wrapper.write_text('//! Execute the actual owner against generated C declarations.\n'
            'extern crate self as kernel;\npub extern crate ffi;\n'
            '#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs)]\n'
            'pub mod bindings { include!("' + str(generated) + '");\n'
            'pub use rust_helper_krealloc_node_align as krealloc_node_align;\n}\n'
            '#[path = "' + str(ROOT / 'rust/kernel/ctype.rs') + '"] pub mod ctype;\n'
            '#[path = "' + str(ROOT / 'lib/argv_split_rust.rs') + '"] mod owner;\n'
            'pub use owner::*;\n')
        obj, library = self.work / 'owner.o', self.work / 'owner.a'
        run([*self.rustc, *rust_flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
             '-Wmissing-docs', '-Wunsafe-op-in-unsafe-fn', '-Wunreachable-pub',
             '--emit=obj=' + str(obj), '--emit=link=' + str(library), wrapper], env=self.env)
        self.assertEqual([(row['name'], row['license'], row['namespace']) for row in read_exports(obj)],
                         [('argv_free', '', ''), ('argv_split', '', '')])
        cflags = [*flags, '-O' + optimization, '-funsigned-char', '-fno-strict-overflow',
                  '-Wall', '-Wextra', '-Werror', *([*C_KCFI] if cfi else [])]
        original = self.work / 'original.o'
        run([*compiler, *cflags, '-Dargv_split=c_argv_split', '-Dargv_free=c_argv_free',
             '-c', ROOT / 'lib/argv_split.c', '-o', original], env=self.env)
        binary = self.work / 'compare'
        run([*compiler, *cflags, ROOT / 'scripts/tests/argv_split/driver.c',
             self.work / 'snapshot.c', ROOT / 'rust/helpers/slab.c', ROOT / 'lib/ctype.c',
             original, library, '-ldl', '-lpthread', '-lm', '-no-pie', '-o', binary], env=self.env)
        self.assertEqual(binary.read_bytes()[4], 1 if bits == 32 else 2)
        result = run([binary], env=self.env)
        self.assertEqual(result.stdout, b'argv_split: snapshot, flags, hidden owner and failures passed\n')

    def test_original_c_snapshot_ownership_and_all_bytes(self):
        for compiler in (shlex.split(os.environ.get('HOSTCC', 'cc')), ['clang']):
            for optimization in ('0', '2', 's'):
                with self.subTest(compiler=compiler, optimization=optimization):
                    self.build(64, [], optimization, compiler)

    def test_real_i686_allocations(self):
        targets = rust_targets()
        if 32 not in targets:
            self.skipTest('the real i686 Rust standard library is required')
        compiler = shlex.split(os.environ.get('ARGV_SPLIT_I686_CC', 'cc -m32'))
        for optimization in ('0', '2', 's'):
            self.build(32, targets[32], optimization, compiler)

    def test_protected_public_c_calls(self):
        self.build(64, [], '2', ['clang'], cfi=True)


class ArgvSplitNative(unittest.TestCase):
    """Compile the actual owner with immutable native kernel metadata."""

    def native(self, variable, architecture):
        from check_polynomial_kernel import elf_target
        from check_prime_numbers_kernel import provider_type_ids, verify_guarded_calls
        from test_sort_native import SortNativeTests
        value = os.environ.get(variable)
        if not value:
            self.skipTest(variable + ' supplies the immutable native donor')
        build = Path(value).resolve()
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT, build))
        with tempfile.TemporaryDirectory(prefix='argv-native-', dir=parent) as directory:
            work = Path(directory)
            env = dict(environment(), RUSTC_BOOTSTRAP='1')
            reader = SortNativeTests()
            reader.rust = Path(shlex.split(os.environ.get('RUSTC', 'rustc'))[0])
            watch = NativeWriteWatch(build)
            self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native donor must remain unmodified'))
            with watch:
                cflags = reader.native_flags(build, 'lib/.scatterlist.o.cmd', False)
                rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
                c, rust = work / 'original.o', work / 'owner.o'
                run([*cflags, '-c', ROOT / 'lib/argv_split.c', '-o', c], env=env)
                run([*rflags, '--crate-name=argv_split_rust', '--emit=obj=' + str(rust),
                     ROOT / 'lib/argv_split_rust.rs'], env=env)
                names = ('argv_free', 'argv_split')
                for obj in (c, rust):
                    elf_target(obj, architecture)
                    self.assertEqual([(row['name'], row['license'], row['namespace'], row['pointer_width'])
                                      for row in read_exports(obj)],
                                     [(name, '', '', 8) for name in names])
                    records, _ = dwarf_versions(dwarf_tools(), obj, names, work)
                    self.assertEqual(set(records), {name.encode() for name in names})
                self.assertEqual(read_exports(c), read_exports(rust))
                self.assertEqual(provider_type_ids(c, names=names), provider_type_ids(rust, names=names))
                imports = {line.split()[-1] for line in run(['nm', '-u', rust], env=env).stdout.splitlines()}
                self.assertTrue({b'kstrndup', b'kfree', b'rust_helper_krealloc_node_align'} <= imports, imports)
                # The shared facade may retain its non-inline mask helper in
                # kernel.o rather than importing the table directly here.
                kernel_masks = {line.split()[-1] for line in
                                run(['nm', '--defined-only', build / 'rust/kernel.o'], env=env).stdout.splitlines()
                                if b'ctype' in line and b'ctype_mask' in line}
                self.assertTrue(b'_ctype' in imports or bool(imports & kernel_masks), imports)
                self.assertFalse({b'isspace', b'kmalloc_array', b'KMALLOC_MAX_SIZE'} & imports)
                # Compile the eventual external fixture with the actual donor
                # metadata now, without asking Kbuild to refresh that donor.
                fixture = ROOT / 'scripts/tests/argv_split'
                native, original, caller = (work / name for name in ('native.o', 'oracle.o', 'caller.o'))
                run([*cflags, '-DMODULE', '-c', fixture / 'native.c', '-o', native], env=env)
                run([*cflags, '-DMODULE', '-I' + str(ROOT / 'lib'), '-c',
                     fixture / 'original.c', '-o', original], env=env)
                run([*rflags, '--cfg', 'MODULE', '--crate-name=argv_caller',
                     '--emit=obj=' + str(caller), fixture / 'caller.rs'], env=env)
                types = provider_type_ids(rust, names=names)
                for obj, language in ((native, 'c'), (caller, 'rust')):
                    verify_guarded_calls(obj, architecture, types,
                        wrappers=tuple((name + '_' + language + '_call', name) for name in names))
                oracle_types = provider_type_ids(original,
                    names=('argv_free_original', 'argv_split_original'))
                self.assertEqual({name.removesuffix('_original'): value for name, value in oracle_types.items()}, types)

    def test_actual_x86_kernel_interfaces_and_exports(self):
        self.native('NATIVE_ARGV_SPLIT_X86_BUILD', 'x86_64')

    def test_actual_arm64_kernel_interfaces_and_exports(self):
        self.native('NATIVE_ARGV_SPLIT_ARM64_BUILD', 'aarch64')


class ArgvSplitSelection(unittest.TestCase):
    def setUp(self):
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT,))
        temporary = tempfile.TemporaryDirectory(prefix='argv-selection-', dir=parent)
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def test_actual_kconfig_requires_native_rust_and_explicit_opt_in(self):
        found = re.search(r'(?ms)^config RUST_ARGV_SPLIT\n.*?(?=^config |\Z)',
                          (ROOT / 'lib/Kconfig').read_text())
        self.assertIsNotNone(found, 'missing actual RUST_ARGV_SPLIT Kconfig option')
        source = self.work / 'Kconfig'
        source.write_text('config RUST\n\tbool "Rust"\n\n' + found.group())
        env = dict(environment(), KCONFIG_CONFIG=str(self.work / '.config'))
        for tool in cached_conf_tools():
            for rust in ('n', 'y'):
                for requested in (None, 'n', 'y'):
                    with self.subTest(tool=tool.name, rust=rust, requested=requested):
                        config = 'CONFIG_RUST=' + rust + '\n'
                        if requested is not None:
                            config += 'CONFIG_RUST_ARGV_SPLIT=' + requested + '\n'
                        (self.work / '.config').write_text(config)
                        run([tool, '--olddefconfig', source], cwd=self.work, env=env)
                        self.assertEqual('CONFIG_RUST_ARGV_SPLIT=y' in
                                         (self.work / '.config').read_text().splitlines(),
                                         rust == requested == 'y')

    def test_actual_makefile_c_rust_c_preserves_archive_position_and_owner(self):
        makefile = self.work / 'Makefile'
        makefile.write_text(f'''srctree := {ROOT}
src := {ROOT}/lib
obj := lib
objtree := {self.work}
include {ROOT}/lib/Makefile
.PHONY: argv-selection lib/argv_split_rust.o
argv-selection:
\t@printf '%s\\n' '$(lib-y)' '$(filter argv_split%.o,$(obj-y) $(obj-m))'
lib/argv_split_rust.o:
\t@printf '%s\\n' '$(modfile)'
''')
        for host in ('c', 'rust'):
            for native in ('n', 'y', 'n'):
                command = [*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory',
                           '-rR', '-f', makefile, 'HOST_TOOLS_LANG=' + host,
                           'CONFIG_RUST=y', 'CONFIG_RUST_ARGV_SPLIT=' + native]
                result = run([*command, 'argv-selection'], cwd=self.work, env=environment())
                archive, objects = result.stdout.splitlines()
                names = archive.split()
                expected = b'argv_split_rust.o' if native == 'y' else b'argv_split.o'
                self.assertEqual([name for name in names if name.startswith(b'argv_split')], [expected])
                position = names.index(expected)
                self.assertEqual(names[position - 1:position + 2],
                                 [b'irq_regs.o', expected, b'flex_proportions.o'])
                self.assertEqual(objects, b'')
                self.assertEqual(result.stderr, b'')
                if native == 'y':
                    owner = run([*command, 'lib/argv_split_rust.o'], cwd=self.work, env=environment())
                    self.assertEqual(owner.stdout, b'lib/argv_split\n')


class ArgvSplitSelected(unittest.TestCase):
    """Audit a completed selected image and build its native caller module."""

    def native(self, variable, expected_arch):
        from check_div64_kernel import architecture, configuration, verify_build_command
        from check_glob_kernel import verify_saved_flags
        from check_int_math_kernel import verify_references
        from check_polynomial_kernel import elf_target, newer
        from check_prime_numbers_kernel import (provider_type_ids, verify_guarded_calls,
                                                verify_module_import_versions)
        from check_rust_exports_bridge import version_records
        value = os.environ.get(variable)
        if not value:
            self.skipTest(variable + ' supplies a completed selected native build')
        build = Path(value).resolve()
        config = configuration(build)
        arch = architecture(config)
        self.assertEqual(arch, expected_arch)
        for name in ('RUST', 'CFI', 'MODVERSIONS', 'GENDWARFKSYMS', 'MODULES'):
            self.assertEqual(config.get(name), 'y', 'native fixture requires ' + name)
        selected = config.get('RUST_ARGV_SPLIT') == 'y'
        parent = outside(Path(os.environ.get('ARGV_SPLIT_NATIVE_ARTIFACTS', tempfile.gettempdir())).resolve(),
                         (ROOT, build))
        work = Path(tempfile.mkdtemp(prefix='argv-split-' + arch + '-', dir=parent))
        print('argv_split native evidence and module: ' + str(work), flush=True)
        env = compiler_environment(work, environment())
        def command(arguments):
            return run(list(map(str, arguments)), cwd=work, env=env)
        watch = NativeWriteWatch(build)
        self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native input must remain unmodified'))
        with watch:
            archive = build / 'lib/lib.a'
            members = {(build / os.fsdecode(line)).resolve() for line in
                       command(['ar', 't', archive]).stdout.splitlines()}
            candidates = {(build / 'lib/argv_split.o').resolve(),
                          (build / 'lib/argv_split_rust.o').resolve()}
            owner = build / ('lib/argv_split_rust.o' if selected else 'lib/argv_split.o')
            self.assertEqual(members & candidates, {owner.resolve()})
            source = ROOT / ('lib/argv_split_rust.rs' if selected else 'lib/argv_split.c')
            dependencies = ([ROOT / 'lib/argv_split.rs', ROOT / 'rust/ffi_export.rs',
                             ROOT / 'include/linux/export_header.rs'] if selected else
                            [ROOT / 'include/linux/string.h', ROOT / 'include/linux/slab.h'])
            verify_build_command(build, owner, source, dependencies)
            verify_saved_flags(owner, 'rust' if selected else 'c', build)
            elf_target(owner, arch)
            names = ('argv_free', 'argv_split')
            self.assertEqual([(row['name'], row['license'], row['namespace'], row['pointer_width'])
                              for row in read_exports(owner)], [(name, '', '', 8) for name in names])
            definitions = command(['nm', '--defined-only', build / 'vmlinux']).stdout.splitlines()
            for name in names:
                self.assertEqual(len([line for line in definitions if line.split()[-1] == name.encode()]), 1)
            crc, _ = dwarf_versions(dwarf_tools(), owner, names, work)
            self.assertEqual(version_records(owner.with_name('.' + owner.name + '.cmd')), crc)
            for name in names:
                rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()
                        if len(line.split()) >= 2 and line.split()[1] == name.encode()]
                self.assertEqual(rows, [[crc[name.encode()], name.encode(), b'vmlinux', b'EXPORT_SYMBOL']])
            for new, old in ((archive, [owner]), (build / 'Module.symvers', [owner]),
                             (build / 'vmlinux.o', [archive]),
                             (build / 'vmlinux', [build / 'vmlinux.o']),
                             (build / ('arch/x86/boot/bzImage' if arch == 'x86_64' else 'arch/arm64/boot/Image'),
                              [build / 'vmlinux'])):
                newer(new, old)
            types = provider_type_ids(owner, names=names)
            for name in ('Makefile', 'native.c', 'original.c', 'caller.rs'):
                shutil.copyfile(ROOT / 'scripts/tests/argv_split' / name, work / name)
            # Kbuild assigns compiler defaults itself, overriding exported
            # environment variables unless they are command-line assignments.
            toolchain = [name + '=' + env[name] for name in
                         ('RUSTC', 'HOSTRUSTC', 'RUSTDOC', 'BINDGEN', 'RUST_LIB_SRC',
                          'HOSTLDLIBS_gendwarfksyms') if env.get(name)]
            command([*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory',
                     '-C', build, 'M=' + str(work), 'ARCH=' + ('x86' if arch == 'x86_64' else 'arm64'),
                     'LLVM=1', *toolchain, 'HOSTRUSTFLAGS=-Dwarnings', 'KRUSTFLAGS=-Dwarnings',
                     'KBUILD_SYMTYPES=1', '-j2', 'modules'])
            module = work / 'argv_split_abi_test.ko'
            elf_target(module, arch)
            verify_module_import_versions(build, module)
            for obj, language in ((work / 'native.o', 'c'), (work / 'caller.o', 'rust')):
                verify_references(obj, names)
                wrappers = tuple((name + '_' + language + '_call', name) for name in names)
                verify_guarded_calls(obj, arch, types, wrappers=wrappers)
                verify_guarded_calls(module, arch, types, wrappers=wrappers)
            original_types = provider_type_ids(work / 'original.o',
                names=('argv_free_original', 'argv_split_original'))
            self.assertEqual({name.removesuffix('_original'): value for name, value in original_types.items()}, types)
            self.assertIn(str(ROOT / 'lib/argv_split.c'), (work / '.original.o.cmd').read_text())

    def test_selected_x86_artifacts_and_original_c_module(self):
        self.native('SELECTED_ARGV_SPLIT_X86_BUILD', 'x86_64')

    def test_selected_arm64_artifacts_and_original_c_module(self):
        self.native('SELECTED_ARGV_SPLIT_ARM64_BUILD', 'aarch64')

    def test_completed_native_console(self):
        value = os.environ.get('NATIVE_ARGV_SPLIT_CONSOLE_BUILD')
        if not value:
            self.skipTest('NATIVE_ARGV_SPLIT_CONSOLE_BUILD supplies a completed fixture boot')
        console = Path(value) / 'rust-boot-test/console.log'
        verify_argv_console(console.read_bytes(),
                            calls=2 if os.environ.get('NATIVE_ARGV_SPLIT_RELOAD') == '1' else 1)


class ArgvSplitConsole(unittest.TestCase):
    def test_complete_native_marker_count_and_contents(self):
        verify_argv_console(NATIVE_MARKER + b'\n')
        verify_argv_console((NATIVE_MARKER + b'\n') * 2, calls=2)
        for console in (b'', NATIVE_MARKER.replace(b'2352', b'2351') + b'\n',
                        NATIVE_MARKER.replace(b'flags=3', b'flags=1') + b'\n',
                        (NATIVE_MARKER + b'\n') * 2,
                        NATIVE_MARKER + b'\nLUPOS_ARGV_SPLIT_ABI_FAIL\n'):
            with self.assertRaises(ValueError):
                verify_argv_console(console)


if __name__ == '__main__':
    unittest.main()
