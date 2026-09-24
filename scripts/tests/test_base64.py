# SPDX-License-Identifier: GPL-2.0-only
"""Original Base64 byte semantics and real native enum/char/bool ABI.

Private compiler plumbing is not a replacement implementation. The original
C body/header and the actual generated enum binding are used unchanged.
"""

import base64
import ctypes
import functools
import hashlib
import itertools
import json
import os
from pathlib import Path
import random
import re
import resource
import shlex
import shutil
import signal
import struct
import subprocess
import sys
import tempfile
import time
import unittest
from unittest.mock import patch

from kconfig_test_support import cached_conf_tools
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_rational_build import environment, headers as export_headers

ROOT = Path(os.environ.get("BASE64_SOURCE_ROOT", Path(__file__).resolve().parents[2])).resolve()
CANDIDATE = Path(os.environ.get("BASE64_CANDIDATE", ROOT)).resolve()
for variable in ("BASE64_SOURCE_ROOT", "BASE64_CANDIDATE"):
    if variable in os.environ and not os.environ[variable]:
        raise ValueError("explicit " + variable + " is empty")
OWNER = CANDIDATE / "lib/base64_rust.rs"
NAMES = ("base64_encode", "base64_decode")


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



def first_saved_command(path):
    # Same first-shell-command parser as the metadata helper, without importing
    # its unrelated gettempdir() side effect before our input preflight.
    lines=path.read_text().splitlines()
    if not lines or ' := ' not in lines[0]: raise ValueError('malformed saved compiler command')
    lexer=shlex.shlex(lines[0].split(' := ',1)[1],posix=True,punctuation_chars=';')
    lexer.whitespace_split=True
    arguments=[]
    for token in lexer:
        if token and set(token)=={';'}: break
        arguments.append(token)
    if not arguments: raise ValueError('empty saved compiler command')
    return arguments


def command(value, label, *, optional=False):
    if not isinstance(value,str) or not value.strip(): raise ValueError(label+' explicitly empty')
    try: words=shlex.split(value)
    except ValueError as error: raise ValueError(label+' malformed command') from error
    if not words or not words[0]: raise ValueError(label+' empty executable')
    executable=shutil.which(words[0])
    if executable is None:
        if optional: return None
        raise ValueError(label+' unavailable executable: '+words[0])
    words[0]=os.path.abspath(executable)
    return words


def native_rust_arguments(build):
    arguments=first_saved_command(build/'lib/math/.int_log_rust.o.cmd')
    while arguments and re.fullmatch(r'[A-Z_]+=.*',arguments[0]): arguments.pop(0)
    if not arguments: raise ValueError('native Rust command has no compiler')
    executable=arguments.pop(0)
    if '/' in executable: executable=str((build/executable).absolute())
    compiler=command(shlex.quote(executable),'native rustc')[0]
    arguments=donor_inputs(arguments,build,rust=True)
    flags=[];iterator=iter(arguments);sources=0
    for argument in iterator:
        if argument in ('--out-dir','--extern','--emit','-o'):
            if next(iterator,None) is None: raise ValueError('missing native Rust operand')
        elif argument.startswith(('--emit=','--out-dir=','--extern=')):
            continue
        elif not argument.startswith('-') and (build/argument).resolve()==(ROOT/'lib/math/int_log_rust.rs').resolve():
            sources+=1
        else: flags.append(argument)
    if sources!=1: raise ValueError('native Rust donor source identity is not unique')
    return compiler,flags+['-Dwarnings']


def native_binding_arguments(build):
    arguments=first_saved_command(build/'rust/bindings/.bindings_generated.rs.cmd')
    if '--' not in arguments: raise ValueError('native bindgen command lacks clang boundary')
    flags=donor_inputs(arguments[arguments.index('--')+1:],build,rust=False)
    tool=str((build/arguments[0]).absolute()) if '/' in arguments[0] else arguments[0]
    return tool,[a for a in flags if not a.startswith('-Wp,-MMD,')]


def settings():
    allowed={'BASE64_SOURCE_ROOT','BASE64_CANDIDATE','BASE64_X86_BUILD','BASE64_ARM64_BUILD','BASE64_LOGS'}
    # Combined discovery also loads test_base64_kunit. These four settings
    # belong solely to that sibling; do not interpret or validate their values.
    sibling={'BASE64_KUNIT_SOURCE_ROOT','BASE64_KUNIT_CANDIDATE',
             'BASE64_KUNIT_NATIVE_X86','BASE64_KUNIT_NATIVE_ARM64'}
    if {key for key in os.environ if key.startswith('BASE64_')}-allowed-sibling: raise ValueError('unknown BASE64 input')
    for key in allowed:
        if key in os.environ and not os.environ[key].strip(): raise ValueError(key+' explicitly empty')
    for root,names in ((ROOT,('lib/base64.c','include/linux/base64.h','rust/ffi.rs','lib/scatterlist.c')),
                       (CANDIDATE,('lib/base64.rs','lib/base64_rust.rs','include/linux/base64_header.rs','lib/Makefile','lib/Kconfig'))):
        if any(not (root/name).is_file() for name in names): raise ValueError('invalid Base64 source/candidate')
    protected=[ROOT.resolve(),CANDIDATE.resolve()]
    native_compilers=set()
    for variable in ('BASE64_X86_BUILD','BASE64_ARM64_BUILD'):
        if variable not in os.environ: continue
        build=Path(os.environ[variable]).resolve()
        for name in ('.config','lib/.scatterlist.o.cmd','lib/math/.int_log_rust.o.cmd',
                     'rust/bindings/.bindings_generated.rs.cmd','include/generated/rustc_cfg'):
            if not (build/name).is_file(): raise ValueError(variable+' missing '+name)
        config=(build/'.config').read_text().splitlines()
        for item in ('CONFIG_CFI=y','CONFIG_RUST_INT_LOG=y',
                     'CONFIG_'+('X86_64' if variable=='BASE64_X86_BUILD' else 'ARM64')+'=y'):
            if item not in config: raise ValueError(variable+' lacks '+item)
        compiler,_=native_c_arguments(build);command(shlex.quote(compiler),'native C compiler')
        compiler,_=native_rust_arguments(build);native_compilers.add(compiler)
        bindgen,_=native_binding_arguments(build)
        command(os.environ.get('BINDGEN',shlex.quote(bindgen)),'native bindgen')
        protected.append(build)
    if 'INT_MATH_I686_SYSROOT' in os.environ:
        value=os.environ['INT_MATH_I686_SYSROOT']
        if not value.strip(): raise ValueError('INT_MATH_I686_SYSROOT explicitly empty')
        root=Path(value).resolve()
        if not list((root/'lib/rustlib/i686-unknown-linux-gnu/lib').glob('libcore*.rlib')):
            raise ValueError('invalid genuine i686 core sysroot')
        protected.append(root)
    for key,default in (('HOSTCC','cc'),('HOSTRUSTC','rustc'),('CLANG','clang'),('OBJDUMP','llvm-objdump')):
        command(os.environ.get(key,default),key)
    for key in ('INT_MATH_I686_RUNNER','BINDGEN','KCONFIG_C_CONF','KCONFIG_RUST_CONF','GENDWARFKSYMS_C','GENDWARFKSYMS_RUST'):
        if key in os.environ: command(os.environ[key],key)
    logs=Path(os.environ['BASE64_LOGS']).resolve() if 'BASE64_LOGS' in os.environ else None
    outside=lambda p: not any(p.is_relative_to(root) for root in protected)
    if logs is not None:
        if not outside(logs): raise ValueError('BASE64_LOGS inside read-only input')
        if any(p.exists() and not p.is_dir() for p in (logs,*logs.parents)): raise ValueError('invalid BASE64_LOGS')
        ancestor=next(p for p in (logs,*logs.parents) if p.exists())
        if not os.access(ancestor,os.W_OK|os.X_OK): raise ValueError('unwritable BASE64_LOGS')
    parent=next((p for value in [os.environ.get(k) for k in ('TMPDIR','TMP','TEMP')]+['/tmp'] if value
                 for p in [Path(value).resolve()] if p.is_dir() and outside(p) and os.access(p,os.W_OK|os.X_OK)),None)
    if parent is None: raise ValueError('no private temporary parent outside inputs')
    rust=command(os.environ.get('HOSTRUSTC','rustc'),'HOSTRUSTC')
    version=subprocess.run([*rust,'--version'],cwd=parent,capture_output=True,text=True,timeout=15)
    match=re.match(r'rustc (\d+)\.(\d+)\.(\d+)',version.stdout)
    if version.returncode or not match or tuple(map(int,match.groups()))<(1,85,0): raise ValueError('Rust >=1.85 required')
    for compiler in native_compilers-{rust[0]}:
        version=subprocess.run([compiler,'--version'],cwd=parent,capture_output=True,text=True,timeout=15)
        match=re.match(r'rustc (\d+)\.(\d+)\.(\d+)',version.stdout)
        if version.returncode or not match or tuple(map(int,match.groups()))<(1,85,0): raise ValueError('native donor requires Rust >=1.85')
    return dict(protected=protected,parent=parent,logs=logs,rust=rust)


