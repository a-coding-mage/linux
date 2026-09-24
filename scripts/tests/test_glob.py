#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Original-C differential and native object evidence, with private fixtures.

Run from any directory. --source is the retained C tree; --root is the candidate.
Native inputs are optional, but explicitly supplied invalid inputs fail.
Discovery: python3 -m unittest discover -s scripts/tests -p test_glob.py -v
Inputs: GLOB_ROOT, GLOB_SOURCE, GLOB_RUSTC, GLOB_NATIVE (os.pathsep-separated),
GLOB_I686_SYSROOT and GLOB_LOGS (an existing output directory). CLI options take
precedence. ROOT/--root are read-only source lookup roots. All fixtures and default
logs use cleaned system TemporaryDirectories. --logs/GLOB_LOGS preserve uniquely
named run directories including failed fixtures; legacy --log/GLOB_LOG preserves
uniquely named log files beside its requested filename and private fixture dirs.
Neither option overwrites previous evidence. Native compilers never run in donors.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
from contextlib import ExitStack, contextmanager, redirect_stderr
import ctypes
import io
import itertools
import json
import os
from pathlib import Path
import random
import re
import shlex
import shutil
import subprocess
import tempfile
import sys
import unittest
from unittest import mock

import glob_runtime_fixtures as transport
from check_glob_kernel import verify_flag_policy

ROOT = Path(__file__).resolve().parents[2]


def explicit_path(value):
    if not value.strip():
        raise argparse.ArgumentTypeError('an explicit path must not be empty')
    return Path(value).resolve()


CLI_ARGS = None


def protected_paths(args):
    return tuple({path.resolve() for path in (args.root, args.source, *args.native,
                 *((args.i686_sysroot,) if args.i686_sysroot is not None else ()))})


def outside_inputs(path, args):
    return not any(Path(path).resolve().is_relative_to(root) for root in protected_paths(args))


def saved_words(path):
    line = path.read_text().splitlines()[0]
    if not line.startswith('savedcmd_') or ' := ' not in line:
        raise ValueError('invalid saved compiler command: ' + str(path))
    return shlex.split(line.split(' := ', 1)[1].replace('$(pound)', '#').split(' ; ', 1)[0])


def validate_relocated_inputs(flags):
    """Reject absent explicit compiler inputs before allocating fixture outputs."""
    options = ('--target', '--extern', '--sysroot', '-include', '-imacros', '-include-pch',
               '-isysroot', '-isystem', '-iquote', '-idirafter', '-I', '-L')
    iterator = iter(flags)
    for flag in iterator:
        key, value = None, None
        if flag in options:
            key, value = flag, next(iterator)
        elif flag.startswith(('--target=', '--extern=', '--sysroot=')):
            key, value = flag.split('=', 1)
        else:
            for option in sorted(options[3:], key=len, reverse=True):
                if flag.startswith(option) and flag != option:
                    key, value = option, flag[len(option):]
                    break
        if key is None:
            continue
        if key == '--target' and '/' not in value and not value.endswith('.json'):
            continue
        if key == '--extern':
            if '=' not in value:
                continue  # genuine compiler search through the recorded -L paths
            value = value.split('=', 1)[1]
        if key == '-L' and '=' in value:
            value = value.split('=', 1)[1]
        if value.startswith(('=', '$SYSROOT')):
            continue  # sysroot-relative C includes are not cwd-relative
        if not Path(value).is_absolute() or not Path(value).exists():
            raise ValueError('missing or unrelocated native input: ' + str(value))


def native_inputs(native):
    """Resolve donor cwd semantics before creating any fixture or log output."""
    donor = next((native / name for name in ('lib/.bcd_rust.o.cmd', 'lib/.hexdump_rust.o.cmd')
                  if (native / name).is_file()), None)
    if donor is None:
        raise ValueError('invalid native Rust donor: ' + str(native))
    words = saved_words(donor)
    environment = {}
    while words and re.match(r'^[A-Za-z_][A-Za-z_0-9]*=', words[0]):
        key, value = words.pop(0).split('=', 1)
        environment[key] = str((native / value).resolve()) if key == 'OBJTREE' else value
    if not words:
        raise ValueError('missing saved Rust compiler')
    if environment.get('OBJTREE') != str(native):
        raise ValueError('saved Rust OBJTREE does not identify native input')
    rust = transport.native_flags(words[1:], native, 'rust')
    validate_relocated_inputs(rust)
    verify_flag_policy(rust, 'rust')
    cwords = saved_words(native / 'lib/.scatterlist.o.cmd')
    cflags = transport.native_flags([word for word in cwords[1:]
                                    if word != '-c' and not word.endswith('/scatterlist.c')], native, 'c')
    validate_relocated_inputs(cflags)
    verify_flag_policy(cflags, 'c')
    bindings = saved_words(native / 'rust/bindings/.bindings_generated.rs.cmd')
    at = bindings.index('--')
    binding_flags = transport.native_flags(bindings[at + 1:], native, 'c')
    validate_relocated_inputs(binding_flags)
    for command in (cwords, bindings):
        executable = command[0]
        if '/' in executable:
            executable = str((native / executable).absolute())
        resolved = shutil.which(executable)
        if resolved is None:
            raise ValueError('invalid native compiler executable: ' + executable)
        command[0] = str(Path(resolved).absolute())
    return words, environment, rust, cwords, cflags, bindings[:at], binding_flags


