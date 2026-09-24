#!/usr/bin/env python3
"""Private native rbtree gates; no full kernel build or donor writes.

RBTREE_TEST_ARTIFACTS is an existing writable output parent outside all source
and native trees. Each process gets a unique retained workspace beneath it.
Explicit invalid source/build/compiler/sysroot/artifact inputs fail before any
output creation. Optional unavailable native targets are reported as skips.
"""
from concurrent.futures import ThreadPoolExecutor
from contextlib import ExitStack, contextmanager
import json
import os
import re
import resource
import signal
import shutil
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest
from unittest import mock
import sys

from rbtree_native.transport import (NATIVE_REQUIRED, NativeWriteWatch, compiler_environment,
    native_flags, outside, require_rustc, response_flags, target_rlib, validate_settings,
    validate_compiler_outputs, verify_flag_policy, verify_strict_lints)

ROOT = Path(__file__).resolve().parents[2]
SETTINGS = validate_settings(ROOT, os.environ)
SOURCE, NATIVE = SETTINGS['source'], SETTINGS['native']
ART = Path(tempfile.mkdtemp(prefix='rbtree-native-tests-', dir=SETTINGS['parent']))
print('Retained rbtree fixture evidence: ' + str(ART), flush=True)
FIXTURES = ROOT / 'scripts/tests/rbtree_native'
EXPORTS = re.findall(r'EXPORT_SYMBOL(?:_GPL)?\((\w+)\)', (SOURCE / 'lib/rbtree.c').read_text())


@contextmanager
def retained_directory(*, prefix, dir):
    """Retain generated sources even when compilation or an assertion fails."""
    yield tempfile.mkdtemp(prefix=prefix, dir=dir)


def require_native(native=NATIVE):
    if all((native / name).is_file() for name in NATIVE_REQUIRED):
        return
    if any(name in os.environ for name in ('RBTREE_NATIVE', 'NATIVE_RUST_KERNEL_BUILD')) or native != NATIVE:
        raise AssertionError('explicit native fixture lacks actual rmeta/commands: '+str(native))
    raise unittest.SkipTest('optional RBTREE_NATIVE/NATIVE_RUST_KERNEL_BUILD not available')


_COMMAND = 0
def run(args, name, cwd=None, env=None, input=None):
    global _COMMAND
    cwd = outside(Path(cwd or ART), SETTINGS['protected'])
    if ART != cwd and ART not in cwd.parents:
        raise ValueError('command cwd must be inside this private fixture workspace')
    env = compiler_environment(cwd, dict(os.environ) if env is None else env)
    args = [str(a) for a in args]
    if Path(args[0]).name in ('rustc', 'clang', 'clang-18', 'bindgen'):
        validate_compiler_outputs(args, cwd, ART, SETTINGS['protected'])
    if Path(args[0]).name == 'rustc':
        require_rustc(args[0], cwd, env)
        verify_strict_lints(args[1:])
    _COMMAND += 1
    result = subprocess.run(args, cwd=cwd, env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, input=input)
    (ART / (f'{_COMMAND:04d}-' + name + '.log')).write_text('cwd=' + str(cwd) +
        '\nTMPDIR=' + env['TMPDIR'] + '\n' + shlex.join(args) + '\n' + result.stdout + f'\nexit={result.returncode}\n')
    if result.returncode:
        raise AssertionError(f'{name}: exit {result.returncode}\n{result.stdout}')
    return result.stdout


def execute(binary, args=()):
    """Run even expected-failure fixtures with private cwd/temp and no cores."""
    return subprocess.run([binary, *args], cwd=ART,
        env=compiler_environment(ART, dict(os.environ)),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))


def native_command(output, source, native=NATIVE):
    require_native(native)
    saved = (native / 'lib/.list_sort_rust.o.cmd').read_text().splitlines()[0]
    args = shlex.split(saved.split(' := ', 1)[1])
    env = os.environ.copy()
    env['RUSTC_BOOTSTRAP'] = '1'
    while '=' in args[0] and not args[0].startswith('/'):
        key, value = args.pop(0).split('=', 1)
        env[key] = value
    flags = native_flags(args[1:], native, 'rust')
    verify_flag_policy(flags, 'rust')
    outside(output.parent, SETTINGS['protected'])
    return [args[0], *flags, '--out-dir=' + str(output.parent),
            '--emit=dep-info=' + str(output.with_suffix('.d')), '--emit=obj=' + str(output), str(source)], env


def binding_facade(native=NATIVE):
    """Compile the actual new binding source over immutable real native types.

    The private facade reexports the real kernel, including its LKMM atomics.
    It replaces only the two declarations under repair; no fake layout/API.
    This avoids changing/rebuilding the supplied native kernel output.
    """
    folder=ART/('facade-'+native.name);folder.mkdir(parents=True,exist_ok=True)
    source=folder/'facade.rs'
    source.write_text('''//! Native type/API preserving private fixture facade.
#![feature(cfi_encoding)]
extern crate kernel as original;
pub use original::*;
mod bindings_raw { pub(crate) use original::bindings::{rb_node,rb_root}; }
#[path="'''+str(ROOT/'rust/bindings/rbtree.rs')+'''"] mod corrected;
/// Original generated types with precisely corrected function declarations.
pub mod bindings {
    pub use original::bindings::*;
    pub use crate::corrected::{RbAugmentRotate,__rb_insert_augmented,__rb_erase_color};
}
''')
    output=folder/'facade.o';metadata=folder/'librbtree_kernel.rmeta'
    args,env=native_command(output,source,native)
    args=[a+',cfi_encoding' if a.startswith('-Zallow-features=') else a for a in args]
    args += ['--crate-name=rbtree_kernel','--emit=metadata='+str(metadata)]
    run(args,'facade-'+native.name,env=env)
    return metadata


