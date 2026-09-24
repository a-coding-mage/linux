# SPDX-License-Identifier: GPL-2.0-only
"""Original-C differential and real Kbuild tests for the Unicode generator.

Set UNICODE_UCD_DIR to a complete official Unicode 12.1 directory to enable
full-data gates. Explicit invalid values fail; only an absent variable skips.
UNICODE_TEST_ARTIFACTS preserves source snapshots, commands and binary logs.
UNICODE_TEST_ROOT and UNICODE_CANDIDATE_DIR support isolated candidate review.
"""

import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest
from unittest import mock


INPUTS = dict(zip('acpd fnt'.replace(' ', ''), (
    'DerivedAge.txt', 'DerivedCombiningClass.txt', 'DerivedCoreProperties.txt',
    'UnicodeData.txt', 'CaseFolding.txt', 'NormalizationCorrections.txt',
    'NormalizationTest.txt')))
# Pin inputs, never generated table contents: this gate promises the complete
# official 12.1 corpus, not a shortened or synthetic normalization test file.
UCD_SHA256 = {
    'CaseFolding.txt': '9c772627c6ee77eea6a17b42927b8ee28ca05dc65d6a511062104baaf3d12294',
    'DerivedAge.txt': '2fc081011d8fabaf7cf4937732dd5a6d6a57e492c43f3adfeded513387ee0ec3',
    'DerivedCombiningClass.txt': 'db6f38fb4aa8b9181b5e6a9f320de9d5c2c9b5687116a619b3cb90138b025e0b',
    'DerivedCoreProperties.txt': 'a6eb7a8671fb532fbd88c37fd7b20b5b2e7dbfc8b121f74c14abe2947db0da68',
    'NormalizationCorrections.txt': '8d485b418eb37a3915e2b0ca3e9bf9cdc07fdb0d57184182b60d3485363992ed',
    'NormalizationTest.txt': '8cabbd6293c88ca05f0b601ade0fd16978ac670a077c0e0f419986ddd33c6941',
    'ReadMe.txt': 'cb3613bcfc0ad962bf17cca18af090aac11c350fef294916e97bcb100839baff',
    'UnicodeData.txt': '93ab1acd8fd9d450463b50ae77eab151a7cda48f98b25b56baed8070f80fc936',
}
ROOT = Path(os.environ.get('UNICODE_TEST_ROOT', Path(__file__).resolve().parents[2]))
CANDIDATE = Path(os.environ.get('UNICODE_CANDIDATE_DIR', ROOT / 'fs/unicode'))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def official_data():
    if 'UNICODE_UCD_DIR' not in os.environ:
        return None
    value = os.environ['UNICODE_UCD_DIR']
    if not value or not Path(value).is_dir():
        raise ValueError('UNICODE_UCD_DIR must name a complete official UCD directory')
    path = Path(value).resolve()
    for name in (*INPUTS.values(), 'ReadMe.txt', 'PROVENANCE.json'):
        if not (path / name).is_file() or not (path / name).stat().st_size:
            raise ValueError('UNICODE_UCD_DIR missing or empty: ' + name)
    if 'Version 12.1.0' not in (path / 'ReadMe.txt').read_text():
        raise ValueError('UNICODE_UCD_DIR requires official Unicode 12.1.0')
    provenance = json.loads((path / 'PROVENANCE.json').read_text())
    if provenance.get('version') != '12.1.0':
        raise ValueError('PROVENANCE.json version must be 12.1.0')
    for name, expected in UCD_SHA256.items():
        if digest(path / name) != expected:
            raise ValueError('UNICODE_UCD_DIR is not complete official 12.1 data: ' + name)
    return path


def environment():
    env = {k: v for k, v in os.environ.items() if not k.startswith('KBUILD_') and k not in (
        'MAKEFLAGS', 'MFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES', 'sub_make_done',
        'srctree', 'srcroot', 'objtree', 'VPATH', 'POSIXLY_CORRECT')}
    env.update(LC_ALL='C', RUSTC_BOOTSTRAP='1')
    return env


class UnicodeEnvironmentTest(unittest.TestCase):
    def test_absent_corpus_is_optional(self):
        with mock.patch.dict(os.environ, {}, clear=True):
            self.assertIsNone(official_data())

    def test_explicit_invalid_corpus_fails(self):
        with tempfile.TemporaryDirectory(prefix='unicode-input-validation-') as tmp:
            for value in ('', str(Path(tmp) / 'missing'), tmp):
                with self.subTest(value=value), mock.patch.dict(os.environ, {'UNICODE_UCD_DIR': value}):
                    with self.assertRaises(ValueError):
                        official_data()


