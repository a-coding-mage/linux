#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Discovery tests and private compiler replay, never a kernel build or VM.

MEMWEIGHT_KERNEL_SOURCE_ROOT defaults to this checkout. Optional NATIVE is an
os.pathsep list of read-only MAIN outputs; absent inputs skip native replay.
RUSTC defaults to rustc and must be >=1.85. LOGS preserves every command/object.
Every environment name above has the MEMWEIGHT_KERNEL_ prefix. Temporary
directories are external to the source and native trees. Failed logs survive.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
from contextlib import redirect_stderr, redirect_stdout
import ctypes
import importlib.util
import io
import os
from pathlib import Path
import re
import shlex
import shutil
import signal
import struct
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

PREFIX = 'MEMWEIGHT_KERNEL_'
HERE = Path(__file__).resolve().parent


class WriteWatch:
    """Observe even deleted compiler intermediates; final snapshots miss them."""
    def __init__(self, paths):
        libc = ctypes.CDLL(None, use_errno=True)
        self.fd = libc.inotify_init1(os.O_NONBLOCK | os.O_CLOEXEC)
        if self.fd < 0: raise OSError(ctypes.get_errno(), 'inotify_init1')
        self.paths = {}
        for path in paths:
            wd = libc.inotify_add_watch(self.fd, os.fsencode(path), 0x100 | 0x200 | 0x2 | 0x4 | 0x8 | 0x40 | 0x80)
            if wd < 0:
                os.close(self.fd)
                raise OSError(ctypes.get_errno(), 'inotify_add_watch', str(path))
            self.paths[wd] = str(path)

    def finish(self):
        events = []
        try:
            while True:
                try: data = os.read(self.fd, 1024 * 1024)
                except BlockingIOError: break
                offset = 0
                while offset < len(data):
                    wd, mask, cookie, length = struct.unpack_from('iIII', data, offset)
                    name = os.fsdecode(data[offset+16:offset+16+length].rstrip(b'\0'))
                    events.append((self.paths.get(wd, 'OVERFLOW'), hex(mask), name))
                    offset += 16 + length
        finally: os.close(self.fd)
        return events


def donor_inputs(flags, native, *, rust=True):
    # Rust responses have one argument per line; C uses shell quoting. Paths
    # remain relative to the compiler cwd, including nested response files.
    def expand(words, active=()):
        result = []
        for token in words:
            if token.startswith('@'):
                response = (native / token[1:]).resolve()
                if response in active: raise ValueError('recursive compiler response file')
                data = response.read_text()
                result.extend(expand(data.splitlines() if rust else shlex.split(data), (*active, response)))
            else: result.append(token)
        return result
    flags = expand(flags)
    def path(value): return str((native / value).resolve())
    def library(value):
        kind, sep, name = value.partition('=')
        return kind + sep + path(name) if sep else path(value)
    def target(value): return path(value) if value.endswith('.json') or '/' in value else value
    def external(value):
        name, sep, filename = value.partition('=')
        return name + sep + path(filename) if sep else value
    options = {'-I': path, '-L': library, '-include': path, '-isystem': path,
               '-iquote': path, '-idirafter': path, '-imacros': path, '-isysroot': path,
               '--sysroot': path, '--target': target, '--extern': external}
    result, index = [], 0
    while index < len(flags):
        token = flags[index]
        if token in options:
            if index + 1 == len(flags): raise ValueError('missing donor path argument: ' + token)
            result += [token, options[token](flags[index + 1])]; index += 2; continue
        for option, convert in options.items():
            if token.startswith(option + '='):
                token = option + '=' + convert(token[len(option) + 1:]); break
            if option in ('-I', '-L', '-include', '-isystem', '-iquote', '-idirafter', '-imacros') and token.startswith(option):
                token = option + convert(token[len(option):]); break
        result.append(token); index += 1
    return result