def native_compile(output, extra=(), source=None, native=NATIVE):
    facade=binding_facade(native)
    args,env=native_command(output,source or ROOT/'lib/rbtree_rust.rs',native)
    args=[('kernel='+str(facade)) if a=='kernel' else a for a in args]
    args.extend(extra)
    return run(args, output.stem + '-compile', env=env)


def c_compile(source, output, extra=(), native=NATIVE, remove=()):
    require_native(native)
    # Always-C donor. Never infer C flags from a potentially selected Rust file.
    saved = (native / 'lib/.scatterlist.o.cmd').read_text().splitlines()[0]
    args = shlex.split(saved.split(' := ', 1)[1])
    if not args[-1].endswith('.c'):
        raise ValueError('saved C donor command has no single original source')
    flags = native_flags(args[1:-1], native, 'c')
    verify_flag_policy(flags, 'c')
    outside(output.parent, SETTINGS['protected'])
    args = [args[0], *[a for a in flags if a not in remove], *extra,
            '-Wp,-MMD,' + str(output.with_suffix('.d')), '-o', output, source]
    return run(args, output.stem + '-compile')


def atomic_helpers(native=NATIVE):
    """Use unchanged real kernel C atomic helper definitions in process tests."""
    output=ART/('atomic-'+native.name+'.o')
    c_compile(SOURCE/'rust/helpers/atomic_ext.c',output,
              ['-D__rust_helper=','-ffunction-sections'],native=native)
    return output


def bindgen_command(native, source, output, raw=False):
    """Actual target preprocessing flags, with all output paths redirected."""
    require_native(native)
    compiler=SETTINGS['bindgen']
    if not compiler:
        if 'BINDGEN' in os.environ:raise AssertionError('explicit BINDGEN is not executable')
        raise unittest.SkipTest('optional bindgen executable missing')
    saved=(native/'rust/bindings/.bindings_generated.rs.cmd').read_text().splitlines()[0]
    args=shlex.split(saved.split(' := ',1)[1].split(' ; ',1)[0].replace('$(pound)','#'))
    cpp=native_flags(args[args.index('--')+1:], native, 'c')
    verify_flag_policy(cpp, 'c')
    if raw:
        params=['--allowlist-function','__rb_insert_augmented|__rb_erase_color',
                '--blocklist-type','.*','--no-doc-comments']
    else:
        params=shlex.split('\n'.join(line for line in (ROOT/'rust/bindgen_parameters').read_text().splitlines()
                                    if not line.lstrip().startswith('#')))
        params+=['--with-derive-default','--no-debug','.*','--enable-function-attribute-detection']
    return [compiler,source,*params,'--rust-target','1.85','--use-core','--ctypes-prefix','ffi',
            '--no-layout-tests','-o',output,'--',*cpp, '-Wp,-MMD,' + str(output.with_suffix('.d'))]


class PrivateTestCase(unittest.TestCase):
    def setUp(self):
        stack = ExitStack()
        watches = [stack.enter_context(NativeWriteWatch(build)) for build in
                   (NATIVE, SETTINGS['arm']) if build is not None and build.is_dir()]
        def finish():
            stack.close()
            observed = [dict(root=str(w.root), directories=len(w.paths), writes=w.events) for w in watches]
            (ART / (self.id() + '-donor-observation.json')).write_text(json.dumps(observed, indent=2) + '\n')
            self.assertFalse(any(w.events for w in watches), observed)
        self.addCleanup(finish)


