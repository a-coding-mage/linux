# SPDX-License-Identifier: GPL-2.0-only
"""Original-C differential and real Kbuild checks for the logo converter."""
import os
from pathlib import Path
import random
import resource
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest

from test_migration_invariants import environment
from test_selinux_genheaders import host_rust_flags


ROOT = Path(__file__).resolve().parents[2]
COMPONENT = Path('drivers/video/logo')


class Pnmtologo(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix='pnmtologo-')
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.env = dict(environment(), RUSTC_BOOTSTRAP='1')
        cls.cc = shlex.split(os.environ.get('HOSTCC', 'cc'))
        cls.rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        cls.flags = host_rust_flags(cls.work)
        cls.binaries = []
        for compiler, label in ((cls.cc, 'c'), (['clang'], 'clang')):
            binary = cls.work / label
            cls.command([*compiler, '-O2', '-Wall', '-Wextra', '-Werror',
                         ROOT / COMPONENT / 'pnmtologo.c', '-o', binary])
            cls.binaries.append(binary)
        for optimization in ('0', '2', 's'):
            binary = cls.work / ('rust-' + optimization)
            cls.command([*cls.rustc, *cls.flags, '-Copt-level=' + optimization,
                         ROOT / COMPONENT / 'pnmtologo.rs', '-o', binary])
            cls.binaries.append(binary)

    @classmethod
    def command(cls, arguments, check=True, **kwargs):
        result = subprocess.run(list(map(str, arguments)), env=kwargs.pop('env', cls.env),
            cwd=kwargs.pop('cwd', cls.work), capture_output=True, timeout=60, **kwargs)
        if check and result.returncode:
            raise AssertionError((arguments, result.returncode, result.stdout, result.stderr))
        return result

    def compare(self, data, args=(), expected=0, *, filename='image.pnm', output=None,
                limit=False, env=None, binaries=None, positional=True):
        results = []
        for binary in binaries or self.binaries:
            with tempfile.TemporaryDirectory(dir=self.work) as temporary:
                work = Path(temporary)
                if data is not None:
                    (work / filename).write_bytes(data)
                target = work / 'out.c'
                if output == 'directory':
                    target.mkdir()
                elif output == 'full':
                    target.symlink_to('/dev/full')
                else:
                    target.write_bytes(b'previous output\n')
                options = {}
                if limit:
                    def memory_limit():
                        resource.setrlimit(resource.RLIMIT_AS, (64 * 1024**2, 64 * 1024**2))
                    options['preexec_fn'] = memory_limit
                arguments = ['pnmtologo', *args]
                if output:
                    arguments += ['-o', 'out.c']
                if positional:
                    arguments.append(filename)
                result = self.command(arguments, executable=str(binary), check=False,
                                      cwd=work, env=self.env if env is None else env, **options)
                contents = target.read_bytes() if output not in ('directory', 'full') else None
                results.append((result.returncode, result.stdout, result.stderr, contents))
        for result in results[1:]:
            self.assertEqual(results[0], result, repr((args, data[:80] if data else data)))
        self.assertEqual(results[0][0], expected)
        return results[0]

    def test_all_shipped_images(self):
        for path in sorted((ROOT / COMPONENT).glob('*.p?m')):
            kind = 'mono' if path.suffix == '.pbm' else ('vga16' if 'vga16' in path.name else 'clut224')
            with self.subTest(path=path.name):
                result = self.compare(path.read_bytes(), ['-t', kind, '-n', 'test_logo'])
                self.assertIn(b'const struct linux_logo test_logo', result[1])
                self.assertIn(b'.data\t\t= test_logo_data', result[1])

    def test_packing_scaling_and_palette(self):
        for width in (0, 1, 2, 7, 8, 9, 12, 13, 25):
            data = b'P1\n' + str(width).encode() + b' 2\n' + b'01' * width
            for kind in ('mono', 'vga16', 'clut224', 'gray256'):
                with self.subTest(width=width, kind=kind):
                    self.compare(data, ['-t', kind])
        rng = random.Random(224)
        for maximum in (1, 2, 15, 255, 256, 65535, 10000000):
            numbers = [0, maximum, maximum // 2, maximum * 2]
            numbers += [rng.randrange(maximum + 1) for _ in range(21)]
            data = ('P2\n25 1\n%d\n' % maximum).encode() + b' '.join(str(n).encode() for n in numbers) + b'\n'
            self.compare(data, ['-t', 'gray256'])
        for colors in (223, 224, 225):
            data = ('P3\n%d 1\n255\n' % colors).encode()
            data += b''.join(('%d 0 0\n' % value).encode() for value in range(colors))
            self.compare(data, ['-t', 'clut224'], 1 if colors == 225 else 0, output='file')
        self.compare(b'P3\n1 1\n255\n1 2 3\n', ['-t', 'gray256'], 1, output='file')
        self.compare(b'P2\n1 1\n255\n100\n', ['-t', 'mono'], 1, output='file')
        self.compare(b'P3\n1 1\n255\n1 2 3\n', ['-t', 'vga16'], 1, output='file')

    def test_generated_c_uses_the_real_logo_header(self):
        image = b'P2\n3 1\n255\n0 255 0\n'
        expected = {'mono': b'\x40', 'vga16': b'\x0f\x00',
                    'clut224': b'\x20\x21\x20\x00\x00\x00\xff\xff\xff',
                    'gray256': b'\x00\xff\x00'}
        for index, (kind, pixels) in enumerate(expected.items(), 1):
            generated = self.compare(image, ['-t', kind, '-n', 'test_logo'])[1]
            (self.work / 'generated.c').write_bytes(generated)
            consumer = self.work / 'consumer.c'
            # Use the maintained linux_logo structure and constants directly.
            # Only its kernel section annotation is elided for this host probe.
            consumer.write_text('''\
#define _LINUX_INIT_H
#define __initconst
#include "generated.c"
#include <stdio.h>
int main(void) {
    printf("%d %u %u\\n", test_logo.type, test_logo.width, test_logo.height);
    fwrite(test_logo.data, 1, sizeof(test_logo_data), stdout);
    if (test_logo.type == LINUX_LOGO_CLUT224)
        fwrite(test_logo.clut, 3, test_logo.clutsize, stdout);
    return 0;
}
''')
            executable = self.work / 'consumer'
            self.command([*self.cc, '-O2', '-Wall', '-Werror', '-I', ROOT / 'include',
                          consumer, '-o', executable])
            self.assertEqual(self.command([executable]).stdout,
                             ('%d 3 1\n' % index).encode() + pixels)

    def test_parser_quirks_and_diagnostics(self):
        fixtures = [
            (b'', 1), (b'X1\n1 1\n0', 1), (b'P7\n', 1),
            (b'P4\n', 1), (b'P5\n', 1), (b'P6\n', 1),
            (b'P1\n# comment without newline', 1),
            (b'P1\n1 1\n# comment\n0', 0),
            (b'P1\n# dimensions\n1 # height\n1\n0', 0),
            (b'P2\n1 1\n255\n0', 1), (b'P2\n1 1\n255\n0\n', 0),
            (b'P2\n1 1\n255\nx', 0),
            (b'P1\n1 1\n2', 0), (b'P1\n1 1\nx', 0),
            (b'P2\n0 0\n0\n', 0), (b'P3\n1 1\n255\n0 0', 1),
            (b'P3\n1 1\n255\n0# discarded separator\n0 0\n', 0),
        ]
        for data, expected in fixtures:
            with self.subTest(data=data):
                self.compare(data, expected=expected)
        # maxval=0 with actual pixels divides by zero in C. Keep that domain
        # separate from differential assertions; Rust rejects it safely.
        for binary in self.binaries[2:]:
            self.compare(b'P2\n1 1\n0\n1\n', expected=1, binaries=[binary])

    def test_getopt_paths_and_output_order(self):
        data = b'P1\n1 1\n0'
        for arguments in ([], ['-h'], ['-z'], ['-n'], ['-o'], ['-t'], ['-t', 'bad']):
            self.compare(data, arguments, 1, positional=False)
        for args in (['-nraw_name', '-tmono'], ['-n', os.fsdecode(b'raw-\xff')],
                     ['-t', 'mono', '-t', 'gray256']):
            self.compare(data, args, output='file')
        self.compare(data, ['-n', 'test'], filename=os.fsdecode(b'input-\xff.pbm'))
        self.compare(None, expected=1)
        self.compare(data, expected=1, output='directory')
        self.compare(data, expected=0, output='full')
        # GNU getopt permutes a leading positional argument unless POSIX mode
        # is requested. The original libc handles both invocation forms.
        self.compare(data, ['image.pnm', '-t', 'mono'], positional=False)
        self.compare(data, ['image.pnm', '-t', 'mono'], 1, positional=False,
                     env=dict(self.env, POSIXLY_CORRECT='1'))
        for value in (b'P1\n1000000000 1\n', b'P1\n1 1000000000\n'):
            self.compare(value, expected=1, limit=True)

    def test_sigpipe_and_ignored_stdout_errors(self):
        image = self.work / 'pipe.pbm'
        image.write_bytes(b'P1\n1 1\n0')
        for ignored in (False, True):
            for binary in self.binaries:
                read, write = os.pipe()
                os.close(read)
                try:
                    result = subprocess.run([str(binary), str(image)], env=self.env,
                        stdout=write, stderr=subprocess.PIPE, restore_signals=not ignored, timeout=20)
                finally:
                    os.close(write)
                self.assertEqual(result.returncode, 0 if ignored else -signal.SIGPIPE)
                self.assertEqual(result.stderr, b'')

    def test_real_kbuild_switch_and_generated_sources(self):
        tree = self.work / 'source'
        component = tree / COMPONENT
        component.mkdir(parents=True)
        for name in ('Makefile', 'pnmtologo.c', 'pnmtologo.rs', 'logo_linux_mono.pbm',
                     'logo_linux_vga16.ppm', 'logo_linux_clut224.ppm'):
            shutil.copyfile(ROOT / COMPONENT / name, component / name)
        (tree / 'scripts').symlink_to(ROOT / 'scripts', target_is_directory=True)
        out = self.work / 'build'
        (out / 'scripts/basic').mkdir(parents=True)
        self.command([*self.cc, '-O2', '-I', ROOT / 'scripts/include',
                      ROOT / 'scripts/basic/fixdep.c', '-o', out / 'scripts/basic/fixdep'])
        targets = [COMPONENT / 'pnmtologo']
        generated = {}
        options = []
        for kind, extension in (('mono', 'pbm'), ('vga16', 'ppm'), ('clut224', 'ppm')):
            name = 'logo_linux_' + kind
            image = component / (name + '.' + extension)
            options.append('CONFIG_LOGO_LINUX_' + kind.upper() + '_FILE=' + str(image))
            targets.append(COMPONENT / (name + '.c'))
            generated[out / COMPONENT / (name + '.c')] = self.compare(image.read_bytes(),
                ['-t', kind, '-n', name])[1]
        base = shlex.split(os.environ.get('MAKE', 'make')) + [
            '-j4', '-f', str(tree / 'scripts/Makefile.build'), 'srctree=' + str(tree),
            'srcroot=' + str(tree), 'objtree=.', 'VPATH=' + str(tree),
            'building_out_of_srctree=1', 'obj=' + str(COMPONENT), 'quiet=quiet_', 'Q=@',
            'HOSTCC=' + shlex.join(self.cc), 'KBUILD_HOSTCFLAGS=-O2 -Wall -Werror',
            'KBUILD_HOSTRUSTFLAGS=' + shlex.join(self.flags), *options, *map(str, targets)]
        products = [out / targets[0], *generated]
        for language in ('c', 'rust', 'c', 'rust'):
            args = [*base, 'HOST_TOOLS_LANG=' + language,
                    'HOSTRUSTC=' + (shlex.join(self.rustc) if language == 'rust' else 'false')]
            self.command(args, cwd=out)
            record = (out / COMPONENT / '.pnmtologo.cmd').read_text()
            self.assertIn('pnmtologo.' + ('rs' if language == 'rust' else 'c'), record)
            for path, expected in generated.items():
                self.assertEqual(path.read_bytes(), expected)
            stamps = [path.stat().st_mtime_ns for path in products]
            self.command(args, cwd=out)
            self.assertEqual(stamps, [path.stat().st_mtime_ns for path in products])
        source = component / 'pnmtologo.rs'
        source.write_bytes(source.read_bytes() + b'\n// Dependency edit.\n')
        self.command(args, cwd=out)
        self.assertTrue(all(path.stat().st_mtime_ns > stamp for path, stamp in zip(products, stamps)))


if __name__ == '__main__':
    unittest.main()