class UnicodeGeneratorTest(unittest.TestCase):
    def setUp(self):
        self.case = Path(tempfile.mkdtemp(prefix=self._testMethodName + '-', dir=self.work))

    @classmethod
    def setUpClass(cls):
        # Validate before building: an explicit bad corpus must never silently skip.
        cls.ucd = official_data()
        artifact = os.environ.get('UNICODE_TEST_ARTIFACTS')
        if artifact:
            Path(artifact).mkdir(parents=True, exist_ok=True)
            cls.work = Path(tempfile.mkdtemp(prefix='unicode-', dir=artifact))
        else:
            temp = tempfile.TemporaryDirectory(prefix='unicode-')
            cls.addClassCleanup(temp.cleanup)
            cls.work = Path(temp.name)
        cls.env = environment()
        cls.logs = cls.work / 'logs'
        cls.logs.mkdir()
        cls.src = cls.work / 'src/fs/unicode'
        cls.src.mkdir(parents=True)
        cls.case = cls.work / 'case'
        cls.case.mkdir()
        cls.sequence = 0
        cls.commands = []
        hashes = {}
        for name in ('mkutf8data.c', 'mkutf8data.rs', 'Makefile', 'utf8data.c_shipped'):
            source = (CANDIDATE if name in ('mkutf8data.rs', 'Makefile') else ROOT / 'fs/unicode') / name
            shutil.copyfile(source, cls.src / name)
            hashes[str(source)] = digest(cls.src / name)
        # The generator may split its implementation into sibling Rust modules.
        for source in sorted(CANDIDATE.glob('mkutf8data_*.rs')):
            shutil.copyfile(source, cls.src / source.name)
            hashes[str(source)] = digest(cls.src / source.name)
        for name in ('Makefile', 'scripts/Makefile.build', 'scripts/Makefile.host', 'scripts/Makefile.compiler'):
            hashes[str(ROOT / name)] = digest(ROOT / name)
        if cls.ucd:
            for source in cls.ucd.iterdir():
                if source.name in (*INPUTS.values(), 'ReadMe.txt', 'PROVENANCE.json'):
                    hashes[str(source)] = digest(source)
                    shutil.copyfile(source, cls.src / source.name)
        (cls.work / 'source-hashes.json').write_text(json.dumps(hashes, indent=2) + '\n')
        for source, expected in hashes.items():
            if digest(Path(source)) != expected:
                raise AssertionError('Source changed during snapshot; retry: ' + source)
        def record_current_sources():
            current = {source: digest(Path(source)) if Path(source).is_file() else None for source in hashes}
            (cls.work / 'source-hashes-after.json').write_text(json.dumps(current, indent=2) + '\n')
            (cls.work / 'sources-changed.json').write_text(json.dumps(
                [source for source in hashes if current[source] != hashes[source]], indent=2) + '\n')
        cls.addClassCleanup(record_current_sources)
        cls.cc = shlex.split(os.environ.get('HOSTCC', 'cc'))
        cls.rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        cls.make = shlex.split(os.environ.get('MAKE', 'make'))
        cls.c = cls.work / 'oracle'
        cls.rust = cls.work / 'candidate'
        cls.checked('compile-c', cls.cc + ['-O2', str(cls.src / 'mkutf8data.c'), '-o', str(cls.c)])
        cls.flags = cls.host_rust_flags()
        cls.checked('rust-version', cls.rustc + ['--version', '--verbose'])
        cls.checked('compile-rust', cls.rustc + cls.flags + [str(cls.src / 'mkutf8data.rs'), '-o', str(cls.rust)])

    @classmethod
    def execute(cls, name, argv, **kwargs):
        cls.sequence += 1
        stem = cls.logs / ('%04d-%s' % (cls.sequence, name))
        command = dict(name=name, argv=[os.fsdecode(x) for x in argv],
                       cwd=str(kwargs.get('cwd', cls.work)))
        if 'executable' in kwargs:
            command['executable'] = str(kwargs['executable'])
        cls.commands.append(command)
        (cls.work / 'commands.json').write_text(json.dumps(cls.commands, indent=2) + '\n')
        try:
            result = subprocess.run(argv, env=cls.env, cwd=kwargs.pop('cwd', cls.work),
                                    capture_output=not ('stdout' in kwargs or 'stderr' in kwargs),
                                    timeout=180, **kwargs)
        except subprocess.TimeoutExpired as error:
            stem.with_suffix('.stdout').write_bytes(error.stdout or b'')
            stem.with_suffix('.stderr').write_bytes(error.stderr or b'')
            stem.with_suffix('.status').write_text('TIMEOUT\n')
            raise
        stem.with_suffix('.stdout').write_bytes(result.stdout or b'')
        stem.with_suffix('.stderr').write_bytes(result.stderr or b'')
        stem.with_suffix('.status').write_text(str(result.returncode) + '\n')
        return result

    @classmethod
    def checked(cls, name, argv, **kwargs):
        result = cls.execute(name, argv, **kwargs)
        if result.returncode:
            raise AssertionError('%s failed (%s): %r' % (name, result.returncode, result.stderr))
        return result

    @classmethod
    def host_rust_flags(cls):
        # Evaluate the actual assignments, including version-dependent additions,
        # in GNU make without invoking any kernel target or touching the source tree.
        text = (ROOT / 'Makefile').read_text().replace('\\\n', ' ')
        names = ('rust_common_flags', 'rust_common_flags_per_version', 'KBUILD_HOSTRUSTFLAGS')
        assignments = [line for line in text.splitlines() if re.match(
            r'^(?:export )?(?:' + '|'.join(names) + r')\s*(?::=|\+=|=)', line)]
        if len(assignments) < 5:
            raise AssertionError('Could not extract actual KBUILD_HOSTRUSTFLAGS assignments')
        version = cls.checked('flag-rust-version', cls.rustc + ['--version']).stdout.decode()
        major, minor, patch = map(int, re.search(r'(\d+)\.(\d+)\.(\d+)', version).groups())
        probe = cls.work / 'flags.mk'
        probe.write_text('srctree := ' + str(ROOT) + '\nCONFIG_RUSTC_VERSION := ' +
                         str(major * 10000 + minor * 100 + patch) + '\n' +
                         'include $(srctree)/scripts/Kbuild.include\n' +
                         'include $(srctree)/scripts/Makefile.compiler\n' +
                         '\n'.join(assignments) + '\n.PHONY: flags\nflags:\n\t@printf "%s\\n" "$(KBUILD_HOSTRUSTFLAGS)"\n')
        result = cls.checked('actual-host-rust-flags', cls.make + ['-s', '-f', str(probe),
                            'HOSTRUSTFLAGS=-Dwarnings', 'flags'])
        flags = shlex.split(result.stdout.decode())
        if not {'-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '-Zbinary_dep_depinfo=y'}.issubset(flags):
            raise AssertionError('Incomplete actual host Rust flags')
        return flags

    def pair(self, name, args, output='result.c', expected=None, stream=None):
        observations = []
        for lang, binary in (('c', self.c), ('rust', self.rust)):
            target = self.case / output
            if target.is_file() and not target.is_symlink():
                target.unlink()
            if output == 'result.c':
                target.write_bytes(b'OUTPUT SENTINEL\n')
            options = {}
            if stream == 'full':
                options.update(stdout=open('/dev/full', 'wb'), stderr=subprocess.PIPE)
            elif stream in ('closed', 'closed-ignored'):
                read, write = os.pipe()
                os.close(read)
                options.update(stdout=os.fdopen(write, 'wb'), stderr=subprocess.PIPE)
                if stream == 'closed-ignored':
                    # Python starts with SIGPIPE ignored; preserve that inherited
                    # disposition as well as testing exec's default restoration.
                    options['restore_signals'] = False
            try:
                p = self.execute(name + '-' + lang, ['mkutf8data'] + args,
                                 executable=binary, cwd=self.case, **options)
            finally:
                if options:
                    options['stdout'].close()
            emitted = target.read_bytes() if target.is_file() and not target.is_symlink() else None
            observations.append((p.returncode, p.stdout, p.stderr, emitted))
            if emitted is not None:
                (self.logs / (name + '-' + lang + '.output')).write_bytes(emitted)
        for field, left, right in zip(('status', 'stdout', 'stderr', 'output'), *observations):
            if isinstance(left, bytes) and isinstance(right, bytes):
                self.assertTrue(left == right, '%s %s differ: C %d bytes sha256=%s; Rust %d bytes sha256=%s; logs=%s' % (
                    name, field, len(left), hashlib.sha256(left).hexdigest(),
                    len(right), hashlib.sha256(right).hexdigest(), self.logs))
            else:
                self.assertEqual(left, right, name + ' ' + field)
        if expected is not None:
            self.assertEqual(observations[0][0], expected, name)
        return observations[0]

    def require_ucd(self):
        if self.ucd is None:
            self.skipTest('UNICODE_UCD_DIR absent: full official-data gate skipped')

    def official_args(self):
        return [arg for flag, name in INPUTS.items() for arg in ('-' + flag, str(self.src / name))]

    def test_cli_and_early_errors(self):
        (self.case / 'empty').write_bytes(b'')
        cases = [([], 1), (['-h'], 0), (['-vh'], 0), (['-z'], 1),
                 (['-hz'], 0), (['-zh'], 1), (['-\udcff'], 1),
                 (['--help'], 1), (['--'], 1), (['operand', '-h'], 0),
                 (['--', '-h'], 1), (['-', '-h'], 0),
                 (['-a', 'absent'], 1), (['-aabsent'], 1),
                 (['-a', 'raw-\udcff-missing'], 1), (['-a', 'empty'], 1),
                 (['-a', 'ignored', '-a', 'empty'], 1), (['-vv', '-aempty'], 1)]
        cases += [(['-' + flag], 1) for flag in sorted(INPUTS.keys() | {'o'})]
        for i, (args, status) in enumerate(cases):
            with self.subTest(args=args):
                self.pair('cli-%02d' % i, args, expected=status)
        age_cases = {
            'invalid-age': b'# Age=V65536_0\n0041 ; 1.0\n',
            'invalid-minor': b'# Age=V1_256\n0041 ; 1.0\n',
            'invalid-revision': b'# Age=V1_0_256\n0041 ; 1.0\n',
            'no-entries': b'# Age=V1_0\n',
            'too-many': b'# Age=V1_0\n' * 256 + b'0041 ; 1.0\n',
            'raw-\udcff-age': b'# Age=V1_0\n0041 ; 1.0\n',
        }
        # C stores scanf %d fields in unsigned integers but prints signed %d.
        # Exercise each component and high-bit patterns with verbose diagnostics.
        for component in range(3):
            for value in ('-1', '-2147483648', '2147483648', '4294967295'):
                version = ['1', '0', '0']
                version[component] = value
                age_cases['signed-age-%d-%s' % (component, value)] = (
                    '# Age=V' + '_'.join(version) + '\n0041 ; 1.0\n').encode()
        for name, content in age_cases.items():
            (self.case / name).write_bytes(content)
            with self.subTest(name=name):
                self.pair('early-' + name, ['-vv', '-a', name], expected=1)

    def test_official_output_and_diagnostics(self):
        self.require_ucd()
        result = self.pair('official', ['-v'] + self.official_args() + ['-o', 'result.c'], expected=0)
        rows = sum(1 for line in (self.src / INPUTS['t']).read_bytes().splitlines()
                   if line and not line.startswith((b'#', b'@')) and line.count(b';') >= 5)
        self.assertGreater(rows, 10000)
        self.assertIn(('Ran %d tests with 0 failures' % rows).encode(), result[1])
        # C verifier reports failures without necessarily failing its exit status.
        self.assertNotRegex(result[1], rb'(?m)^.* code .*$')
        self.assertGreaterEqual(result[1].count(b'Verifying nfdi_'), 1)
        self.assertGreaterEqual(result[1].count(b'Verifying nfdicf_'), 1)
        self.assertEqual(result[3], (self.src / 'utf8data.c_shipped').read_bytes())
        self.pair('official-quiet', self.official_args() + ['-o', 'result.c'], expected=0)
        self.pair('official-verbose2', ['-vv'] + self.official_args() + ['-o', 'result.c'], expected=0)

    def test_nonseekable_age_input(self):
        # A pipe cannot be rewound: buffering and replaying its contents changes
        # the original C parser's second-pass behavior.
        for index, data in enumerate((b'# Age=V1_0\n0041 ; 1.0 #\n',
                                      b'# Age=V65536_0\n', b'# Age=V-1_0\n')):
            observations = []
            for lang, binary in (('c', self.c), ('rust', self.rust)):
                p = self.execute('pipe-age-%d-%s' % (index, lang),
                                 ['mkutf8data', '-a', '/dev/stdin', '-vv'],
                                 executable=binary, cwd=self.case, input=data)
                observations.append((p.returncode, p.stdout, p.stderr))
            self.assertEqual(observations[0], observations[1])
            self.assertEqual(observations[0][0], 1)

    def test_official_default_names(self):
        self.require_ucd()
        for name in INPUTS.values():
            shutil.copyfile(self.src / name, self.case / name)
        result = self.pair('default-names', [], output='utf8data.c', expected=0)
        self.assertEqual(result[3], (self.src / 'utf8data.c_shipped').read_bytes())
        self.pair('option-permutation', ['operand', '-oresult.c', '-aabsent',
                  '-aDerivedAge.txt', '-v'], expected=0)

    def test_official_input_and_output_errors(self):
        self.require_ucd()
        (self.case / 'empty').write_bytes(b'')
        (self.case / 'directory').mkdir(exist_ok=True)
        for flag in INPUTS:
            with self.subTest(flag=flag):
                self.pair('missing-' + flag, self.official_args() + ['-' + flag, 'absent', '-o', 'result.c'], expected=1)
                self.pair('empty-' + flag, self.official_args() + ['-' + flag, 'empty', '-o', 'result.c'])
        self.pair('directory-input', ['-a', 'directory', '-o', 'result.c'], expected=1)
        for out in ('absent-parent/result', 'directory'):
            self.pair('output-' + out.replace('/', '-'), self.official_args() + ['-o', out], output=out, expected=1)
        self.pair('byte-output', self.official_args() + ['-o', 'raw-\udcff-output'], output='raw-\udcff-output', expected=0)
        raw_age = self.case / 'raw-\udcff-valid-age'
        shutil.copyfile(self.src / INPUTS['a'], raw_age)
        self.pair('byte-input', self.official_args() + ['-a', raw_age.name, '-o', 'result.c'], expected=0)
        if Path('/dev/full').exists():
            full = self.case / 'full'
            if not full.exists():
                full.symlink_to('/dev/full')
            self.pair('full-output', self.official_args() + ['-o', 'full'], output='full')
            self.pair('full-stdout', ['-h'], stream='full')
            self.pair('full-stdout-official', ['-v'] + self.official_args() + ['-o', 'result.c'], stream='full')
        self.pair('closed-stdout', ['-h'], stream='closed')
        self.pair('closed-ignored-stdout', ['-h'], stream='closed-ignored')
        self.pair('closed-stdout-official', ['-v'] + self.official_args() + ['-o', 'result.c'], stream='closed')
        self.pair('closed-ignored-stdout-official', ['-v'] + self.official_args() + ['-o', 'result.c'], stream='closed-ignored')

    def test_normalization_negative_control(self):
        self.require_ucd()
        # Change a real normalization row so a skipped self-test cannot pass.
        data = (self.src / INPUTS['t']).read_bytes()
        lines = data.splitlines(keepends=True)
        for i, line in enumerate(lines):
            if line and not line.startswith((b'#', b'@')) and line.count(b';') >= 5:
                columns = line.split(b';')
                columns[2] = b'0042'
                lines[i] = b';'.join(columns)
                break
        corrupt = self.case / 'bad-normalization.txt'
        corrupt.write_bytes(b''.join(lines))
        result = self.pair('bad-normalization', ['-v'] + self.official_args() +
                           ['-t', str(corrupt), '-o', 'result.c'], expected=1)
        self.assertIn(b'failure', result[1])
        self.assertEqual(result[3], b'OUTPUT SENTINEL\n')

    def test_real_kbuild(self):
        for initial in ('c', 'rust'):
            for regeneration in (False, True) if self.ucd else (False,):
                with self.subTest(initial=initial, regeneration=regeneration):
                    self.kbuild_sequence(initial, regeneration)

    def kbuild_sequence(self, initial, regeneration=False, languages=None):
        out = self.work / ('build-' + initial + ('-regen' if regeneration else '-shipped'))
        (out / 'scripts/basic').mkdir(parents=True)
        self.checked('fixdep-' + initial, self.cc + ['-O2', '-I' + str(ROOT / 'scripts/include'),
                     str(ROOT / 'scripts/basic/fixdep.c'), '-o', str(out / 'scripts/basic/fixdep')])
        base = self.make + ['-j4', '-f', str(ROOT / 'scripts/Makefile.build'),
            'srctree=' + str(ROOT), 'srcroot=' + str(self.work / 'src'),
            'objtree=.', 'obj=fs/unicode', 'building_out_of_srctree=1',
            'VPATH=' + str(self.work / 'src'), 'HOSTCC=' + shlex.join(self.cc),
            'HOSTRUSTC=' + shlex.join(self.rustc), 'KBUILD_HOSTCFLAGS=-O2',
            'KBUILD_HOSTRUSTFLAGS=' + shlex.join(self.flags)]
        target = 'fs/unicode/mkutf8data'
        generated = 'fs/unicode/utf8data.c'
        langs = languages or (initial, 'rust' if initial == 'c' else 'c', initial)
        for index, lang in enumerate(langs):
            args = base + ['HOST_TOOLS_LANG=' + lang]
            if regeneration:
                args += ['REGENERATE_UTF8DATA=1']
            label = '%s-%d-%s' % (initial, index, lang)
            self.checked('build-' + label, args + [target, generated], cwd=out)
            dep = (out / 'fs/unicode/.mkutf8data.cmd').read_text()
            extension = 'rs' if lang == 'rust' else 'c'
            self.assertIn(str(self.src / ('mkutf8data.' + extension)), dep)
            if lang == 'rust':
                self.assertIn('-Dwarnings', dep)
                self.assertIn('-Dunsafe_op_in_unsafe_fn', dep)
                self.assertIn('-Zbinary_dep_depinfo=y', dep)
            self.assertEqual((out / generated).read_bytes(), (self.src / 'utf8data.c_shipped').read_bytes())
            stamp = (out / target).stat().st_mtime_ns
            shipped_stamp = (out / generated).stat().st_mtime_ns
            self.checked('noop-' + label, args + [target, generated], cwd=out)
            self.assertEqual((out / target).stat().st_mtime_ns, stamp)
            self.assertEqual((out / generated).stat().st_mtime_ns, shipped_stamp)
            source = self.src / ('mkutf8data.' + extension)
            dry = self.checked('source-dep-' + label, args + ['-n', '-W', str(source), target], cwd=out)
            self.assertIn(b'mkutf8data.' + extension.encode(), dry.stdout)
            inactive = self.src / ('mkutf8data.c' if lang == 'rust' else 'mkutf8data.rs')
            dry = self.checked('inactive-source-' + label, args + ['-n', '-W', str(inactive), target], cwd=out)
            self.assertNotIn(b'HOSTCC', dry.stdout)
            self.assertNotIn(b'HOSTRUSTC', dry.stdout)
            if lang == 'rust':
                for module in sorted(self.src.glob('mkutf8data_*.rs')):
                    self.assertIn(str(module), dep)
                    dry = self.checked('module-dep-' + label + '-' + module.stem,
                                       args + ['-n', '-W', str(module), target], cwd=out)
                    self.assertIn(b'--emit=link=', dry.stdout)
            if self.ucd:
                regen = args + ['REGENERATE_UTF8DATA=1', generated]
                self.checked('regen-' + label, regen, cwd=out)
                self.assertEqual((out / generated).read_bytes(), (self.src / 'utf8data.c_shipped').read_bytes())
                stamp = (out / generated).stat().st_mtime_ns
                self.checked('regen-noop-' + label, regen, cwd=out)
                self.assertEqual((out / generated).stat().st_mtime_ns, stamp)
                for name in INPUTS.values():
                    dry = self.checked('ucd-dep-' + label + '-' + name,
                                       regen + ['-n', '-W', str(self.src / name)], cwd=out)
                    self.assertIn(b'mkutf8data', dry.stdout)
                    self.assertIn(b'-a ', dry.stdout)
                    before = (out / generated).stat().st_mtime_ns
                    os.utime(self.src / name, None)
                    self.checked('ucd-rebuild-' + label + '-' + name, regen, cwd=out)
                    self.assertNotEqual((out / generated).stat().st_mtime_ns, before)
                    self.assertEqual((out / generated).read_bytes(),
                                     (self.src / 'utf8data.c_shipped').read_bytes())
                shipped_args = [arg for arg in args if arg != 'REGENERATE_UTF8DATA=1']
                self.checked('return-shipped-' + label, shipped_args + [generated], cwd=out)
                self.assertEqual((out / generated).read_bytes(), (self.src / 'utf8data.c_shipped').read_bytes())


if __name__ == '__main__':
    unittest.main()