def configuration(argv=None):
    """CLI flags override explicit GLOB_* environment inputs; empty is invalid."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=explicit_path)
    parser.add_argument('--source', type=explicit_path)
    parser.add_argument('--rustc', type=explicit_path)
    parser.add_argument('--native', action='append', type=explicit_path)
    parser.add_argument('--i686-sysroot', type=explicit_path)
    parser.add_argument('--log', type=explicit_path)
    parser.add_argument('--logs', type=explicit_path)
    args = parser.parse_args(argv if argv is not None else [])
    cli_log,cli_logs=args.log is not None,args.logs is not None
    for name, variable in [('root', 'GLOB_ROOT'), ('source', 'GLOB_SOURCE'),
                           ('rustc', 'GLOB_RUSTC'), ('i686_sysroot', 'GLOB_I686_SYSROOT'),
                           ('log', 'GLOB_LOG'), ('logs', 'GLOB_LOGS')]:
        if name=='log' and cli_logs or name=='logs' and cli_log: continue
        if getattr(args, name) is None and variable in os.environ:
            setattr(args, name, explicit_path(os.environ[variable]))
    if args.native is None:
        args.native = ([explicit_path(value) for value in os.environ['GLOB_NATIVE'].split(os.pathsep)]
                       if 'GLOB_NATIVE' in os.environ else [])
    args.root = args.root if args.root is not None else ROOT
    args.source = args.source if args.source is not None else args.root
    if args.rustc is None:
        compiler = shutil.which('rustc')
        if compiler is None:
            raise ValueError('rustc missing; set GLOB_RUSTC or --rustc')
        args.rustc = Path(compiler).absolute()
    for base, file in [(args.root, 'lib/glob.rs'), (args.root, 'lib/glob_rust.rs'),
                       (args.root, 'lib/Makefile'), (args.source, 'lib/glob.c'),
                       (args.source, 'include/linux/glob.h'), (args.source, 'scripts/Makefile.build'),
                       (args.source, 'rust/ffi_export.rs'), (args.source, 'include/linux/export_header.rs')]:
        if not (base / file).is_file():
            raise ValueError(f'invalid source input: {base / file}')
    if not args.rustc.is_file() or not os.access(args.rustc, os.X_OK):
        raise ValueError(f'invalid rustc tool: {args.rustc}')
    if len(set(args.native)) != len(args.native) or len({path.name for path in args.native}) != len(args.native):
        raise ValueError('duplicate or ambiguous native directory names')
    for native in args.native:
        if not native.is_dir():
            raise ValueError(f'invalid native directory: {native}')
        for relative in ['.config', 'include/config/auto.conf', 'include/generated/rustc_cfg',
                         'lib/.scatterlist.o.cmd', 'rust/bindings/.bindings_generated.rs.cmd',
                         'rust/bindings/bindings_generated.rs',
                         'rust/libkernel.rmeta', 'rust/libbindings.rmeta', 'scripts/basic/fixdep',
                         'scripts/gendwarfksyms/gendwarfksyms']:
            if not (native / relative).is_file():
                raise ValueError(f'invalid native input: {native / relative}')
        if not any((native / p).is_file() for p in ('lib/.bcd_rust.o.cmd', 'lib/.hexdump_rust.o.cmd')):
            raise ValueError(f'invalid native Rust donor: {native}')
    if args.i686_sysroot is not None:
        libdir=args.i686_sysroot / 'lib/rustlib/i686-unknown-linux-gnu/lib'
        libraries = [{path.resolve() for pattern in (name + '.rlib', name + '-*.rlib')
                      for path in libdir.glob(pattern) if path.is_file()}
                     for name in ('libcore', 'libcompiler_builtins')]
        if not libdir.is_dir() or any(len(paths) != 1 for paths in libraries):
            raise ValueError(f'invalid i686 sysroot: {args.i686_sysroot}')
    if args.log is not None and args.logs is not None:
        raise ValueError('choose --logs/GLOB_LOGS or legacy --log/GLOB_LOG, not both')
    if args.logs is not None and not args.logs.is_dir():
        raise ValueError(f'log output directory must already exist: {args.logs}')
    if args.log is not None and (not args.log.parent.is_dir() or args.log.is_dir()):
        raise ValueError(f'invalid log filename/parent: {args.log}')
    destination=args.logs if args.logs is not None else (args.log.parent if args.log is not None else None)
    if destination is not None and not outside_inputs(destination, args):
        raise ValueError('log output must be outside source/native/sysroot inputs')
    if destination is not None and not os.access(destination, os.W_OK):
        raise ValueError(f'log directory is not writable: {destination}')
    # Validate an explicit compiler before creating logs or skipping an optional
    # gate, including an executable that is not actually a supported rustc.
    # Do not call gettempdir(): its probes can create files inside an inherited
    # source/donor TMPDIR even before that directory is rejected.
    parents = ([destination] if destination is not None else []) + [Path(os.environ[name])
        for name in ('TMPDIR', 'TEMP', 'TMP') if os.environ.get(name)] + [Path('/tmp'), Path('/var/tmp')]
    args.temp_parent = next((path.resolve() for path in parents
                            if path.is_dir() and outside_inputs(path, args)), None)
    if args.temp_parent is None:
        raise ValueError('no temporary parent outside read-only inputs')
    args.native_inputs = {native: native_inputs(native) for native in args.native}
    required_tools = ['clang']
    if args.native:
        required_tools += ['nm', 'readelf', 'llvm-objcopy', 'make', 'ld.lld', 'llvm-ar', 'llvm-nm', 'llvm-readelf', 'awk']
    if args.i686_sysroot is not None:
        required_tools += ['ld.lld']
    for name in required_tools:
        if shutil.which(name) is None:
            raise ValueError('missing required executable: ' + name)
    # Even the read-only version probe gets an isolated cwd/temp. All explicit
    # inputs and output roots are already checked; no logs exist at this point.
    with tempfile.TemporaryDirectory(prefix='glob-version-', dir=args.temp_parent) as temporary:
        version=subprocess.run([args.rustc, '--version'], cwd=temporary,
            env=transport.compiler_environment(temporary, os.environ),
            capture_output=True, text=True, timeout=30)
    match=re.search(r'rustc (\d+)\.(\d+)\.(\d+)', version.stdout)
    if version.returncode or not match or tuple(map(int, match.groups())) < (1, 85, 0):
        raise ValueError(f'unsupported rustc tool: {args.rustc}: {version.stdout}{version.stderr}')
    return args


@contextmanager
def fixture_outputs(args, group):
    """Explicit evidence retains fixtures and failures in unique directories."""
    with ExitStack() as stack:
        if args.logs is not None or args.log is not None:
            work=Path(tempfile.mkdtemp(prefix='glob-'+group+'-', dir=args.temp_parent))
        else:
            work=Path(stack.enter_context(tempfile.TemporaryDirectory(prefix='glob-test-', dir=args.temp_parent)))
        if args.logs is not None:
            log_path=work / 'commands.log'
        elif args.log is not None:
            fd,name=tempfile.mkstemp(prefix=args.log.stem+'-'+group+'-',
                                    suffix=args.log.suffix or '.log',dir=args.log.parent)
            os.close(fd)
            log_path=Path(name)
        else:
            log_path=work/'commands.log'
        with log_path.open('a') as log:
            yield work,log,log_path


@contextmanager
def observe_native(args, label):
    """Observe every existing donor directory, including deleted intermediates."""
    with fixture_outputs(args, label) as (work, log, _):
        watches = []
        try:
            with ExitStack() as stack:
                for native in args.native:
                    watches.append(stack.enter_context(transport.NativeWriteWatch(native)))
                yield
        finally:
            records = [{'root': str(watch.root), 'directories': len(watch.paths), 'writes': watch.events}
                       for watch in watches]
            (work / 'donor-observation.json').write_text(json.dumps(records, indent=2) + '\n')
            log.write(json.dumps(records) + '\n')
        assert all(not watch.events for watch in watches), records


def run_group(args, group):
    with fixture_outputs(args, group) as (work,log,log_path):
        log.write(f"GROUP {group}\n")

        def run(cmd, env=None, cwd=work, expect_failure=False):
            cwd = Path(cwd).resolve()
            if not cwd.is_relative_to(work) or not outside_inputs(cwd, args):
                raise ValueError('command cwd must stay inside private fixture')
            scratch = Path(tempfile.mkdtemp(prefix='command-', dir=work))
            environment = transport.compiler_environment(scratch, os.environ if env is None else env)
            log.write('$ ' + shlex.join(map(str, cmd)) + '\nCWD ' + str(cwd) + '\nTMP ' + environment['TMPDIR'] + '\n')
            log.flush()
            try:
                result = subprocess.run(list(map(str, cmd)), cwd=cwd, env=environment,
                                        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=180)
            except (OSError, subprocess.TimeoutExpired) as error:
                log.write('LAUNCH/TIMEOUT ' + repr(error) + '\n')
                if isinstance(error, subprocess.TimeoutExpired) and error.stdout:
                    log.write(error.stdout.decode(errors='replace'))
                log.flush()
                raise
            log.write(result.stdout.decode(errors='replace'))
            log.write(f'EXIT {result.returncode}\n')
            log.flush()
            if bool(result.returncode) != expect_failure:
                raise RuntimeError(f'command failed ({result.returncode}); log {log_path}; '
                                   + result.stdout[-4096:].decode(errors='replace'))
            return result.stdout.decode(errors='replace')

        version = run([args.rustc, '--version'])
        match = re.search(r'rustc (\d+)\.(\d+)\.(\d+)', version)
        assert match and tuple(map(int, match.groups())) >= (1, 85, 0), version
        source = (args.source / 'lib/glob.c').read_text()
        # Keep the entire original implementation; only replace kernel includes
        # and metadata macros in this temporary userspace oracle.
        source = '\n'.join(x for x in source.splitlines() if not x.startswith('#include'))
        oracle = work / 'oracle.c'
        oracle.write_text('#include <stdbool.h>\n#include <stddef.h>\n'
                          '#define __pure\n#define MODULE_DESCRIPTION(x)\n'
                          '#define MODULE_LICENSE(x)\n#define EXPORT_SYMBOL(x)\n'
                          '#define fallthrough __attribute__((fallthrough))\n' + source)
        rust = work / 'rust.so'
        run([args.rustc, '--edition=2021', '-Dwarnings', '--crate-type=cdylib',
             '-Copt-level=2', args.root / 'lib/glob.rs', '-o', rust])
        provider = ctypes.CDLL(str(rust))

        def api(lib):
            lib.glob_match.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
            lib.glob_match.restype = ctypes.c_bool
            lib.glob_match_len.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_size_t]
            lib.glob_match_len.restype = ctypes.c_bool
            return lib

        api(provider)
        if group == 'differential':
            alphabet = b'a![]-*?\\\x80\xff'
            patterns = [bytes(p) for n in range(4) for p in itertools.product(alphabet, repeat=n)]
            patterns += [b'[!a-z]', b'[]a]', b'[a-]', b'[z-a]', b'[\x80-\xff]',
                         b'*a*b*c', b'*[!a]?', b'a\\', b'[^a]', b'a\0*']
            inputs = [b'', b'a', b'[', b']', b'abc', b'aaa', b'\x80', b'\xff', b'a\0z']
            rng = random.Random(731)
            cases = [(p, s) for p in patterns for s in inputs]
            cases += [(bytes(rng.choices(alphabet, k=rng.randrange(20))),
                       bytes(rng.choices(alphabet + b'\0', k=rng.randrange(20))))
                      for _ in range(20000)]
            for opt in ('0', '2', 's'):
                shared = work / f'c{opt}.so'
                run(['clang', '-shared', '-fPIC', '-Wall', '-Wextra', '-Werror',
                     '-funsigned-char', '-O' + opt, oracle, '-o', shared])
                original = api(ctypes.CDLL(str(shared)))
                checks = 0
                for p, s in cases:
                    assert provider.glob_match(p, s) == original.glob_match(p, s), (p, s)
                    for length in {0, len(s), max(0, len(s) - 1)}:
                        assert provider.glob_match_len(p, s, length) == original.glob_match_len(p, s, length), (p, s, length)
                        checks += 1
                log.write(f'PASS x86_64 C -O{opt}: {checks} bounded and {len(cases)} unbounded comparisons\n')
        if group == 'guards':
            # Guard-page fixtures: zero length at PROT_NONE, exact nonterminated
            # prefix, NUL immediately before inaccessible memory, malformed classes.
            guard = work / 'guard.c'
            guard.write_text('''#include <sys/mman.h>
    #include <unistd.h>
    #include <stdbool.h>
    #include <assert.h>
    extern bool glob_match(const char *, const char *);
    extern bool glob_match_len(const char *, const char *, unsigned long);
    int main(void) {
     long n = sysconf(_SC_PAGESIZE);
     char *p = mmap(0, n*2, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
     assert(p != MAP_FAILED); assert(!mprotect(p+n,n,PROT_NONE));
     assert(glob_match_len("",p+n,0)); assert(!glob_match_len("?",p+n,0));
     p[n-1]='x'; assert(glob_match_len("?",p+n-1,1));
     assert(glob_match_len("*x",p+n-1,1)); assert(!glob_match_len("xx",p+n-1,1));
     p[n-1]=0; assert(glob_match("\\\\",p+n-1));
     p[n-2]='['; assert(glob_match(p+n-2,"["));
     p[n-3]='a'; p[n-2]='-'; p[n-4]='[';
     assert(glob_match(p+n-4,"[a-"));
     return 0;
    }
    ''')
            run(['clang', guard, rust, '-o', work / 'guard'])
            run([work / 'guard'])
            log.write('PASS inaccessible-page bounds and malformed pattern checks\n')
            # Independent negative expectations detect a vacuous always-true/false oracle.
            assert provider.glob_match(b'*', b'abc')
            assert not provider.glob_match(b'a', b'b')
        for native in args.native if group == 'native' else []:
            assert native.is_dir(), f'invalid native directory: {native}'
            words, saved_env, rust_flags, cwords, cflags, binding_words, binding_flags = args.native_inputs[native]
            env = dict(os.environ, **saved_env, RUSTC_BOOTSTRAP='1')
            env['RUST_MODFILE'] = 'lib/glob'
            out = work / ('native-' + native.name)
            out.mkdir()
            cmd = [str(args.rustc), *rust_flags, '-Dwarnings', '--out-dir', str(out),
                   '--emit=dep-info=' + str(out / 'glob.d'), '--emit=obj=' + str(out / 'glob.o'),
                   str(args.root / 'lib/glob_rust.rs')]
            log.write('ENV ' + json.dumps({k: env[k] for k in ('OBJTREE', 'RUST_MODFILE', 'RUSTC_BOOTSTRAP')}) + '\n')
            run(cmd, env=env)
            symbols = run(['nm', out / 'glob.o'])
            for name in ('glob_match', 'glob_match_len', '__export_symbol_glob_match', '__export_symbol_glob_match_len'):
                assert any(line.endswith(' ' + name) for line in symbols.splitlines()), name
            sections = run(['readelf', '-SW', out / 'glob.o'])
            assert '.export_symbol' in sections and '.debug_info' in sections
            metadata = run(['readelf', '-p', '.modinfo', out / 'glob.o'])
            assert 'glob.description=glob(7) matching' in metadata and 'glob.file=lib/glob' in metadata
            # Compile the original provider using the always-C scatterlist donor.
            ccmd = [cwords[0], *[word.replace('scatterlist', 'glob') if 'KBUILD_' in word else word for word in cflags],
                    '-Wp,-MMD,' + str(out / 'original.d'), '-c', str(args.source / 'lib/glob.c'), '-o', str(out / 'original.o')]
            run(ccmd)

            def hashes(obj):
                table = run(['nm', obj])
                raw = obj.with_suffix('.text')
                run(['llvm-objcopy', '--dump-section', '.text=' + str(raw), obj])
                data = raw.read_bytes()
                result = {}
                for name in ('glob_match', 'glob_match_len'):
                    address = int(re.search(r'^([0-9a-f]+) T ' + name + r'$', table, re.M)[1], 16)
                    result[name] = data[address-4:address].hex()
                return result

            expected, actual = hashes(out / 'original.o'), hashes(out / 'glob.o')
            log.write('KCFI original=' + repr(expected) + ' Rust=' + repr(actual) + '\n')
            assert expected == actual and all(x != '00000000' for x in actual.values())
            negative = out / 'signed_char.rs'
            negative.write_text('''//! Deliberately incorrect signed-char ABI control.
/// Wrong kernel character type.
#[no_mangle]
pub extern "C" fn glob_match(_p: *const i8, _s: *const i8) -> bool { false }
/// Wrong kernel character type.
#[no_mangle]
pub extern "C" fn glob_match_len(_p: *const i8, _s: *const i8, _n: usize) -> bool { false }
''')
            negative_cmd = [str(negative) if x == str(args.root / 'lib/glob_rust.rs') else
                            '--emit=obj=' + str(out / 'negative.o') if x.startswith('--emit=obj=') else
                            '--emit=dep-info=' + str(out / 'negative.d') if x.startswith('--emit=dep-info=') else x for x in cmd]
            run(negative_cmd, env=env)
            wrong_hashes = hashes(out / 'negative.o')
            assert all(wrong_hashes[name] != expected[name] for name in expected)
            log.write('PASS signed-char KCFI negative control ' + repr(wrong_hashes) + '\n')
            # A wrong-length ABI must produce a different KCFI signature.
            bad = out / 'bad.c'
            bad.write_text('bool glob_match(const char *p, const char *s) { return p == s; }\n'
                           'bool glob_match_len(const char *p, const char *s, unsigned int n) { return p == s && n; }\n')
            badcmd = [str(bad) if x == str(args.source / 'lib/glob.c') else
                      str(out / 'bad.o') if x == str(out / 'original.o') else x for x in ccmd]
            # Real kernel compiler types provide bool through the original header.
            badcmd += ['-include', str(args.source / 'include/linux/glob.h')]
            # The incompatible prototype itself is also a required rejection.
            run(badcmd, expect_failure=True)
            log.write('PASS native C/Rust KCFI equality and wrong-ABI rejection\n')
            # Generate declarations from the authoritative header with the
            # native bindgen donor, then check both functions against them.
            bcmd = list(binding_words)
            bcmd[1] = str(args.source / 'include/linux/glob.h')
            bcmd[bcmd.index('-o') + 1] = str(out / 'bindings.rs')
            bcmd += ['--allowlist-function', 'glob_match.*', '--', *binding_flags,
                     '-Wp,-MMD,' + str(out / 'bindings.d')]
            run(bcmd)
            checker = out / 'check.rs'
            checker.write_text('''//! Genuine generated header ABI checks.
#[allow(dead_code, non_camel_case_types, unreachable_pub)]
mod generated {
 use kernel::ffi;
 include!("bindings.rs");
}
#[path = ''' + json.dumps(str(args.root / 'lib/glob.rs')) + ''']
mod implementation;
pub use implementation::*;
const _: unsafe extern "C" fn(*const kernel::ffi::c_char, *const kernel::ffi::c_char) -> bool = generated::glob_match;
const _: unsafe extern "C" fn(*const kernel::ffi::c_char, *const kernel::ffi::c_char, usize) -> bool = generated::glob_match_len;
const _: unsafe extern "C" fn(*const kernel::ffi::c_char, *const kernel::ffi::c_char) -> bool = implementation::glob_match;
const _: unsafe extern "C" fn(*const kernel::ffi::c_char, *const kernel::ffi::c_char, usize) -> bool = implementation::glob_match_len;
''')
            checkcmd = [str(checker) if x == str(args.root / 'lib/glob_rust.rs') else
                        '--emit=obj=' + str(out / 'check.o') if x.startswith('--emit=obj=') else
                        '--emit=dep-info=' + str(out / 'check.d') if x.startswith('--emit=dep-info=') else x for x in cmd]
            run(checkcmd, env=env)
            log.write('PASS genuine bindgen glob.h declarations and native implementation ABI type checks\n')
            # A private, scoped fixture runs the real Makefile.build rules.
            # The retained component Makefile is read in full; only unrelated
            # library targets are filtered out after selection.
            fixture = out / 'fixture'
            build = out / 'build'
            (fixture / 'lib').mkdir(parents=True)
            (fixture / 'rust').mkdir()
            (fixture / 'include/linux').mkdir(parents=True)
            (build / 'lib').mkdir(parents=True)
            for name in ('glob.rs', 'glob_rust.rs'):
                shutil.copyfile(args.root / 'lib' / name, fixture / 'lib' / name)
            shutil.copyfile(args.source / 'lib/glob.c', fixture / 'lib/glob.c')
            shutil.copyfile(args.source / 'rust/ffi_export.rs', fixture / 'rust/ffi_export.rs')
            shutil.copyfile(args.source / 'include/linux/export_header.rs', fixture / 'include/linux/export_header.rs')
            (fixture / 'lib/Makefile').write_text((args.root / 'lib/Makefile').read_text() +
                '\nobj-y := $(filter glob.o,$(obj-y))\nobj-m := $(filter glob.o,$(obj-m))\nlib-y :=\n')
            # Never place writable donor-directory symlinks in the output.
            shutil.copytree(native / 'include', build / 'include', symlinks=False)
            for name in ('scripts/basic/fixdep', 'scripts/gendwarfksyms/gendwarfksyms'):
                (build / name).parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(native / name, build / name)
            (build / 'rust').mkdir()
            rflags = words[1:words.index('-Zallow-features=arbitrary_self_types,asm_goto,generic_arg_infer,used_with_arg')]
            rflags = transport.native_flags(rflags, native, 'rust') + ['-Dwarnings', '-L' + str(native / 'rust')]
            # Expanded CONFIG responses exceed Make's shell argument limit.
            # Rust responses are one argument per line, not shell quoting.
            response = build / 'rust-flags.rsp'
            response.write_text('\n'.join(rflags) + '\n')
            cflags = [word for word in cflags if 'KBUILD_' not in word]
            driver = build / 'driver.mk'
            driver.write_text('include ' + str(args.source / 'scripts/Makefile.build') + '\n')
            base = ['make', '-rR', '-f', driver, 'srctree=' + str(args.source), 'VPATH=' + str(fixture),
                    'srcroot=' + str(fixture), 'objtree=' + str(build), 'obj=lib',
                    'V=1', 'need-builtin=1', 'CONFIG_GLOB=y', 'CONFIG_RUST=y',
                    'RUSTC=' + str(args.rustc), 'RUSTC_OR_CLIPPY=' + str(args.rustc),
                    'rust_flags=@' + str(response),
                    'rust_crate_features=arbitrary_self_types,asm_goto,generic_arg_infer,used_with_arg',
                    'c_flags=-Wp,-MMD,$(depfile) ' + shlex.join(cflags) +
                    ' -DKBUILD_MODFILE=\'"lib/glob"\' -DKBUILD_MODNAME=\'"glob"\' -DKBUILD_BASENAME=\'"glob"\'',
                    'CC=clang', 'LD=ld.lld', 'AR=llvm-ar', 'NM=llvm-nm', 'OBJCOPY=llvm-objcopy',
                    'AWK=awk', 'READELF=llvm-readelf', 'CONFIG_SHELL=/bin/sh',
                    'CONFIG_OBJTOOL=', 'CONFIG_FTRACE_MCOUNT_USE_RECORDMCOUNT=',
                    'KBUILD_BUILTIN=1', 'lib/built-in.a']
            for selection in ('', 'y', '', 'y'):
                command = base + ['CONFIG_RUST_GLOB=' + selection]
                run(command, env=env, cwd=build)
                archive = run(['llvm-ar', 't', build / 'lib/built-in.a'])
                assert archive.strip().endswith('/glob.o'), archive
                stamp = (build / 'lib/glob.o').stat().st_mtime_ns
                run(command, env=env, cwd=build)
                assert stamp == (build / 'lib/glob.o').stat().st_mtime_ns, 'no-op rebuilt owner'
                if selection:
                    dependencies = (build / 'lib/.glob.o.cmd').read_text()
                    assert 'glob.rs' in dependencies and 'export_header.rs' in dependencies
                    log.write('SAVED KBUILD COMMAND/DEPENDENCIES/CRCs\n' + dependencies + '\n')
                    assert '#SYMVER glob_match ' in dependencies and '#SYMVER glob_match_len ' in dependencies
                    (fixture / 'lib/glob.rs').touch()
                    run(command, env=env, cwd=build)
                    assert stamp != (build / 'lib/glob.o').stat().st_mtime_ns, 'dependency failed to rebuild'
                    stamp = (build / 'lib/glob.o').stat().st_mtime_ns
                    (fixture / 'include/linux/export_header.rs').touch()
                    run(command, env=env, cwd=build)
                    assert stamp != (build / 'lib/glob.o').stat().st_mtime_ns, 'export dependency failed to rebuild'
            # Listing targets must select the same language as the owner, not
            # silently fall through to the original C pattern rule. Read saved
            # source/dependencies and exercise their real if_changed_dep rules.
            for suffix in ('s','ll'):
                target='lib/glob.'+suffix
                previous=None
                for selection in ('','y','','y'):
                    listing=[x for x in base if x!='lib/built-in.a']+[target,'CONFIG_RUST_GLOB='+selection]
                    run(listing,env=env,cwd=build)
                    output=build/target
                    saved=build/('lib/.glob.'+suffix+'.cmd')
                    text=saved.read_text()
                    selected=fixture/('lib/glob_rust.rs' if selection else 'lib/glob.c')
                    assert re.search(r'^source_'+re.escape(target)+r' := '+re.escape(str(selected))+r'$',text,re.M), text
                    stamp=output.stat().st_mtime_ns
                    assert output.stat().st_size and (previous is None or stamp!=previous)
                    run(listing,env=env,cwd=build)
                    assert output.stat().st_mtime_ns==stamp,'no-op rebuilt '+target
                    if selection:
                        recorded={selected.resolve(),*(Path(line.strip().removesuffix('\\').strip()).resolve()
                            for line in text.splitlines() if line.startswith('  /'))}
                        for dependency in ('lib/glob_rust.rs','lib/glob.rs','include/linux/export_header.rs'):
                            assert (fixture/dependency).resolve() in recorded, (target,dependency,text)
                            (fixture/dependency).touch()
                            run(listing,env=env,cwd=build)
                            assert output.stat().st_mtime_ns!=stamp,('listing dependency did not rebuild',target,dependency)
                            stamp=output.stat().st_mtime_ns
                            run(listing,env=env,cwd=build)
                            assert output.stat().st_mtime_ns==stamp,'listing dependency no-op failed'
                    previous=stamp
                    log.write('SAVED LISTING SOURCE/DEPENDENCIES\n'+saved.read_text()+'\n')
            log.write('PASS real private Kbuild C/Rust/C/Rust .o/.s/.ll switching, no-op, dependencies, glob.o archive slot\n')
            bindings = (native / 'rust/bindings/bindings_generated.rs').read_text()
            log.write(f'PASS native object exports/DWARF/metadata {native}\n')
            if 'pub fn glob_match(' not in bindings:
                log.write('UNPROVEN genuine generated glob bindings absent: root regeneration gate\n')
        if group == 'native' and not args.native:
            log.write('SKIP native evidence: no explicit native input supplied\n')
        if group == 'elf32' and args.i686_sysroot is not None:
            assert (args.i686_sysroot / 'lib/rustlib/i686-unknown-linux-gnu/lib').is_dir(), 'invalid i686 sysroot'
            shim = work / 'freestanding.rs'
            shim.write_text('#![no_std]\n#[path=' + json.dumps(str(args.root / 'lib/glob.rs')) + '] pub mod glob;\n'
                            '#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }\n')
            env = dict(os.environ, RUSTC_BOOTSTRAP='1')
            run([args.rustc, '--edition=2021', '--target=i686-unknown-linux-gnu',
                 '--sysroot=' + str(args.i686_sysroot), '-Dwarnings', '-Cpanic=abort',
                 '-Copt-level=2', '--crate-type=staticlib', shim, '-o', work / 'rust32.a'], env=env)
            runner = work / 'runner.c'
            runner.write_text('''typedef unsigned int size_t;
typedef _Bool bool;
extern bool glob_match(const char *, const char *);
extern bool glob_match_len(const char *, const char *, size_t);
extern bool original_match(const char *, const char *);
extern bool original_len(const char *, const char *, size_t);
void *memcpy(void *d, const void *s, size_t n) { char *a=d; const char *b=s; while(n--) *a++=*b++; return d; }
void *memset(void *d, int c, size_t n) { char *a=d; while(n--) *a++=c; return d; }
void *memmove(void *d, const void *s, size_t n) { char *a=d; const char *b=s; if(a<b) return memcpy(d,s,n); while(n) { --n; a[n]=b[n]; } return d; }
void rust_eh_personality(void) {}
static unsigned int state=731;
static unsigned int next(void) { state=state*1664525+1013904223; return state; }
static int test(void) {
 const unsigned char alphabet[]="a![]-*?\\\\\\200\\377";
 char p[24], s[24];
 for(unsigned int i=0;i<100000;i++) {
  unsigned int pn=next()%23, sn=next()%23;
  for(unsigned int j=0;j<pn;j++) p[j]=alphabet[next()%(sizeof(alphabet)-1)];
  for(unsigned int j=0;j<sn;j++) s[j]=alphabet[next()%sizeof(alphabet)];
  p[pn]=0; s[sn]=0;
  if(glob_match(p,s)!=original_match(p,s)) return 1;
  for(unsigned int n=0;n<=sn;n++) if(glob_match_len(p,s,n)!=original_len(p,s,n)) return 2;
 }
 return 0;
}
void _start(void) { int status=test(); __asm__ volatile("int $0x80"::"a"(1),"b"(status):"memory"); __builtin_unreachable(); }
''')
            failures = []
            for opt in ('0', '2', 's'):
                cobj = work / ('original32-' + opt + '.o')
                run(['clang', '-m32', '-ffreestanding', '-fno-pie', '-funsigned-char', '-O' + opt,
                     '-Dglob_match=original_match', '-Dglob_match_len=original_len', '-c', oracle, '-o', cobj])
                exe = work / ('runner32-' + opt)
                run(['clang', '-m32', '-ffreestanding', '-fno-pie', '-fno-stack-protector', '-O2',
                     '-nostdlib', '-static', '-fuse-ld=lld', runner, cobj, work / 'rust32.a', '-o', exe])
                try:
                    run([exe])
                    log.write('PASS ELF32 original-C -O' + opt + ' 100000 generated inputs, all bounds\n')
                except RuntimeError as error:
                    failures.append(str(error))
            if failures:
                raise RuntimeError('ELF32 execution failures (not skipped): ' + '; '.join(failures))
        elif group == 'elf32':
            log.write('SKIP i686: no explicit sysroot supplied\n')
        if group == 'scaling':
            binary = work / 'read-counts'
            flags = [args.rustc, '--edition=2021', '-Dwarnings', '-Copt-level=2', '--test']
            run(flags + [args.root / 'lib/glob.rs', '-o', binary])
            run([binary, '--nocapture'])
            # Reintroduce the actual eager-scan regression through the same
            # instrumented accessor. The identical durable test must reject it.
            negative = work / 'eager.rs'
            text = (args.root / 'lib/glob.rs').read_text()
            marker = "    // SAFETY: Forward the caller's string contracts to each lazy access."
            assert text.count(marker) == 1
            text = text.replace(marker, """    unsafe {
        let mut i = 0;
        while read_byte(pat, i, usize::MAX) != 0 { i += 1; }
        i = 0;
        while read_byte(input, i, len) != 0 { i += 1; }
    }