def setUpModule():
    global SETTINGS,EVIDENCE
    SETTINGS=settings()
    base=SETTINGS['logs'] or SETTINGS['parent'];base.mkdir(parents=True,exist_ok=True)
    EVIDENCE=Path(tempfile.mkdtemp(prefix='base64-evidence-',dir=base))
    (EVIDENCE/'settings.txt').write_text(repr(SETTINGS)+'\n')
    print('base64 retained evidence:',EVIDENCE,flush=True)
    # Shared host-tool builders also allocate only in this private evidence
    # tree. Fixture cwd is private; direct commands get their own cwd below.
    old_temp=tempfile.tempdir
    tempfile.tempdir=str(EVIDENCE)
    unittest.addModuleCleanup(setattr,tempfile,'tempdir',old_temp)
    environment_patch=patch.dict(os.environ,TMPDIR=str(EVIDENCE),TMP=str(EVIDENCE),TEMP=str(EVIDENCE))
    environment_patch.start();unittest.addModuleCleanup(environment_patch.stop)


CURRENT=None


def run(arguments, *, cwd=None, env=None, input=None, expected=0):
    if CURRENT is None: raise ValueError('command needs private fixture')
    work,logs=CURRENT
    arguments=list(map(str,arguments))
    if not arguments or not arguments[0]: raise ValueError('empty command')
    executable=shutil.which(arguments[0])
    if executable is None: raise ValueError('unavailable executable: '+arguments[0])
    arguments[0]=os.path.abspath(executable)
    cwd=Path(cwd).resolve() if cwd is not None else None
    if cwd is not None and not cwd.is_relative_to(work):
        # git show is explicitly read-only, but even it runs from private cwd.
        if Path(arguments[0]).name=='git' and arguments[1:2]==['show']:
            arguments[1:1]=['-C',str(cwd)];cwd=None
        else: raise ValueError('command cwd must be private')
    if any(a=='--out-dir' or a.startswith('--out-dir=') for a in arguments):
        raise ValueError('direct compiler command retains output directory')
    outputs=[arguments[i+1] for i,a in enumerate(arguments[:-1]) if a in ('-o','-MF')]
    outputs += [item.split('=',1)[1] for a in arguments if a.startswith('--emit=')
                for item in a[7:].split(',') if '=' in item]
    if any(not Path(p).is_absolute() or not Path(p).resolve().is_relative_to(work) for p in outputs):
        raise ValueError('command output must be absolute and private')
    scratch=Path(tempfile.mkdtemp(prefix='command-',dir=logs))
    if any(a.startswith('--emit=') for a in arguments) and '-o' not in arguments:
        arguments += ['--out-dir',str(scratch)]
    environment=dict(os.environ if env is None else env,TMPDIR=str(scratch),TMP=str(scratch),TEMP=str(scratch))
    watch=WriteWatch([scratch,work]);started=time.monotonic_ns()
    heading='$ '+shlex.join(arguments)+'\ncwd='+str(cwd or scratch)+'\nTMPDIR=TMP=TEMP='+str(scratch)+'\n'
    (scratch/'command.log').write_text(heading)
    try:
        result=subprocess.run(arguments,cwd=cwd or scratch,env=environment,input=input,capture_output=True,
                              timeout=120,preexec_fn=lambda:resource.setrlimit(resource.RLIMIT_CORE,(0,0)))
    except (OSError,subprocess.TimeoutExpired) as error:
        with (scratch/'command.log').open('a') as stream: stream.write('launch-error='+repr(error)+'\n')
        raise
    finally:
        (scratch/'interval.json').write_text(json.dumps([arguments,started,time.monotonic_ns()])+'\n')
        (scratch/'write-events.json').write_text(json.dumps(watch.finish())+'\n')
    with (scratch/'command.log').open('a') as stream:
        for name,data in (('stdout',result.stdout),('stderr',result.stderr)):
            stream.write(name+': bytes='+str(len(data))+' sha256='+hashlib.sha256(data).hexdigest()+'\n')
            if len(data)<=65536: stream.write(data.decode(errors='backslashreplace')+'\n')
            else: stream.write('(large binary/diagnostic output: exact digest above; comparison remains in original test)\n')
        stream.write('exit='+str(result.returncode)+'\n')
    if expected is not None and result.returncode!=expected:
        raise AssertionError('command failed ('+str(result.returncode)+'); see '+str(scratch/'command.log'))
    return result



def native_c_arguments(build, makefile=None):
    """Replay common lib C flags without relying on the unselected C owner.

    scatterlist.o is always C and shares lib's common flags. Source-specific
    overrides would invalidate this donor, so fail rather than silently lose
    one. Keep all target/config/user flags and replace the four Kbuild identity
    macros with the actual original base64 identities.
    """
    text = (makefile or CANDIDATE / "lib/Makefile").read_text()
    overrides = re.findall(r"(?m)^\s*(\S*(?:base64|scatterlist)\.o)\s*[:+?]?=", text)
    if overrides:
        raise ValueError("source-specific flags require a new native C donor audit: " + ", ".join(overrides))
    path = build / "lib/.scatterlist.o.cmd"
    if not path.is_file():
        raise ValueError("native C flag proof requires always-C lib/.scatterlist.o.cmd")
    arguments = first_saved_command(path)
    arguments[1:]=donor_inputs(arguments[1:],build,rust=False)
    if '/' in arguments[0]: arguments[0]=str((build/arguments[0]).absolute())
    replacements = {
        '-DKBUILD_MODFILE="lib/scatterlist"': '-DKBUILD_MODFILE="lib/base64"',
        '-DKBUILD_BASENAME="scatterlist"': '-DKBUILD_BASENAME="base64"',
        '-DKBUILD_MODNAME="scatterlist"': '-DKBUILD_MODNAME="base64"',
        '-D__KBUILD_MODNAME=scatterlist': '-D__KBUILD_MODNAME=base64',
    }
    seen = {key: 0 for key in replacements}
    flags = []
    sources = 0
    iterator = iter(arguments[1:])
    for argument in iterator:
        if argument == "-o":
            next(iterator)
        elif argument in ("-MF", "-MT", "-MQ"):
            next(iterator)
        elif argument in ("-c", "-MD", "-MMD", "-MP") or argument.startswith("-Wp,-MMD,"):
            continue
        elif argument.endswith("/lib/scatterlist.c") or argument == "lib/scatterlist.c":
            original = Path(argument)
            if not original.is_absolute():
                original = build / original
            if original.resolve() != (ROOT / "lib/scatterlist.c").resolve():
                raise ValueError("native C donor is not the original scatterlist source")
            sources += 1
        elif argument in replacements:
            seen[argument] += 1
            flags.append(replacements[argument])
        else:
            flags.append(argument)
    if sources != 1 or any(count != 1 for count in seen.values()):
        raise ValueError("unexpected source or Kbuild identity in native C donor command")
    return arguments[0], flags


def headers(work):
    flags = export_headers(work)
    include = work / "include"
    # The original fixed-width typedef chain is retained for DWARF, not an
    # independently invented enum or function declaration.
    (include / "linux/types.h").write_text("#include <asm-generic/int-ll64.h>\n"
                                           "typedef _Bool bool;\n")
    (include / "asm").mkdir(exist_ok=True)
    (include / "asm/bitsperlong.h").write_text('#include "' +
        str(ROOT / "arch/x86/include/uapi/asm/bitsperlong.h") + '"\n')
    (include / "linux/kernel.h").write_text("#define likely(x) __builtin_expect(!!(x), 1)\n"
                                            "#define unlikely(x) __builtin_expect(!!(x), 0)\n")
    (include / "linux/string.h").write_text("/* Original Base64 uses no string functions. */\n")
    return flags