def inputs(environ=None):
    env = os.environ if environ is None else environ
    def value(name, default=None):
        raw = env.get(PREFIX + name, default)
        if raw is not None and not raw.strip(): raise ValueError('empty explicit ' + name)
        return raw
    root = Path(value('SOURCE_ROOT', str(HERE.parents[1]))).resolve()
    for name in ('lib/memweight.c', 'lib/memweight.rs', 'scripts/tests/check_hweight_kernel.py'):
        if not (root / name).is_file(): raise ValueError('invalid source root: ' + name)
    rustc = shutil.which(value('RUSTC', 'rustc'))
    if rustc is None: raise ValueError('invalid rustc executable')
    rustc = os.path.abspath(rustc)
    version = subprocess.run([rustc, '--version'], check=True, capture_output=True, text=True).stdout
    match = re.match(r'rustc (\d+)\.(\d+)\.', version)
    if not match or tuple(map(int, match.groups())) < (1, 85): raise ValueError('rustc >=1.85 required')
    raw = value('NATIVE')
    native = []
    for item in ([] if raw is None else raw.split(os.pathsep)):
        if not item.strip(): raise ValueError('empty explicit native entry')
        path = Path(item).resolve()
        if 'modular' in str(path).lower(): raise ValueError('MODULAR outputs forbidden')
        if not list((path/'lib').glob('.*_rust.o.cmd')): raise ValueError('no native Rust donor')
        for name in ('.config', 'lib/.scatterlist.o.cmd',
                     'rust/libkernel.rmeta', 'scripts/gendwarfksyms/gendwarfksyms'):
            if not (path / name).is_file(): raise ValueError('invalid native input: ' + name)
        native.append(path)
    raw = value('LOGS')
    logs = Path(raw).resolve() if raw is not None else None
    protected = [root, *native]
    if logs is not None:
        if any(logs.is_relative_to(path) for path in protected): raise ValueError('logs inside read-only input')
        if any(p.exists() and not p.is_dir() for p in (logs, *logs.parents)):
            raise ValueError('invalid logs directory')
    parents = [Path(env[key]) for key in ('TMPDIR', 'TEMP', 'TMP') if env.get(key)] + [Path('/tmp')]
    parent = next((p.resolve() for p in parents if p.is_dir() and not any(p.resolve().is_relative_to(x) for x in protected)), None)
    if parent is None: raise ValueError('no external temporary directory')
    return argparse.Namespace(root=root, rustc=rustc, native=native, logs=logs, parent=parent)


def setUpModule():
    global SETTINGS, check, EVIDENCE
    SETTINGS = inputs()  # All explicit inputs validated before writes or skips.
    sys.path.insert(0, str(SETTINGS.root / 'scripts/tests'))
    spec = importlib.util.spec_from_file_location('memweight_private_checker', HERE / 'check_memweight_kernel.py')
    check = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(check)
    check.ROOT = SETTINGS.root
    if SETTINGS.logs is not None: SETTINGS.logs.mkdir(parents=True, exist_ok=True)
    EVIDENCE = Path(tempfile.mkdtemp(prefix='memweight-kernel-evidence-', dir=SETTINGS.logs or SETTINGS.parent))
    (EVIDENCE / 'inputs.txt').write_text(repr(vars(SETTINGS)) + '\n')
    print('memweight kernel checker evidence:', EVIDENCE, flush=True)


