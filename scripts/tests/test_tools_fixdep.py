# SPDX-License-Identifier: GPL-2.0
"""Compile and exercise tools/build/fixdep, independently of scripts/basic.

Run with python3 -m unittest discover -s scripts/tests -p test_tools_fixdep.py -v.
All fixtures and logs are retained under TOOLS_FIXDEP_ARTIFACT_ROOT (default:
the system temporary directory), outside the source trees. Explicit invalid
inputs fail before any fixture is created. The optional saved native command
is read-only; every compiler/build output goes to a new private directory.
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


ROOT = Path(__file__).resolve().parents[2]


def configured(name, default, directory=False):
    value = os.environ.get(name, str(default))
    if not value.strip():
        raise ValueError(name + ' must not be empty')
    path = Path(value) if directory else Path(shutil.which(value) or value)
    if not (path.is_dir() if directory else path.is_file() and os.access(path, os.X_OK)):
        raise ValueError(name + ': invalid path: ' + value)
    return path.resolve()


class ToolsFixdep(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.original = configured('TOOLS_FIXDEP_ORIGINAL', ROOT, True)
        cls.rustc = configured('TOOLS_FIXDEP_RUSTC', 'rustc')
        cls.gcc = configured('TOOLS_FIXDEP_GCC', 'gcc')
        cls.clang = configured('TOOLS_FIXDEP_CLANG', 'clang')
        cls.make = configured('TOOLS_FIXDEP_MAKE', 'make')
        cls.bindgen = configured('TOOLS_FIXDEP_BINDGEN', os.environ.get('BINDGEN', 'bindgen'))
        cls.native = os.environ.get('TOOLS_FIXDEP_NATIVE_CMD')
        if cls.native is not None and (not cls.native or not Path(cls.native).is_file()):
            raise ValueError('TOOLS_FIXDEP_NATIVE_CMD: invalid or empty file')
        cls.i686 = os.environ.get('TOOLS_FIXDEP_I686_SYSROOT')
        if cls.i686 is not None and (not cls.i686 or not Path(cls.i686).is_dir()):
            raise ValueError('TOOLS_FIXDEP_I686_SYSROOT: invalid or empty directory')
        if cls.i686 is not None and not list((Path(cls.i686) /
                'lib/rustlib/i686-unknown-linux-gnu/lib').glob('libcore*.rlib')):
            raise ValueError('TOOLS_FIXDEP_I686_SYSROOT: missing actual i686 core')
        if cls.i686 is not None and not list((Path(cls.i686) /
                'lib/rustlib/i686-unknown-linux-gnu/lib').glob('libstd-*.rlib')):
            raise ValueError('TOOLS_FIXDEP_I686_SYSROOT: missing actual i686 std')
        cls.i686cc = (configured('TOOLS_FIXDEP_I686_CC', 'gcc')
                      if 'TOOLS_FIXDEP_I686_CC' in os.environ else None)
        cls.i686_bindgen_flags = shlex.split(os.environ.get('TOOLS_FIXDEP_I686_BINDGEN_FLAGS', ''))
        cls.flags = ['--edition=2021', '-O', '-Dwarnings']
        if cls.native:
            lines = Path(cls.native).read_text().splitlines()
            if not lines or ' := ' not in lines[0]:
                raise ValueError('TOOLS_FIXDEP_NATIVE_CMD: missing saved compiler command')
            command = shlex.split(lines[0].split(' := ', 1)[1])
            start = command.index('--edition=2021')
            end = next(i for i in range(start, len(command)) if command[i].startswith('--emit=link='))
            cls.flags = [x for x in command[:start] if x.startswith('-Clink')] + command[start:end]
        cls.flags += ['-Zon-broken-pipe=inherit']
        for name in ('fixdep.c', 'Makefile.build', 'Build.include'):
            if not (cls.original / 'tools/build' / name).is_file():
                raise ValueError('missing original tools/build/' + name)
        artifact = os.environ.get('TOOLS_FIXDEP_ARTIFACT_ROOT', tempfile.gettempdir())
        if not artifact.strip():
            raise ValueError('TOOLS_FIXDEP_ARTIFACT_ROOT must not be empty')
        parent = Path(artifact).resolve()
        protected = [ROOT, cls.original]
        if cls.native:
            protected.append(Path(cls.native).resolve().parents[2])
        if cls.i686:
            protected.append(Path(cls.i686).resolve())
        if any(parent.is_relative_to(source) for source in protected):
            raise ValueError('artifact root must be outside source and donor trees')
        if not parent.is_dir():
            raise ValueError('artifact root must be an existing directory')
        cls.work = Path(tempfile.mkdtemp(prefix='tools-fixdep-', dir=parent))
        cls.env = dict(os.environ, RUSTC_BOOTSTRAP='1', TMPDIR=str(cls.work), LC_ALL='C',
                       BINDGEN=str(cls.bindgen))
        cls.count = 0
        cls.tree = cls.work / 'tree'
        for rel in ('tools/build/fixdep.c', 'tools/build/Makefile.build',
                    'tools/build/Build.include', 'tools/build/Makefile.include',
                    'tools/scripts/Makefile.include'):
            dest = cls.tree / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(cls.original / rel, dest)
        for rel in ('tools/build/fixdep.rs', 'tools/build/fixdep-libc.h',
                    'tools/build/fixdep-abi.py', 'tools/build/Makefile', 'tools/build/Build'):
            shutil.copyfile(ROOT / rel, cls.tree / rel)
        abi = cls.work / 'fixdep-libc.rs'
        cls.generate_bindings(abi)
        cls.env['TOOLS_FIXDEP_LIBC_BINDINGS'] = str(abi)
        cls.bins = []
        for cc, name in ((cls.gcc, 'gcc'), (cls.clang, 'clang')):
            out = cls.work / name
            cls.run_command([cc, '-O2', '-Wall', '-Wextra', '-Werror',
                             cls.tree / 'tools/build/fixdep.c', '-o', out])
            cls.bins.append(out)
        out = cls.work / 'rust'
        cls.run_command([cls.rustc, *cls.flags, '--out-dir', cls.work,
                         '--emit=link=' + str(out), cls.tree / 'tools/build/fixdep.rs'])
        cls.bins.append(out)

    @classmethod
    def generate_bindings(cls, out, flags=()):
        cls.run_command([cls.bindgen, cls.tree / 'tools/build/fixdep-libc.h',
            '--allowlist-function', '^(open|fstat|mmap|munmap)$',
            '--allowlist-var', '^(O_RDONLY|PROT_READ|MAP_PRIVATE)$',
            '--opaque-type', '^timespec$',
            '--no-layout-tests', '--no-doc-comments', '--use-core', '--wrap-unsafe-ops',
            '--rust-target', '1.85', '--output', out, '--', *flags])

    @classmethod
    def run_command(cls, args, check=True, **kwargs):
        cls.count += 1
        args = [str(x) for x in args]
        result = subprocess.run(args, cwd=kwargs.pop('cwd', cls.work),
                                env=kwargs.pop('env', cls.env), capture_output=True, **kwargs)
        log = cls.work / ('%04d.log' % cls.count)
        log.write_bytes((repr(args) + '\nreturncode=' + str(result.returncode) + '\nstdout:\n').encode()
                        + result.stdout + b'\nstderr:\n' + result.stderr)
        if check and result.returncode:
            raise AssertionError(str(log) + ': ' + result.stderr.decode(errors='replace'))
        return result.returncode, result.stdout, result.stderr

    def compare(self, data, args=None):
        dep = os.fsencode(self.work) + b'/dep-\xff.d'
        if data is not None:
            with open(dep, 'wb') as stream:
                stream.write(data)
        elif os.path.exists(dep):
            os.unlink(dep)
        args = [dep, b'target-\xfe.o', b'cc -D\xff'] if args is None else args
        results = []
        for binary in self.bins:
            result = subprocess.run([os.fsencode(binary), *args], cwd=self.work,
                                    env=self.env, capture_output=True)
            results.append((result.returncode, result.stdout, result.stderr))
        self.count += 1
        (self.work / ('diff-%04d.log' % self.count)).write_text(repr((data, args, results)))
        self.assertEqual(results[0], results[1])
        self.assertEqual(results[0], results[2])
        self.assertNotEqual(results[0][0], -signal.SIGSYS)

    def test_differential(self):
        for data in (b'', None, b'x: x.c a.h\n', b'x: x.c a.h\n  ',
                     b'x: x.c a.h\nx2: temp.c b.h\n', b'x: x.c\nphony.h:\n',
                     b'no-target\n', b' \n', b'x:', b'x: ', b'x:  ',
                     b'x: x.c\tfoo a\x00z.h\r\n', b'x: x.c \xff.h',
                     b'x: ' + b'a' * 4095, b'x: x.c \\\n h.h\n'):
            with self.subTest(data=data):
                self.compare(data)
        for args in ([], [b'a'], [b'a', b'b'], [b'a', b'b', b'c', b'd'],
                     [b'', b'', b''], [os.fsencode(self.work), b't', b'cmd'],
                     [b'/dev/null', b't', b'cmd'], [b'/proc/self/cmdline', b't', b'cmd']):
            self.compare(None, args)
        rng = random.Random(812)
        for _ in range(100):
            parts = [b'ignored', b'x:', b'source.c']
            parts += [rng.choice([b'header.h', b'\xff.h', b'next:', b'temp.c', b'a\x00b'])
                      for _ in range(rng.randrange(30))]
            self.compare(rng.choice([b' ', b' \\\n ', b'\n']).join(parts) + rng.choice([b'', b' ', b'\n  ']))

    def test_output_errors_and_sigpipe(self):
        dep = self.work / 'large.d'
        dep.write_bytes(b'x: source.c ' + b'header.h ' * 5000)
        for ignored in (False, True):
            codes = []
            for binary in self.bins:
                read, write = os.pipe()
                os.close(read)
                result = subprocess.run([binary, dep, 'target', 'cmd'], cwd=self.work,
                    env=self.env, stdout=write, stderr=subprocess.PIPE,
                    restore_signals=not ignored)
                os.close(write)
                codes.append(result.returncode)
            self.assertEqual(codes, ([0] if ignored else [-signal.SIGPIPE]) * 3)
        with open('/dev/full', 'wb') as full:
            for binary in self.bins:
                result = subprocess.run([binary, dep, 't', 'cmd'], cwd=self.work,
                                        env=self.env, stdout=full, stderr=subprocess.PIPE)
                self.assertEqual(result.returncode, 0)

    def build(self, out, language=None, extra=(), env=None):
        args = [self.make, '-j8', '-C', self.tree / 'tools/build',
                'srctree=' + str(self.tree), 'OUTPUT=' + str(out) + '/',
                'HOSTRUSTC=' + str(self.rustc), 'HOSTCC=' + str(self.gcc),
                'KBUILD_HOSTCFLAGS=-O2 -Wall -Werror',
                'KBUILD_HOSTRUSTFLAGS=' + shlex.join(self.flags)]
        if language is not None:
            args.append('HOST_TOOLS_LANG=' + language)
        return self.run_command([*args, *extra], env=env or self.env)

    def test_bootstrap_and_switches(self):
        out = self.work / 'output'
        out.mkdir()
        for lang in ('c', 'rust', 'c', 'rust', 'rust', 'c'):
            self.build(out, lang)
            binary = out / 'fixdep'
            command = out / ('.fixdep.o.cmd' if lang == 'c' else '.fixdep.cmd')
            text = command.read_text()
            self.assertIn('source_', text)
            self.assertNotIn('cannot find fixdep', text)
            snapshot = {p.name: p.stat().st_mtime_ns for p in out.iterdir()}
            self.build(out, lang)
            self.assertEqual(snapshot, {p.name: p.stat().st_mtime_ns for p in out.iterdir()})
            binary.unlink()
            self.build(out, lang)
            self.assertTrue(binary.is_file())
        for lang, flag in (('rust', 'KBUILD_HOSTRUSTFLAGS=' + shlex.join(self.flags + ['-Copt-level=1'])),
                           ('c', 'KBUILD_HOSTCFLAGS=-O1 -Wall -Werror')):
            self.build(out, lang)
            before = (out / 'fixdep').stat().st_mtime_ns
            self.build(out, lang, [flag])
            self.assertNotEqual(before, (out / 'fixdep').stat().st_mtime_ns)
            source = self.tree / 'tools/build' / ('fixdep.rs' if lang == 'rust' else 'fixdep.c')
            before = (out / 'fixdep').stat().st_mtime_ns
            os.utime(source, None)
            self.build(out, lang, [flag])
            self.assertNotEqual(before, (out / 'fixdep').stat().st_mtime_ns)
        self.build(out, 'rust', ['clean'])
        self.assertFalse(list(out.iterdir()))

    def test_default_and_invalid_language(self):
        out = self.work / 'default'
        out.mkdir()
        env = dict(self.env)
        env.pop('HOST_TOOLS_LANG', None)
        self.build(out, env=env)
        self.assertIn('fixdep.rs', (out / '.fixdep.cmd').read_text())
        for lang in ('', 'bad', 'c rust'):
            before = {p.name: p.stat().st_mtime_ns for p in out.iterdir()}
            code, _, err = self.run_command([self.make, '-C', self.tree / 'tools/build',
                'HOST_TOOLS_LANG=' + lang, 'OUTPUT=' + str(out) + '/'], check=False)
            self.assertNotEqual(code, 0)
            self.assertIn(b"HOST_TOOLS_LANG must be 'c' or 'rust'", err)
            self.assertEqual(before, {p.name: p.stat().st_mtime_ns for p in out.iterdir()})

    def test_invalid_compiler_before_outputs(self):
        out = self.work / 'invalid-compiler'
        out.mkdir()
        for value in ('', '/no/such/rustc'):
            code, _, err = self.run_command([self.make, '-C', self.tree / 'tools/build',
                'HOST_TOOLS_LANG=rust', 'HOSTRUSTC=' + value,
                'OUTPUT=' + str(out) + '/'], check=False)
            self.assertNotEqual(code, 0)
            self.assertIn(b'HOSTRUSTC', err)
            self.assertEqual(list(out.iterdir()), [])
        self.build(out, 'rust')
        snapshot = {p.name: p.stat().st_mtime_ns for p in out.iterdir()}
        missing = [self.make, '-C', self.tree / 'tools/build',
                   'HOST_TOOLS_LANG=rust', 'HOSTRUSTC=/no/such/rustc',
                   'BINDGEN=/no/such/bindgen', 'OUTPUT=' + str(out) + '/']
        code, _, err = self.run_command([*missing, 'clean', 'all'], check=False)
        self.assertNotEqual(code, 0)
        self.assertIn(b'HOSTRUSTC', err)
        self.assertEqual(snapshot, {p.name: p.stat().st_mtime_ns for p in out.iterdir()})
        self.run_command([*missing, 'clean'])
        self.assertEqual(list(out.iterdir()), [])

    def test_tools_caller_out_and_inherited_environment(self):
        caller = self.work / 'caller'
        shutil.copytree(ROOT / 'scripts/tests/tools_fixdep', caller)
        for lang in ('c', 'rust'):
            out = self.work / ('caller-' + lang)
            out.mkdir()
            env = dict(self.env, HOST_TOOLS_LANG=lang, HOSTRUSTC=str(self.rustc),
                       KBUILD_HOSTRUSTFLAGS=shlex.join(self.flags),
                       KBUILD_HOSTCFLAGS='-O2 -Wall -Werror', HOSTCC=str(self.clang),
                       HOSTLD='ld', HOSTAR='ar', CC=str(self.clang), LD='ld', AR='ar',
                       CFLAGS='-O2 -Wall -Werror', MAKEFLAGS='--no-print-directory')
            args = [self.make, '-j8', '-C', caller, 'OUT=' + str(out),
                    'srctree=' + str(self.tree)]
            self.run_command(args, env=env)
            depcmd = out / '.sample.o.cmd'
            self.assertIn('sample.h', depcmd.read_text())
            self.assertNotIn('cannot find fixdep', depcmd.read_text())
            snapshot = {p.name: p.stat().st_mtime_ns for p in out.iterdir()}
            self.run_command(args, env=env)
            self.assertEqual(snapshot, {p.name: p.stat().st_mtime_ns for p in out.iterdir()})
            os.utime(caller / 'sample.h', None)
            self.run_command(args, env=env)
            self.assertNotEqual(snapshot['sample.o'], (out / 'sample.o').stat().st_mtime_ns)
            before = (out / 'sample.o').stat().st_mtime_ns
            self.run_command([*args, 'CFLAGS=-O1 -Wall -Werror'], env=env)
            self.assertNotEqual(before, (out / 'sample.o').stat().st_mtime_ns)
            (out / 'sample.o').unlink()
            self.run_command([*args, 'CFLAGS=-O1 -Wall -Werror'], env=env)
            self.assertTrue((out / 'sample.o').is_file())
            # No OUTPUT/O caller: tools build's O= normalization is independent.
            odir = self.work / ('o-option-' + lang)
            odir.mkdir()
            self.run_command([self.make, '-j8', '-C', self.tree / 'tools/build',
                              'O=' + str(odir)], env=env)
            self.assertTrue((odir / 'fixdep').is_file())

    def test_unconfigured_standalone_and_two_pass(self):
        for lang in ('c', 'rust'):
            out = self.work / ('cold-' + lang)
            out.mkdir()
            env = dict(self.env)
            for name in ('KBUILD_HOSTRUSTFLAGS', 'KBUILD_HOSTCFLAGS', 'RUSTC_BOOTSTRAP',
                         'HOST_TOOLS_LANG', 'OUTPUT', 'O'):
                env.pop(name, None)
            code, stdout, _ = self.run_command([self.make, '-j8', '-C',
                self.tree / 'tools/build', 'OUTPUT=' + str(out) + '/',
                'HOSTRUSTC=' + str(self.rustc), 'HOST_TOOLS_LANG=' + lang], env=env)
            self.assertEqual(code, 0)
            marker = ('HOSTRUSTC ' + str(out / 'fixdep') if lang == 'rust'
                      else 'HOSTCC  ' + str(out / 'fixdep.o')).encode()
            self.assertEqual(stdout.count(marker), 2, stdout.decode())
            self.assertFalse(list((self.tree / 'tools/build').glob('*.o')))

    def test_native_strict_flags(self):
        if self.native is None:
            self.skipTest('optional native command donor not specified; standalone flags tested')
        for flag in ('-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '-Zbinary_dep_depinfo=y'):
            self.assertIn(flag, self.flags)

    def test_hostcc_command_and_header_dependencies(self):
        out = self.work / 'hostcc-command'
        out.mkdir()
        cc = shlex.quote(str(self.gcc)) + ' -DTOOLS_FIXDEP_TEST_WRAPPER=1'
        self.build(out, 'rust', ['HOSTCC=' + cc])
        dep = self.work / 'hostcc-command.d'
        dep.write_bytes(b'x: x.c header.h\n')
        self.assertEqual(self.run_command([out / 'fixdep', dep, 't', 'cc']),
                         self.run_command([self.bins[0], dep, 't', 'cc']))
        snapshot = {p.name: p.stat().st_mtime_ns for p in out.iterdir()}
        self.build(out, 'rust', ['HOSTCC=' + cc])
        self.assertEqual(snapshot, {p.name: p.stat().st_mtime_ns for p in out.iterdir()})
        for source in ('fixdep-libc.h', 'fixdep-abi.py'):
            previous = (out / 'fixdep').stat().st_mtime_ns
            os.utime(self.tree / 'tools/build' / source, None)
            self.build(out, 'rust', ['HOSTCC=' + cc])
            self.assertGreater((out / 'fixdep').stat().st_mtime_ns, previous)
        previous = (out / 'fixdep').stat().st_mtime_ns
        self.build(out, 'rust', ['HOSTCC=' + cc.replace('=1', '=2')])
        self.assertGreater((out / 'fixdep').stat().st_mtime_ns, previous)
        # Preserve flags containing quotes/spaces through the real linker;
        # inspect the resulting dynamic tag rather than the generated command.
        for suffix in ('one', 'two'):
            rpath = str(self.work / ('runtime libraries ' + suffix))
            extra = ['HOSTCC=' + cc,
                     'KBUILD_HOSTLDFLAGS=-Wl,-rpath,' + shlex.quote(rpath)]
            previous = (out / 'fixdep').stat().st_mtime_ns
            self.build(out, 'rust', extra)
            self.assertGreater((out / 'fixdep').stat().st_mtime_ns, previous)
            dynamic = self.run_command(['readelf', '-d', out / 'fixdep'])[1]
            self.assertIn(('[' + rpath + ']').encode(), dynamic)
            snapshot = {p.name: p.stat().st_mtime_ns for p in out.iterdir()}
            self.build(out, 'rust', extra)
            self.assertEqual(snapshot, {p.name: p.stat().st_mtime_ns for p in out.iterdir()})

    def test_hostcc_hidden_i686_target(self):
        if self.i686 is None or self.i686cc is None:
            self.skipTest('genuine i686 std and host compiler wrapper not supplied')
        out = self.work / 'hostcc-i686'
        out.mkdir()
        flags = [*self.flags, '--sysroot=' + self.i686, '--target=i686-unknown-linux-gnu']
        extra = ['HOSTCC=' + str(self.i686cc),
                 'KBUILD_HOSTRUSTFLAGS=' + shlex.join(flags),
                 'TOOLS_FIXDEP_BINDGEN_FLAGS=']
        self.build(out, 'rust', extra)
        self.assertEqual((out / 'fixdep').read_bytes()[4], 1)
        dep = self.work / 'hostcc-i686.d'
        dep.write_bytes(b'x: x.c header.h\n')
        original = self.work / 'hostcc-i686-original'
        self.run_command([self.i686cc, '-O2', self.tree / 'tools/build/fixdep.c', '-o', original])
        self.assertEqual(self.run_command([out / 'fixdep', dep, 't', 'cc']),
                         self.run_command([original, dep, 't', 'cc']))
        snapshot = {p.name: p.stat().st_mtime_ns for p in out.iterdir()}
        self.build(out, 'rust', extra)
        self.assertEqual(snapshot, {p.name: p.stat().st_mtime_ns for p in out.iterdir()})

    def test_hostcc_abi_mismatch_is_rejected(self):
        # A wrapper's hidden layout options cannot be inferred from Rust's
        # target. Detect the mismatch before compiling a callable bad stat ABI.
        out = self.work / 'hostcc-packed'
        out.mkdir()
        wrapper = self.work / 'packed-cc'
        wrapper.write_text('#!/bin/sh\nexec ' + shlex.quote(str(self.gcc)) +
                           ' -fpack-struct=1 "$@"\n')
        wrapper.chmod(0o755)
        code, _, err = self.run_command([self.make, '-C', self.tree / 'tools/build',
            'srctree=' + str(self.tree), 'OUTPUT=' + str(out) + '/',
            'HOST_TOOLS_LANG=rust', 'HOSTCC=' + str(wrapper),
            'HOSTRUSTC=' + str(self.rustc), 'TOOLS_FIXDEP_BINDGEN_FLAGS='], check=False)
        self.assertNotEqual(code, 0)
        self.assertIn(b'HOSTCC/Rust ABI mismatch', err)
        self.assertFalse((out / 'fixdep').exists())

    def test_fstat_fault(self):
        strace = shutil.which('strace')
        if not strace:
            self.skipTest('strace unavailable for genuine fstat fault injection')
        # Discover each executable's actual metadata syscall and occurrence;
        # no guessed Rust/libc stat interface or replacement implementation.
        import re
        dep = self.work / 'dep-fault.d'
        dep.write_bytes(b'x: x.c header.h\n')
        results = []
        for binary in self.bins:
            trace = self.work / (binary.name + '-stat.trace')
            code, _, err = self.run_command([strace, '-yy', '-o', trace,
                '-e', 'trace=fstat,fstat64,newfstatat,statx', binary, dep, 't', 'cc'], check=False)
            if code and b'Operation not permitted' in err:
                self.skipTest('ptrace denied; retained trace/probe log, fstat fault unproven')
            self.assertEqual(code, 0)
            occurrences = {}
            for line in trace.read_text().splitlines():
                match = re.match(r'(\w+)\(', line)
                if not match:
                    continue
                call = match[1]
                occurrences[call] = occurrences.get(call, 0) + 1
                if str(dep) in line:
                    result = self.run_command([strace, '-o', str(trace) + '.fault',
                        '-e', 'inject=' + call + ':error=EIO:when=' + str(occurrences[call]),
                        binary, dep, 't', 'cc'], check=False)
                    results.append(result)
                    break
            else:
                self.fail('no actual depfile metadata syscall in ' + str(trace))
        self.assertEqual(results[0], results[1])
        self.assertEqual(results[0], results[2])
        self.assertEqual(results[0][0], 2)
        self.assertIn(b"error fstat'ing depfile", results[0][2])

    def test_i686_std(self):
        # core-only sysroots cannot build File/metadata users. Do not replace
        # the program with a no_std surrogate and claim host coverage.
        sysroot = self.i686
        if sysroot is None:
            self.skipTest('no optional i686 std sysroot specified (core alone is insufficient)')
        if not sysroot or not Path(sysroot).is_dir():
            self.fail('invalid explicit TOOLS_FIXDEP_I686_SYSROOT')
        libs = Path(sysroot) / 'lib/rustlib/i686-unknown-linux-gnu/lib'
        if not list(libs.glob('libstd-*.rlib')):
            self.skipTest('genuine i686 donor lacks libstd; full host compile/runtime unproven')
        cc = self.i686cc or self.gcc
        ccflags = [] if self.i686cc else ['-m32']
        bindgen_flags = ['--target=i686-unknown-linux-gnu', *self.i686_bindgen_flags]
        dep = self.work / 'i686.d'
        import resource
        def limited_memory():
            resource.setrlimit(resource.RLIMIT_AS, (128 * 1024**2, 128 * 1024**2))
        for label, cflags in (('default', []), ('lfs', ['-D_FILE_OFFSET_BITS=64']),
                ('time64', ['-D_FILE_OFFSET_BITS=64', '-D_TIME_BITS=64'])):
            with self.subTest(abi=label):
                original = self.work / ('c-i686-' + label)
                self.run_command([cc, *ccflags, *cflags, '-O2', '-Wall', '-Werror',
                    self.tree / 'tools/build/fixdep.c', '-o', original])
                abi = self.work / ('libc-i686-' + label + '.rs')
                self.generate_bindings(abi, [*bindgen_flags, *cflags])
                out = self.work / ('rust-i686-' + label)
                self.run_command([self.rustc, *self.flags, '--sysroot', sysroot,
                    '--target=i686-unknown-linux-gnu', '-Clinker=' + str(cc),
                    '--out-dir', self.work, '--emit=link=' + str(out),
                    self.tree / 'tools/build/fixdep.rs'],
                    env=dict(self.env, TOOLS_FIXDEP_LIBC_BINDINGS=str(abi)))
                for binary in (original, out):
                    self.assertEqual(binary.read_bytes()[4], 1)
                # Sparse inputs exercise open overflow, mmap ENOMEM, size_t
                # wrap-to-zero and wrap-to-small without entering C's oversized
                # token-buffer undefined domain. The limit keeps >2GiB maps
                # from succeeding; wrapped 32-byte maps remain valid fixtures.
                for size in (32, 2**31, 2**32, 2**32 + 32):
                    with dep.open('wb') as stream:
                        stream.write(b'x: x.c h.h\n' + b' ' * 21)
                        stream.truncate(size)
                    # The previous ABI's future-time fixture changes atime as
                    # well as mtime. Rewriting only resets mtime, so reset both
                    # before testing size overflow and mmap behavior.
                    os.utime(dep, (1_700_000_000, 1_700_000_000))
                    actual = [self.run_command([binary, dep, 't', 'cc'], check=False,
                        preexec_fn=limited_memory) for binary in (original, out)]
                    self.assertEqual(actual[0], actual[1], (label, size))
                    code, stdout, stderr = actual[0]
                    if label == 'default' and size >= 2**31:
                        self.assertEqual(code, 2)
                        self.assertIn(b'error opening depfile', stderr)
                    else:
                        self.assertEqual(code, 0)
                        if size == 2**31:
                            self.assertEqual(stderr, b'fixdep: mmap: Cannot allocate memory\n')
                        elif size == 2**32:
                            self.assertEqual(stderr, b'fixdep: mmap: Invalid argument\n')
                        else:
                            self.assertEqual(stderr, b'')
                            self.assertIn(b'source_t := x.c\n', stdout)
                if label == 'lfs':
                    with self.subTest(mapping='above_isize'):
                        # No target means C never copies the long token into
                        # its PATH_MAX buffer. A sparse zero file therefore
                        # exercises successful >isize mapping/traversal safely.
                        with dep.open('wb') as stream:
                            stream.truncate(2**31 + 1)
                        reference = self.run_command([original, dep, 't', 'cc'],
                                                     check=False, timeout=30)
                        if reference[0] == 0 and reference[2] == b'fixdep: mmap: Cannot allocate memory\n':
                            self.skipTest('host cannot map >2GiB in the genuine i686 C process')
                        self.assertEqual(reference[0], 1)
                        self.assertEqual(reference[2], b'fixdep: parse error; no targets found\n')
                        self.assertEqual(reference, self.run_command([out, dep, 't', 'cc'],
                                                                    check=False, timeout=30))
                dep.write_bytes(b'x: x.c header.h\n')
                # A future timestamp is rejected by a 32-bit time_t fstat,
                # even when the file's contents and length are ordinary.
                os.utime(dep, (2**32 + 100, 2**32 + 100))
                actual = [self.run_command([binary, dep, 't', 'cc'], check=False)
                          for binary in (original, out)]
                self.assertEqual(actual[0], actual[1], label)
                if label != 'time64':
                    self.assertEqual(actual[0][0], 2)
                    self.assertIn(b"error fstat'ing depfile", actual[0][2])
                else:
                    self.assertEqual(actual[0][0], 0)


if __name__ == '__main__':
    unittest.main()