@functools.lru_cache(maxsize=1)
def corpus():
    """All byte pairs, valid alphabet tails, invalid positions and aliasing."""
    values = []
    for variant, padding in itertools.product(range(3), range(2)):
        for size in (0, 1, 2):
            for item in itertools.product(range(256), repeat=size):
                data = bytes(item)
                values.append((0, padding, variant, data))
                values.append((1, padding, variant, data))
        alphabet = (b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
                    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
                    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,")[variant]
        for a, b in itertools.product(alphabet, repeat=2):
            for suffix in (b"=", b"==", b"A=", b"B=", b"AA", b"AB"):
                values.append((1, padding, variant, bytes((a, b)) + suffix))
        for bad, position in itertools.product(range(256), range(8)):
            data = bytearray(b"Zm9vYmFy")
            data[position] = bad
            values.append((1, padding, variant, bytes(data)))
            values.append((2, padding, variant, bytes(data)))
        for data in (b"", b"Zg", b"Zg==", b"Zm8", b"Zm8=", b"Zm9v", b"Zm9vYg==",
                     b"Zm9vZ===", b"Zm9v====", b"Zm9vZg=", b"Zm9vZg==!", b"Zm\0v", b"////"):
            values.append((2, padding, variant, data))
    rng = random.Random(0x626173653634)
    for length in (*range(64), 127, 128, 129, 255, 256, 257, 511, 512):
        data = bytes(rng.getrandbits(8) for _ in range(length))
        for variant, padding in itertools.product(range(3), range(2)):
            values.append((0, padding, variant, data))
            values.append((1, padding, variant, data))
            encoded = base64.b64encode(data).translate(bytes.maketrans(b"+/", (b"+/", b"-_", b"+,")[variant]))
            if not padding:
                encoded = encoded.rstrip(b"=")
            values.append((1, padding, variant, encoded))
            values.append((2, padding, variant, encoded))
    # Negative lengths are not uniformly undefined: encode skips its switch;
    # padded decode rejects, unpadded decode executes the 3-character tail.
    for length, variant, padding in itertools.product((-(1 << 31), -2147483647, -65536, -4, -3, -2, -1), range(3), range(2)):
        values.append((0, padding, variant, b"", length))
        for data in (b"Zm8", b"Zm9", b"AAA", b"//8", b"__8", b",,8", b"\xffAA", b"A\0A", b"AA="):
            values.append((1, padding, variant, data, length))
            values.append((2, padding, variant, data, length))
    return tuple(values)


def serialize(values):
    return struct.pack("<I", len(values)) + b"".join(struct.pack("<4Ii", op, pad, variant, len(data),
        native_length[0] if native_length else len(data)) + data for op, pad, variant, data, *native_length in values)


DRIVER = r'''
#include <linux/base64.h>
_Static_assert(sizeof(void *) * 8 == TEST_BITS, "genuine pointer width");
_Static_assert(sizeof(enum base64_variant) == sizeof(unsigned int), "actual enum width");
#ifdef TEST_PANIC
extern void trigger_fixture_panic(void);
#endif
#ifdef TEST_KCFI
static int (*volatile encode)(const u8 *, int, char *, bool, enum base64_variant) = base64_encode;
static int (*volatile decode)(const char *, int, u8 *, bool, enum base64_variant) = base64_decode;
#define base64_encode(s,n,d,p,v) encode(s,n,d,p,v)
#define base64_decode(s,n,d,p,v) decode(s,n,d,p,v)
#endif
static long transfer(unsigned call, unsigned fd, void *data, unsigned length)
{
    long result;
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : "=a"(result) : "0"(call), "b"(fd), "c"(data), "d"(length) : "memory", "cc");
#else
    asm volatile("syscall" : "=a"(result) : "0"(call == 3 ? 0L : 1L), "D"((unsigned long)fd),
                 "S"(data), "d"((unsigned long)length) : "rcx", "r11", "memory", "cc");
#endif
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status)
{
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
#else
    asm volatile("syscall" : : "a"(60L), "D"((unsigned long)status) : "rcx", "r11", "memory");
#endif
    __builtin_unreachable();
}
static void exact(unsigned call, unsigned fd, void *data, unsigned length)
{
    unsigned done = 0;
    while (done != length) {
        long count = transfer(call, fd, (char *)data + done, length - done);
        if (count <= 0) finish(2);
        done += count;
    }
}
static unsigned char input[1024], buffer[1536];
__attribute__((noreturn)) void base64_abi_main(void)
{
    unsigned total;
    exact(3, 0, &total, sizeof(total));
#ifdef TEST_PANIC
    if (total == 0xffffffff) { trigger_fixture_panic(); finish(98); }
#endif
    while (total--) {
        unsigned record[5];
        exact(3, 0, record, sizeof(record));
        unsigned operation = record[0], length = record[3];
        if (length > sizeof(input)) finish(3);
        exact(3, 0, input, length);
        unsigned extent = (operation == 0 ? (length + 2) / 3 * 4 : length) + 16;
        if (extent > sizeof(buffer)) finish(4);
        for (unsigned i = 0; i < extent; ++i) buffer[i] = 0xa5;
        if (operation == 2)
            for (unsigned i = 0; i < length; ++i) buffer[8 + i] = input[i];
        int native_length = (int)record[4];
        int result = operation == 0 ? base64_encode(input, native_length, (char *)buffer + 8, record[1], record[2]) :
            base64_decode((char *)(operation == 2 ? buffer + 8 : input), native_length, buffer + 8, record[1], record[2]);
        exact(4, 1, &result, sizeof(result));
        exact(4, 1, buffer, extent);
    }
    finish(0);
}
#if __SIZEOF_POINTER__ == 4
asm(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\ncall base64_abi_main\n.size _start,.-_start\n");
#else
asm(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\ncall base64_abi_main\n.size _start,.-_start\n");
#endif
'''

PANIC = r'''
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
/// Actual core panic negative control.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("Base64 ABI panic control"); }
'''


class Fixture(unittest.TestCase):
    def setUp(self):
        global CURRENT
        settings()  # Explicit invalid options fail before allocating this fixture.
        temporary = tempfile.TemporaryDirectory(prefix="base64-native-",dir=EVIDENCE)
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.logs=Path(tempfile.mkdtemp(prefix=self._testMethodName+'-',dir=EVIDENCE))
        CURRENT=(self.work,self.logs)
        self.addCleanup(lambda: globals().__setitem__('CURRENT',None))
        # Shared helper builders retain their original implementations and run
        # from this private cwd, with temp allocations under module evidence.
        old_cwd=Path.cwd();os.chdir(self.work);self.addCleanup(os.chdir,old_cwd)
        directories=sorted({Path(directory) for key in ('BASE64_X86_BUILD','BASE64_ARM64_BUILD')
                            if key in os.environ for directory,_,_ in os.walk(Path(os.environ[key]).resolve())})
        watch=WriteWatch(directories) if directories else None
        def finish_watch():
            events=watch.finish() if watch is not None else []
            (self.logs/'donor-events.json').write_text(json.dumps(dict(directories=len(directories),events=events))+'\n')
            self.assertEqual(events,[],'transient write in read-only Base64 donor')
        self.addCleanup(finish_watch)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)
        self.targets = rust_targets()
        if "INT_MATH_I686_SYSROOT" in os.environ and not os.environ["INT_MATH_I686_SYSROOT"]:
            self.fail("explicit INT_MATH_I686_SYSROOT is empty")
        if "INT_MATH_I686_RUNNER" in os.environ:
            runner = shlex.split(os.environ["INT_MATH_I686_RUNNER"])
            self.assertTrue(runner and shutil.which(runner[0]), "explicit i686 runner is unavailable")

    def c_flags(self, bits=64):
        return [*self.flags, "-m" + str(bits), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS",
                "-funsigned-char", "-fno-strict-overflow", *(["-DCONFIG_64BIT"] if bits == 64 else [])]

    def bindings(self, bits):
        library = self.work / ("libkernel" + str(bits) + ".rlib")
        if not library.exists():
            bindgen = shlex.split(os.environ.get("BINDGEN", "bindgen"))
            if not bindgen or not shutil.which(bindgen[0]):
                if "BINDGEN" in os.environ:
                    self.fail("explicit BINDGEN is unavailable")
                self.skipTest("actual-header bindings require bindgen; set BINDGEN")
            generated = self.work / ("bindings" + str(bits) + ".rs")
            run([*bindgen, ROOT / "include/linux/base64.h", "--use-core", "--rust-target=1.85",
                 "--allowlist-type=base64_variant", "--allowlist-function=base64_.*", "--ctypes-prefix=ffi",
                 "--no-layout-tests", "--no-doc-comments",
                 "--newtype-enum=base64_variant",
                 '--with-attribute-custom-enum=base64_variant=#[cfi_encoding="14base64_variant"]',
                 "-o", generated, "--", *self.c_flags(bits)])
            source = self.work / ("kernel" + str(bits) + ".rs")
            source.write_text('#![no_std]\n#![feature(cfi_encoding)]\n#![allow(non_camel_case_types, non_upper_case_globals)]\n'
                'pub mod bindings { use crate::ffi; include!("' + str(generated) + '"); }\n'
                '#[allow(unused_attributes)] #[path="' + str(ROOT / "rust/ffi.rs") + '"] pub mod ffi;\n')
            run([*self.rustc, *self.targets[bits], "--edition=2021", "--crate-name=kernel",
                 "--crate-type=rlib", "-Cpanic=abort", "-Dwarnings", source, "-o", library],
                env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        return ["--extern", "kernel=" + str(library)]

    def rust_object(self, bits=64, optimize="2", dwarf=5, library=False, extra=(), mutant=False):
        # Only import paths change in the private carrier, with unique-match
        # assertions. All implementation and actual binding types are retained.
        owner = OWNER.read_text()
        for old, new in (("../rust/ffi_export.rs", str(ROOT / "rust/ffi_export.rs")),
                         ("base64.rs", str(CANDIDATE / "lib/base64.rs"))):
            token = '#[path = "' + old + '"]'
            self.assertEqual(owner.count(token), 1)
            owner = owner.replace(token, '#[path = "' + new + '"]')
        if mutant:
            self.assertEqual(owner.count("pub type Base64Variant = bindings::base64_variant;"), 1)
            owner = owner.replace("pub type Base64Variant = bindings::base64_variant;",
                                  "pub type Base64Variant = i32;")
            for name in ("STD", "URLSAFE", "IMAP"):
                owner = owner.replace("bindings::base64_variant::BASE64_" + name + " =>",
                                      "value if value == bindings::base64_variant::BASE64_" + name + ".0 as i32 =>")
        path = self.work / "production.rs"
        path.write_text(owner)
        source = self.work / "owner.rs"
        source.write_text('//! Actual Base64 native owner.\n#![no_std]\nmod production;\npub use production::*;\n' +
                          (PANIC if library else ""))
        output = self.work / ("native.a" if library else "native.o")
        run([*self.rustc, *self.targets[bits], *self.bindings(bits), "--edition=2021", "--crate-name=base64_owner",
             "--crate-type=" + ("staticlib" if library else "rlib"), "-Cpanic=abort", "-Crelocation-model=static",
             "-Dwarnings", "-Dunsafe_op_in_unsafe_fn", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
             "-Coverflow-checks=yes", "-Cdebug-assertions=yes", "-Copt-level=" + optimize,
             "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf), "-Zbinary_dep_depinfo=y",
             "--emit=" + ("link" if library else "obj"), "--emit=dep-info=" + str(self.work / "native.d"),
             *extra, source, "-o", output], env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        return output

    def c_object(self, bits=64, optimize="2", dwarf=5, compiler=None, extra=()):
        output = self.work / "original.o"
        run([*(compiler or self.cc), *self.c_flags(bits), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
             "-fno-stack-protector", *extra, "-c", ROOT / "lib/base64.c", "-o", output])
        return output

    def driver(self, bits, obj, name, compiler=None, extra=(), panic=False):
        source = self.work / "caller.c"
        source.write_text(DRIVER)
        linker = self.work / "no-unwind.lds"
        linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
                          "} } INSERT AFTER .text;\n")
        output = self.work / name
        run([*(compiler or self.cc), *self.c_flags(bits), "-O2", "-DTEST_BITS=" + str(bits),
             *(["-DTEST_PANIC"] if panic else []), "-ffreestanding", "-fno-stack-protector",
             "-fno-pic", "-fno-pie", "-nostdlib", "-static", "-no-pie", "-Wl,-e,_start",
             "-Wl,--gc-sections", "-Wl,-T," + str(linker), *extra, source,
             *(obj if isinstance(obj, list) else [obj]), "-o", output])
        self.assertEqual(output.read_bytes()[:6], b"\x7fELF" + bytes((1 if bits == 32 else 2, 1)))
        return output


class Base64NativeTests(Fixture):
    def abi(self, bits):
        if bits not in self.targets:
            self.skipTest("genuine i686 core unavailable; set INT_MATH_I686_SYSROOT")
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
        if bits == 32 and "INT_MATH_I686_RUNNER" in os.environ and not runner:
            self.fail("explicit INT_MATH_I686_RUNNER is empty")
        data = serialize(corpus())
        expected = None
        for language, optimize in (("C", "0"), ("C", "2"), ("Rust", "0"), ("Rust", "2"), ("Rust", "s")):
            native = language == "Rust"
            obj = self.rust_object(bits, optimize, library=True) if native else self.c_object(bits, optimize)
            executable = self.driver(bits, obj, language + optimize, panic=native and optimize == "0")
            result = run([*runner, executable], input=data)
            self.assertEqual(result.stderr, b"")
            if expected is None:
                expected = result.stdout
                self.assertGreater(len(expected), len(corpus()) * 20)
            else:
                self.assertEqual(result.stdout, expected, (bits, language, optimize))
            if native and optimize == "0":
                result = run([*runner, executable], input=struct.pack("<I", 0xffffffff),expected=None)
                self.assertEqual((result.returncode, result.stdout, result.stderr), (97, b"", b""))

    def test_native_64(self):
        self.abi(64)

    def test_native_32(self):
        self.abi(32)

    def test_negative_lengths_retain_original_defined_paths(self):
        values = [(op, padding, variant, data, length)
                  for length, variant in itertools.product((-(1 << 31), -65536, -3, -1), range(3))
                  for op, padding, data in ((0, 0, b""), (0, 1, b""), (1, 1, b"Zm8"),
                                             (1, 0, b"Zm8"), (1, 0, b"Zm9"), (2, 0, b"Zm8"))]
        expected = bytearray()
        for op, padding, _, data, _ in values:
            buffer = bytearray([0xa5] * (len(data) + 16))
            if op == 2:
                buffer[8:8+len(data)] = data
            result = 0 if op == 0 else -1
            if op and not padding and data == b"Zm8":
                result = 2
                buffer[8:10] = b"fo"
            expected += struct.pack("<i", result) + buffer
        for bits in self.targets:
            runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
            for native in (False, True):
                obj = self.rust_object(bits, library=True) if native else self.c_object(bits)
                driver = self.driver(bits, obj, "negative-lengths")
                self.assertEqual(run([*runner, driver], input=serialize(values)).stdout, bytes(expected))

    def test_actual_bindings_and_exact_gpl_exports(self):
        for bits in self.targets:
            obj = self.rust_object(bits)
            generated = (self.work / ("bindings" + str(bits) + ".rs")).read_text()
            self.assertRegex(generated, r"pub struct base64_variant\(pub .*c_uint\);")
            self.assertIn('#[cfi_encoding = "14base64_variant"]', generated)
            original = self.c_object(bits)
            self.assertEqual(read_exports(obj), read_exports(original))
            for file in (obj, original):
                records = read_exports(file)
                self.assertEqual([(r["name"], r["license"], r["namespace"], r["relocation_target"],
                                   r["relocation_addend"], r["pointer_width"]) for r in records],
                                 [(name, "GPL", "", name, 0, bits // 8) for name in NAMES])
            deps = (self.work / "native.d").read_text()
            for name in ("base64.rs", "ffi_export.rs", "export_header.rs", "libkernel"):
                self.assertIn(name, deps)

    def test_c_kcfi_identity_and_protected_native_calls(self):
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        cflags = ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        rflags = ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
        original_ir = self.work / "original.ll"
        run([*clang, *self.c_flags(), *cflags, "-O2", "-S", "-emit-llvm", ROOT / "lib/base64.c", "-o", original_ir])

        def ids(path):
            source = path.read_text()
            result = {}
            for name in NAMES:
                function = re.search(r"(?m)^define .*@" + name + r"\(.*!kcfi_type !(\d+)", source)
                self.assertIsNotNone(function, source)
                value = re.search(r"(?m)^!" + function[1] + r" = !\{i32 (-?\d+)\}$", source)
                self.assertIsNotNone(value)
                result[name] = int(value[1]) & 0xffffffff
            return result

        expected_ids = ids(original_ir)
        data = serialize(corpus()[::503] + ((0, 1, 0, b"foo"), (1, 1, 0, b"Zm9v")))
        original = self.c_object(compiler=clang, extra=cflags)
        driver = self.driver(64, original, "original-protected", compiler=clang, extra=(*cflags, "-DTEST_KCFI"))
        expected = run([driver], input=data).stdout
        for optimize in ("0", "2"):
            native_ir = self.work / "native.ll"
            native = self.rust_object(optimize=optimize, library=True, extra=(*rflags, "--emit=llvm-ir=" + str(native_ir)))
            self.assertEqual(ids(native_ir), expected_ids)
            driver = self.driver(64, native, "rust-protected", compiler=clang, extra=(*cflags, "-DTEST_KCFI"))
            self.assertEqual(run([driver], input=data).stdout, expected)
        # The old i32 alias retains ordinary calling convention but changes
        # normalized signedness. Only a runtime trap counts as rejection.
        mutant_ir = self.work / "mutant.ll"
        native = self.rust_object(library=True, mutant=True, extra=(*rflags, "--emit=llvm-ir=" + str(mutant_ir)))
        for name in NAMES:
            self.assertNotEqual(ids(mutant_ir)[name], expected_ids[name])
        driver = self.driver(64, native, "mutant-ordinary", compiler=clang)
        self.assertEqual(run([driver], input=data).stdout, expected)
        driver = self.driver(64, native, "mutant-protected", compiler=clang, extra=(*cflags, "-DTEST_KCFI"))
        for value in ((0, 1, 0, b"f"), (1, 1, 0, b"Zg==")):
            result = run([driver], input=serialize((value,)),expected=None)
            self.assertEqual((result.returncode, result.stdout, result.stderr), (-signal.SIGILL, b"", b""))

    def test_provenance(self):
        for relative in ("lib/base64.rs", "include/linux/base64_header.rs"):
            baseline = run(["git", "show", "68f3e0875:" + relative], cwd=ROOT).stdout.decode()
            expected = re.findall(r"(?m)^// SOURCE-COMMIT: .*$", baseline)
            self.assertEqual(re.findall(r"(?m)^// SOURCE-COMMIT: .*$", (CANDIDATE / relative).read_text()), expected)
            self.assertEqual(len(expected), 1)

    def test_safe_header_lengths_capacities_and_partial_writes(self):
        source = self.work / "safe.rs"
        source.write_text('''//! Safe header behavior and checked lengths.
#[path="''' + str(CANDIDATE / "include/linux/base64_header.rs") + '''"]
pub mod base64;
use base64::*;
const _: () = assert!(matches!(BASE64_CHARS(1), Some(2)));
const _: () = assert!(matches!(encoded_len(1, true), Some(4)));
fn main() {
    assert_eq!(encoded_len(usize::MAX, true), None);
    assert_eq!(encoded_len(usize::MAX, false), None);
    assert_eq!(BASE64_CHARS(usize::MAX / 4 + 1), None);
    assert_eq!(BASE64_CHARS(usize::MAX / 4), Some((usize::MAX / 4 * 4 + 2) / 3));
    for variant in [BASE64_STD, BASE64_URLSAFE, BASE64_IMAP] {
        let mut out = [0xa5; 12];
        assert_eq!(base64_encode(b"foo", &mut out[..3], false, variant), Err(Base64Error::OutputTooSmall));
        assert_eq!(out, [0xa5; 12]);
        assert_eq!(base64_encode(b"foo", &mut out, false, variant), Ok(4));
        assert_eq!(&out[..4], b"Zm9v");
        assert_eq!(&out[4..], &[0xa5;8]);
        out.fill(0xa5);
        assert_eq!(base64_decode(b"Zm9vZ===", &mut out, true, variant), Err(Base64Error::InvalidEncoding));
        assert_eq!(&out[..3], b"foo");
        assert_eq!(&out[3..], &[0xa5;9]);
        for size in 0..3 {
            out.fill(0xa5);
            assert_eq!(base64_decode(b"Zm9v", &mut out[..size], false, variant), Err(Base64Error::OutputTooSmall));
            assert_eq!(&out[..size], &b"foo"[..size]);
            assert!(out[size..].iter().all(|v| *v == 0xa5));
        }
        assert_eq!(base64_decode(b"Zh", &mut out, false, variant), Err(Base64Error::InvalidEncoding));
        assert_eq!(base64_decode(b"Zm9", &mut out, false, variant), Err(Base64Error::InvalidEncoding));
    }
}
''')
        for optimize in ("0", "2", "s"):
            output = self.work / "safe"
            run([*self.rustc, "--edition=2021", "-Dwarnings", "-Dunsafe_op_in_unsafe_fn", "-Coverflow-checks=yes",
                 "-Wmissing-docs", "-Wunreachable-pub", "-Copt-level=" + optimize, source, "-o", output])
            run([output])

    def test_real_dwarf_tools_agree_without_false_historical_crc_claim(self):
        tools = dwarf_tools()
        names = {name.encode() for name in NAMES}
        compilers = [self.cc]
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        if clang != self.cc:
            compilers.append(clang)
        for bits, optimize, dwarf in itertools.product(self.targets, ("0", "2"), (4, 5)):
            native = self.rust_object(bits, optimize, dwarf)
            versions, types = dwarf_versions(tools, native, names, self.work)
            self.assertIn(b"base64_variant", types)
            for compiler in compilers:
                original = self.c_object(bits, optimize, dwarf, compiler)
                old, old_types = dwarf_versions(tools, original, names, self.work)
                self.assertIn(b"base64_variant", old_types)
                for name in names:
                    self.assertNotEqual(versions[name], old[name])

    def test_independent_rust_public_ffi_consumer_calls_selected_c_or_rust_provider(self):
        consumer = self.work / "consumer.rs"
        consumer.write_text('''//! Independent consumer of the translated public native interface.
#![no_std]
#[allow(dead_code, unreachable_pub, unused_imports)]
#[path="''' + str(CANDIDATE / "include/linux/base64_header.rs") + '''"] mod header;
use kernel::ffi;
use header::ffi::{base64_encode, base64_decode, base64_variant};
/// Dispatch through the selected provider. See original C pointer contract.
/// # Safety
/// Original C arguments must be valid.
#[no_mangle]
pub unsafe extern "C" fn consumer_encode(s: *const u8, n: ffi::c_int, d: *mut ffi::c_char,
    p: bool, v: base64_variant) -> ffi::c_int {
    let pointer: unsafe extern "C" fn(*const u8, ffi::c_int, *mut ffi::c_char, bool, base64_variant) -> ffi::c_int = base64_encode;
    // SAFETY: volatile prevents indirect-call devirtualization; arguments retain the original contract.
    unsafe { core::ptr::read_volatile(&pointer)(s,n,d,p,v) }
}
/// Dispatch through the selected provider. See original C pointer contract.
/// # Safety
/// Original C arguments must be valid.
#[no_mangle]
pub unsafe extern "C" fn consumer_decode(s: *const ffi::c_char, n: ffi::c_int, d: *mut u8,
    p: bool, v: base64_variant) -> ffi::c_int {
    let pointer: unsafe extern "C" fn(*const ffi::c_char, ffi::c_int, *mut u8, bool, base64_variant) -> ffi::c_int = base64_decode;
    // SAFETY: volatile prevents indirect-call devirtualization; arguments retain the original contract.
    unsafe { core::ptr::read_volatile(&pointer)(s,n,d,p,v) }
}
''')
        values = corpus()[::127] + ((0, 1, 0, b"foo"), (2, 1, 0, b"Zm9v"), (2, 1, 0, b"Zm9v===="))
        data = serialize(values)
        for bits in self.targets:
            clang = shlex.split(os.environ.get("CLANG", "clang"))
            kcfi = bits == 64  # rustc 1.85 does not support i686 KCFI.
            rflags = ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"] if kcfi else []
            cflags = ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"] if kcfi else []
            output = self.work / "consumer.o"
            run([*self.rustc, *self.targets[bits], *self.bindings(bits), "--cfg=CONFIG_RUST", "--edition=2021",
                 "--crate-type=rlib", "-Dwarnings", "-Dunsafe_op_in_unsafe_fn", "-Wmissing-docs", "-Wunreachable-pub",
                 "-Cpanic=abort", "-Copt-level=2", "-Crelocation-model=static", *rflags,
                 "--emit=obj", consumer, "-o", output], env={**environment(), "RUSTC_BOOTSTRAP": "1"})
            undefined = run(["nm", "-u", output]).stdout.splitlines()
            self.assertEqual({line.split()[-1] for line in undefined}, {name.encode() for name in NAMES})
            runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
            expected = run([*runner, self.driver(bits, self.c_object(bits), "original-direct")], input=data).stdout
            for language in ("C", "Rust"):
                native = self.c_object(bits, compiler=clang, extra=cflags) if language == "C" else \
                    self.rust_object(bits, library=True, extra=rflags)
                driver = self.driver(bits, [output, native], "rust-public-caller", compiler=clang,
                    extra=("-Dbase64_encode=consumer_encode", "-Dbase64_decode=consumer_decode"))
                self.assertEqual(run([*runner, driver], input=data).stdout, expected)


class Base64SelectionTests(Fixture):
    def test_native_c_donor_retains_flags_and_audits_source_identity(self):
        lib = self.work / "lib"
        lib.mkdir()
        command = ("clang -Wp,-MMD,lib/.scatterlist.o.d --target=aarch64-linux-gnu "
                   "-funsigned-char -fsanitize=kcfi -DKEEP_USER_FLAG=23 "
                   "-DKBUILD_MODFILE='\"lib/scatterlist\"' -DKBUILD_BASENAME='\"scatterlist\"' "
                   "-DKBUILD_MODNAME='\"scatterlist\"' -D__KBUILD_MODNAME=scatterlist "
                   "-c -o lib/scatterlist.o " + str(ROOT / "lib/scatterlist.c"))
        saved = lib / ".scatterlist.o.cmd"
        saved.write_text("savedcmd_lib/scatterlist.o := " + command + "\n")
        makefile = self.work / "Makefile"
        makefile.write_text((CANDIDATE / "lib/Makefile").read_text())
        compiler, flags = native_c_arguments(self.work, makefile)
        self.assertEqual(compiler, "clang")
        self.assertEqual(flags, ["--target=aarch64-linux-gnu", "-funsigned-char", "-fsanitize=kcfi",
            "-DKEEP_USER_FLAG=23", '-DKBUILD_MODFILE="lib/base64"', '-DKBUILD_BASENAME="base64"',
            '-DKBUILD_MODNAME="base64"', '-D__KBUILD_MODNAME=base64'])
        for key in ("CFLAGS_scatterlist.o", "CFLAGS_REMOVE_base64.o", "KASAN_SANITIZE_base64.o"):
            makefile.write_text((CANDIDATE / "lib/Makefile").read_text() + "\n" + key + " := n\n")
            with self.assertRaisesRegex(ValueError, "source-specific flags"):
                native_c_arguments(self.work, makefile)
        makefile.write_text((CANDIDATE / "lib/Makefile").read_text())
        saved.write_text("savedcmd_lib/scatterlist.o := " + command.replace("-D__KBUILD_MODNAME=scatterlist", "") + "\n")
        with self.assertRaisesRegex(ValueError, "Kbuild identity"):
            native_c_arguments(self.work, makefile)

    def test_actual_kconfig_default_off_dependency_and_original_makefile_slot(self):
        text = (CANDIDATE / "lib/Kconfig").read_text()
        selector = re.search(r"(?ms)^config RUST_BASE64\n.*?(?=^config |\Z)", text)
        self.assertIsNotNone(selector)
        source = self.work / "Kconfig"
        source.write_text('config RUST\n\tbool "Rust"\n\n' + selector[0])
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", "n", False), ("y", None, False), ("y", "y", True)):
                config = self.work / ".config"
                config.write_text("CONFIG_RUST=" + rust + "\n" + ("CONFIG_RUST_BASE64=" + requested + "\n" if requested else ""))
                run([tool, "--olddefconfig", source], cwd=self.work,
                    env={**environment(), "KCONFIG_CONFIG": str(config)})
                self.assertEqual("CONFIG_RUST_BASE64=y" in config.read_text().splitlines(), expected)
        harness = self.work / "Makefile"
        harness.write_text("srctree := " + str(ROOT) + "\nsrc := " + str(ROOT / "lib") +
            "\ninclude " + str(CANDIDATE / "lib/Makefile") + "\n.PHONY: selection\nselection:\n\t@printf '%s\\n' '$(obj-y)'\n")
        baseline = None
        for host, choice in itertools.product(("c", "rust"), ("", "n", "y", "n")):
            objects = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                           "HOST_TOOLS_LANG=" + host, "CONFIG_RUST_BASE64=" + choice], cwd=self.work,
                          env=environment()).stdout.split()
            if baseline is None:
                baseline = objects
                self.assertEqual(baseline.count(b"base64.o"), 1)
                index = baseline.index(b"base64.o")
                self.assertEqual(baseline[index-1:index+2], [b"rhashtable.o", b"base64.o", b"once.o"])
            self.assertEqual(objects, [b"base64_rust.o" if item == b"base64.o" and choice == "y" else item for item in baseline])

    def test_actual_kbuild_switch_noop_dependency_and_native_version_records(self):
        src = self.work / "src"
        (src / "lib").mkdir(parents=True)
        (src / "rust").symlink_to(ROOT / "rust", target_is_directory=True)
        (src / "lib/Makefile").write_text((CANDIDATE / "lib/Makefile").read_text())
        for name in ("base64.rs", "base64_rust.rs"):
            (src / "lib" / name).write_text((CANDIDATE / "lib" / name).read_text())
        (src / "lib/base64.c").symlink_to(ROOT / "lib/base64.c")
        for directory in ("lib", "include/config", "scripts/basic", "scripts/gendwarfksyms"):
            (self.work / directory).mkdir(parents=True, exist_ok=True)
        fixdep = self.work / "scripts/basic/fixdep"
        run([*self.rustc, "--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs", "-o", fixdep])
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(dwarf_tools()[1])
        flags = [*self.rustc, *self.bindings(64), "--crate-type=rlib", "--edition=2021", "-O", "-g",
                 "-Cpanic=abort", "-Dwarnings", "-Dunsafe_op_in_unsafe_fn", "-Wmissing-docs", "-Wunreachable-pub",
                 "-Wrust-2018-idioms", "-Zcrate-attr=no_std", "-Zbinary_dep_depinfo=y"]
        command = ["make", "--no-print-directory", "-rR", "-j4", "-f", str(ROOT / "scripts/Makefile.build"),
                   "obj=lib", "srcroot=" + str(src), "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(src),
                   "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y", "KBUILD_SYMTYPES=1", "NM=nm",
                   "rust_common_cmd=" + shlex.join(flags) + " --emit=dep-info=$(depfile)",
                   "cmd_cc_o_c=" + shlex.join(self.cc + self.c_flags()) + " -O2 -g -MMD -MF $(depfile) -c $< -o $@"]
        env = {**environment(), "RUSTC_BOOTSTRAP": "1"}
        history = {}
        for selected in (False, True, False, True):
            name = "base64_rust" if selected else "base64"
            target = "lib/" + name + ".o"
            args = [*command, "CONFIG_RUST_BASE64=" + ("y" if selected else ""), target]
            run(args, cwd=self.work, env=env)
            obj = self.work / target
            dep = obj.with_name("." + obj.name + ".cmd")
            self.assertIn("source_" + target, dep.read_text())
            versions, _ = dwarf_versions(dwarf_tools(), obj, {n.encode() for n in NAMES}, self.work)
            self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", dep.read_bytes())), versions)
            if selected in history:
                self.assertEqual(versions, history[selected])
            history[selected] = versions
            stamp = obj.stat().st_mtime_ns
            run(args, cwd=self.work, env=env)
            self.assertEqual(obj.stat().st_mtime_ns, stamp)
            if selected:
                for ending in ("base64.rs", "ffi_export.rs", "export_header.rs", "libkernel64.rlib"):
                    self.assertIn(ending, dep.read_text())
                self.assertIn("$(wildcard include/config/MODVERSIONS)", dep.read_text())
                dependency = next(word for word in dep.read_text().split() if word.endswith("/base64.rs"))
                run([*args, "-W", dependency], cwd=self.work, env=env)
                self.assertGreater(obj.stat().st_mtime_ns, stamp)
                for suffix in ("s", "ll"):
                    listing = "lib/base64_rust." + suffix
                    run([*command, "CONFIG_RUST_BASE64=y", listing], cwd=self.work, env=env)
                    text = (self.work / listing).read_text()
                    self.assertIn("base64_encode", text)
                    self.assertIn("base64_decode", text)
                    old = (self.work / listing).stat().st_mtime_ns
                    run([*command, "CONFIG_RUST_BASE64=y", listing], cwd=self.work, env=env)
                    self.assertEqual((self.work / listing).stat().st_mtime_ns, old)
        self.assertNotEqual(history[False], history[True])


class Base64NativeConfigurationTests(Fixture):
    """Read-only strict CFI kernel inputs; requires a built RUST_INT_LOG donor.

    These optional object proofs are not gates for arbitrary Rust configs or
    evidence that the selected Base64 kernel itself has been built/booted.
    """

    def check_native(self, variable, architecture):
        if variable not in os.environ:
            self.skipTest("set " + variable + " for actual kernel-config/header KCFI proof")
        self.assertTrue(os.environ[variable], "explicit " + variable + " is empty")
        build = Path(os.environ[variable]).resolve()
        self.assertTrue(build.is_dir(), "explicit native build is not a directory")
        configuration = (build / ".config").read_text()
        self.assertIn("CONFIG_CFI=y", configuration.splitlines())
        self.assertIn("CONFIG_RUST_INT_LOG=y", configuration.splitlines(),
                      "native Rust flags require the selected RUST_INT_LOG donor")
        self.assertIn("CONFIG_" + ("ARM64" if architecture == "arm64" else "X86_64") + "=y", configuration)
        ccompiler, cflags = native_c_arguments(build)
        self.assertIn("-funsigned-char", cflags)
        self.assertIn("-fsanitize=kcfi", cflags)
        original, original_ir = self.work / "original-native.o", self.work / "original-native.ll"
        run([ccompiler, *cflags, "-c", ROOT / "lib/base64.c", "-o", original])
        run([ccompiler, *cflags, "-S", "-emit-llvm", ROOT / "lib/base64.c", "-o", original_ir])
        bindgen,bflags=native_binding_arguments(build)
        binding = self.work / "native-bindings.rs"
        run([*command(os.environ.get("BINDGEN", shlex.quote(bindgen)),"native bindgen"), ROOT / "include/linux/base64.h",
             "--rust-target=1.85", "--use-core", "--ctypes-prefix=ffi", "--allowlist-type=base64_variant",
             "--newtype-enum=base64_variant", '--with-attribute-custom-enum=base64_variant=#[cfi_encoding="14base64_variant"]',
             "--no-layout-tests", "--no-doc-comments", "-o", binding, "--", *bflags])
        self.assertIn("pub struct base64_variant(pub ffi::c_uint);", binding.read_text())
        # This optional gate requires the completed Rust int_log donor; it is
        # not advertised as accepting an arbitrary CONFIG_RUST kernel build.
        rpath = build / "lib/math/.int_log_rust.o.cmd"
        self.assertTrue(rpath.is_file(), "native Rust flag proof requires selected lib/math/.int_log_rust.o.cmd")
        compiler,rflags=native_rust_arguments(build)
        self.assertIn("-Zsanitizer=kcfi", rflags)
        self.assertIn("-Zsanitizer-cfi-normalize-integers", rflags)
        self.assertIn("-Dunsafe_op_in_unsafe_fn", rflags)
        kernel = self.work / "native-kernel.rs"
        kernel.write_text('//! Actual generated header and kernel FFI fixture.\n'
            '#![feature(cfi_encoding)]\n#[allow(unused_attributes)]\n#[path="' + str(ROOT / "rust/ffi.rs") + '"] pub mod ffi;\n'
            '#[allow(missing_docs, non_camel_case_types)] pub mod bindings { use crate::ffi; include!("' + str(binding) + '"); }\n')
        library = self.work / "libbase64_fixture_kernel.rlib"
        allow = next(arg.split("=", 1)[1] for arg in rflags if arg.startswith("-Zallow-features="))
        run([compiler, *rflags, "-Zallow-features=" + allow + ",cfi_encoding", "--crate-name=base64_fixture_kernel",
             "--emit=link", kernel, "-o", library], env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        source = self.work / "native-owner.rs"
        owner = OWNER.read_text()
        for old, new in (("../rust/ffi_export.rs", str(ROOT / "rust/ffi_export.rs")),
                         ("base64.rs", str(CANDIDATE / "lib/base64.rs"))):
            needle = '#[path = "' + old + '"]'
            self.assertEqual(owner.count(needle), 1)
            owner = owner.replace(needle, '#[path = "' + new + '"]')
        source.write_text(owner)
        native, native_ir = self.work / "native-owner.o", self.work / "native-owner.ll"
        run([compiler, *rflags, "--crate-name=base64_native", "--extern", "kernel=" + str(library),
             "--emit=obj=" + str(native), "--emit=llvm-ir=" + str(native_ir),
             "--emit=dep-info=" + str(self.work / "native-config.d"), source],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        def ids(path):
            text = path.read_text()
            result = {}
            for name in NAMES:
                match = re.search(r"(?m)^define .*@" + name + r"\(.*!kcfi_type !(\d+)", text)
                self.assertIsNotNone(match, text)
                value = re.search(r"(?m)^!" + match[1] + r" = !\{i32 (-?\d+)\}$", text)
                result[name] = int(value[1]) & 0xffffffff
            return result
        self.assertEqual(ids(native_ir), ids(original_ir))
        for file in (original, native):
            self.assertEqual([(r["name"], r["license"], r["namespace"], r["relocation_target"], r["pointer_width"])
                              for r in read_exports(file)], [(n, "GPL", "", n, 8) for n in NAMES])
        self.assertIn(str(library), (self.work / "native-config.d").read_text())
        caller = self.work / "indirect.c"
        caller.write_text('''#include <linux/base64.h>
int guarded_encode(const u8 *s, int n, char *d, bool p, enum base64_variant v);
int guarded_decode(const char *s, int n, u8 *d, bool p, enum base64_variant v);
int guarded_encode(const u8 *s, int n, char *d, bool p, enum base64_variant v) {
    int (*volatile f)(const u8 *, int, char *, bool, enum base64_variant) = base64_encode;
    return f(s,n,d,p,v);
}
int guarded_decode(const char *s, int n, u8 *d, bool p, enum base64_variant v) {
    int (*volatile f)(const char *, int, u8 *, bool, enum base64_variant) = base64_decode;
    return f(s,n,d,p,v);
}
''')
        callobj = self.work / "indirect.o"
        run([ccompiler, *cflags, "-c", caller, "-o", callobj])
        assembly = run([*shlex.split(os.environ.get("OBJDUMP", "llvm-objdump")), "-dr", callobj]).stdout.decode()
        for name in NAMES:
            self.assertIn(name, assembly)
        if architecture == "arm64":
            self.assertRegex(assembly, r"\bbrk\b")
            self.assertRegex(assembly, r"\b(?:blr|br)\s+x\d+")
        else:
            self.assertIn("ud2", assembly)
            self.assertRegex(assembly, r"(?:callq?|jmpq?)\s+\*")
        for artifact in self.work.iterdir():
            if artifact.is_file() and artifact.suffix in ('.o','.ll','.d','.rs','.rlib'):
                shutil.copyfile(artifact,self.logs/artifact.name)

    def test_actual_x86_headers_flags_and_protected_calls(self):
        self.check_native("BASE64_X86_BUILD", "x86")

    def test_actual_arm64_headers_flags_and_protected_calls(self):
        self.check_native("BASE64_ARM64_BUILD", "arm64")

    def check_without_original_owner_command(self, variable, architecture):
        if variable not in os.environ:
            self.skipTest("set " + variable + " for clean-selected native flag proof")
        self.assertTrue(os.environ[variable], "explicit " + variable + " is empty")
        build = Path(os.environ[variable]).resolve()
        self.assertTrue(build.is_dir(), "explicit native build is not a directory")
        view = self.work / "selected-view"
        view.mkdir()
        # This is a read-only flag/header transport view, not a claim of a
        # fresh selected kernel build: all compile outputs stay in self.work.
        for entry in build.iterdir():
            if entry.name == "lib":
                (view / "lib").mkdir()
                for child in entry.iterdir():
                    if child.name not in ("base64.o", ".base64.o.cmd", "base64.s", "base64.ll"):
                        (view / "lib" / child.name).symlink_to(child, target_is_directory=child.is_dir())
            elif entry.name == ".config":
                content = entry.read_text()
                content = re.sub(r"(?m)^(?:CONFIG_RUST_BASE64=.*|# CONFIG_RUST_BASE64 is not set)\n?", "", content)
                (view / entry.name).write_text(content + "\nCONFIG_RUST_BASE64=y\n")
            else:
                (view / entry.name).symlink_to(entry, target_is_directory=entry.is_dir())
        self.assertFalse((view / "lib/.base64.o.cmd").exists())
        with patch.dict(os.environ, {variable: str(view)}):
            self.check_native(variable, architecture)
        self.assertFalse((view / "lib/.base64.o.cmd").exists())

    def test_actual_x86_clean_selected_build_has_no_original_c_command(self):
        self.check_without_original_owner_command("BASE64_X86_BUILD", "x86")

    def test_actual_arm64_clean_selected_build_has_no_original_c_command(self):
        self.check_without_original_owner_command("BASE64_ARM64_BUILD", "arm64")


class Base64TransportTests(Fixture):
    def test_composed_sibling_settings_are_not_host_inputs(self):
        before=settings()
        sibling={'BASE64_KUNIT_SOURCE_ROOT':'/not/a/host/source',
                 'BASE64_KUNIT_CANDIDATE':'',
                 'BASE64_KUNIT_NATIVE_X86':'/not/a/host/native',
                 'BASE64_KUNIT_NATIVE_ARM64':'"not host argv'}
        with patch.dict(os.environ,sibling),patch.object(Path,'mkdir') as mkdir,patch.object(tempfile,'mkdtemp') as mkdtemp:
            self.assertEqual(settings(),before)
            mkdir.assert_not_called();mkdtemp.assert_not_called()
        with patch.dict(os.environ,BASE64_KUNIT_UNKNOWN='invalid'),self.assertRaises(ValueError): settings()
        with patch.dict(os.environ,{**sibling,'BASE64_X86_BUILD':''}),self.assertRaises(ValueError): settings()

    def test_nested_response_and_quoted_command_transport(self):
        build=self.work/'donor with spaces';build.mkdir()
        (build/'rust.rsp').write_text('-I\ninclude directory\n-Ldependency=rust\n--extern\nkernel=rust/libkernel.rmeta\n--target=scripts/target.json\n')
        (build/'outer.rsp').write_text('@rust.rsp\n--sysroot=/dev/null\n')
        self.assertEqual(donor_inputs(['@outer.rsp'],build,rust=True),
            ['-I',str(build/'include directory'),'-Ldependency='+str(build/'rust'),'--extern',
             'kernel='+str(build/'rust/libkernel.rmeta'),'--target='+str(build/'scripts/target.json'),'--sysroot=/dev/null'])
        (build/'c.rsp').write_text('-I "include directory" -include "generated header.h"')
        self.assertEqual(donor_inputs(['@c.rsp'],build,rust=False),
                         ['-I',str(build/'include directory'),'-include',str(build/'generated header.h')])
        (build/'operand.rsp').write_text('include directory\n')
        self.assertEqual(donor_inputs(['-I','@operand.rsp'],build,rust=True),['-I',str(build/'include directory')])
        (build/'cycle.rsp').write_text('@cycle.rsp\n')
        for args,rust in ((['@cycle.rsp'],True),(['@missing.rsp'],True),(['-I'],False)):
            with self.subTest(args=args),self.assertRaises((ValueError,OSError)): donor_inputs(args,build,rust=rust)
        (build/'bad.rsp').write_text('"unterminated')
        with self.assertRaises(ValueError): donor_inputs(['@bad.rsp'],build,rust=False)
        for bad in ('',' ','"unterminated','/missing/compiler','"" -arg'):
            with self.subTest(command=bad),self.assertRaises(ValueError): command(bad,'probe')
        alias=build/'compiler with spaces';alias.symlink_to(sys.executable)
        self.assertEqual(command(shlex.quote(str(alias))+' -V','probe'),[str(alias),'-V'])
        (build/'lib/math').mkdir(parents=True)
        (build/'all.rsp').write_text('@outer.rsp\n--out-dir\nread-only-output\n--emit=obj=read-only.o\n'+str(ROOT/'lib/math/int_log_rust.rs')+'\n-Dunsafe_op_in_unsafe_fn\n')
        saved=build/'lib/math/.int_log_rust.o.cmd'
        saved.write_text('savedcmd_x := '+shlex.join(self.rustc)+' @all.rsp\n')
        _,flags=native_rust_arguments(build)
        self.assertIn('-Dwarnings',flags);self.assertIn('-Dunsafe_op_in_unsafe_fn',flags)
        self.assertFalse(any('read-only' in flag or flag.startswith(('--out-dir','--emit=','--extern')) for flag in flags))
        for bad in ('','savedcmd_x := ','savedcmd_x := "unterminated','savedcmd_x := rustc @missing.rsp'):
            saved.write_text(bad+'\n')
            with self.subTest(saved=bad),self.assertRaises((ValueError,OSError)): native_rust_arguments(build)

    def test_private_cwd_temp_outputs_and_preflight(self):
        trap=self.work/'forbidden-temp';trap.mkdir();watch=WriteWatch([trap])
        code='import os,pathlib,tempfile;assert all(os.environ[k]==os.getcwd() for k in ("TMPDIR","TMP","TEMP"));p=pathlib.Path("transient");p.write_text("x");p.unlink();f=tempfile.NamedTemporaryFile();raise SystemExit(7)'
        try:
            result=run([sys.executable,'-c',code],env=dict(os.environ,TMPDIR=str(trap),TMP=str(trap),TEMP=str(trap)),expected=7)
            self.assertEqual(result.returncode,7)
            before=set(self.logs.iterdir())
            for argv,cwd in (([],None),(['/missing/compiler'],None),([sys.executable],ROOT),
                             ([sys.executable,'--out-dir',str(ROOT)],None),([sys.executable,'-o',str(ROOT/'forbidden.o')],None)):
                with self.subTest(argv=argv),self.assertRaises(ValueError): run(argv,cwd=cwd)
            self.assertEqual(set(self.logs.iterdir()),before)
        finally: events=watch.finish()
        self.assertEqual(events,[])
        private=[row for p in self.logs.glob('command-*/write-events.json') for row in json.loads(p.read_text())]
        self.assertTrue(any(row[2]=='transient' and row[1]=='0x100' for row in private))
        self.assertTrue(any(row[2]=='transient' and row[1]=='0x200' for row in private))

    def test_explicit_invalid_inputs_before_work_or_skips(self):
        for name,value in (('BASE64_LOGS',str(ROOT)),('BASE64_LOGS',str(OWNER/'child')),
                           ('BASE64_X86_BUILD',''),('BASE64_ARM64_BUILD','/missing/native'),
                           ('INT_MATH_I686_SYSROOT',''),('INT_MATH_I686_RUNNER','"unterminated'),
                           ('HOSTCC',''),('HOSTRUSTC','/missing/compiler'),('HOSTRUSTC','true'),
                           ('BINDGEN','"" -arg'),('BASE64_UNKNOWN','bad')):
            with self.subTest(name=name,value=value),patch.dict(os.environ,{name:value}),patch.object(Path,'mkdir') as mkdir,patch.object(tempfile,'mkdtemp') as mkdtemp:
                with self.assertRaises(ValueError): settings()
                mkdir.assert_not_called();mkdtemp.assert_not_called()
        alias=self.work/'source alias';alias.symlink_to(ROOT,target_is_directory=True)
        with patch.dict(os.environ,BASE64_LOGS=str(alias/'logs')),self.assertRaises(ValueError): settings()

    def test_bounded_concurrent_native_transport(self):
        donors=[(variable,arch,Path(os.environ[variable]).resolve()) for variable,arch in
                (('BASE64_X86_BUILD','x86'),('BASE64_ARM64_BUILD','arm64')) if variable in os.environ]
        if not donors: self.skipTest('native inputs absent; concurrent compiler transport unproven')
        directories=sorted({Path(p) for _,_,donor in donors for p,_,_ in os.walk(donor)})
        watch=WriteWatch(directories);jobs=[];streams=[]
        try:
            for variable,arch,_ in donors:
                for index in range(2):
                    out=self.logs/(arch+'-'+str(index));out.mkdir()
                    stream=(out/'worker.log').open('x');streams.append(stream)
                    env=dict(os.environ,BASE64_LOGS=str(out),PYTHONDONTWRITEBYTECODE='1',TMPDIR=str(out),TMP=str(out),TEMP=str(out))
                    job=subprocess.Popen([sys.executable,str(Path(__file__).resolve()),'--base64-transport-worker',variable,arch,str(out)],cwd=self.work,env=env,stdout=stream,stderr=subprocess.STDOUT)
                    jobs.append((job,out))
            deadline=time.monotonic()+30
            while not all((out/'ready').is_file() for _,out in jobs):
                if time.monotonic()>deadline or any(job.poll() is not None for job,_ in jobs): self.fail('bounded compiler barrier failed; see worker.log')
                time.sleep(0.01)
            (self.logs/'start-replays').write_text('release real native compiler workers\n')
            for job,out in jobs: self.assertEqual(job.wait(timeout=120),0,str(out/'worker.log'))
        finally:
            for job,_ in jobs:
                if job.poll() is None: job.kill();job.wait()
            for stream in streams: stream.close()
            events=watch.finish()
            (self.logs/'all-donor-events.json').write_text(json.dumps(dict(directories=len(directories),events=events))+'\n')
        self.assertEqual(events,[])
        intervals=[];transients=[]
        for _,out in jobs:
            private=[row for p in out.rglob('write-events.json') for row in json.loads(p.read_text())]
            created={row[2] for row in private if row[1]=='0x100' and row[2].endswith('.rcgu.o')}
            deleted={row[2] for row in private if row[1]=='0x200' and row[2].endswith('.rcgu.o')}
            self.assertTrue(created&deleted,'real private Rust intermediate creation/deletion required')
            transients.append((out.name,sorted(created&deleted)))
            for p in out.rglob('interval.json'):
                argv,start,stop=json.loads(p.read_text())
                if Path(argv[0]).name in ('rustc','clang'): intervals.append((out.name,argv[0],start,stop))
        overlaps=[(a,b) for a in intervals for b in intervals if a[0]!=b[0] and max(a[2],b[2])<min(a[3],b[3])]
        self.assertTrue(overlaps,'actual compiler invocation intervals must overlap')
        (self.logs/'concurrent-proof.json').write_text(json.dumps(dict(intervals=intervals,overlaps=overlaps,private_transients=transients),indent=2)+'\n')


def bounded_transport_worker(variable,architecture,out):
    options=settings()
    if (variable,architecture) not in (('BASE64_X86_BUILD','x86'),('BASE64_ARM64_BUILD','arm64')) or variable not in os.environ:
        raise ValueError('invalid bounded native worker')
    if not out.is_dir() or any(out.is_relative_to(p) for p in options['protected']): raise ValueError('worker output must be private')
    setUpModule()
    case=Base64NativeConfigurationTests()
    try:
        case.setUp()
        (out/'ready').write_text('real native worker ready\n')
        deadline=time.monotonic()+30
        while not (out.parent/'start-replays').is_file():
            if time.monotonic()>deadline: raise RuntimeError('bounded native compiler barrier timeout')
            time.sleep(0.01)
        case.check_native(variable,architecture)
    finally:
        clean=case.doCleanups();unittest.doModuleCleanups()
        if not clean: raise AssertionError('bounded native worker cleanup failed')


if __name__ == "__main__":
    if len(sys.argv)==5 and sys.argv[1]=='--base64-transport-worker':
        bounded_transport_worker(sys.argv[2],sys.argv[3],Path(sys.argv[4]).resolve())
    else:
        unittest.main()