class CheckerTests(unittest.TestCase):
    def test_donor_input_routing(self):
        directory = EVIDENCE/'donor-inputs'; directory.mkdir()
        (directory/'c.rsp').write_text('-I "space include" -include forced.h @nested.rsp -iquote')
        (directory/'nested.rsp').write_text('-imacros macros.h -isystemsystem')
        self.assertEqual(donor_inputs(['@c.rsp', 'quoted'], directory, rust=False),
            ['-I', str(directory/'space include'), '-include', str(directory/'forced.h'),
             '-imacros', str(directory/'macros.h'), '-isystem'+str(directory/'system'), '-iquote', str(directory/'quoted')])
        (directory/'rust.rsp').write_text('--target\ntarget.json\n-Ldependency=rust\n--extern\n')
        self.assertEqual(donor_inputs(['@rust.rsp', 'kernel=rust/libkernel.rmeta', '--target=aarch64-unknown-none',
                                       '--extern=core', '--extern=alloc=rust/liballoc.rmeta'], directory),
            ['--target', str(directory/'target.json'), '-Ldependency='+str(directory/'rust'),
             '--extern', 'kernel='+str(directory/'rust/libkernel.rmeta'), '--target=aarch64-unknown-none',
             '--extern=core', '--extern=alloc='+str(directory/'rust/liballoc.rmeta')])
        (directory/'cycle.rsp').write_text('@cycle.rsp')
        for rust in (False, True):
            with self.assertRaisesRegex(ValueError, 'recursive'): donor_inputs(['@cycle.rsp'], directory, rust=rust)
            with self.assertRaisesRegex(ValueError, 'missing'): donor_inputs(['--extern'], directory, rust=rust)
        compiler = directory/'rustc with spaces'; compiler.symlink_to(SETTINGS.rustc)
        options = inputs({PREFIX+'SOURCE_ROOT': str(SETTINGS.root), PREFIX+'RUSTC': os.path.relpath(compiler)})
        self.assertEqual(options.rustc, str(compiler))

    def test_explicit_empty_and_invalid_inputs_before_writes(self):
        base = {PREFIX+'SOURCE_ROOT': str(SETTINGS.root), PREFIX+'RUSTC': SETTINGS.rustc}
        for name in ('SOURCE_ROOT', 'RUSTC', 'NATIVE', 'LOGS'):
            for value in ('', ' ', '/missing/memweight-runtime-input'):
                with self.subTest(name=name, value=value):
                    if name == 'LOGS' and value.startswith('/missing'): continue
                    with mock.patch.object(tempfile, 'mkdtemp', side_effect=AssertionError('write before validation')):
                        with self.assertRaises((ValueError, OSError)):
                            inputs({**base, PREFIX+name: value})
        with self.assertRaises(argparse.ArgumentTypeError): check.nonempty_path('')

    def test_selection(self):
        config = {'X86_64': 'y', '64BIT': 'y', 'RUST': 'y', 'RUST_MEMWEIGHT': 'y'}
        self.assertEqual(check.provider(config), 'Rust')
        self.assertEqual(check.provider({**config, 'RUST_MEMWEIGHT': 'n'}), 'C')
        for bad in ({**config, 'RUST': 'n'}, {**config, 'RUST_MEMWEIGHT': 'm'}):
            with self.assertRaises(ValueError): check.provider(bad)

    def test_runtime_option_rejections_before_work(self):
        with tempfile.TemporaryDirectory(dir=SETTINGS.parent, prefix='memweight-options-') as td:
            build = Path(td)
            (build/'.config').write_text('CONFIG_64BIT=y\n')
            values = dict(build=build, qemu=None, qemu_data=None, make_arg=[])
            check.validate_options(argparse.Namespace(**values))
            for changed in ({'build': build/'absent'}, {'qemu': ''}, {'qemu': ' '}, {'qemu': "''"}, {'qemu': "'"},
                            {'qemu': '/missing/memweight-qemu'}, {'qemu': str(build)},
                            {'qemu_data': build/'absent'}, {'make_arg': ['']}):
                with self.assertRaises(ValueError): check.validate_options(argparse.Namespace(**{**values, **changed}))
            for value in ('', ' ', "''", "'", '/missing/memweight-make', str(build)):
                with mock.patch.dict(os.environ, {'MAKE': value}):
                    with self.assertRaises(ValueError): check.validate_options(argparse.Namespace(**values))
            valid = argparse.Namespace(**{**values, 'qemu': shlex.join([sys.executable, '--version'])})
            check.validate_options(valid)
            self.assertEqual(shlex.split(valid.qemu), [os.path.abspath(sys.executable), '--version'])
            # Exercise the CLI ordering: neither artifact checks nor any fixture
            # creation/build can be reached by invalid explicit commands.
            for variable in ('QEMU', 'MAKE'):
                for value in ('', ' ', "''", "'", '/missing/memweight-command'):
                    with mock.patch.dict(os.environ, {variable: value}), \
                         mock.patch.object(sys, 'argv', ['check', str(build)]), \
                         mock.patch.object(check, 'verify_linked_implementation', side_effect=AssertionError('audit before validation')), \
                         mock.patch.object(check.tempfile, 'mkdtemp', side_effect=AssertionError('fixture before validation')), \
                         mock.patch.object(check.subprocess, 'run', side_effect=AssertionError('launch before validation')), \
                         redirect_stderr(io.StringIO()):
                        with self.assertRaises(SystemExit) as error: check.main()
                        self.assertEqual(error.exception.code, 2)

    def test_quoted_runtime_commands_and_forwarding(self):
        directory = EVIDENCE/'quoted-commands'; directory.mkdir()
        executable = directory/"tool with ' quotes"; executable.symlink_to(sys.executable)
        qemu = [str(executable), '-bios', str(directory/'BIOS with spaces.bin'), '-vga', 'none', '', '$literal']
        make = [str(executable), '--jobs=2', 'argument with spaces']
        self.assertEqual(check.executable_command(shlex.join(qemu), 'QEMU'), qemu)
        with self.assertRaisesRegex(ValueError, 'NUL'): check.executable_command(shlex.join([str(executable), 'nul\0arg']), 'MAKE')
        build = directory/'build'; build.mkdir(); (build/'.config').touch()
        data = directory/'qemu data'; data.mkdir()
        config = dict(X86_64='y', MODULES='y', PRINTK='y', MULTIUSER='y', CFI='y', MODVERSIONS='y', GENDWARFKSYMS='y', **{'64BIT': 'y'})
        boot = build/'rust-boot-test'; boot.mkdir()
        (boot/'console.log').write_bytes(check.result_marker('c')+b'\nLUPOS_RUST_MODULE_LOAD_OK\nLUPOS_RUST_BUILD_BOOT_OK\n')
        argv = ['check', str(build), '--allow-c-baseline', '--qemu', shlex.join(qemu), '--qemu-data', os.path.relpath(data)]
        with mock.patch.dict(os.environ, {'MAKE': shlex.join(make)}), mock.patch.object(sys, 'argv', argv), \
             mock.patch.object(check, 'configuration', return_value=config), \
             mock.patch.object(check, 'verify_linked_implementation', return_value=build/'lib/memweight.o'), \
             mock.patch.object(check, 'verify_consumer'), mock.patch.object(check, 'run_logged') as run, redirect_stdout(io.StringIO()):
            check.main()
        self.assertEqual(run.call_count, 2)
        self.assertEqual(run.call_args_list[0].args[0][:len(make)], make)
        command = run.call_args_list[1].args[0]
        self.assertEqual(shlex.split(command[command.index('--qemu') + 1]), qemu)
        self.assertEqual(command[command.index('--qemu-data') + 1], str(data))

    def test_launch_errors_are_retained(self):
        directory = EVIDENCE/'launch-errors'; directory.mkdir()
        log = directory/'missing.log'
        with self.assertRaisesRegex(RuntimeError, 'preserved log'):
            check.run_logged(['/missing/memweight-command'], log, os.environ)
        self.assertIn('launch-error=', log.read_text())
        self.assertIn('/missing/memweight-command', log.read_text())
        with self.assertRaises(OSError): self.run_logged(['/missing/memweight-command'], directory)
        self.assertTrue(any('launch-error=' in p.read_text() for p in directory.glob('command-*/command.log')))

    def test_strict_lint_order_and_real_warning(self):
        directory = EVIDENCE/'lint-controls'; directory.mkdir()
        source = directory/'warning.rs'; source.write_text('fn main() { let unused = 0; }\n')
        strict = [('-Dwarnings',), ('-D', 'warnings'), ('--deny=warnings',), ('--deny', 'warnings'),
                  ('-Fwarnings',), ('--forbid', 'warnings'), ('-Wwarnings', '-Dwarnings'),
                  ('-Awarnings', '--deny=warnings'), ('-Dwarnings', '--forbid=warnings'),
                  ('-Dwarnings', '--cap-lints=deny'), ('-Dwarnings', '--cap-lints', 'forbid'),
                  ('-Dwarnings', '--cap-lints=deny', '--cap-lints=warn'),
                  ('-Fwarnings', '-Dwarnings'), ('-Fwarnings', '-Wwarnings'),
                  ('--forbid=warnings', '--allow', 'warnings'), ('--forbid=warnings', '--warn=warnings', '--cap-lints=deny')]
        weak = [('-Dwarnings', suffix) for suffix in ('-Wwarnings', '--warn=warnings', '-Awarnings', '--allow=warnings')]
        weak += [('-Dwarnings', flag, 'warnings') for flag in ('-W', '--warn', '-A', '--allow')]
        weak += [('-Dwarnings', '--cap-lints=warn'), ('-Dwarnings', '--cap-lints', 'allow'),
                 ('-Dwarnings', '--force-warn', 'unused_variables'),
                 ('-Dwarnings', '--cap-lints=warn', '--cap-lints=deny'), ('-Fwarnings', '--cap-lints=warn'),
                 ('-Dwarnings,unused_variables',), ('-Dwarnings,dead_code',)]
        for index, flags in enumerate(strict + weak):
            with self.subTest(flags=flags):
                command = [SETTINGS.rustc, *flags, str(source), '-o', str(directory/('out-'+str(index)))]
                if flags in strict: check.verify_strict_lints(command)
                else:
                    with self.assertRaises(ValueError): check.verify_strict_lints(command)
                result = self.run_logged(command, directory, failure=flags in strict)
                if flags in strict: self.assertIn('error: unused variable', result.stdout)
                # Exercise the defining-object validator too, including response
                # files containing otherwise hidden warning overrides.
                obj = directory/('out-'+str(index)+'.o')
                obj.with_name('.'+obj.name+'.cmd').write_text('savedcmd_'+obj.name+' := '+shlex.join(command)+'\n')
                if flags in strict: check.verify_rust_compiler(obj)
                else:
                    with self.assertRaises(ValueError): check.verify_rust_compiler(obj)
        response = directory/'flags'; response.write_text('--warn=warnings\n')
        with mock.patch.object(check, 'compilation_flags', return_value=[SETTINGS.rustc, '-Dwarnings', '@flags']):
            with self.assertRaises(ValueError): check.verify_rust_compiler(directory/'out.o')
        for flags in (['-Dwarnings', '--force-warn=warnings'], ['-Dwarnings', '-A=warnings'],
                      ['-Dwarnings', '--cap-lints'], ['-Dwarnings', '--warn']):
            with self.assertRaises(ValueError): check.verify_strict_lints(flags)
            self.run_logged([SETTINGS.rustc, str(source), '-o', str(directory/'invalid'), *flags], directory, failure=True)

    def test_transient_and_concurrent_compiler_isolation(self):
        directory = EVIDENCE/'isolation'; directory.mkdir()
        trap = directory/'inherited-temp'; trap.mkdir()
        watch = WriteWatch([trap])
        source = directory/'same.rs'
        source.write_text('pub fn value() -> u32 { 17 }\n')
        def compile_one(index):
            output = directory/str(index); output.mkdir()
            env = dict(os.environ, TMPDIR=str(trap), TEMP=str(trap), TMP=str(trap))
            self.run_logged([SETTINGS.rustc, '-Dwarnings', '--crate-type=rlib',
                '--crate-name=same', '--emit=obj', str(source), '--out-dir', str(output)], output, env=env)
            self.assertTrue((output/'same.o').is_file())
        run = subprocess.run
        def observed_run(command, **kwargs):
            cwd = Path(kwargs['cwd']).resolve()
            self.assertTrue(cwd.is_relative_to(directory))
            self.assertNotEqual(cwd, directory)
            for key in ('TMPDIR', 'TEMP', 'TMP'): self.assertEqual(kwargs['env'][key], str(cwd))
            return run(command, **kwargs)
        try:
            with mock.patch.object(subprocess, 'run', side_effect=observed_run):
                compile_one(0)
                with ThreadPoolExecutor(max_workers=2) as pool: list(pool.map(compile_one, (1, 2)))
        finally: events = watch.finish()
        (directory/'inherited-temp-events.txt').write_text(repr(events)+'\n')
        self.assertEqual(events, [])
        event_files = list(directory.glob('*/command-*/events.txt'))
        self.assertEqual(len(event_files), 3)
        self.assertTrue(all('rmeta' in p.read_text() for p in event_files))

    def test_workloads_and_count(self):
        self.assertEqual(check.COUNT, 1 + 5*8*(257+8) + 2048 + 2)
        for caller in ('c', 'rust'):
            source = check.sources(caller)
            self.assertNotIn('@COUNT@', source)
            self.assertIn('134217735', source)
            self.assertIn('65535', source)
            self.assertIn('kvfree', source)
        self.assertIn('bindings::__kvmalloc_node_noprof', check.sources('rust'))
        self.assertNotIn('memweight_exercise(', check.sources('rust'))
        with self.assertRaises(ValueError): check.sources('')

    def test_console_protocol_and_fault_negatives(self):
        for caller in ('c', 'rust'):
            for reload in (False, True):
                lines = [check.result_marker(caller), b'LUPOS_RUST_MODULE_LOAD_OK']
                if reload: lines += [b'LUPOS_RUST_MODULE_UNLOAD_OK 0', check.result_marker(caller), b'LUPOS_RUST_MODULE_RELOAD_OK 0']
                lines += [b'LUPOS_RUST_BUILD_BOOT_OK']
                console = b'\n'.join(lines) + b'\n'
                self.assertEqual(check.verify_console(console, caller, reload=reload), 2 if reload else 1)
                bad = [b'', console.replace(b'12651', b'12650'), console + lines[0],
                       b'\n'.join(reversed(lines)), console.replace(b'LOAD_OK', b'LOAD_FAILED')]
                bad += [console+b'\n'+fault for fault in (b'BUG:', b'WARNING:', b'CFI failure', b'Kernel panic', b'Oops:', b'no symbol version', b'not ok 1')]
                for invalid in bad:
                    with self.assertRaises(ValueError): check.verify_console(invalid, caller, reload=reload)

    def test_symvers_metadata_negatives(self):
        with tempfile.TemporaryDirectory(dir=SETTINGS.parent, prefix='memweight-unit-') as td:
            path = Path(td) / 'Module.symvers'
            valid = '0x12345678\tmemweight\tvmlinux\tEXPORT_SYMBOL\n'
            path.write_text(valid)
            self.assertEqual(check.selected_versions(Path(td)), {b'memweight': b'0x12345678'})
            for invalid in ('', valid*2, valid.replace('vmlinux', 'lib/memweight'), valid.replace('EXPORT_SYMBOL', 'EXPORT_SYMBOL_GPL'), valid.rstrip()+' namespace\n', valid.replace('12345678', 'xyz')):
                path.write_text(invalid)
                with self.assertRaises(ValueError): check.selected_versions(Path(td))

    def test_source_and_freshness_negative(self):
        with tempfile.TemporaryDirectory(dir=SETTINGS.parent, prefix='memweight-unit-') as td:
            work = Path(td)
            (work/'memweight_rust_main.rs').write_text(check.sources('rust')+'// changed\n')
            cfg = {'X86_64': 'y', '64BIT': 'y', 'RUST': 'y', 'RUST_MEMWEIGHT': 'y'}
            with mock.patch.object(check, 'configuration', return_value=cfg):
                with self.assertRaisesRegex(ValueError, 'caller source changed'):
                    check.verify_consumer(work, work, 'rust')
            src, obj = work/'in', work/'out'
            src.write_text('new'); obj.write_text('old')
            os.utime(obj, ns=(1, 1))
            with self.assertRaises(ValueError): check.newer(obj, [src])

    def run_logged(self, command, directory, *, env=None, data=None, failure=False):
        scratch = Path(tempfile.mkdtemp(prefix='command-', dir=directory))
        log = scratch/'command.log'
        env = dict(os.environ if env is None else env,
                   TMPDIR=str(scratch), TEMP=str(scratch), TMP=str(scratch))
        log.write_text(shlex.join(map(str, command))+'\ncwd='+str(scratch)+'\nTMPDIR=TEMP=TMP='+str(scratch)+'\n')
        watch = WriteWatch([scratch, directory])
        try:
            result = subprocess.run(command, cwd=scratch, env=env, input=data, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=180)
        except (OSError, subprocess.TimeoutExpired) as error:
            with log.open('a') as output: output.write('launch-error='+repr(error)+'\n')
            raise
        finally: (scratch/'events.txt').write_text(repr(watch.finish())+'\n')
        with log.open('a') as output: output.write(result.stdout+'\nexit='+str(result.returncode)+'\n')
        self.assertNotIn(result.returncode, (-signal.SIGSYS, 128+signal.SIGSYS), 'SIGSYS is never an expected negative: '+str(log))
        if failure: self.assertNotEqual(result.returncode, 0, str(log))
        else: self.assertEqual(result.returncode, 0, str(log))
        return result

    def test_threshold_contract_and_compiled_negative(self):
        directory = EVIDENCE / 'threshold'; directory.mkdir()
        original = (SETTINGS.root/'lib/memweight.c').read_text()
        self.assertIn('BUG_ON(longs >= INT_MAX / BITS_PER_LONG);', original)
        self.assertLess(original.index('BUG_ON('), original.index('ret += bitmap_weight('))
        source = '''#[path = "@PATH@"] mod implementation;
fn main() {
    let word = core::mem::size_of::<core::ffi::c_ulong>();
    let limit = (i32::MAX as usize / (word * 8)) * word;
    let p = core::ptr::null();
    assert_eq!(unsafe { implementation::memweight_with(p, 0, |_,_| panic!("zero callback")) }, Some(0));
    for n in [limit, limit+word, usize::MAX] {
        assert_eq!(unsafe { implementation::memweight_with(p, n, |_,_| panic!("threshold callback before BUG")) }, None);
    }
    // Returning u32::MAX checks zero extension independently of VM allocation.
    let n = limit-word;
    let allocation = vec![0u8; n];
    assert_eq!(unsafe { implementation::memweight_with(allocation.as_ptr(), n, |_,bits| {
        assert_eq!(bits as usize, n*8); u32::MAX
    }) }, Some(u32::MAX as usize));
    for offset in 1..word {
        let bytes = vec![255u8; word*2];
        let prefix = word-offset;
        assert_eq!(unsafe { implementation::memweight_with(bytes.as_ptr().add(offset), limit+prefix, |_,_| panic!("BUG callback")) }, None);
    }
}
'''
        core = (SETTINGS.root/'lib/memweight.rs').read_text()
        for negative in (False, True):
            stem = 'negative' if negative else 'positive'
            implementation = directory/(stem+'_core.rs')
            implementation.write_text(core.replace('longs >=', 'longs >') if negative else core)
            main = directory/(stem+'.rs')
            main.write_text(source.replace('@PATH@', str(implementation)))
            exe = directory/stem
            self.run_logged([SETTINGS.rustc, '--edition=2021', '-Dwarnings', str(main), '-o', str(exe)], directory)
            self.run_logged([str(exe)], directory, failure=negative)

    def test_genuine_native_objects_and_compiled_negatives(self):
        if not SETTINGS.native: self.skipTest('optional read-only MAIN input absent; no native artifact claim')
        for native in SETTINGS.native:
            with self.subTest(native=native):
                watch = WriteWatch([native, native/'lib', native/'rust'])
                try: self.native_replay(native)
                finally:
                    events = watch.finish()
                    (EVIDENCE/(native.name+'-write-events.txt')).write_text(repr(events)+'\n')
                self.assertEqual(events, [], 'transient writes in read-only MAIN donor')

    def native_replay(self, native):
        directory = EVIDENCE/native.name; directory.mkdir()
        arch = check.architecture(check.configuration(native))
        env = dict(os.environ, RUSTC_BOOTSTRAP='1')
        donors = sorted((native/'lib').glob('.*_rust.o.cmd'))
        donor = next((p for p in donors if p.name == '.bcd_rust.o.cmd'), donors[0])
        shutil.copyfile(donor, directory/'rust-donor.cmd')
        rust = shlex.split(donor.read_text().splitlines()[0].split(' := ', 1)[1])
        while '=' in rust[0] and not rust[0].startswith('/'):
            key, value = rust.pop(0).split('=', 1); env[key] = value
        self.assertEqual(Path(rust[0]).resolve(), Path(SETTINGS.rustc).resolve())
        self.assertIn('-Dwarnings', rust)
        self.assertIn('-Zsanitizer=kcfi', rust)
        cdonor = native/'lib/.scatterlist.o.cmd'
        shutil.copyfile(cdonor, directory/'c-donor.cmd')
        cflags = shlex.split(cdonor.read_text().splitlines()[0].split(' := ', 1)[1])
        cflags = cflags[:cflags.index(';')] if ';' in cflags else cflags
        self.assertIn('-fsanitize=kcfi', cflags)

        def compile_source(stem, source, language):
            path = directory/(stem+('.rs' if language == 'rust' else '.c'))
            path.write_text(source)
            obj = directory/(stem+'.o')
            flags = donor_inputs(list(rust if language == 'rust' else cflags), native, rust=language == 'rust')
            executable = str(native/flags[0]) if '/' in flags[0] else flags[0]
            executable = shutil.which(executable)
            self.assertIsNotNone(executable, 'native compiler executable missing')
            flags[0] = os.path.abspath(executable)
            for i, token in enumerate(flags):
                if token in ('--out-dir', '-o'): flags[i+1] = str(directory if token == '--out-dir' else obj)
                elif token.startswith('--emit=dep-info='): flags[i] = '--emit=dep-info='+str(directory/(stem+'.d'))
                elif token.startswith('--emit=obj='): flags[i] = '--emit=obj='+str(obj)
                elif token.startswith('-Wp,-MMD,'): flags[i] = '-Wp,-MMD,'+str(directory/(stem+'.d'))
                elif token.endswith('.rs') or token.endswith('/lib/scatterlist.c'): flags[i] = str(path)
                elif token.startswith('-DKBUILD_BASENAME='): flags[i] = '-DKBUILD_BASENAME="'+stem+'"'
                elif token.startswith('-DKBUILD_MODNAME='): flags[i] = '-DKBUILD_MODNAME="'+stem+'"'
                elif token.startswith('-D__KBUILD_MODNAME='): flags[i] = '-D__KBUILD_MODNAME=kmod_'+stem
            if language == 'c': flags += ['-DMODULE', '-D__DISABLE_EXPORTS', '-Werror']
            else: check.verify_strict_lints(flags)
            self.run_logged(flags, directory, env=env)
            check.elf_target(obj, arch)
            return obj

        reference = compile_source('memweight_reference', check.reference_source(), 'c')
        types = check.provider_type_ids(reference, names=('memweight_reference',))
        expected = {'memweight': types['memweight_reference']}
        crc, row = check.dwarf_proof(native, reference, 'memweight_reference')
        (directory/'reference-proof.txt').write_text(crc.decode()+'\n'+row+'\n')
        self.assertFalse(check.read_exports(reference))
        # Genuine current provider, with its original dependency files and MAIN
        # flags/rmeta. This is a PRIVATE object, never a final-kernel claim.
        for name in ('lib/memweight.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs'):
            target = directory/name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(SETTINGS.root/name, target)
        provider = compile_source('lib/memweight_rust', (SETTINGS.root/'lib/memweight_rust.rs').read_text(), 'rust')
        self.assertEqual(check.provider_type_ids(provider, names=check.EXPORTS), expected)
        check.verify_references(provider, ('__bitmap_weight',))
        records = check.read_exports(provider)
        self.assertEqual(len(records), 1)
        self.assertEqual((records[0]['name'], records[0]['license'], records[0]['namespace']), ('memweight', '', ''))
        rust_crc, rust_row = check.dwarf_proof(native, provider, 'memweight')
        (directory/'provider-proof.txt').write_text(rust_crc.decode()+'\n'+rust_row+'\n'
            'Private replay only; Rust/C nominal DWARF CRCs may differ. Final modules must import the selected actual CRC.\n')
        actual_versions = check.selected_versions(native)
        selected_crc = rust_crc if check.provider(check.configuration(native)) == 'Rust' else crc
        self.assertEqual(actual_versions, {b'memweight': selected_crc})
        for caller in ('c', 'rust'):
            obj = compile_source('memweight_'+caller+'_main', check.sources(caller), caller)
            check.verify_references(obj, check.EXPORTS)
            check.verify_guarded_calls(obj, arch, expected, wrappers=check.WRAPPERS)
            info = check.metadata_fields(obj)
            check.require_metadata_field(info, b'license', b'Proprietary')
            check.require_metadata_field(info, b'description', check.DESCRIPTION)
            if caller == 'rust':
                check.verify_rust_workload(obj)
                check.verify_rust_entrypoints(obj, arch)
            corrupted = directory/(caller+'-bad-license.o')
            corrupted.write_bytes(obj.read_bytes().replace(b'license=Proprietary\0', b'license=XXXXXXXXXXX\0'))
            with self.assertRaises(ValueError):
                check.require_metadata_field(check.metadata_fields(corrupted), b'license', b'Proprietary')
            with self.assertRaises(ValueError):
                check.verify_guarded_calls(obj, arch, {'memweight': expected['memweight'] ^ 1}, wrappers=check.WRAPPERS)
        # Compile a real source mutation which destroys the protected call.
        bad = check.sources('rust').replace('core::ptr::read_volatile(&pointer)(ptr, bytes)', 'pointer(ptr, bytes).wrapping_add(1)')
        obj = compile_source('memweight_corrupt', bad, 'rust')
        with self.assertRaises(ValueError): check.verify_guarded_calls(obj, arch, expected, wrappers=check.WRAPPERS)
        # A compiled C dispatcher dependency is forbidden even with valid exports.
        bad = check.sources('rust').replace('let mut count = 0u32;', 'unsafe extern "C" { fn memweight_c_dispatch(); }\n    unsafe { memweight_c_dispatch() };\n    let mut count = 0u32;')
        obj = compile_source('memweight_delegated', bad, 'rust')
        with self.assertRaises(ValueError): check.verify_rust_workload(obj)
        # Native signature corruption must change the real KCFI identity.
        wrong = check.reference_source().replace('size_t memweight_reference(const void *, size_t);', '')
        wrong = wrong.replace('#include "'+str(SETTINGS.root/'lib/memweight.c')+'"',
            'unsigned int memweight_reference(const void *p, size_t n);\nunsigned int memweight_reference(const void *p, size_t n) { return n + (p != 0); }\n')
        obj = compile_source('memweight_wrong_signature', wrong, 'c')
        self.assertNotEqual(check.provider_type_ids(obj, names=('memweight_reference',)), types)
        with self.assertRaises(ValueError): check.dwarf_proof(native, obj, 'memweight_reference')


if __name__ == '__main__': unittest.main()