class RbtreeTransport(PrivateTestCase):
    def test_invalid_settings_fail_before_outputs_or_commands(self):
        base = {'RBTREE_SOURCE': str(SOURCE), 'RBTREE_TEST_ARTIFACTS': str(ART)}
        if all((NATIVE/name).is_file() for name in NATIVE_REQUIRED):
            base['RBTREE_NATIVE'] = str(NATIVE)
        if SETTINGS['sysroot'] is not None:
            base['RBTREE_I686_CORE'] = str(SETTINGS['sysroot'])
        cases = [(name, value) for name in ('RBTREE_SOURCE', 'RBTREE_NATIVE',
                 'NATIVE_RUST_KERNEL_BUILD', 'RBTREE_ARM64', 'RBTREE_TEST_ARTIFACTS',
                 'HOSTRUSTC', 'BINDGEN', 'RBTREE_I686_CORE', 'INT_MATH_I686_SYSROOT')
                 for value in ('', 'bad\0path', str(ART/'missing-input'))]
        cases += [('RBTREE_TEST_ARTIFACTS', str(path)) for path in
                  (ROOT, SOURCE, NATIVE, NATIVE/'rust', ROOT/'scripts')]
        if SETTINGS['sysroot'] is not None:
            cases += [('RBTREE_TEST_ARTIFACTS', str(SETTINGS['sysroot']))]
        for name, value in cases:
            with self.subTest(name=name, value=repr(value)):
                with mock.patch('tempfile.mkdtemp') as create, mock.patch('subprocess.run') as spawn:
                    with self.assertRaises(ValueError):
                        validate_settings(ROOT, {**base, name: value})
                    create.assert_not_called()
                    spawn.assert_not_called()
        # Exercise import order too: no fixture workspace may be created first.
        for forbidden in (SOURCE, NATIVE):
            before = set(ART.iterdir())
            env = compiler_environment(ART, dict(os.environ, RBTREE_SOURCE=str(SOURCE),
                RBTREE_TEST_ARTIFACTS=str(forbidden), PYTHONDONTWRITEBYTECODE='1',
                PYTHONPATH=str(ROOT/'scripts/tests')))
            result = subprocess.run([sys.executable, '-B', '-c', 'import test_rbtree_native'],
                cwd=ART, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('outside source/native trees', result.stdout)
            self.assertNotIn('Retained rbtree fixture evidence:', result.stdout)
            self.assertEqual(set(ART.iterdir()), before | {ART/'tmp'})
            (ART/('invalid-import-'+forbidden.name+'.log')).write_text(result.stdout)

    def test_responses_paths_outputs_and_strict_policy(self):
        work=ART/'transport-control';work.mkdir()
        (work/'rust.rsp').write_text('--target=target.json\n-Ldependency=deps\n--extern=kernel=kernel.rmeta\n--out-dir=unsafe\n--emit=obj=unsafe.o\n@nested.rsp\n')
        (work/'nested.rsp').write_text('-Dwarnings\n-Zsanitizer=kcfi\n-Zsanitizer=address\n-Zsanitizer-cfi-normalize-integers\n')
        flags=native_flags(['@rust.rsp'],work,'rust')
        self.assertEqual(flags[:3], ['--target='+str(work/'target.json'),
            '-Ldependency='+str(work/'deps'), '--extern=kernel='+str(work/'kernel.rmeta')])
        self.assertNotIn('unsafe', ' '.join(flags))
        verify_flag_policy(flags,'rust')
        (work/'c.rsp').write_text('-I "include space" -include forced.h -isystem system -Llib -MF unsafe.d -o unsafe.o')
        self.assertEqual(native_flags(['@c.rsp'],work,'c'),
            ['-I',str(work/'include space'),'-include',str(work/'forced.h'),
             '-isystem',str(work/'system'),'-L'+str(work/'lib')])
        (work/'loop.rsp').write_text('@loop.rsp\n')
        with self.assertRaisesRegex(ValueError,'recursive'):
            list(response_flags(['@loop.rsp'],work,'rust'))
        for arguments in (['--extern'], ['--out-dir'], ['-Cincremental=elsewhere']):
            with self.assertRaises(ValueError): native_flags(arguments,work,'rust')
        for arguments in (['-Dwarnings','-Awarnings'], ['-Dwarnings','--cap-lints=warn'],
                          ['-Dwarnings','--force-warn=unused_variables'], ['-Wwarnings']):
            with self.assertRaises(ValueError): verify_strict_lints(arguments)
        for arguments in (['-Dwarnings'], ['-D','warnings'], ['--deny=warnings'],
                          ['-Fwarnings','-Awarnings'], ['-Dwarnings','--cap-lints=deny','--cap-lints=warn']):
            verify_strict_lints(arguments)
        for arguments in (['-Zsanitizer=kcfi','-Zsanitizer-cfi-normalize-integers=no','-Dwarnings'],
                          ['-Zsanitizer=address','-Zsanitizer-cfi-normalize-integers','-Dwarnings']):
            with self.assertRaises(ValueError): verify_flag_policy(arguments,'rust')
        for arguments in (['-o',str(SOURCE/'forbidden.o')], ['--out-dir='+str(NATIVE)],
                          ['--emit=obj='+str(SOURCE/'forbidden.o')], ['-MF'+str(NATIVE/'unsafe.d')],
                          ['-Zdump-mir-dir='+str(NATIVE)]):
            with self.assertRaises(ValueError):
                validate_compiler_outputs(['rustc',*arguments],work,ART,SETTINGS['protected'])
        (work/'output.rsp').write_text('--out-dir='+str(NATIVE)+'\n')
        with self.assertRaises(ValueError):
            validate_compiler_outputs(['rustc','@output.rsp'],work,ART,SETTINGS['protected'])

    def test_transient_write_observer_control(self):
        mimic=ART/'private-donor-mimic';mimic.mkdir()
        with NativeWriteWatch(mimic) as watch:
            transient=mimic/'deleted.rcgu.o'
            transient.write_bytes(b'private observation control')
            transient.unlink()
        self.assertFalse(list(mimic.iterdir()))
        self.assertTrue(any(int(mask,16)&0x100 for _,mask in watch.events), watch.events)
        self.assertTrue(any(int(mask,16)&0x200 for _,mask in watch.events), watch.events)
        (ART/'transient-control.json').write_text(json.dumps(watch.events,indent=2)+'\n')

    def test_actual_rustc_transient_control(self):
        # Reproduce the lost out-dir failure only in a disposable private cwd,
        # never a native donor. Actual rustc creates and deletes its rcgu there.
        require_native()
        work=ART/'actual-transient';work.mkdir()
        mimic=work/'private-donor-cwd';mimic.mkdir()
        source=work/'transient.rs'
        source.write_text('//! Private actual compiler output-location control.\n'
            '/// An emitted nonempty C-ABI function.\n#[no_mangle]\n'
            'pub extern "C" fn transport_probe(x: u32) -> u32 { x.wrapping_add(1) }\n')
        args,env=native_command(work/'transient.o',source)
        args += ['--emit=metadata='+str(work/'libtransient.rmeta')]
        compiler_environment(mimic,env)
        negative=[a for a in args if not str(a).startswith('--out-dir=')]
        with NativeWriteWatch(mimic) as missing_out_dir:
            run(negative,'actual-transient-missing-out-dir',cwd=mimic,env=env)
        events=[(path,int(mask,16)) for path,mask in missing_out_dir.events if path.endswith('.rcgu.o')]
        self.assertTrue(any(mask&0x100 for _,mask in events),missing_out_dir.events)
        self.assertTrue(any(mask&0x200 for _,mask in events),missing_out_dir.events)
        self.assertFalse(list(mimic.glob('*.rcgu.o')))
        with NativeWriteWatch(mimic) as private_out_dir:
            run(args,'actual-transient-private-out-dir',cwd=mimic,env=env)
        self.assertFalse(private_out_dir.events,private_out_dir.events)
        (work/'actual-compiler-transients.json').write_text(json.dumps({
            'without_private_out_dir':missing_out_dir.events,
            'with_private_out_dir':private_out_dir.events},indent=2)+'\n')

    def test_concurrent_real_bindings_and_native_outputs(self):
        require_native()
        env=compiler_environment(ART, dict(os.environ, RBTREE_SOURCE=str(SOURCE),
            RBTREE_TEST_ARTIFACTS=str(ART), PYTHONDONTWRITEBYTECODE='1',
            PYTHONPATH=str(ROOT/'scripts/tests')))
        command=[sys.executable,'-B','-m','unittest','-v',
                 'test_rbtree_native.RbtreeNative.test_actual_binding_crate',
                 'test_rbtree_native.RbtreeNative.test_native_selection_gate']
        def launch(index):
            proc=subprocess.run(command,cwd=ART,env=env,stdout=subprocess.PIPE,
                                stderr=subprocess.STDOUT,text=True)
            (ART/f'concurrent-{index}.log').write_text(shlex.join(command)+'\n'+proc.stdout+f'\nexit={proc.returncode}\n')
            return proc
        with ThreadPoolExecutor(max_workers=2) as pool:
            results=list(pool.map(launch,range(2)))
        workspaces=[]
        for result in results:
            self.assertEqual(result.returncode,0,result.stdout)
            self.assertIn('Ran 2 tests',result.stdout)
            self.assertNotIn('skipped=',result.stdout)
            self.assertIn('\nOK\n',result.stdout)
            workspaces += re.findall(r'Retained rbtree fixture evidence: (.+)', result.stdout)
        self.assertEqual(len(set(workspaces)),2,workspaces)
        for workspace in workspaces:
            self.assertEqual(Path(workspace).parent,ART)
            self.assertTrue(list(Path(workspace).glob('binding-crate-*/bindings.o')))


class RbtreeNative(PrivateTestCase):
    @classmethod
    def setUpClass(cls):
        ART.mkdir(parents=True,exist_ok=True)

    def test_actual_binding_crate(self):
        # Regenerate from the original headers, actual parameters and target
        # preprocessing; no guessed/deleted struct layouts or typedef fixtures.
        natives=[NATIVE]
        if 'RBTREE_ARM64' in os.environ:natives.append(Path(os.environ['RBTREE_ARM64']))
        for native in natives:
            with self.subTest(native=native):
                work=ART/('binding-crate-'+native.name)
                generated=work/'rust/bindings/bindings_generated.rs'
                generated.parent.mkdir(parents=True,exist_ok=True)
                run(bindgen_command(native,ROOT/'rust/bindings/bindings_helper.h',generated),
                    'bindings-generate-'+native.name)
                data=generated.read_text()
                data=re.sub(r'pub const RUST_CONST_HELPER_([a-zA-Z0-9_]*)',r'pub const \1',data)
                generated.write_text(data)
                self.assertNotRegex(data,r'pub fn __(?:rb_insert_augmented|rb_erase_color)\(')
                self.assertIn('pub struct rb_node',data)
                self.assertIn('pub struct rb_augment_callbacks',data)
                shutil.copyfile(native/'rust/bindings/bindings_helpers_generated.rs',
                                work/'rust/bindings/bindings_helpers_generated.rs')
                saved=(native/'rust/.bindings.o.cmd').read_text().splitlines()[0]
                args=shlex.split(saved.split(' := ',1)[1]);env=dict(os.environ,RUSTC_BOOTSTRAP='1')
                while '=' in args[0] and not args[0].startswith('/'):
                    key,value=args.pop(0).split('=',1);env[key]=value
                env['OBJTREE']=str(work)
                flags=native_flags(args[1:],native,'rust')
                verify_flag_policy(flags,'rust')
                args=[args[0], *flags, '--out-dir=' + str(work),
                      '--emit=dep-info=' + str(work/'.bindings.o.d'),
                      '--emit=obj=' + str(work/'bindings.o'), '--emit=metadata=' + str(work/'libbindings.rmeta'),
                      str(ROOT/'rust/bindings/lib.rs')]
                run(args,'bindings-compile-'+native.name,env=env)
                self.assertTrue((work/'libbindings.rmeta').is_file())
                dep=(work/'.bindings.o.d').read_text()
                self.assertIn(str(ROOT/'rust/bindings/rbtree.rs'),dep)
                self.assertIn(str(generated),dep)

    def test_native_compile(self):
        native_compile(ART / 'native.o')

    def test_native_selection_gate(self):
        # Hard gate: failure cannot be waived by the negative-control test.
        native_compile(ART / 'selection-rust.o')
        c_compile(SOURCE / 'lib/rbtree.c', ART / 'selection-c.o')
        ids = []
        for stem in ('selection-c', 'selection-rust'):
            output = run(['llvm-objdump', '-d', ART / (stem + '.o')], stem + '-ids')
            ids.append(dict(re.findall(r'<__cfi_(\w+)>:[\s\S]*?movl\s+\$(0x[0-9a-f]+), %eax', output)))
        mismatch = {n: (ids[0][n], ids[1][n]) for n in EXPORTS if ids[0][n] != ids[1][n]}
        self.assertEqual(mismatch, {}, 'BLOCKED: native outer callback KCFI ABI must match C for every export')

    def test_full_c_differential(self):
        # Inputs are immutable authoritative sources, never mutable peer files.
        with retained_directory(prefix='rbtree-', dir=ART) as directory:
            work = Path(directory)
            oracle = work / 'oracle.c'
            oracle.write_text((SOURCE / 'lib/rbtree.c').read_text() + '\n' +
                              (FIXTURES / 'oracle_tail.c').read_text())
            c_compile(oracle, ART / 'oracle.o', ['-D' + n + '=oracle_' + n for n in EXPORTS])
            native_compile(ART / 'header_owner.o', source=FIXTURES / 'header_owner.rs')
            c_compile(FIXTURES / 'driver.c', ART / 'driver.o')
            run(['ld.lld', '--gc-sections', '-T', FIXTURES / 'discard.lds', '-o', ART / 'differential', '-e', '_start', ART / 'driver.o',
                 ART / 'oracle.o', ART / 'header_owner.o', atomic_helpers(), NATIVE / 'rust/core.o', NATIVE / 'rust/compiler_builtins.o'], 'differential-link')
            run([ART / 'differential'], 'differential-run')

    def test_kcfi_exports_dwarf(self):
        native_compile(ART / 'native.o', ['--emit=llvm-ir=' + str(ART / 'native.ll')])
        c_compile(SOURCE / 'lib/rbtree.c', ART / 'original.o')
        symbols = run(['nm', ART / 'native.o'], 'native-symbols')
        self.assertEqual(set(re.findall(r'__export_symbol_(\w+)', symbols)), set(EXPORTS))
        undefined=run(['nm', '-u', ART / 'native.o'], 'native-undefined')
        self.assertEqual(set(re.findall(r' U (\w+)',undefined)),
                         {'rust_helper_atomic_ptr_set','rust_helper_atomic_ptr_set_release',
                          'rust_helper_atomic_ptr_read','rust_helper_atomic_ptr_read_acquire'})
        for stem in ('native', 'original'):
            run(['llvm-objcopy', '--dump-section', '.export_symbol=' + str(ART / (stem + '-exports.bin')),
                 ART / (stem + '.o'), ART / (stem + '-export-copy.o')], stem + '-export-bytes')
        self.assertEqual((ART / 'native-exports.bin').read_bytes(), (ART / 'original-exports.bin').read_bytes())
        def export_offsets(stem):
            output = run(['nm', ART / (stem + '.o')], stem + '-export-offsets')
            return dict(re.findall(r'([0-9a-f]+) [a-zA-Z] __export_symbol_(\w+)', output))
        self.assertEqual(export_offsets('native'), export_offsets('original'))
        rust = run(['llvm-objdump', '-d', ART / 'native.o'], 'native-disassembly')
        c = run(['llvm-objdump', '-d', ART / 'original.o'], 'original-disassembly')
        def ids(disassembly):
            return dict(re.findall(r'<__cfi_(\w+)>:[\s\S]*?movl\s+\$(0x[0-9a-f]+), %eax', disassembly))
        ri, ci = ids(rust), ids(c)
        mismatch = {n for n in EXPORTS if ri[n] != ci[n]}
        self.assertEqual(mismatch, set())
        (ART / 'kcfi-gate.txt').write_text('\n'.join(f'{n}: C={ci[n]} Rust={ri[n]}' for n in EXPORTS) + '\nAll original export type IDs match.\n')
        for stem in ('native', 'original'):
            out = run([NATIVE / 'scripts/gendwarfksyms/gendwarfksyms', '-T', ART / (stem + '.symtypes'), ART / (stem + '.o')],
                      stem + '-versions', input='\n'.join(EXPORTS) + '\n')
            self.assertEqual(set(re.findall(r'#SYMVER (\w+) ', out)), set(EXPORTS))
        ir = (ART / 'native.ll').read_text()
        self.assertIn('@rust_helper_atomic_ptr_set_release', ir)
        self.assertIn('@rust_helper_atomic_ptr_set', ir)
        self.assertNotIn('store atomic ptr', ir)

    def test_nullable_outer_kcfi_c_callers(self):
        # NULL is valid whenever the original branch does not invoke rotation.
        native_compile(ART / 'nullable-provider.o')
        with retained_directory(prefix='rbtree-null-', dir=ART) as directory:
            source = Path(directory) / 'oracle.c'
            source.write_text((SOURCE / 'lib/rbtree.c').read_text())
            c_compile(source, ART / 'nullable-oracle.o', ['-D' + n + '=oracle_' + n for n in EXPORTS])
        for erase in (0, 1):
            for oracle in (0, 1):
                name = f'nullable-{erase}-{oracle}'
                c_compile(FIXTURES / 'nullable.c', ART / (name + '.o'),
                          [f'-DERASING={erase}', f'-DORACLE={oracle}'])
                run(['ld.lld', '--gc-sections', '-T', FIXTURES / 'discard.lds', '-e', '_start', '-o', ART / name,
                     ART / (name + '.o'), ART / 'nullable-provider.o', ART / 'nullable-oracle.o', atomic_helpers()], name + '-link')
                proc = execute(ART / name)
                (ART / (name + '-run.log')).write_text(f'exit={proc.returncode}\n')
                self.assertEqual(proc.returncode, 0)

    def test_arm64_publication(self):
        variable = 'RBTREE_ARM64'
        default = '/nonexistent-rbtree-arm64'
        if variable in os.environ and not os.environ[variable]:
            self.fail('RBTREE_ARM64 is explicitly empty')
        native = Path(os.environ.get(variable, default))
        if not (native / 'rust/libkernel.rmeta').is_file():
            if variable in os.environ:
                self.fail('explicit RBTREE_ARM64 lacks native kernel metadata')
            self.skipTest('optional native arm64 metadata missing')
        native_compile(ART / 'arm64.o', ['--emit=llvm-ir=' + str(ART / 'arm64.ll')], native=native)
        assembly = run(['llvm-objdump', '-d', atomic_helpers(native)], 'arm64-disassembly')
        self.assertIn('stlr', assembly)
        self.assertIn('rust_helper_atomic_ptr_set_release', (ART / 'arm64.ll').read_text())
        self.assertNotIn('store atomic ptr', (ART / 'arm64.ll').read_text())
        undefined=run(['nm', '-u', ART / 'arm64.o'], 'arm64-undefined')
        self.assertEqual(set(re.findall(r' U (\w+)',undefined)),
                         {'rust_helper_atomic_ptr_set','rust_helper_atomic_ptr_set_release',
                          'rust_helper_atomic_ptr_read','rust_helper_atomic_ptr_read_acquire'})

    def test_algorithm_negative_control(self):
        # Restore the old missing case-3 assignment in a disposable fixture.
        with retained_directory(prefix='rbtree-broken-', dir=ART) as directory:
            work = Path(directory)
            (work/'include/linux').mkdir(parents=True)
            for name in ('rbtree_header.rs','rbtree_types_header.rs','rbtree_augmented_header.rs'):
                shutil.copyfile(ROOT/'include/linux'/name,work/'include/linux'/name)
            (work / 'lib').mkdir()
            original = (ROOT / 'lib/rbtree.rs').read_text()
            broken, n = re.subn(r'tmp1\s*=\s*sibling;\s*sibling\s*=\s*tmp2', 'sibling = tmp2', original)
            self.assertEqual(n, 2)
            (work / 'lib/rbtree.rs').write_text(broken)
            (work / 'fixtures').mkdir()
            (work / 'fixtures/header_owner.rs').write_text((FIXTURES / 'header_owner.rs').read_text().replace('../../../lib/rbtree.rs', str(work / 'lib/rbtree.rs')))
            native_compile(ART / 'broken.o', source=work / 'fixtures/header_owner.rs')
            oracle = work / 'oracle.c'
            oracle.write_text((SOURCE / 'lib/rbtree.c').read_text() + '\n' + (FIXTURES / 'oracle_tail.c').read_text())
            c_compile(oracle, ART / 'negative-oracle.o', ['-D' + n + '=oracle_' + n for n in EXPORTS])
            c_compile(FIXTURES / 'driver.c', ART / 'negative-driver.o')
            run(['ld.lld', '--gc-sections', '-T', FIXTURES / 'discard.lds', '-e', '_start', '-o', ART / 'broken',
                 ART / 'negative-driver.o', ART / 'negative-oracle.o', ART / 'broken.o', atomic_helpers(), NATIVE / 'rust/core.o', NATIVE / 'rust/compiler_builtins.o'], 'broken-link')
            proc = execute(ART / 'broken')
            (ART / 'broken-run.log').write_text(f'exit={proc.returncode}\n')
            self.assertNotEqual(proc.returncode, 0)

    def test_genuine_i686(self):
        sysroot = SETTINGS['sysroot']
        if sysroot is None:
            self.skipTest('optional genuine i686 core is missing')
        core = target_rlib(sysroot, 'core')
        builtins = target_rlib(sysroot, 'compiler_builtins')
        bindgen = SETTINGS['bindgen']
        if not bindgen:
            if 'BINDGEN' in os.environ:self.fail('explicit BINDGEN is not executable')
            self.skipTest('optional bindgen executable missing')
        rustc = SETTINGS['rustc']
        if rustc is None:
            self.skipTest('optional HOSTRUSTC is unavailable')
        with retained_directory(prefix='rbtree-i686-', dir=ART) as directory:
            work = Path(directory)
            inc = work / 'include/linux'
            inc.mkdir(parents=True)
            # Only platform primitives are shimmed. All tree algorithms/types are
            # verbatim original C/header input, parsed for the genuine 32-bit ABI.
            primitives = '''#ifndef FIXTURE_PRIMITIVES
#define FIXTURE_PRIMITIVES
typedef _Bool bool;
typedef unsigned long __kernel_size_t;
#define true 1
#define false 0
#define NULL ((void *)0)
#define __always_inline inline __attribute__((always_inline))
#define unlikely(x) (x)
#define READ_ONCE(x) (*(volatile __typeof__(x) *)&(x))
#define WRITE_ONCE(x,v) (*(volatile __typeof__(x) *)&(x) = (v))
#define rcu_assign_pointer(x,v) __atomic_store_n(&(x),(v),__ATOMIC_RELEASE)
#define rcu_dereference_raw(x) __atomic_load_n(&(x),__ATOMIC_ACQUIRE)
#define container_of(p,t,m) ((t *)((char *)(p)-__builtin_offsetof(t,m)))
#define EXPORT_SYMBOL(x)
#define EXPORT_SYMBOL_GPL(x)
#endif
'''
            for name in ('compiler.h', 'container_of.h', 'stddef.h', 'rcupdate.h', 'export.h'):
                (inc / name).write_text(primitives)
            for name in ('rbtree.h', 'rbtree_types.h', 'rbtree_augmented.h'):
                shutil.copyfile(SOURCE / 'include/linux' / name, inc / name)
            wrapper = work / 'bindings.h'
            wrapper.write_text('#include <linux/rbtree_augmented.h>\n')
            bindings = ART / 'i686-bindings.rs'
            run([bindgen, wrapper, '--use-core', '--ctypes-prefix', 'crate::ffi', '--no-layout-tests',
                 '--allowlist-type', 'rb_.*', '--no-doc-comments', '--rust-target', '1.85', '-o', bindings,
                 '--', '-target', 'i686-unknown-linux-gnu', '-I' + str(work / 'include')], 'i686-bindgen')
            owner = work / 'owner.rs'
            owner.write_text('''#![no_std]
#![feature(cfi_encoding)]
extern crate self as kernel;
#[allow(unused_attributes)]
#[path="'''+str(SOURCE/'rust/ffi.rs')+'''"] pub mod ffi;
#[allow(non_camel_case_types)]
pub mod generated { include!("''' + str(bindings) + '''"); }
mod bindings_raw { pub(crate) use crate::generated::{rb_node,rb_root}; }
#[path="'''+str(ROOT/'rust/bindings/rbtree.rs')+'''"] mod corrected;
pub mod bindings {
 pub use crate::generated::*;
 pub use crate::corrected::{RbAugmentRotate,__rb_insert_augmented,__rb_erase_color};
}
pub mod sync { pub mod atomic { include!("'''+str(FIXTURES/'atomic_i686.rs')+'''"); } }
''' + (FIXTURES / 'header_owner.rs').read_text().replace('//! Native binding based fixture for translated inline algorithms.', '').replace('../../../lib/rbtree.rs', str(ROOT / 'lib/rbtree.rs')))
            # c_ulong is native unsigned long = usize on the genuine i686 target.
            source = owner.read_text().replace('pub mod ffi {', '#[allow(non_camel_case_types)]\npub mod ffi {')
            owner.write_text(source)
            env = dict(os.environ, RUSTC_BOOTSTRAP='1')
            run([rustc, '--edition=2021', '--target=i686-unknown-linux-gnu', '--sysroot', sysroot,
                 '-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '-Cpanic=abort', '-Copt-level=2', '--crate-type=rlib',
                 '--out-dir=' + str(work), '--emit=obj=' + str(ART / 'i686-rust.o'), owner], 'i686-rust', env=env)
            oracle = work / 'oracle.c'
            oracle.write_text((SOURCE / 'lib/rbtree.c').read_text() + '\n' + (FIXTURES / 'oracle_tail.c').read_text())
            common = ['clang', '-target', 'i686-unknown-linux-gnu', '-std=gnu11', '-O2', '-ffreestanding',
                      '-fno-pic', '-fno-stack-protector', '-I' + str(work / 'include'), '-Werror']
            run(common + ['-D' + n + '=oracle_' + n for n in EXPORTS] + ['-c', oracle, '-o', ART / 'i686-oracle.o'], 'i686-oracle')
            run(common + ['-c', FIXTURES / 'driver.c', '-o', ART / 'i686-driver.o'], 'i686-driver')
            # Preserve unchanged helper definitions; only platform primitives
            # substitute for a complete i686 kernel that is not available here.
            macros=(SOURCE/'rust/helpers/atomic_ext.c').read_text()
            macros=macros[macros.index('#define GEN_READ_HELPER'):macros.index('GEN_READ_SET_HELPERS(i8, s8)')]
            helpers=work/'atomic.c'
            helpers.write_text('#include <linux/compiler.h>\n#define __rust_helper\n'
                '#define smp_load_acquire(p) __atomic_load_n(p, __ATOMIC_ACQUIRE)\n'
                '#define smp_store_release(p,v) __atomic_store_n(p,v,__ATOMIC_RELEASE)\n'
                +macros+'\nGEN_READ_SET_HELPERS(ptr, const void *)\n')
            run(common+['-c',helpers,'-o',ART/'i686-atomic.o'],'i686-atomic')
            run(['ld.lld', '--gc-sections', '-T', FIXTURES/'discard.lds', '-m', 'elf_i386', '-e', '_start', '-o', ART / 'i686-differential',
                 ART / 'i686-driver.o', ART / 'i686-oracle.o', ART / 'i686-rust.o', ART/'i686-atomic.o',
                 core, builtins], 'i686-link')
            elf = run(['readelf', '-h', ART / 'i686-differential'], 'i686-elf')
            self.assertIn('ELF32', elf)
            # SIGSYS is a failed runtime gate, never an expected pass.
            run([ART / 'i686-differential'], 'i686-run')

    def test_actual_private_kbuild(self):
        with retained_directory(prefix='rbtree-kbuild-', dir=ART) as directory:
            work = Path(directory)
            (work / 'lib').mkdir()
            src = work / 'source'
            for name in ('lib/Makefile','lib/rbtree.rs','lib/rbtree_rust.rs',
                         'include/linux/rbtree_header.rs','include/linux/rbtree_augmented_header.rs',
                         'include/linux/rbtree_types_header.rs','include/linux/export_header.rs',
                         'rust/ffi_export.rs'):
                path=src/name;path.parent.mkdir(parents=True,exist_ok=True)
                shutil.copyfile(ROOT/name,path)
            shutil.copyfile(SOURCE/'lib/rbtree.c',src/'lib/rbtree.c')
            facade=binding_facade()
            saved = (NATIVE / 'lib/.list_sort_rust.o.cmd').read_text().splitlines()[0]
            args = shlex.split(saved.split(' := ', 1)[1])
            compiler = next(a for a in args if a.endswith('/rustc'))
            flags = args[args.index(compiler) + 1:]
            flags = flags[:next(i for i, a in enumerate(flags) if a.startswith('@'))]
            flags = native_flags(flags, NATIVE, 'rust')
            verify_flag_policy(flags, 'rust')
            require_rustc(compiler, work, compiler_environment(work, dict(os.environ)))
            makefile = work / 'Makefile'
            makefile.write_text(f'''srctree := {SOURCE}
srcroot := {src}
objtree := {NATIVE}
obj := lib
VPATH := {src}
SRCARCH := x86
ARCH := x86
CC := clang
NM := llvm-nm
AR := llvm-ar
RUSTC := {compiler}
RUSTC_OR_CLIPPY := {compiler}
KBUILD_RUSTFLAGS := {shlex.join(flags)}
RUSTFLAGS_KERNEL := --extern kernel={facade}
include {SOURCE}/scripts/Makefile.build
.PHONY: inspect
inspect: target-stem=rbtree_rust
inspect:
\t@echo LIBS=$(lib-y)
\t@echo KCOV_RUST=$(KCOV_INSTRUMENT_rbtree_rust.o)
\t@echo KCOV_C=$(KCOV_INSTRUMENT_rbtree.o)
\t@echo RUSTFLAGS=$(_rust_flags)
''')
            env = dict(os.environ, RUSTC_BOOTSTRAP='1')
            run(['make', '-f', makefile, 'CONFIG_RUST_RBTREE=y', 'lib/rbtree_rust.o', 'V=1'],
                'actual-kbuild', cwd=work, env=env)
            command = (work / 'lib/.rbtree_rust.o.cmd').read_text()
            (ART / 'actual-kbuild.cmd').write_text(command)
            shutil.copyfile(work / 'lib/rbtree_rust.o', ART / 'actual-kbuild.o')
            for token in ('rbtree.rs', 'rbtree_header.rs', 'rbtree_types_header.rs', 'rbtree_augmented_header.rs',
                          'libbindings.rmeta', 'CONFIG_RUST_RBTREE', '#SYMVER rb_erase '):
                # Kconfig selection is a Makefile input, not a Rust source token.
                if token == 'CONFIG_RUST_RBTREE':
                    continue
                self.assertIn(token, command)
            output = run(['make', '-f', makefile, 'CONFIG_RUST_RBTREE=y', 'inspect'], 'selector-rust', cwd=work, env=env)
            self.assertIn('lib/rbtree_rust.o', output)
            self.assertNotIn('lib/rbtree.o ', output)
            self.assertIn('KCOV_RUST=n', output)
            rust_order=re.search(r'^LIBS=(.*)$',output,re.M)[1].split()
            output = run(['make', '-f', makefile, 'CONFIG_RUST_RBTREE=n', 'inspect'], 'selector-c', cwd=work, env=env)
            self.assertIn('lib/rbtree.o', output)
            self.assertNotIn('lib/rbtree_rust.o', output)
            self.assertIn('KCOV_C=n', output)
            c_order=re.search(r'^LIBS=(.*)$',output,re.M)[1].split()
            self.assertEqual(rust_order,[p.replace('/rbtree.o','/rbtree_rust.o') for p in c_order])
            self.assertEqual(c_order.count('lib/rbtree.o'),1)
            coverage_args = ['make', '-f', makefile, 'CONFIG_RUST_RBTREE=y', 'CONFIG_KCOV=y',
                             'CONFIG_KCOV_INSTRUMENT_ALL=y', 'RUSTFLAGS_KCOV=KCOV_SENTINEL', 'inspect']
            output = run(coverage_args, 'kcov-exempt', cwd=work, env=env)
            self.assertNotIn('KCOV_SENTINEL', output)
            output = run(coverage_args + ['KCOV_INSTRUMENT_rbtree_rust.o=y'], 'kcov-negative-control', cwd=work, env=env)
            self.assertIn('KCOV_SENTINEL', output)
            before = (work / 'lib/rbtree_rust.o').stat().st_mtime_ns
            run(['make','-f',makefile,'CONFIG_RUST_RBTREE=y','lib/rbtree_rust.o','V=1'],
                'actual-kbuild-noop',cwd=work,env=env)
            self.assertEqual((work/'lib/rbtree_rust.o').stat().st_mtime_ns,before)
            os.utime(src / 'include/linux/rbtree_augmented_header.rs', None)
            run(['make', '-f', makefile, 'CONFIG_RUST_RBTREE=y', 'lib/rbtree_rust.o', 'V=1'],
                'actual-kbuild-header-rebuild', cwd=work, env=env)
            self.assertGreater((work / 'lib/rbtree_rust.o').stat().st_mtime_ns, before)
            # Keep actual Makefile.build/lib.a recipe; narrow only its transport
            # fixture input list, after separately proving full original order.
            with (src/'lib/Makefile').open('a') as stream:
                stream.write('\nlib-y := $(filter rbtree.o rbtree_rust.o,$(lib-y))\n')
            saved_c=(NATIVE/'lib/.scatterlist.o.cmd').read_text().splitlines()[0]
            ca=shlex.split(saved_c.split(' := ',1)[1])
            cflags=native_flags(ca[1:-1], NATIVE, 'c')
            verify_flag_policy(cflags, 'c')
            cflags=[a for a in cflags if a != '-c' and not a.startswith(('-DKBUILD_', '-D__KBUILD_'))]
            with makefile.open('a') as stream:
                stream.write('\nKBUILD_CFLAGS := '+shlex.join(cflags)+'\n')
            for lang in ('n','y','n'):
                name='rbtree_rust.o' if lang=='y' else 'rbtree.o'
                cmd=['make','-f',makefile,'CONFIG_RUST_RBTREE='+lang,'lib/lib.a','V=1']
                run(cmd,'archive-select-'+lang,cwd=work,env=env)
                members=run(['llvm-ar','t',work/'lib/lib.a'],'archive-members-'+lang,cwd=work)
                self.assertEqual([Path(p).name for p in members.splitlines()],[name])
                snap={p:(work/p).stat().st_mtime_ns for p in ('lib/lib.a','lib/'+name)}
                run(cmd,'archive-noop-'+lang,cwd=work,env=env)
                self.assertEqual(snap,{p:(work/p).stat().st_mtime_ns for p in snap})


if __name__ == '__main__':
    unittest.main()