""" + marker)
            negative.write_text(text)
            bad_binary = work / 'eager-counts'
            run(flags + [negative, '-o', bad_binary])
            rejected = run([bad_binary, '--nocapture'], expect_failure=True)
            assert 'assertion `left == right` failed' in rejected
            log.write('PASS operation counts, suffix scaling and eager-scan negative control\n')


class GlobTests(unittest.TestCase):
    """Discoverable independent gates; optional missing inputs are visible skips."""

    def setUp(self):
        self.args = CLI_ARGS if CLI_ARGS is not None else configuration()

    def test_original_c_differential(self):
        run_group(self.args, 'differential')

    def test_guard_pages_and_nonvacuous_results(self):
        self.output_isolation_and_invalid_inputs()
        run_group(self.args, 'guards')

    def output_isolation_and_invalid_inputs(self):
        # Preserve the original discovery gates while checking setup failures
        # before any logging or optional-input skip can mask them.
        env={key:value for key,value in os.environ.items() if not key.startswith('GLOB_')}
        with tempfile.TemporaryDirectory(prefix='glob-setup-test-', dir=self.args.temp_parent) as directory, mock.patch.dict(os.environ,env,clear=True):
            private=Path(directory)
            file=private/'file';file.write_text('not a directory/tool')
            empty=private/'empty';empty.mkdir()
            base=['--root',str(self.args.root),'--source',str(self.args.source),'--rustc',str(self.args.rustc)]
            before=set(private.rglob('*'))
            for flag in ('--root','--source','--rustc','--native','--i686-sysroot','--logs'):
                for value in ('',str(private/'missing'),str(file),str(empty)):
                    if flag=='--logs' and value==str(empty): continue
                    with self.subTest(flag=flag,value=value),redirect_stderr(io.StringIO()),self.assertRaises((ValueError,SystemExit)):
                        configuration([*base,flag,value])
                    self.assertEqual(set(private.rglob('*')),before)
            for value in ('',str(private/'missing'/'new.log'),str(empty)):
                with redirect_stderr(io.StringIO()),self.assertRaises((ValueError,SystemExit)):
                    configuration([*base,'--log',value])
                self.assertEqual(set(private.rglob('*')),before)
            with self.assertRaises(ValueError): configuration([*base,'--rustc','/bin/true'])
            with mock.patch.object(shutil, 'which', return_value=None), mock.patch.object(subprocess, 'run') as launch, self.assertRaises(ValueError):
                configuration(base)
            launch.assert_not_called()
            for path in self.args.native:
                with mock.patch.object(subprocess, 'run') as launch, self.assertRaises(ValueError):
                    configuration([*base, '--native', str(path), '--native', str(path)])
                launch.assert_not_called()
            for root in protected_paths(self.args):
                for flag, value in (('--logs', root), ('--log', root / 'forbidden.log')):
                    with self.subTest(flag=flag, value=value), mock.patch.object(subprocess, 'run') as launch, self.assertRaises(ValueError):
                        configuration([*base, *sum((['--native', str(path)] for path in self.args.native), []),
                                       *(['--i686-sysroot', str(self.args.i686_sysroot)] if self.args.i686_sysroot else []), flag, str(value)])
                    launch.assert_not_called()
            for variable in ('GLOB_ROOT','GLOB_SOURCE','GLOB_RUSTC','GLOB_NATIVE','GLOB_I686_SYSROOT','GLOB_LOGS','GLOB_LOG'):
                for value in ('',str(private/'missing'/'input')):
                    supplied={**env,'GLOB_ROOT':str(self.args.root),'GLOB_SOURCE':str(self.args.source),
                              'GLOB_RUSTC':str(self.args.rustc),variable:value}
                    with mock.patch.dict(os.environ,supplied,clear=True),self.assertRaises((ValueError,argparse.ArgumentTypeError)):
                        configuration()
                    self.assertEqual(set(private.rglob('*')),before)
            # No explicit logs: even a source-tree TMPDIR is not used; every
            # work/log path disappears on success or exception.
            temporary_directory = tempfile.TemporaryDirectory
            def confined_version(*positional, **keywords):
                self.assertTrue(outside_inputs(keywords['dir'], self.args))
                return temporary_directory(*positional, **{**keywords, 'dir': self.args.temp_parent})
            with mock.patch.dict(os.environ, {'TMPDIR': str(self.args.root), 'TEMP': str(self.args.root), 'TMP': str(self.args.root)}), \
                    mock.patch.object(tempfile, 'TemporaryDirectory', side_effect=confined_version):
                args=configuration(base)
            self.assertTrue(outside_inputs(args.temp_parent, args))
            # The fallback choice is tested above; keep the test's own cleaned
            # fixtures under its caller-authorized output parent as well.
            args.temp_parent = self.args.temp_parent
            root_children=set(args.root.iterdir())
            for fail in (False,True):
                try:
                    with mock.patch.object(tempfile,'tempdir',str(args.root)),fixture_outputs(args,'isolation') as (work,log,path):
                        self.assertFalse(work==args.root or args.root in work.parents)
                        log.write('temporary evidence\n')
                        if fail: raise RuntimeError('controlled cleanup')
                except RuntimeError:
                    self.assertTrue(fail)
                self.assertFalse(work.exists())
                self.assertFalse(path.exists())
                self.assertEqual(set(args.root.iterdir()),root_children)
            # Explicit logs are uniquely preserved, never overwritten, and the
            # legacy single filename is only a naming hint, not an append sink.
            for flag,value in (('--logs',private),('--log',private/'legacy.log')):
                args=configuration([*base,flag,str(value)])
                records=[]
                for index in range(2):
                    try:
                        with fixture_outputs(args,'isolation') as (work,log,path):
                            log.write(str(index));records.append(path)
                            if index: raise RuntimeError('controlled retained failure')
                    except RuntimeError:
                        self.assertEqual(index, 1)
                    self.assertTrue(work.exists())
                self.assertNotEqual(*records)
                self.assertEqual([p.read_text() for p in records],['0','1'])
            self.assertFalse((private/'legacy.log').exists())

    def test_lazy_access_scaling_and_eager_negative_control(self):
        run_group(self.args, 'scaling')

    def test_native_abi_exports_bindgen_and_kbuild(self):
        if not self.args.native:
            self.skipTest('no explicit GLOB_NATIVE or --native input supplied')
        with observe_native(self.args, 'serial-observation'):
            run_group(self.args, 'native')

    def test_concurrent_native_transport(self):
        if not self.args.native:
            self.skipTest('no explicit GLOB_NATIVE; concurrent native evidence unavailable')
        with observe_native(self.args, 'concurrent-observation'):
            with ThreadPoolExecutor(max_workers=2) as pool:
                list(pool.map(lambda _: run_group(self.args, 'native'), range(2)))

    def test_response_paths_and_actual_deleted_compiler_intermediates(self):
        with fixture_outputs(self.args, 'transport-controls') as (work, log, _):
            donor = work / 'synthetic-input'; donor.mkdir()
            (donor / 'inner.rsp').write_text('--target=./scripts/target.json\n-Ldependency=rust\n--extern\nkernel=rust/libkernel.rmeta\n')
            (donor / 'outer.rsp').write_text('@inner.rsp\n--out-dir=lib\n--emit=obj=lib/old.o\n-Dwarnings\n-Zsanitizer=kcfi\n')
            flags = transport.native_flags(['@outer.rsp', 'lib/old.rs'], donor, 'rust')
            self.assertEqual(flags, ['--target=' + str(donor / 'scripts/target.json'), '-Ldependency=' + str(donor / 'rust'),
                                    '--extern', 'kernel=' + str(donor / 'rust/libkernel.rmeta'), '-Dwarnings', '-Zsanitizer=kcfi'])
            (donor / 'c.rsp').write_text('-I"include with space" -include include/config.h -o lib/old.o -Wp,-MMD,lib/old.d -Werror')
            self.assertEqual(transport.native_flags(['@c.rsp'], donor, 'c'),
                ['-I' + str(donor / 'include with space'), '-include', str(donor / 'include/config.h'), '-Werror'])
            (donor / 'recursive.rsp').write_text('@recursive.rsp\n')
            for bad in (['@missing'], ['@recursive.rsp'], ['--extern'], ['-Cincremental=lib/cache'], ['-C', 'incremental=lib/cache']):
                with self.subTest(flags=bad), self.assertRaises((ValueError, FileNotFoundError)):
                    transport.native_flags(bad, donor, 'rust')
            for bad in (['--target=' + str(donor / 'missing.json')], ['--extern', 'kernel=' + str(donor / 'absent.rmeta')],
                        ['-Iinclude'], ['-Ldependency=' + str(donor / 'absent')]):
                with self.subTest(flags=bad), self.assertRaises(ValueError): validate_relocated_inputs(bad)
            # The observer must detect create+delete even when final snapshots
            # are identical. These controls are synthetic, never real donors.
            def deleted(index):
                path = donor / f'synthetic-{index}.rcgu.o'
                path.write_bytes(b'observer negative control, not an ABI object')
                path.unlink()
            with transport.NativeWriteWatch(donor) as control:
                with ThreadPoolExecutor(max_workers=2) as pool: list(pool.map(deleted, range(2)))
            self.assertTrue(any(int(mask, 16) & 0x100 for _, mask in control.events))
            self.assertTrue(any(int(mask, 16) & 0x200 for _, mask in control.events))
            # Actual Rust1.85 compilation: observe real rcgu intermediates, not
            # just an invented filename. All watched outputs are private.
            probe = work / 'probe.rs'
            probe.write_text('#![no_std]\n#[no_mangle]\npub extern "C" fn probe(value: u32) -> u32 { value.wrapping_add(1) }\n')
            environment = transport.compiler_environment(work, os.environ)
            command = [str(self.args.rustc), '--edition=2021', '-Dwarnings', '--crate-type=rlib',
                       '-Ccodegen-units=1', '--emit=obj=' + str(work / 'probe.o'), '--out-dir=' + str(work), str(probe)]
            with transport.NativeWriteWatch(work) as observed:
                result = subprocess.run(command, cwd=work, env=environment, capture_output=True, timeout=120)
            log.write(shlex.join(command) + '\nCWD ' + str(work) + '\n' + result.stdout.decode(errors='replace') + result.stderr.decode(errors='replace') + f'\nEXIT {result.returncode}\n')
            (work / 'transient-observation.json').write_text(json.dumps({'synthetic': control.events, 'actual': observed.events}, indent=2) + '\n')
            self.assertEqual(result.returncode, 0, result.stderr)
            actual = [(path, int(mask, 16)) for path, mask in observed.events if '.rcgu.' in path]
            self.assertTrue(any(mask & 0x100 for _, mask in actual), observed.events)
            self.assertTrue(any(mask & (0x200 | 0x40) for _, mask in actual), observed.events)
            self.assertFalse(list(work.rglob('*.rcgu.o')))

    def test_real_elf32_differential(self):
        if self.args.i686_sysroot is None:
            self.skipTest('no explicit GLOB_I686_SYSROOT or --i686-sysroot supplied')
        run_group(self.args, 'elf32')


def main():
    global CLI_ARGS
    CLI_ARGS = configuration(sys.argv[1:])
    unittest.main(argv=[sys.argv[0], '-v'])


if __name__ == '__main__':
    main()
