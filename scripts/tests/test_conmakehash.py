# SPDX-License-Identifier: GPL-2.0-or-later
"""Compare the console font generator with the original C and exercise Kbuild."""

import os
from pathlib import Path
import random
import resource
import shlex
import shutil
import signal
import struct
import subprocess
import tempfile
import unittest

from test_migration_invariants import environment
from test_selinux_genheaders import host_rust_flags


ROOT = Path(__file__).resolve().parents[2]
COMPONENT = Path('drivers/tty/vt')


class Conmakehash(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix='conmakehash-')
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
                         ROOT / COMPONENT / 'conmakehash.c', '-o', output])
            cls.binaries.append(output)
        for level in ('0', '2', 's'):
            output = cls.work / ('rust-' + level)
            cls.command([*cls.rustc, *cls.flags, '-Copt-level=' + level,
                         ROOT / COMPONENT / 'conmakehash.rs', '-o', output])
            cls.binaries.append(output)

    @classmethod
    def command(cls, arguments, check=True, **kwargs):
        result = subprocess.run(list(map(str, arguments)), env=cls.env,
            cwd=kwargs.pop('cwd', cls.work), capture_output=True, timeout=60, **kwargs)
        if check and result.returncode:
            raise AssertionError((arguments, result.returncode, result.stdout, result.stderr))
        return result

    def compare(self, data=b'', expected=None, arguments=('-',)):
        observations = []
        for binary in self.binaries:
            # A common argv[0] also checks exact usage text without normalization.
            result = subprocess.run(['conmakehash', *arguments], executable=binary,
                input=data, capture_output=True, cwd=self.work, env=self.env, timeout=30)
            observations.append((result.returncode, result.stdout, result.stderr))
        for actual in observations[1:]:
            self.assertEqual(observations[0], actual, repr(data[:100]))
        if expected is not None:
            self.assertEqual(observations[0][0], expected)
        return observations[0]

    def test_shipped_font_and_generated_tables(self):
        data = (ROOT / COMPONENT / 'cp437.uni').read_bytes()
        generated = self.compare(data, 0)[1]
        (self.work / 'generated.c').write_bytes(generated)
        include = self.work / 'linux'
        include.mkdir(exist_ok=True)
        (include / 'types.h').write_text('typedef unsigned char u8; typedef unsigned short u16;\n')
        consumer = self.work / 'consumer.c'
        consumer.write_text('''\
#include "generated.c"
int main(void) {
    unsigned sum = 0;
    for (unsigned i = 0; i < 256; ++i) sum += dfont_unicount[i];
    return sum != sizeof(dfont_unitable) / sizeof(dfont_unitable[0]) || sum < 256;
}
''')
        binary = self.work / 'consumer'
        self.command([*self.cc, '-Wall', '-Werror', '-I', self.work, consumer, '-o', binary])
        self.command([binary])
        filename = os.fsdecode(b'font-\xff.uni')
        (self.work / filename).write_bytes(data)
        self.assertEqual(generated, self.compare(expected=0, arguments=(filename,))[1])
        self.assertEqual(generated, self.compare(data, 0, ('-', 'ignored', 'even', 'these'))[1])

    def test_syntax_ranges_aliases_and_diagnostics(self):
        fixtures = [
            (b'', 0), (b'\n # comment\n', 0), (b'0 U+0000\n', 0),
            (b'0-0 U+0000 U+0001\n', 0), (b'1-0 U+0001\n', 0),
            (b'0-255 idem\n', 0), (b'0x0-0377 U+0000-U+00FF\n', 0),
            (b'0 U+FFFE U+FFFF U+FFFE U+0000\n0 U+FFFE\n', 0),
            (b'1-2 idemjunk\n3 U+0061junk\n', 0),
            (b'1 U+123 U+0000\n2 U+12345\n3 u+1234\n4 U+abcd\r\n', 0),
            (b'\v1 U+0001\n\f2 U+0002\n', 0),
            (b'+1 U+0001\n010 U+0008\n0x10 U+0010\n08 U+0000\n', 0),
            (b'4294967296 U+0000\n', 0 if struct.calcsize('l') == 8 else 65),
            (b'bad\n', 65), (b'1-no\n', 65), (b'-1 U+0000\n', 65),
            (b'256 U+0000\n', 65), (b'1-256 idem\n', 65),
            (b'2-1 idem\n', 65), (b'1-2 U+0000\n', 65),
            (b'1-2 -U+0001\n', 65), (b'1-2 U+0000-U+QQQQ\n', 65),
            (b'1-2 U+0000-U+0002\n', 65), (b'1-2 U+0002-U+0001\n', 65),
            (b'99999999999999999999999999999999\n', 65),
            (b'0 U+0000\n1-2 U+0000-U+0001 # end\n', 0),
        ]
        for data, expected in fixtures:
            with self.subTest(data=data):
                self.compare(data, expected)
        for arguments, expected in (((), 64), (('-', '1', '2', '3', '4'), 64),
                                     (('missing',), 66), (('.',), 0)):
            self.compare(expected=expected, arguments=arguments)

    def test_limits_chunking_binary_input_and_seeded_corpus(self):
        values = b' '.join(f'U+{number:04x}'.encode() for number in range(255))
        self.compare(b'0 ' + values + b' U+0000 U+FFFF\n', 0)
        self.compare(b'0 ' + values + b' U+0100\n', 65)
        for data in (b'1 U+0001', b'1 U+0001\0junk\n2 U+0002\n',
                     b'\0ignored\n', b'1 U+0001\xff\n',
                     b'#' + b'x' * 65533 + b'\n',
                     b'#' + b'x' * 65534 + b'\n',
                     b'#' + b'x' * 65535 + b'\n',
                     b'0 ' + b'U+0000 ' * 10000 + b'\n'):
            with self.subTest(length=len(data), prefix=data[:30]):
                self.compare(data)
        rng = random.Random(0xc0437)
        pieces = [b'# comment\n', b'0-20 idem\n', b'1 U+FFFF\n', b'2 U+0002 U+0002\n',
                  b'0x10-0x12 U+0100-U+0102\n', b'3 U+abcd\n', b'4 U+0000#tail\n',
                  b'\0hidden\n', b'bad\n', b'-1\n', b'0-1 U+0000-U+0002\n']
        for _ in range(100):
            self.compare(b''.join(rng.choices(pieces, k=rng.randrange(1, 15))))

    def test_stdio_failures_and_inherited_sigpipe(self):
        for ignored in (False, True):
            for destination in ('full', 'pipe'):
                statuses = []
                for binary in self.binaries:
                    if destination == 'full':
                        write = os.open('/dev/full', os.O_WRONLY)
                    else:
                        read, write = os.pipe()
                        os.close(read)
                    try:
                        result = subprocess.run([binary, '-'], input=b'0-255 idem\n',
                            stdout=write, stderr=subprocess.PIPE, cwd=self.work, env=self.env,
                            restore_signals=not ignored, timeout=30)
                        statuses.append((result.returncode, result.stderr))
                    finally:
                        os.close(write)
                expected = -signal.SIGPIPE if destination == 'pipe' and not ignored else 0
                self.assertEqual(statuses, [(expected, b'')] * len(self.binaries))

    def test_original_static_table_stack_budget(self):
        def limit_stack():
            resource.setrlimit(resource.RLIMIT_STACK, (128 * 1024, 128 * 1024))
            resource.setrlimit(resource.RLIMIT_CORE, (0, 0))

        data = (ROOT / COMPONENT / 'cp437.uni').read_bytes()
        expected = self.compare(data, 0)
        for binary in self.binaries:
            result = subprocess.run([binary, '-'], input=data, capture_output=True,
                cwd=self.work, env=self.env, timeout=30, preexec_fn=limit_stack)
            self.assertEqual((result.returncode, result.stdout, result.stderr), expected,
                             str(binary))

    def test_actual_kbuild_switch_and_dependencies(self):
        tree = self.work / 'source'
        component = tree / COMPONENT
        component.mkdir(parents=True)
        for name in ('Makefile', 'conmakehash.c', 'conmakehash.rs', 'cp437.uni'):
            shutil.copyfile(ROOT / COMPONENT / name, component / name)
        (tree / 'scripts').symlink_to(ROOT / 'scripts', target_is_directory=True)
        golden = self.compare((component / 'cp437.uni').read_bytes(), 0)[1]
        for sequence in (('c', 'rust', 'c'), ('rust', 'c', 'rust')):
            out = self.work / ('build-' + sequence[0])
            (out / 'scripts/basic').mkdir(parents=True)
            self.command([*self.cc, '-O2', '-I', ROOT / 'scripts/include',
                          ROOT / 'scripts/basic/fixdep.c', '-o', out / 'scripts/basic/fixdep'])
            base = shlex.split(os.environ.get('MAKE', 'make')) + [
                '-j4', '-f', str(tree / 'scripts/Makefile.build'),
                'srctree=' + str(tree), 'srcroot=' + str(tree), 'objtree=.',
                'VPATH=' + str(tree), 'building_out_of_srctree=1', 'obj=' + str(COMPONENT),
                'HOSTCC=' + shlex.join(self.cc), 'KBUILD_HOSTCFLAGS=-O2 -Wall -Werror',
                'KBUILD_HOSTRUSTFLAGS=' + shlex.join(self.flags),
                str(COMPONENT / 'conmakehash'), str(COMPONENT / 'consolemap_deftbl.c')]
            products = [out / COMPONENT / name for name in ('conmakehash', 'consolemap_deftbl.c')]

            def build(language, *extra):
                return self.command([*base, 'HOST_TOOLS_LANG=' + language,
                    'HOSTRUSTC=' + (shlex.join(self.rustc) if language == 'rust' else 'false'),
                    *extra], cwd=out)

            for language in sequence:
                build(language)
                dep = (out / COMPONENT / '.conmakehash.cmd').read_text()
                extension = 'rs' if language == 'rust' else 'c'
                self.assertIn('conmakehash.' + extension, dep)
                self.assertEqual(products[1].read_bytes(), golden)
                stamps = [path.stat().st_mtime_ns for path in products]
                build(language)
                self.assertEqual(stamps, [path.stat().st_mtime_ns for path in products])
                inactive = component / ('conmakehash.c' if language == 'rust' else 'conmakehash.rs')
                dry = build(language, '-n', '-W', str(inactive))
                self.assertNotIn(b'--emit=link=', dry.stdout)
                self.assertNotIn(b' -o drivers/tty/vt/conmakehash', dry.stdout)
            active = component / ('conmakehash.' + ('rs' if sequence[-1] == 'rust' else 'c'))
            active.write_bytes(active.read_bytes() + b'\n// Dependency edit.\n')
            build(sequence[-1])
            self.assertGreater(products[0].stat().st_mtime_ns, stamps[0])
            self.assertGreater(products[1].stat().st_mtime_ns, stamps[1])
            binary_stamp = products[0].stat().st_mtime_ns
            font = component / 'cp437.uni'
            font.write_bytes(font.read_bytes() + b'\n0 U+1234\n')
            build(sequence[-1])
            self.assertEqual(products[0].stat().st_mtime_ns, binary_stamp)
            golden = self.compare(font.read_bytes(), 0)[1]
            self.assertEqual(products[1].read_bytes(), golden)


if __name__ == '__main__':
    unittest.main()
