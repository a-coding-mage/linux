# SPDX-License-Identifier: GPL-2.0
"""Original-C differential and real Kbuild tests for the Zorro name generator.

Run with unittest discovery. All executables, generated files and dependency
fixtures live in temporary directories outside the source tree.
"""
import os
from pathlib import Path
import random
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest

from test_migration_invariants import environment
from test_selinux_genheaders import host_rust_flags


ROOT = Path(__file__).resolve().parents[2]
COMPONENT = Path('drivers/zorro')


class GenDevlist(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix='gen-devlist-')
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.env = dict(environment(), RUSTC_BOOTSTRAP='1')
        cls.rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        cls.cc = shlex.split(os.environ.get('HOSTCC', 'cc'))
        cls.flags = host_rust_flags(cls.work)
        cls.binaries = []
        for compiler, label in ((cls.cc, 'c'), (['clang'], 'clang')):
            output = cls.work / label
            cls.command([*compiler, '-O2', '-Wall', '-Wextra', '-Werror',
                         str(ROOT / COMPONENT / 'gen-devlist.c'), '-o', str(output)])
            cls.binaries.append(output)
        for level in ('0', '2', 's'):
            output = cls.work / ('rust-' + level)
            cls.command([*cls.rustc, *cls.flags, '-Copt-level=' + level,
                         str(ROOT / COMPONENT / 'gen-devlist.rs'), '-o', str(output)])
            cls.binaries.append(output)

    @classmethod
    def command(cls, arguments, check=True, **kwargs):
        result = subprocess.run(list(map(str, arguments)), env=cls.env,
            cwd=kwargs.pop('cwd', cls.work), capture_output=True, timeout=60, **kwargs)
        if check and result.returncode:
            raise AssertionError((arguments, result.returncode, result.stdout, result.stderr))
        return result

    def compare(self, data, expected=None, output='regular', ignored=False, bad_input=False):
        results = []
        for binary in self.binaries:
            with tempfile.TemporaryDirectory(dir=self.work) as temporary:
                work = Path(temporary)
                destination = work / 'devlist.h'
                read = write = None
                options = {}
                if output == 'directory':
                    destination.mkdir()
                elif output == 'full':
                    destination.symlink_to('/dev/full')
                elif output == 'pipe':
                    read, write = os.pipe()
                    os.close(read)
                    destination.symlink_to('/proc/self/fd/' + str(write))
                    options = {'pass_fds': (write,), 'restore_signals': not ignored}
                else:
                    destination.write_bytes(b'old contents\n' * 200)
                if bad_input:
                    read = os.open(work, os.O_RDONLY | os.O_DIRECTORY)
                    options['stdin'] = read
                else:
                    options['input'] = data
                try:
                    result = self.command([binary, 'ignored', os.fsdecode(b'\xff')],
                                          check=False, cwd=work, **options)
                    contents = destination.read_bytes() if output == 'regular' else None
                    results.append((result.returncode, result.stdout, result.stderr, contents))
                finally:
                    if write is not None:
                        os.close(write)
                    if bad_input:
                        os.close(read)
        for result in results[1:]:
            self.assertEqual(results[0], result, repr(data[:100]))
        if expected is not None:
            self.assertEqual(results[0][0], expected)
        return results[0]

    def test_shipped_ids_and_generated_header(self):
        ids = (ROOT / COMPONENT / 'zorro.ids').read_bytes()
        actual = self.compare(ids, 0)[3]
        self.assertEqual(sum(line.startswith(b'MANUF(') for line in actual.splitlines()),
                         sum(bool(line) and line[0] not in b'#\t' for line in ids.splitlines()))
        self.assertEqual(actual.count(b'\tPRODUCT('),
                         sum(line.startswith(b'\t') for line in ids.splitlines()))
        (self.work / 'devlist.h').write_bytes(actual)
        # Compile and consume the generated macro stream, as names.c does.
        source = self.work / 'consumer.c'
        source.write_text('''\
#include <stdio.h>
struct entry { unsigned int id; const char *name; };
static const struct entry entries[] = {
#define MANUF(id, name) { 0x##id, name },
#define PRODUCT(manuf, prod, name) { (0x##manuf << 16) | 0x##prod, name },
#define ENDMANUF()
#include "devlist.h"
};
int main(void) {
    for (unsigned int i = 0; i < sizeof(entries) / sizeof(entries[0]); ++i)
        printf("%08x %s\\n", entries[i].id, entries[i].name);
    return 0;
}
''')
        executable = self.work / 'consumer'
        self.command([*self.cc, '-O2', '-Wall', '-Werror', source, '-o', executable])
        self.assertIn(b'Pacific Peripherals', self.command([executable]).stdout)

    def test_names_syntax_and_partial_output(self):
        fixtures = [
            (b'', 0), (b'# comment\n\n', 0),
            (b'0001 Vendor\n\t0002 Device\n', 0),
            (b'0001 \n\t0002 \n0003 Next\n', 0),
            (b'0001 A\\B"\xff\n\t0002 Device"\\\xfe\n', 0),
            (b'0001 Vendor\n\t0002 Device\r\n', 0),
            (b'0001 ' + b'v' * 39 + b'\n\t0002 ' + b'p' * 23 + b'\n', 0),
            (b'0001 ' + b'v' * 40 + b'\n', 1),
            (b'0001 ' + b'v' * 39 + b'\n\t0002 ' + b'p' * 24 + b'\n', 1),
            (b'0001 ' + b'v' * 39 + b'\n\t0002 short [long description follows]\n', 0),
            (b'0001 ' + b'v' * 39 + b'\n\t0002 short[long description follows]\n', 1),
            (b'0001 Vendor\n\t0002 [' + b'x' * 70 + b']\n', 1),
            (b'0001 Vendor\n0002 ' + b'x' * 40 + b'\n', 1),
            (b'\t0002 Device\n', 1), (b'0001\n', 1),
            (b'0001\tVendor\n', 1), (b'0001 Vendor\n\t0002\tDevice\n', 1),
            (b'0001 Vendor\n\t\t0002 Device\n', 1),
            (b'not hexadecimal\n', 1), (b'zzzz Vendor\n\tzzzz Device\n', 0),
            (b'0001 Vendor\ninvalid', 1),
        ]
        for data, expected in fixtures:
            with self.subTest(data=data):
                self.compare(data, expected)

    def test_fgets_chunking_and_embedded_nul(self):
        for data in (b'0001 Vendor\0ignored\n\t0002 Device\n',
                     b'\0ignored\n0001 Vendor\n', b'0001 Vendor',
                     b'#' + b'x' * 1020 + b'\n0001 Vendor\n',
                     b'#' + b'x' * 1021 + b'\n0001 Vendor\n',
                     b'#' + b'x' * 1022 + b'\n0001 Vendor\n',
                     b'0001 Vendor\n\t0002 x [' + b'd' * 2100 + b']\n'):
            with self.subTest(prefix=data[:60], length=len(data)):
                self.compare(data)
        rng = random.Random(0x5a0770)
        pieces = [b'# comment\n', b'\n', b'0001 Vendor\n', b'0002 Other\n',
                  b'\t0002 Device\n', b'\t0003 x [description]\n', b'\0tail\n',
                  b'0001 \xff"\\raw\n', b'bad line\n', b'\t0001\tbad\n']
        for _ in range(120):
            data = b''.join(rng.choices(pieces, k=rng.randrange(1, 15)))
            self.compare(data)

    def test_stdio_failures_and_inherited_signals(self):
        data = b'0001 Vendor\n\t0002 Device\n'
        self.compare(data, 1, output='directory')
        self.compare(data, 0, output='full')
        self.compare(data, -signal.SIGPIPE, output='pipe')
        self.compare(data, 0, output='pipe', ignored=True)
        self.compare(data, 0, bad_input=True)

    def test_actual_kbuild_switch_and_dependencies(self):
        tree = self.work / 'source'
        component = tree / COMPONENT
        component.mkdir(parents=True)
        for name in ('Makefile', 'gen-devlist.c', 'gen-devlist.rs', 'zorro.ids'):
            shutil.copyfile(ROOT / COMPONENT / name, component / name)
        (tree / 'scripts').symlink_to(ROOT / 'scripts', target_is_directory=True)
        for sequence in (('c', 'rust', 'c'), ('rust', 'c', 'rust')):
            out = self.work / ('build-' + sequence[0])
            (out / 'scripts/basic').mkdir(parents=True)
            self.command([*self.cc, '-O2', '-I', ROOT / 'scripts/include',
                          ROOT / 'scripts/basic/fixdep.c', '-o', out / 'scripts/basic/fixdep'])
            base = shlex.split(os.environ.get('MAKE', 'make')) + [
                '-j4', '-f', str(tree / 'scripts/Makefile.build'),
                'srctree=' + str(tree), 'srcroot=' + str(tree), 'objtree=.',
                'VPATH=' + str(tree), 'building_out_of_srctree=1', 'obj=' + str(COMPONENT),
                'quiet=quiet_', 'Q=@', 'HOSTCC=' + shlex.join(self.cc),
                'KBUILD_HOSTCFLAGS=-O2 -Wall -Werror',
                'KBUILD_HOSTRUSTFLAGS=' + shlex.join(self.flags),
                str(COMPONENT / 'gen-devlist'), str(COMPONENT / 'devlist.h')]
            products = [out / COMPONENT / name for name in ('gen-devlist', 'devlist.h')]
            golden = self.compare((component / 'zorro.ids').read_bytes(), 0)[3]

            def build(language):
                return self.command([*base, 'HOST_TOOLS_LANG=' + language,
                    'HOSTRUSTC=' + (shlex.join(self.rustc) if language == 'rust' else 'false')], cwd=out)

            for language in sequence:
                build(language)
                command = (out / COMPONENT / '.gen-devlist.cmd').read_text()
                self.assertIn('gen-devlist.' + ('rs' if language == 'rust' else 'c'), command)
                if language == 'rust':
                    self.assertIn('-Dwarnings', command)
                self.assertEqual(products[1].read_bytes(), golden)
                stamps = [path.stat().st_mtime_ns for path in products]
                build(language)
                self.assertEqual(stamps, [path.stat().st_mtime_ns for path in products])
            source = component / ('gen-devlist.' + ('rs' if sequence[-1] == 'rust' else 'c'))
            source.write_bytes(source.read_bytes() + b'\n// Dependency edit.\n')
            build(sequence[-1])
            self.assertGreater(products[0].stat().st_mtime_ns, stamps[0])
            self.assertGreater(products[1].stat().st_mtime_ns, stamps[1])
            binary_stamp = products[0].stat().st_mtime_ns
            with (component / 'zorro.ids').open('ab') as stream:
                stream.write(b'\nffff Test Vendor\n\t0001 Test Product\n')
            build(sequence[-1])
            self.assertEqual(products[0].stat().st_mtime_ns, binary_stamp)
            self.assertEqual(products[1].read_bytes(), self.compare((component / 'zorro.ids').read_bytes(), 0)[3])
            self.assertIn(b'MANUF(ffff,"Test Vendor")', products[1].read_bytes())


if __name__ == '__main__':
    unittest.main()
