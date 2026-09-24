#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Generic hweight: original-C differential, native ABI and real Kbuild.

Run with unittest discovery: python3 -m unittest discover -s scripts/tests -p test_hweight.py -v
HWEIGHT_RUSTC selects a compiler executable (default: rustc on PATH).
HWEIGHT_I686_SYSROOT enables real ELF32 compilation AND execution using matching
i686 core libraries (official hashed or local un-hashed names). SIGSYS fails.
HWEIGHT_NATIVE_X86 / HWEIGHT_NATIVE_ARM64 optionally supply read-only configured
native trees with genuine rmeta, saved commands and gendwarfksyms.
Unset optional inputs skip; explicitly empty, missing or incomplete inputs fail.
HWEIGHT_SOURCE_ROOT selects the source tree for private checker revisions.
HWEIGHT_LOGS selects a retained external evidence parent; otherwise TMPDIR is
used when safely outside every input tree. Each run/group has unique evidence.
HWEIGHT_VERBOSE_COMMANDS=1 emits full compiler commands and diagnostics to stderr.
No native tree is built or written. C/Rust DWARF CRCs may differ by type names;
unsigned parameter/result shapes, genuine exports and normalized KCFI must agree.
"""
import ctypes
import json
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import subprocess
import struct
import sys
import tempfile
import time
import unittest
from unittest.mock import patch

ROOT = Path(os.environ.get('HWEIGHT_SOURCE_ROOT', Path(__file__).resolve().parents[2])).resolve()
FIXTURES = ROOT / 'scripts/tests/fixtures/hweight'
NATIVE_COMMANDS = {
    # An unselected original object's .cmd need not exist in a clean build.
    # scatterlist is always C and has no source-specific compile flags in
    # lib/Makefile; its KBUILD identity is replaced below for our C oracle.
    'x86': ('lib/.list_sort_rust.o.cmd', 'lib/.scatterlist.o.cmd'),
    'arm64': ('lib/math/.cordic_rust.o.cmd', 'lib/.scatterlist.o.cmd'),
}
NAMES = {'__sw_hweight'+str(n) for n in (8, 16, 32, 64)}


def optional_directory(var, required):
    if var not in os.environ:
        raise unittest.SkipTest(var + ' not supplied')
    value = os.environ[var]
    if not value.strip():
        raise ValueError(var + ' explicitly empty')
    path = Path(value).resolve()
    if not path.is_dir():
        raise ValueError(var + ' is not a directory: ' + str(path))
    for name in required:
        if not (path / name).is_file():
            raise ValueError(var + ' missing ' + name)
    return path


def optional_i686_sysroot():
    path = optional_directory('HWEIGHT_I686_SYSROOT', [])
    library = path / 'lib/rustlib/i686-unknown-linux-gnu/lib'
    cores = {core.resolve() for pattern in ('libcore.rlib', 'libcore-*.rlib')
             for core in library.glob(pattern) if core.is_file()}
    if len(cores) != 1:
        raise ValueError('HWEIGHT_I686_SYSROOT missing or ambiguous i686 core')
    return path


def settings():
    """Validate every explicit input before any work directory or optional skip."""
    allowed={'HWEIGHT_RUSTC','HWEIGHT_I686_SYSROOT','HWEIGHT_NATIVE_X86','HWEIGHT_NATIVE_ARM64',
             'HWEIGHT_SOURCE_ROOT','HWEIGHT_LOGS','HWEIGHT_VERBOSE_COMMANDS'}
    if {name for name in os.environ if name.startswith('HWEIGHT_')}-allowed: raise ValueError('unknown HWEIGHT setting')
    for name in allowed:
        if name in os.environ and not os.environ[name].strip(): raise ValueError(name+' explicitly empty')
    if os.environ.get('HWEIGHT_VERBOSE_COMMANDS','0') not in ('0','1'): raise ValueError('invalid HWEIGHT_VERBOSE_COMMANDS')
    if Path(os.environ.get('HWEIGHT_SOURCE_ROOT',ROOT)).resolve()!=ROOT: raise ValueError('source root changed after module load')
    for name in ('lib/hweight.c','lib/hweight.rs','lib/hweight_rust.rs','scripts/tests/fixtures/hweight/driver.c.txt'):
        if not (ROOT/name).is_file(): raise ValueError('invalid HWEIGHT_SOURCE_ROOT: '+name)
    rust=shutil.which(os.environ.get('HWEIGHT_RUSTC','rustc'))
    if rust is None: raise ValueError('HWEIGHT_RUSTC must name a usable compiler executable')
    rust=os.path.abspath(rust)
    protected=[ROOT]
    for arch in ('x86','arm64'):
        variable='HWEIGHT_NATIVE_'+arch.upper()
        if variable in os.environ:
            native=optional_directory(variable,[*NATIVE_COMMANDS[arch],'.config','rust/libkernel.rmeta','rust/libbindings.rmeta',
                'include/generated/rustc_cfg','scripts/gendwarfksyms/gendwarfksyms'])
            if 'modular' in str(native).lower(): raise ValueError('MODULAR donors forbidden')
            protected.append(native)
    if 'HWEIGHT_I686_SYSROOT' in os.environ: protected.append(optional_i686_sysroot())
    logs=Path(os.environ['HWEIGHT_LOGS']).resolve() if 'HWEIGHT_LOGS' in os.environ else None
    if logs is not None:
        if any(logs.is_relative_to(p) for p in protected): raise ValueError('HWEIGHT_LOGS inside read-only input')
        if any(p.exists() and not p.is_dir() for p in (logs,*logs.parents)): raise ValueError('invalid HWEIGHT_LOGS directory')
        parent=next(p for p in (logs,*logs.parents) if p.exists())
        if not os.access(parent,os.W_OK|os.X_OK): raise ValueError('unusable HWEIGHT_LOGS directory')
    parent=next((p.resolve() for p in [Path(os.environ.get('TMPDIR','/tmp')),Path('/tmp')]
                 if p.is_dir() and not any(p.resolve().is_relative_to(x) for x in protected)),None)
    if parent is None: raise ValueError('no external temporary directory')
    # --version is read-only and catches a wrong/old explicit executable before
    # creating evidence or reaching an optional architecture skip.
    version=subprocess.run([rust,'--version'],cwd=parent,capture_output=True,text=True,timeout=15)
    match=re.match(r'rustc (\d+)\.(\d+)\.(\d+)',version.stdout)
    if version.returncode or not match or tuple(map(int,match.groups()))<(1,85,0):
        raise ValueError('HWEIGHT_RUSTC requires Rust >=1.85')
    return dict(rust=rust,protected=protected,logs=logs,parent=parent)


def setUpModule():
    global SETTINGS,EVIDENCE
    SETTINGS=settings()
    base=SETTINGS['logs'] or SETTINGS['parent']
    base.mkdir(parents=True,exist_ok=True)
    EVIDENCE=Path(tempfile.mkdtemp(prefix='hweight-evidence-',dir=base))
    (EVIDENCE/'settings.txt').write_text(repr(SETTINGS)+'\n')
    print('hweight retained evidence:',EVIDENCE,flush=True)


class WriteWatch:
    """Inotify includes intermediates deleted before final filesystem snapshots."""
    def __init__(self,paths):
        libc=ctypes.CDLL(None,use_errno=True)
        self.fd=libc.inotify_init1(os.O_NONBLOCK|os.O_CLOEXEC)
        if self.fd<0: raise OSError(ctypes.get_errno(),'inotify_init1')
        self.paths={}
        for path in paths:
            wd=libc.inotify_add_watch(self.fd,os.fsencode(path),0x100|0x200|0x2|0x4|0x8|0x40|0x80)
            if wd<0:
                os.close(self.fd);raise OSError(ctypes.get_errno(),'inotify_add_watch',str(path))
            self.paths[wd]=str(path)

    def finish(self):
        events=[]
        try:
            while True:
                try: data=os.read(self.fd,1024*1024)
                except BlockingIOError: break
                offset=0
                while offset<len(data):
                    wd,mask,_,length=struct.unpack_from('iIII',data,offset)
                    name=os.fsdecode(data[offset+16:offset+16+length].rstrip(b'\0'))
                    events.append((self.paths.get(wd,'OVERFLOW'),hex(mask),name));offset+=16+length
        finally: os.close(self.fd)
        return events


def donor_inputs(flags,build,*,rust):
    def expand(words,active=()):
        result=[]
        for token in words:
            if token.startswith('@'):
                response=(build/token[1:]).resolve()
                if response in active: raise ValueError('recursive compiler response file')
                data=response.read_text()
                result.extend(expand(data.splitlines() if rust else shlex.split(data),(*active,response)))
            else: result.append(token)
        return result
    flags=expand(flags)
    def path(value): return str((build/value).resolve())
    def library(value):
        kind,sep,name=value.partition('=')
        return kind+sep+path(name) if sep else path(value)
    def external(value):
        name,sep,filename=value.partition('=')
        return name+sep+path(filename) if sep else value
    def target(value): return path(value) if value.endswith('.json') or '/' in value else value
    options={'-I':path,'-L':library,'-include':path,'-isystem':path,'-iquote':path,'-idirafter':path,
             '-imacros':path,'-isysroot':path,'--sysroot':path,'--target':target,'--extern':external}
    result=[];index=0
    while index<len(flags):
        token=flags[index]
        if token in options:
            if index+1==len(flags): raise ValueError('missing donor path argument: '+token)
            result += [token,options[token](flags[index+1])];index+=2;continue
        for option,convert in options.items():
            if token.startswith(option+'='):
                token=option+'='+convert(token[len(option)+1:]);break
            if option in ('-I','-L','-include','-isystem','-iquote','-idirafter','-imacros') and token.startswith(option):
                token=option+convert(token[len(option):]);break
        result.append(token);index+=1
    return result


class Harness:
    def __init__(self, case, out):
        self.case, self.out = case, out
        self.report = []
        self.rust = os.environ.get('HWEIGHT_RUSTC', 'rustc')
        if not self.rust.strip() or not shutil.which(self.rust):
            raise ValueError('HWEIGHT_RUSTC must name a usable compiler executable')
        self.rust = os.path.abspath(shutil.which(self.rust))
        self.env = dict(os.environ, RUSTC_BOOTSTRAP='1', TMPDIR=str(out),TEMP=str(out),TMP=str(out),
                        CARGO_TARGET_DIR=str(out/'cargo'))
        version=self.run([self.rust, '-vV']).stdout
        match=re.search(r'^release: (\d+)\.(\d+)\.(\d+)',version,re.M)
        if not match or tuple(map(int,match.groups()))<(1,85,0): raise ValueError('hweight requires Rust >=1.85')

    def fixture(self, name):
        dest = self.out / name
        if not dest.exists():
            content = (FIXTURES / (name + '.txt')).read_text()
            if name == 'binding_probe.rs':
                content = content.replace('@OWNER@', str(ROOT/'lib/hweight_rust.rs'))
            dest.write_text(content)
        return dest

    def check_dwarf(self, directory):
        for filename in ('rust.symtypes', 'c.symtypes'):
            text = (directory/filename).read_text()
            # Resolve the actual C __u64 typedef before comparing ABI shapes.
            for alias, body in re.findall(r'^(t#\w+) typedef_type \w+ \{ (.*?) \}$', text, re.M):
                text = text.replace(alias, body)
            rows = {line.split()[0]: line for line in text.splitlines()
                    if line.startswith('__sw_hweight')}
            self.case.assertEqual(set(rows), NAMES)
            for name, row in rows.items():
                widths = re.findall(r'byte_size\((\d+)\) encoding\((\d+)\)', row)
                expected = [('8', '7'), ('8', '7')] if name.endswith('64') else [('4', '7'), ('4', '7')]
                self.case.assertEqual(widths, expected, row)

    def run(self, args, *, cwd=None, expected=0, input=None):
        if cwd is not None and not Path(cwd).resolve().is_relative_to(self.out.resolve()):
            raise ValueError('command cwd must remain in private harness output')
        scratch=Path(tempfile.mkdtemp(prefix='command-',dir=self.out))
        cwd=Path(cwd).resolve() if cwd is not None else scratch
        args = list(map(str, args))
        executable=shutil.which(args[0])
        if executable is not None: args[0]=os.path.abspath(executable)
        if args[0]==self.rust and any(a.startswith('--emit=') for a in args):
            if any(a=='--out-dir' or a.startswith('--out-dir=') for a in args): raise ValueError('replay must not retain donor output directory')
            args += ['--out-dir',str(scratch)]
        env=dict(self.env,TMPDIR=str(scratch),TEMP=str(scratch),TMP=str(scratch))
        heading='$ '+shlex.join(args)+'\ncwd='+str(cwd)+'\nTMPDIR=TEMP=TMP='+str(scratch)+'\n'
        command_log=scratch/'command.log';command_log.write_text(heading)
        watch=WriteWatch([scratch,self.out])
        try:
            p = subprocess.run(args, cwd=cwd, env=env, text=True, capture_output=True,
                               input=input, timeout=55,
                               preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        except (OSError,subprocess.TimeoutExpired) as error:
            with command_log.open('a') as stream: stream.write('launch-error='+repr(error)+'\n')
            raise
        finally: (scratch/'write-events.json').write_text(json.dumps(watch.finish())+'\n')
        result=p.stdout+p.stderr+f'\n[exit {p.returncode}]\n'
        with command_log.open('a') as stream: stream.write(result)
        log = heading+result
        with (self.out/'commands.log').open('a') as f:
            f.write(log)
        if os.environ.get('HWEIGHT_VERBOSE_COMMANDS') == '1':
            print(log, file=sys.stderr, end='')
        if expected is not None and p.returncode != expected:
            raise AssertionError(f'{shlex.join(args)}\nexit {p.returncode}, expected {expected}\n{p.stdout}{p.stderr}')
        return p


    def ordinary(self, bits):
        sysroot = optional_i686_sysroot() if bits == 32 else None
        source = re.sub(r'^#include.*$', '', (ROOT/'lib/hweight.c').read_text(), flags=re.M)
        source = '#define EXPORT_SYMBOL(x)\ntypedef unsigned long long __u64;\n'+source
        (self.out/'oracle.c').write_text(source)
        for fast in (False, True):
            for opt in ('0', '2', 's'):
                for debug in (True, False):
                    with self.case.subTest(bits=bits, fast=fast, opt=opt, debug=debug):
                        tag = f'host{bits}-fast{int(fast)}-O{opt}-debug{int(debug)}'
                        d = self.out/tag
                        d.mkdir(exist_ok=True)
                        rf = [self.rust, '--edition=2021', '--crate-type=rlib', '-Zcrate-attr=no_std',
                              '-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '-Cpanic=abort',
                              '-Copt-level='+opt, '-Cdebug-assertions='+str(debug).lower(),
                              '-Coverflow-checks='+str(debug).lower()]
                        if bits == 32:
                            rf += ['--target=i686-unknown-linux-gnu', '--sysroot='+str(sysroot)]
                        if fast:
                            rf += ['--cfg', 'CONFIG_ARCH_HAS_FAST_MULTIPLIER']
                        self.run([*rf, ROOT/'lib/hweight_rust.rs', '--emit=obj='+str(d/'rust.o')+',dep-info='+str(d/'rust.d')])
                        cf = ['clang', '-m'+str(bits), '-O'+opt, '-ffreestanding', '-fno-builtin',
                              '-fno-pie', '-fno-stack-protector', '-Wall', '-Wextra', '-Werror']
                        defines = ['-DBITS_PER_LONG='+str(bits)]
                        if fast:
                            defines += ['-DCONFIG_ARCH_HAS_FAST_MULTIPLIER']
                        defines += [f'-D__sw_hweight{n}=c_hweight{n}' for n in (8, 16, 32, 64)]
                        self.run([*cf, *defines, '-c', self.out/'oracle.c', '-o', d/'c.o'])
                        self.run([*cf, '-c', self.fixture('driver.c'), '-o', d/'driver.o'])
                        extra = ['-no-pie'] if bits == 64 else ['-nostdlib', '-static', '-Wl,-e,_start', self.fixture('start32.S')]
                        self.run([*cf, *extra, d/'rust.o', d/'c.o', d/'driver.o', '-o', d/'differential'])
                        assert (d/'differential').read_bytes()[4] == (1 if bits == 32 else 2)
                        p = self.run([d/'differential'], expected=None)
                        self.report.append(dict(test=tag, exit=p.returncode, passed=p.returncode == 0))
                        self.case.assertEqual(p.returncode, 0, f"{tag}: execution failed (SIGSYS is a failure)")


    def rust_flags(self, build, command):
        args = shlex.split((build/command).read_text().splitlines()[0].split(' := ', 1)[1])
        while '=' in args[0] and not args[0].startswith('-'):
            args.pop(0)
        args = donor_inputs(args[1:],build,rust=True)
        sources=[a for a in args if not a.startswith('-') and a.endswith('.rs')]
        if len(sources)!=1: raise ValueError('native Rust donor must have one source')
        args.remove(sources[0])
        # The ARM donor is a module; generic hweight is a built-in bool owner.
        if '--cfg' in args:
            i = args.index('--cfg')
            if args[i + 1] == 'MODULE':
                del args[i:i + 2]
        result = []
        skip = False
        for a in args:
            if skip:
                skip = False
                continue
            if a == '--out-dir':
                skip = True
                continue
            if a.startswith(('--emit=','--out-dir=','-Copt-level=')):
                continue
            result.append(a)
        return result+['-Dwarnings', '-Dunsafe_op_in_unsafe_fn']


    def c_flags(self, build, command):
        args = donor_inputs(shlex.split((build/command).read_text().splitlines()[0].split(' := ', 1)[1])[1:],build,rust=False)
        identities = ('-DKBUILD_MODFILE=', '-DKBUILD_BASENAME=',
                      '-DKBUILD_MODNAME=', '-D__KBUILD_MODNAME=')
        flags = [a for a in args[:args.index('-c')]
                 if not a.startswith(('-Wp,-MMD,', *identities)) and a not in ('-O0', '-O1', '-O2', '-O3', '-Os', '-Oz')]
        return flags + ['-DKBUILD_MODFILE="lib/hweight"', '-DKBUILD_BASENAME="hweight"',
                        '-DKBUILD_MODNAME="hweight"', '-D__KBUILD_MODNAME=hweight']


    def ids(self, path):
        s = path.read_text()
        values = {n: int(v) & 0xffffffff for n, v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}$', s, re.M)}
        return {name: values[n] for name, n in re.findall(r'^define [^\n]*?@([\w]+)\([^\n]*!kcfi_type !(\d+)', s, re.M)}


    def native(self,arch):
        build=optional_directory('HWEIGHT_NATIVE_'+arch.upper(),NATIVE_COMMANDS[arch])
        watch=WriteWatch([build,build/'lib',build/'rust'])
        try: self._native(arch)
        finally:
            events=watch.finish()
            (self.out/(arch+'-donor-write-events.json')).write_text(json.dumps(events)+'\n')
        self.case.assertEqual(events,[],'transient writes in read-only native donor')

    def _native(self, arch):
        from rust_exports_test_support import read_exports
        rcmd, ccmd = NATIVE_COMMANDS[arch]
        build = optional_directory('HWEIGHT_NATIVE_'+arch.upper(),
            [rcmd, ccmd, 'rust/libkernel.rmeta', 'rust/libbindings.rmeta',
             'include/generated/rustc_cfg', 'scripts/gendwarfksyms/gendwarfksyms'])
        for fast in (False, True):
            rf = self.rust_flags(build, rcmd)
            cf = self.c_flags(build, ccmd)
            assert '-Zsanitizer=kcfi' in rf and '-fsanitize=kcfi' in cf
            # Responses were expanded before path relocation. Toggle only the
            # actual multiplier cfg, leaving every other native option intact.
            rf = [a for a in rf if a!='--cfg=CONFIG_ARCH_HAS_FAST_MULTIPLIER']
            if fast: rf += ['--cfg=CONFIG_ARCH_HAS_FAST_MULTIPLIER']
            switch = self.out / (arch + '-fast' + str(int(fast)) + '.h')
            switch.write_text('#undef CONFIG_ARCH_HAS_FAST_MULTIPLIER\n' +
                              ('#define CONFIG_ARCH_HAS_FAST_MULTIPLIER 1\n' if fast else ''))
            cf += ['-include', str(switch)]
            for opt in ('0', '2', 's'):
                d = self.out/(arch+'-fast'+str(int(fast))+'-O'+opt)
                d.mkdir(exist_ok=True)
                self.run([self.rust, *rf, '-Copt-level='+opt, ROOT/'lib/hweight_rust.rs',
                     '--emit=obj='+str(d/'rust.o')+',llvm-ir='+str(d/'rust.ll')+',dep-info='+str(d/'rust.d')])
                self.run([self.rust, *rf, '-Copt-level='+opt, self.fixture('binding_probe.rs'), '--emit=obj='+str(d/'bindings.o')])
                self.run(['clang', *cf, '-O'+opt, '-S', '-emit-llvm', ROOT/'lib/hweight.c', '-o', d/'c.ll'])
                self.run(['clang', *cf, '-O'+opt, '-c', ROOT/'lib/hweight.c', '-o', d/'c.o'])
                ri, ci = self.ids(d/'rust.ll'), self.ids(d/'c.ll')
                for n in (8, 16, 32, 64):
                    name = '__sw_hweight'+str(n)
                    assert ri[name] == ci[name], (arch, name, ri, ci)
                exports = read_exports(d/'rust.o')
                original_exports = read_exports(d/'c.o')
                self.case.assertEqual({row['name'] for row in exports}, NAMES)
                self.case.assertEqual({row['name'] for row in original_exports}, NAMES)
                assert len(exports) == len(original_exports) == 4
                for row in exports + original_exports:
                    assert row['name'] in ['__sw_hweight'+str(n) for n in (8,16,32,64)]
                    assert row['license'] == '' and row['namespace'] == ''
                    assert row['relocation_target'] == row['name'] and row['relocation_addend'] == 0
                    assert row['pointer_width'] == 8
                undefined = self.run(['nm', '-u', d/'rust.o']).stdout.strip()
                assert not undefined, undefined
                dwarf = self.run([build/'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions', '-T', d/'rust.symtypes', d/'rust.o'],
                            input=''.join('__sw_hweight'+str(n)+'\n' for n in (8,16,32,64)))
                (d/'versions.log').write_text(dwarf.stdout+dwarf.stderr)
                original_versions = self.run([build/'scripts/gendwarfksyms/gendwarfksyms', '--dump-versions',
                                         '-T', d/'c.symtypes', d/'c.o'],
                                        input=''.join('__sw_hweight'+str(n)+'\n' for n in (8,16,32,64)))
                (d/'c-versions.log').write_text(original_versions.stdout+original_versions.stderr)
                for n in (8,16,32,64):
                    assert '__sw_hweight'+str(n) in dwarf.stdout
                    assert '__sw_hweight'+str(n) in original_versions.stdout
                self.run(['llvm-objdump', '-dr', d/'rust.o'])
                self.report.append(dict(test=arch+'-O'+opt, passed=True, kcfi=ri, exports=exports))
                self.check_dwarf(d)
                if arch == 'x86':
                    self.native_controls(build, cf, d, opt)
        if arch == 'x86':
            self.architecture_control(build)
            self.binding_control(build)


    def native_controls(self, build, cf, d, opt):
        renames = [f'-D__sw_hweight{n}=c_hweight{n}' for n in (8,16,32,64)]
        self.run(['clang', *cf, '-O'+opt, *renames, '-c', ROOT/'lib/hweight.c', '-o', d/'renamed-c.o'])
        self.run(['clang', '-O'+opt, '-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers',
             '-c', self.fixture('driver.c'), '-o', d/'driver.o'])
        self.run(['clang', '-no-pie', d/'driver.o', d/'rust.o', d/'renamed-c.o', '-o', d/'differential'])
        self.run([d/'differential'])
        self.run(['clang', '-O'+opt, '-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers',
             '-c', self.fixture('kcfi_bad.c'), '-o', d/'bad.o'])
        self.run(['clang', '-no-pie', d/'bad.o', d/'rust.o', '-o', d/'bad'])
        self.run([d/'bad'], expected=-4)
        self.run(['clang', '-no-pie', self.fixture('abi_driver.c'), self.fixture('abi_x86.S'), d/'rust.o', '-o', d/'abi'])
        self.run([d/'abi'])
        self.run([d/'abi', 'corrupt'], expected=1)
        special = self.run([d/'abi', 'special', 'registers'], expected=1)
        self.report.append(dict(test='native-x86-controls-O'+opt, passed=True,
                           special_register_probe_exit=special.returncode,
                           note='special preservation is not the generic C ABI'))


    def architecture_control(self, build):
        # The retained actual architecture object must pass the stronger ABI probe.
        d = self.out/'abi-architecture'
        d.mkdir(exist_ok=True)
        (d/'main.c').write_text('extern int special_probe(void); int main(void) { return special_probe(); }\n')
        self.run(['clang', '-no-pie', d/'main.c', self.fixture('abi_x86.S'), build/'arch/x86/lib/hweight.o', '-o', d/'probe'])
        self.run([d/'probe'])
        self.report.append(dict(test='retained-x86-assembly-special-registers', passed=True))


    def semantic_controls(self):
        self.ordinary(64)
        # Compile semantic mutations of the real owner, preserving its export machinery.
        original = (ROOT/'lib/hweight.rs').read_text()
        mutants = {
            'full-word-count8': original.replace('(res.wrapping_add(res >> 4)) & 0x0f', 'w.count_ones().wrapping_add(res & 0)'),
            'mask16': original.replace('& 0x00ff', '& 0x000f'),
            'fast32': original.replace('0x0101_0101)', '0x0101_0100)'),
            'slow32': original.replace('& 0x0000_00ff', '& 0x0000_000f'),
            'high64': original.replace('pub fn __sw_hweight64(w: u64) -> usize {', 'pub fn __sw_hweight64(w: u64) -> usize {\n    let w = w & 0xffff_ffff;'),
        }
        for name, source in mutants.items():
            assert source != original
            d = self.out/('mutation-'+name)
            d.mkdir(exist_ok=True)
            (d/'hweight.rs').write_text(source)
            owner = (ROOT/'lib/hweight_rust.rs').read_text().replace('../rust/ffi_export.rs', str(ROOT/'rust/ffi_export.rs'))
            (d/'owner.rs').write_text(owner)
            fast = name == 'fast32'
            flags = ['--cfg', 'CONFIG_ARCH_HAS_FAST_MULTIPLIER'] if fast else []
            self.run([self.rust, '--edition=2021', '--crate-type=rlib', '-Zcrate-attr=no_std', '-Dwarnings',
                 '-Dunsafe_op_in_unsafe_fn', '-Cpanic=abort', '-Copt-level=2', *flags,
                 d/'owner.rs', '--emit=obj='+str(d/'rust.o')])
            base = self.out/f'host64-fast{int(fast)}-O2-debug1'
            self.run(['clang', '-no-pie', base/'c.o', base/'driver.o', d/'rust.o', '-o', d/'differential'])
            p = self.run([d/'differential'], expected=None)
            assert p.returncode in (8,16,32,64), (name, p.returncode)
            self.report.append(dict(test='mutation-'+name, passed=True, rejected_exit=p.returncode))


    def binding_control(self, build):
        # Return-type mutation must fail against the genuine generated binding.
        d = self.out/'mutation-type'
        d.mkdir(exist_ok=True)
        probe = self.fixture('binding_probe.rs').read_text()
        probe = probe.replace('unsafe extern "C" fn(u64) -> usize', 'unsafe extern "C" fn(u64) -> u32')
        (d/'probe.rs').write_text(probe)
        rf = self.rust_flags(build, 'lib/.list_sort_rust.o.cmd')
        p = self.run([self.rust, *rf, d/'probe.rs', '--emit=obj='+str(d/'bad.o')], expected=None)
        assert p.returncode != 0 and 'mismatched types' in p.stderr
        self.report.append(dict(test='binding-return-width-mutation', passed=True))


class HweightTests(unittest.TestCase):
    def setUp(self):
        out=Path(tempfile.mkdtemp(prefix=self._testMethodName+'-',dir=EVIDENCE))
        self.h = Harness(self, out)

    def tearDown(self):
        if hasattr(self,'h'): (self.h.out/'report.json').write_text(json.dumps(self.h.report,indent=2,default=repr)+'\n')

    def test_nested_response_input_and_output_transport(self):
        d=self.h.out
        (d/'inner.rsp').write_text('--extern\nkernel=rust/libkernel.rmeta\n-L\ndependency=rust\n-I\n')
        (d/'outer.rsp').write_text('@inner.rsp\ninclude path\n--target=target.json\n')
        self.assertEqual(donor_inputs(['@outer.rsp'],d,rust=True),['--extern','kernel='+str(d/'rust/libkernel.rmeta'),
            '-L','dependency='+str(d/'rust'),'-I',str(d/'include path'),'--target='+str(d/'target.json')])
        (d/'c.rsp').write_text('-include "header name.h" -I')
        self.assertEqual(donor_inputs(['@c.rsp','include path'],d,rust=False),['-include',str(d/'header name.h'),'-I',str(d/'include path')])
        (d/'cycle.rsp').write_text('@cycle.rsp')
        for rust in (False,True):
            with self.assertRaisesRegex(ValueError,'recursive'): donor_inputs(['@cycle.rsp'],d,rust=rust)
            with self.assertRaisesRegex(ValueError,'missing'): donor_inputs(['--extern'],d,rust=rust)
        (d/'all.rsp').write_text('--cfg\nMODULE\n@outer.rsp\n--out-dir\nread-only-output\n--emit=obj=read-only.o\nsource.rs\n-Dwarnings\n')
        command=d/'donor.cmd';command.write_text('savedcmd_x := rustc @all.rsp\n')
        flags=self.h.rust_flags(d,'donor.cmd')
        self.assertNotIn('MODULE',flags);self.assertNotIn('source.rs',flags)
        self.assertFalse(any(a.startswith(('--emit=','--out-dir')) or 'read-only' in a for a in flags))
        self.assertIn('kernel='+str(d/'rust/libkernel.rmeta'),flags)
        self.assertIn('-Dwarnings',flags);self.assertIn('-Dunsafe_op_in_unsafe_fn',flags)

    def test_bounded_concurrent_native_transport(self):
        donors=[]
        for arch in ('x86','arm64'):
            variable='HWEIGHT_NATIVE_'+arch.upper()
            if variable in os.environ: donors.append((arch,optional_directory(variable,NATIVE_COMMANDS[arch])))
        if not donors: self.skipTest('native inputs absent; concurrent compiler transport unproven')
        directories=sorted({Path(directory) for _,donor in donors for directory,_,_ in os.walk(donor)})
        watch=WriteWatch(directories)
        jobs=[];streams=[];d=self.h.out
        try:
            for arch,build in donors:
                for copy in range(2):
                    out=d/(arch+'-'+str(copy));out.mkdir()
                    stream=(out/'worker.log').open('x');streams.append(stream)
                    process=subprocess.Popen([sys.executable,str(Path(__file__).resolve()),'--hweight-transport-worker',
                        arch,str(build),str(out)],cwd=d,env=dict(self.h.env,PYTHONDONTWRITEBYTECODE='1'),
                        stdout=stream,stderr=subprocess.STDOUT)
                    jobs.append((process,out))
            deadline=time.monotonic()+20
            while not all((out/'ready').is_file() for _,out in jobs):
                if time.monotonic()>deadline or any(process.poll() is not None for process,_ in jobs):
                    self.fail('bounded compiler worker did not reach barrier; see retained worker logs')
                time.sleep(0.01)
            (d/'start-replays').write_text('release real compiler workers\n')
            for process,out in jobs: self.assertEqual(process.wait(timeout=55),0,str(out/'worker.log'))
        finally:
            for process,_ in jobs:
                if process.poll() is None: process.kill();process.wait()
            for stream in streams: stream.close()
            events=watch.finish()
            (d/'all-donor-events.json').write_text(json.dumps(dict(directories=len(directories),events=events),indent=2)+'\n')
        self.assertEqual(events,[],'concurrent transient writes in read-only donors')
        intervals=[];observed=[]
        for _,out in jobs:
            intervals.extend((out.name,*row) for row in json.loads((out/'intervals.json').read_text()))
            private_events=[row for path in out.glob('command-*/write-events.json') for row in json.loads(path.read_text())]
            created={row[2] for row in private_events if row[1]=='0x100' and row[2].endswith('.rcgu.o')}
            deleted={row[2] for row in private_events if row[1]=='0x200' and row[2].endswith('.rcgu.o')}
            self.assertTrue(created&deleted,'must observe genuine private rustc intermediate creation/deletion')
            observed.append((out.name,sorted(created&deleted)))
        overlaps=[(a,b) for a in intervals for b in intervals if a[0]!=b[0] and max(a[2],b[2])<min(a[3],b[3])]
        self.assertTrue(overlaps,'real compiler invocation intervals must overlap across workers')
        (d/'concurrent-proof.json').write_text(json.dumps(dict(intervals=intervals,overlaps=overlaps,private_transients=observed),indent=2)+'\n')

    def test_private_cwd_temp_transients_and_launch_failures(self):
        d=self.h.out;trap=d/'trap';trap.mkdir();watch=WriteWatch([trap])
        self.h.env.update(TMPDIR=str(trap),TEMP=str(trap),TMP=str(trap))
        code='import os,pathlib,tempfile; assert all(os.environ[k]==os.getcwd() for k in ("TMPDIR","TEMP","TMP")); p=pathlib.Path("transient");p.write_text("x");p.unlink();f=tempfile.NamedTemporaryFile();raise SystemExit(7)'
        try:
            self.h.run([sys.executable,'-c',code],expected=7)
            with self.assertRaises(FileNotFoundError): self.h.run(['/missing/compiler'])
            with self.assertRaisesRegex(ValueError,'private'): self.h.run([sys.executable,'-c','pass'],cwd=ROOT)
        finally: events=watch.finish()
        self.assertEqual(events,[])
        self.assertTrue(any('launch-error=' in p.read_text() for p in d.glob('command-*/command.log')))
        self.assertTrue(any('transient' in p.read_text() for p in d.glob('command-*/write-events.json')))

    def test_preflight_before_writes_or_optional_skips(self):
        for name,value in (('HWEIGHT_LOGS',str(ROOT)),('HWEIGHT_LOGS',str(self.h.out/'commands.log')),
                           ('HWEIGHT_RUSTC',''),('HWEIGHT_RUSTC','/missing/rustc'),('HWEIGHT_RUSTC',shutil.which('true')),
                           ('HWEIGHT_SOURCE_ROOT','/missing/source'),('HWEIGHT_NATIVE_X86',''),
                           ('HWEIGHT_NATIVE_ARM64','/missing/build'),('HWEIGHT_I686_SYSROOT',''),
                           ('HWEIGHT_VERBOSE_COMMANDS','invalid'),('HWEIGHT_UNKNOWN','bad')):
            with self.subTest(name=name,value=value),patch.dict(os.environ,{name:value}),patch.object(Path,'mkdir') as mkdir,patch.object(tempfile,'mkdtemp') as mkdtemp:
                with self.assertRaises(ValueError): setUpModule()
                mkdir.assert_not_called();mkdtemp.assert_not_called()
        alias=self.h.out/'source-alias';alias.symlink_to(ROOT,target_is_directory=True)
        with patch.dict(os.environ,HWEIGHT_LOGS=str(alias/'logs')),self.assertRaises(ValueError): settings()
        for variable in ('HWEIGHT_NATIVE_X86','HWEIGHT_NATIVE_ARM64'):
            if variable in os.environ:
                with patch.dict(os.environ,HWEIGHT_LOGS=str(Path(os.environ[variable])/'forbidden-logs')):
                    with self.assertRaises(ValueError): settings()

    def test_host64_original_c_matrix(self):
        self.h.ordinary(64)

    def test_elf32_original_c_matrix_and_execution(self):
        self.h.ordinary(32)

    def test_semantic_negative_controls(self):
        self.h.semantic_controls()

    def test_native_x86_bindings_kcfi_exports_dwarf_register_abi(self):
        self.h.native('x86')

    def test_native_arm64_bindings_kcfi_exports_dwarf(self):
        self.h.native('arm64')

    def test_native_flags_without_unselected_c_commands(self):
        # Replay genuine native artifacts through a private read-only view.
        # Neither selected Rust owner requires a previous C-provider build.
        for arch in ('x86', 'arm64'):
            with self.subTest(arch=arch):
                variable = 'HWEIGHT_NATIVE_' + arch.upper()
                original = optional_directory(variable, [*NATIVE_COMMANDS[arch]])
                view = self.h.out / ('without-original-c-' + arch)
                view.mkdir()
                for entry in original.iterdir():
                    if entry.name != 'lib':
                        (view / entry.name).symlink_to(entry)
                (view / 'lib').mkdir()
                for entry in (original / 'lib').iterdir():
                    if entry.name not in ('.list_sort.o.cmd', '.hweight.o.cmd'):
                        (view / 'lib' / entry.name).symlink_to(entry)
                for name in ('.list_sort.o.cmd', '.hweight.o.cmd'):
                    self.assertFalse((view / 'lib' / name).exists())
                flags = self.h.c_flags(view, NATIVE_COMMANDS[arch][1])
                self.assertFalse(any('scatterlist' in flag for flag in flags), flags)
                self.assertIn('-D__KBUILD_MODNAME=hweight', flags)
                self.assertIn('-fsanitize=kcfi', flags)
                self.assertIn('-fsanitize-cfi-icall-experimental-normalize-integers', flags)
                with patch.dict(os.environ, {variable: str(view)}):
                    self.h.native(arch)

    def test_explicit_invalid_inputs_fail(self):
        for variable, required in (
            ('HWEIGHT_I686_SYSROOT', []),
            ('HWEIGHT_NATIVE_X86', [NATIVE_COMMANDS['x86'][0]]),
            ('HWEIGHT_NATIVE_ARM64', [NATIVE_COMMANDS['arm64'][0]]),
        ):
            for value in ('', str(self.h.out/'missing'), str(self.h.out)):
                with self.subTest(variable=variable, value=value), patch.dict(os.environ, {variable: value}):
                    with self.assertRaises(ValueError):
                        if variable == 'HWEIGHT_I686_SYSROOT': optional_i686_sysroot()
                        else: optional_directory(variable, required)
        for value in ('', str(self.h.out/'missing-rustc')):
            with patch.dict(os.environ, HWEIGHT_RUSTC=value), self.assertRaises(ValueError):
                Harness(self, self.h.out)
        source = self.h.out/'empty.rs'
        source.write_text('#![no_std]\npub fn test() {}\n')
        result = self.h.run([self.h.rust, '--crate-type=rlib', '--sysroot='+str(self.h.out),
                             source, '--emit=obj='+str(self.h.out/'invalid.o')], expected=None)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('core', result.stderr)

    def test_official_and_local_core_filenames(self):
        root = self.h.out / 'sysroot'
        library = root / 'lib/rustlib/i686-unknown-linux-gnu/lib'
        library.mkdir(parents=True)
        with patch.dict(os.environ, HWEIGHT_I686_SYSROOT=str(root)):
            with self.assertRaises(ValueError): optional_i686_sysroot()
            # Path-only fixtures; the separate ELF32 matrix uses real rustc
            # target libraries and executes every resulting binary.
            official = library / 'libcore-example.rlib'
            official.touch()
            self.assertEqual(optional_i686_sysroot(), root)
            local = library / 'libcore.rlib'
            official.rename(local)
            self.assertEqual(optional_i686_sysroot(), root)
            official.symlink_to(local.name)
            self.assertEqual(optional_i686_sysroot(), root)
            (library / 'libcore-another.rlib').touch()
            with self.assertRaises(ValueError): optional_i686_sysroot()

    def test_kconfig_default_dependencies_and_x86_owner(self):
        build = optional_directory('HWEIGHT_NATIVE_X86', ['scripts/kconfig/conf'])
        h, d = self.h, self.h.out
        stanza = (ROOT/'lib/Kconfig').read_text().split('config RUST_HWEIGHT\n', 1)[1].split('\nconfig ', 1)[0]
        config = d/'Kconfig'
        config.write_text('mainmenu "Hweight selection"\nconfig RUST\n\tbool "Rust"\n'
                          'config GENERIC_HWEIGHT\n\tbool "Generic"\nconfig RUST_HWEIGHT\n'+stanza+'\n')
        for rust, generic, selected, expected in (
            ('y', 'y', False, False), ('y', 'y', True, True),
            ('n', 'y', True, False), ('y', 'n', True, False),
        ):
            (d/'.config').write_text(f'CONFIG_RUST={rust}\nCONFIG_GENERIC_HWEIGHT={generic}\n'
                                    + ('CONFIG_RUST_HWEIGHT=y\n' if selected else ''))
            h.run([build/'scripts/kconfig/conf', '--olddefconfig', config],cwd=d)
            self.assertEqual('CONFIG_RUST_HWEIGHT=y' in (d/'.config').read_text(), expected)
        self.assertNotIn('GENERIC_HWEIGHT', (ROOT/'arch/x86/Kconfig').read_text())
        makefile = d/'architecture.mk'
        makefile.write_text('include '+str(ROOT/'arch/x86/lib/Makefile')+
                            '\n.PHONY: owners\nowners:\n\t@echo $(lib-y) $(obj-y)\n')
        for selected in ('', 'y'):
            objects = h.run(['make', '-s', '-f', makefile, 'CONFIG_RUST_HWEIGHT='+selected,
                             'CONFIG_X86_64=y', 'owners']).stdout.split()
            self.assertEqual(objects.count('hweight.o'), 1)
            self.assertNotIn('hweight_rust.o', objects)

    def test_real_kbuild_selection_archive_slot_dependencies_and_noop(self):
        # Exercise the complete component Makefile and real Makefile.build/fixdep.
        # Unrelated members are inert objects; only hweight C/Rust is compiled.
        h, d = self.h, self.h.out
        for directory in ('lib', 'scripts/basic', 'include/config', 'stubs/linux', 'stubs/asm'):
            (d/directory).mkdir(parents=True, exist_ok=True)
        h.run([h.rust, '--edition=2021', '-Dwarnings', '-O', ROOT/'scripts/basic/fixdep.rs',
               '-o', d/'scripts/basic/fixdep'])
        (d/'stubs/linux/export.h').write_text('#define EXPORT_SYMBOL(x)\n')
        (d/'stubs/linux/bitops.h').write_text('#define BITS_PER_LONG 64\n')
        (d/'stubs/asm/types.h').write_text('typedef unsigned long long __u64;\n')
        rust = shlex.join([h.rust, '--edition=2021', '--crate-type=rlib', '-Zcrate-attr=no_std',
                          '-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '-Cpanic=abort', '-Copt-level=2'])
        base = ['make', '--no-print-directory', '-rR', '-j4', '-f', str(ROOT/'scripts/Makefile.build'),
                'obj=lib', 'srcroot='+str(ROOT), 'srctree='+str(ROOT), 'objtree='+str(d),
                'VPATH='+str(ROOT), 'need-builtin=1', 'KBUILD_BUILTIN=1', 'AR=ar', 'NM=nm',
                'CONFIG_GENERIC_HWEIGHT=y', 'CONFIG_BTREE=y',
                'rust_common_cmd='+rust+' --emit=dep-info=$(depfile)',
                'cmd_cc_o_c=clang -O2 -Wall -Wextra -Werror -ffreestanding -I'+str(d/'stubs')+
                ' -MMD -MF $(depfile) -c $< -o $@']
        listing = d/'listing.mk'
        listing.write_text('.PHONY: members\nmembers:\n\t@echo $(real-obj-y)\n')
        original = h.run([*base, '-f', listing, 'CONFIG_RUST_HWEIGHT=', 'members'],cwd=d).stdout.split()
        self.assertEqual(original.count('lib/hweight.o'), 1, original)
        # BTREE is the next selected original slot, not an appended owner.
        self.assertEqual(original[original.index('lib/hweight.o') + 1], 'lib/btree.o')
        empty = d/'empty.c'
        empty.write_text('/* unrelated archive member */\n')
        ignored = []
        for member in original:
            if member == 'lib/hweight.o':
                continue
            path = d/member
            path.parent.mkdir(parents=True, exist_ok=True)
            if path.suffix == '.a':
                h.run(['ar', 'cr', path])
                ignored += ['-o', str(Path(member).parent)]
            else:
                h.run(['clang', '-c', empty, '-o', path])
            ignored += ['-o', member]
        expected_c = [member for member in original if not member.endswith('.a')]
        archive = d/'lib/built-in.a'

        def build(selected, *extra, generic='y'):
            h.run([*base, *ignored, 'CONFIG_RUST_HWEIGHT='+('y' if selected else ''),
                   'CONFIG_GENERIC_HWEIGHT='+generic, *extra, 'lib/built-in.a'],cwd=d)
            actual = [str(Path(member).relative_to(d)) if Path(member).is_absolute() else member
                      for member in h.run(['ar', 't', archive]).stdout.splitlines()]
            expected = [m.replace('hweight.o', 'hweight_rust.o') if selected else m for m in expected_c]
            if generic != 'y':
                expected = [m for m in expected if Path(m).name not in ('hweight.o', 'hweight_rust.o')]
            self.assertEqual(actual, expected)
            symbols = h.run(['nm', '--defined-only', archive]).stdout
            for name in NAMES:
                self.assertEqual(len(re.findall(r'\bT '+name+r'$', symbols, re.M)), int(generic == 'y'))
            return d/('lib/hweight_rust.o' if selected else 'lib/hweight.o')

        for selected in (False, True, False, True):
            obj = build(selected)
            stamps = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
            build(selected)
            self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamps)
            unselected = ROOT/('lib/hweight.c' if selected else 'lib/hweight_rust.rs')
            build(selected, '-W', str(unselected))
            self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamps)
        command = (d/'lib/.hweight_rust.o.cmd').read_text()
        for source in ('lib/hweight_rust.rs', 'lib/hweight.rs', 'rust/ffi_export.rs',
                       'include/linux/export_header.rs'):
            dependencies = [token for token in command.split() if token.startswith('/')
                            and Path(token).resolve() == (ROOT/source).resolve()]
            self.assertTrue(dependencies, source + ' absent from fixdep command')
            dependency = dependencies[-1]
            before = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
            build(True, '-W', dependency)
            self.assertTrue(all(a > b for a, b in zip(
                (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), before)))
        self.assertIn('$(wildcard include/config/ARCH_HAS_FAST_MULTIPLIER)', command)
        before = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
        (d/'include/config/ARCH_HAS_FAST_MULTIPLIER').touch()
        build(True)
        self.assertTrue(all(a > b for a, b in zip(
            (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), before)))
        stamps = obj.stat().st_mtime_ns, archive.stat().st_mtime_ns
        build(True)
        self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamps)
        cobj = build(False)
        before = cobj.stat().st_mtime_ns, archive.stat().st_mtime_ns
        build(False, '-W', str(ROOT/'lib/hweight.c'))
        self.assertTrue(all(a > b for a, b in zip(
            (cobj.stat().st_mtime_ns, archive.stat().st_mtime_ns), before)))
        build(True, generic='')
        build(False, generic='')


def bounded_transport_worker(arch,build,out):
    """Two small C/Rust object replays per donor, never recursive discovery."""
    options=settings()
    if arch not in NATIVE_COMMANDS: raise ValueError('invalid bounded worker architecture')
    expected=optional_directory('HWEIGHT_NATIVE_'+arch.upper(),NATIVE_COMMANDS[arch])
    if build.resolve()!=expected or not out.is_dir() or any(out.resolve().is_relative_to(p) for p in options['protected']):
        raise ValueError('bounded worker requires validated donor and private output')
    h=Harness(unittest.TestCase(),out)
    rf=h.rust_flags(build,NATIVE_COMMANDS[arch][0]);cf=h.c_flags(build,NATIVE_COMMANDS[arch][1])
    assert '-Zsanitizer=kcfi' in rf and '-fsanitize=kcfi' in cf
    (out/'ready').write_text('real donor flags prepared\n')
    deadline=time.monotonic()+20
    while not (out.parent/'start-replays').is_file():
        if time.monotonic()>deadline: raise RuntimeError('private compiler barrier timed out')
        time.sleep(0.01)
    intervals=[]
    for name,command in (
        ('rust',[h.rust,*rf,'-Copt-level=2',ROOT/'lib/hweight_rust.rs',
                 '--emit=obj='+str(out/'rust.o')+',llvm-ir='+str(out/'rust.ll')+',dep-info='+str(out/'rust.d')]),
        ('c',['clang',*cf,'-O2','-c',ROOT/'lib/hweight.c','-o',out/'c.o']),
        ('c-ir',['clang',*cf,'-O2','-S','-emit-llvm',ROOT/'lib/hweight.c','-o',out/'c.ll'])):
        start=time.monotonic_ns();h.run(command);intervals.append((name,start,time.monotonic_ns()))
    ri,ci=h.ids(out/'rust.ll'),h.ids(out/'c.ll')
    assert all(ri[name]==ci[name] for name in NAMES)
    (out/'intervals.json').write_text(json.dumps(intervals)+'\n')


if __name__ == '__main__':
    if len(sys.argv)==5 and sys.argv[1]=='--hweight-transport-worker':
        bounded_transport_worker(sys.argv[2],Path(sys.argv[3]).resolve(),Path(sys.argv[4]).resolve())
    else:
        unittest.main()
