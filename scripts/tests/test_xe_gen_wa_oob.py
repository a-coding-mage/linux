# SPDX-License-Identifier: MIT
"""Compare the Xe workaround generator with its retained C implementation."""
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
COMPONENT = Path('drivers/gpu/drm/xe')


class XeGenWaOob(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix='xe-gen-wa-oob-')
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
                ROOT / COMPONENT / 'xe_gen_wa_oob.c', '-o', binary])
            cls.binaries.append(binary)
        for optimization in ('0', '2', 's'):
            binary = cls.work / ('rust-' + optimization)
            cls.command([*cls.rustc, *cls.flags, '-Copt-level=' + optimization,
                ROOT / COMPONENT / 'xe_gen_wa_oob.rs', '-o', binary])
            cls.binaries.append(binary)

    @classmethod
    def command(cls, args, check=True, **kwargs):
        result = subprocess.run(list(map(str, args)), cwd=kwargs.pop('cwd', cls.work),
            env=cls.env, capture_output=True, timeout=60, **kwargs)
        if check and result.returncode:
            raise AssertionError((args, result.returncode, result.stdout, result.stderr))
        return result

    def compare(self, data, expected=0, names=('input.rules', 'out.c', 'out.h'),
                failure=None, binaries=None):
        results = []
        for binary in binaries or self.binaries:
            with tempfile.TemporaryDirectory(dir=self.work) as temporary:
                work = Path(temporary)
                paths = [work / name for name in names]
                options = {}
                pipe = None
                for path in paths:
                    path.parent.mkdir(parents=True, exist_ok=True)
                for path in paths[1:]:
                    path.write_bytes(b'previous output\n' * 20)
                if data is not None:
                    paths[0].write_bytes(data)
                if failure == 'input-directory':
                    paths[0].unlink()
                    paths[0].mkdir()
                elif failure in ('source-directory', 'header-directory'):
                    index = 1 if failure == 'source-directory' else 2
                    paths[index].unlink()
                    paths[index].mkdir()
                elif failure == 'full':
                    for path in paths[1:]:
                        path.unlink()
                        path.symlink_to('/dev/full')
                elif failure in ('pipe', 'pipe-ignore'):
                    read, pipe = os.pipe()
                    os.close(read)
                    paths[1].unlink()
                    paths[1].symlink_to('/proc/self/fd/' + str(pipe))
                    options = {'pass_fds': (pipe,), 'restore_signals': failure == 'pipe'}
                try:
                    result = self.command(['xe_gen_wa_oob', *names, 'ignored'],
                        executable=str(binary), check=False, cwd=work, **options)
                finally:
                    if pipe is not None:
                        os.close(pipe)
                outputs = tuple(path.read_bytes() if path.is_file() and not path.is_symlink()
                                else None for path in paths[1:])
                results.append((result.returncode, result.stdout, result.stderr, outputs))
        for result in results[1:]:
            self.assertEqual(results[0], result, repr((data, names, failure)))
        self.assertEqual(results[0][0], expected)
        return results[0]

    def test_shipped_rules_and_c_consumer(self):
        for filename in ('xe_wa_oob', 'xe_device_wa_oob'):
            with self.subTest(filename=filename):
                data = (ROOT / COMPONENT / (filename + '.rules')).read_bytes()
                source, header = self.compare(data, names=(filename + '.rules',
                    filename + '.c', filename + '.h'))[3]
                (self.work / (filename + '.c')).write_bytes(source)
                (self.work / (filename + '.h')).write_bytes(header)
                # Consume both generated files together. Stringifying the rule
                # expressions lets the C compiler check emitted entry boundaries
                # and enum indices without emulating hardware rule matching.
                consumer = self.work / (filename + '-consumer.c')
                consumer.write_text('''\
#include <stdio.h>
struct entry { const char *name, *rules; };
#define XE_RTP_NAME(value) .name = value
#define XE_RTP_RULES(...) .rules = #__VA_ARGS__
#include "''' + filename + '''.h"
static const struct entry entries[] = {
#include "''' + filename + '''.c"
};
_Static_assert(sizeof(entries) / sizeof(entries[0]) == _''' + filename.upper() + '''_COUNT,
               "generated table and enum disagree");
int main(void) {
    for (unsigned int i = 0; i < sizeof(entries) / sizeof(entries[0]); ++i)
        printf("%u %s %s\\n", i, entries[i].name, entries[i].rules);
    return 0;
}
''')
                binary = consumer.with_suffix('')
                self.command([*self.cc, '-O2', '-Wall', '-Werror', consumer, '-o', binary])
                output = self.command([binary]).stdout
                self.assertIn(b'PLATFORM(', output)
                self.assertEqual(len(output.splitlines()), source.count(b'{ XE_RTP_NAME('))

    def test_strip_continuations_nuls_and_errors(self):
        fixtures = [
            (b'', 0), (b'# comment\n\n', 0), (b'1 RULE(A)\n', 0),
            (b'1 RULE(A)', 0), (b'1 RULE(A)   \n', 0),
            (b'1  RULE(A)\n\tRULE(B)\n2\tRULE(C)\n', 0),
            (b'1 RULE(A)\n # indented comment is a rule\n', 0),
            (b'1 RULE(A)\n\n#comment\n\vRULE(B)\n', 0),
            (b'\0ignored\n1 RULE(A)\0tail\n', 0),
            (b'1 RULE(A)\r\n', 0),
            (b'\tRULE(A)\n', 234),
            (b'1 RULE(A)\n \n', 234),
            (b'1 RULE(A)\n' + b'x' * 4095 + b'\n', 234),
            (b'1 ' + b'x' * 4092 + b'\n', 0),
            (b'#' + b'x' * 4095 + b'2 RULE(B)\n', 0),
        ]
        for data, expected in fixtures:
            with self.subTest(length=len(data), prefix=data[:70]):
                self.compare(data, expected)
        rng = random.Random(0x0b00)
        for _ in range(100):
            rows = [b'1 RULE(A)\n']
            for number in range(rng.randrange(1, 20)):
                rows.append(rng.choice([b'# comment\n', b'\n', b'\tRULE(B)\n',
                    str(number).encode() + rng.choice([b' ', b'\t', b'  ']) + b'RULE(C)\n']))
            self.compare(b''.join(rows))

    def test_prefix_filenames_and_io_failures(self):
        data = b'1 RULE(A)\n'
        for header in ('lower.h', 'many.dots.h', '.hidden', 'nodot', 'directory/foo.h',
                       os.fsdecode(b'raw\xff.h'), 'x' * 127, 'x' * 128):
            self.compare(data, 1 if len(os.fsencode(header)) > 127 else 0,
                         names=('nested/rules.name', 'source.c', header))
        self.compare(None, 1)
        self.compare(data, 1, failure='source-directory')
        self.compare(data, 1, failure='header-directory')
        self.compare(data, 0, failure='input-directory')
        self.compare(data, 0, failure='full')
        self.compare(data, -signal.SIGPIPE, failure='pipe')
        self.compare(data, 0, failure='pipe-ignore')
        self.compare(data, names=('input.rules', 'same', 'same'))
        self.compare(data, names=('input.rules', 'input.rules', 'out.h'))

    def test_original_undefined_inputs_are_rejected_safely(self):
        # Do not treat original NULL dereferences as a defined differential
        # oracle: argc == 3 and absent rule tokens are outside its valid domain.
        for binary in self.binaries[2:]:
            result = self.command(['xe_gen_wa_oob', 'input', 'output'],
                                  executable=str(binary), check=False)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b'ERROR: wrong arguments\n', result.stderr)
            for data in (b'1\n', b'1 \n', b'x', b'1 RULE(A)\n2\n'):
                self.compare(data, 234, binaries=[binary])

    def test_actual_kbuild_switch_and_generated_files(self):
        tree = self.work / 'source'
        component = tree / COMPONENT
        component.mkdir(parents=True)
        for name in ('Makefile', 'xe_gen_wa_oob.c', 'xe_gen_wa_oob.rs',
                     'xe_wa_oob.rules', 'xe_device_wa_oob.rules'):
            shutil.copyfile(ROOT / COMPONENT / name, component / name)
        (tree / 'scripts').symlink_to(ROOT / 'scripts', target_is_directory=True)
        out = self.work / 'build'
        (out / 'scripts/basic').mkdir(parents=True)
        self.command([*self.cc, '-O2', '-I', ROOT / 'scripts/include',
                      ROOT / 'scripts/basic/fixdep.c', '-o', out / 'scripts/basic/fixdep'])
        products = [out / COMPONENT / 'xe_gen_wa_oob']
        targets = [COMPONENT / 'xe_gen_wa_oob']
        golden = {}
        for name in ('xe_wa_oob', 'xe_device_wa_oob'):
            data = (component / (name + '.rules')).read_bytes()
            pair = self.compare(data, names=(name + '.rules', name + '.c', name + '.h'))[3]
            for suffix, contents in zip(('.c', '.h'), pair):
                target = COMPONENT / 'generated' / (name + suffix)
                targets.append(target)
                products.append(out / target)
                golden[out / target] = contents
        base = shlex.split(os.environ.get('MAKE', 'make')) + [
            '-j4', '-f', str(tree / 'scripts/Makefile.build'),
            'srctree=' + str(tree), 'srcroot=' + str(tree), 'objtree=.',
            'VPATH=' + str(tree), 'building_out_of_srctree=1', 'obj=' + str(COMPONENT),
            'quiet=quiet_', 'Q=@', 'HOSTCC=' + shlex.join(self.cc),
            'KBUILD_HOSTCFLAGS=-O2 -Wall -Werror',
            'KBUILD_HOSTRUSTFLAGS=' + shlex.join(self.flags), *map(str, targets)]
        for language in ('c', 'rust', 'c', 'rust'):
            args = [*base, 'HOST_TOOLS_LANG=' + language,
                    'HOSTRUSTC=' + (shlex.join(self.rustc) if language == 'rust' else 'false')]
            self.command(args, cwd=out)
            record = (out / COMPONENT / '.xe_gen_wa_oob.cmd').read_text()
            self.assertIn('xe_gen_wa_oob.' + ('rs' if language == 'rust' else 'c'), record)
            for path, contents in golden.items():
                self.assertEqual(path.read_bytes(), contents)
            stamps = [path.stat().st_mtime_ns for path in products]
            self.command(args, cwd=out)
            self.assertEqual(stamps, [path.stat().st_mtime_ns for path in products])
        source = component / 'xe_gen_wa_oob.rs'
        source.write_bytes(source.read_bytes() + b'\n// Dependency edit.\n')
        self.command(args, cwd=out)
        self.assertTrue(all(path.stat().st_mtime_ns > stamp for path, stamp in zip(products, stamps)))


if __name__ == '__main__':
    unittest.main()
