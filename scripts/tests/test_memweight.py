#!/usr/bin/env python3
"""Original-C differential and genuine native flags replay; no kernel build.

Run with --source-root (original Linux tree), --rustc and optional --native.
Or use unittest discovery with explicit MEMWEIGHT_* environment inputs.
Native inputs are read-only. Default workspaces and logs are cleaned after the
suite; explicit --logs/MEMWEIGHT_LOGS preserves evidence in a unique run folder.
"""
import argparse
import ctypes
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import struct
import tempfile
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]


def extract_crc(output):
    """Require one genuine extraction record; never invent a version value."""
    records = re.findall(r'^#SYMVER memweight (0x[0-9a-fA-F]{8})$', output, re.M)
    if len(records) != 1:
        raise ValueError('invalid genuine memweight CRC extraction: '+output)
    return records[0]


def configuration(argv=None):
    """CLI options override MEMWEIGHT_* inputs; explicit empty inputs are errors.

    Discovery reads MEMWEIGHT_SOURCE_ROOT, RUSTC, CC, NM, READELF, MAKE,
    SYSROOT32, NATIVE (os.pathsep-separated directories), and LOGS. All names
    have the MEMWEIGHT_ prefix. Omitted native/sysroot inputs skip only their
    corresponding groups. Rust must be at least 1.85; every compile uses
    -Dwarnings, and native replay retains the genuine saved flags.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('source-root', 'rustc', 'sysroot32', 'logs', 'cc', 'nm', 'readelf', 'make'):
        parser.add_argument('--'+name)
    parser.add_argument('--native', action='append')
    args = parser.parse_args([] if argv is None else argv)

    def value(name, default=None):
        supplied = getattr(args, name.replace('-', '_'))
        if supplied is None:
            supplied = os.environ.get('MEMWEIGHT_'+name.upper().replace('-', '_'), default)
        if supplied is not None and not str(supplied).strip():
            raise ValueError('invalid empty '+name+' input')
        return supplied

    args.source_root = Path(value('source-root', str(ROOT))).resolve()
    for name in ('memweight.c', 'bitmap.c', 'hweight.c'):
        if not (args.source_root / 'lib' / name).is_file():
            raise ValueError('invalid source input: missing lib/'+name)
    for name, default in (('rustc', 'rustc'), ('cc', 'cc'), ('nm', 'nm'),
                          ('readelf', 'readelf'), ('make', 'make')):
        raw = value(name, default)
        executable = shutil.which(raw)
        if executable is None:
            raise ValueError('invalid '+name+' executable: '+raw)
        setattr(args, name, str(Path(executable).absolute()))
    version = subprocess.run([args.rustc, '--version'], check=True,
                             text=True, capture_output=True).stdout
    match = re.match(r'rustc (\d+)\.(\d+)\.', version)
    if not match or tuple(map(int, match.groups())) < (1, 85):
        raise ValueError('rustc 1.85 or newer required: '+version)
    raw = value('sysroot32')
    args.sysroot32 = Path(raw).resolve() if raw is not None else None
    if args.sysroot32 is not None and not (args.sysroot32 /
            'lib/rustlib/i686-unknown-linux-gnu/lib/libcore.rlib').is_file():
        raise ValueError('invalid 32-bit sysroot input')
    native = args.native
    if native is None:
        raw = os.environ.get('MEMWEIGHT_NATIVE')
        native = [] if raw is None else raw.split(os.pathsep)
    args.native = []
    for raw in native:
        if not raw.strip():
            raise ValueError('invalid empty native input')
        path = Path(raw).resolve()
        if not path.is_dir() or not list((path / 'lib').glob('.*_rust.o.cmd')):
            raise ValueError('invalid native input: '+raw)
        for required in ('lib/.scatterlist.o.cmd', 'scripts/gendwarfksyms/gendwarfksyms'):
            if not (path / required).is_file():
                raise ValueError('invalid native input: missing '+required)
        args.native.append(path)
    raw = value('logs')
    logs = Path(raw).resolve() if raw is not None else None
    protected = [args.source_root, *args.native]
    if args.sysroot32 is not None:
        protected.append(args.sysroot32)
    if logs is not None:
        if any(logs.is_relative_to(path) for path in protected):
            raise ValueError('logs must be outside source/native/sysroot inputs')
        for parent in (logs, *logs.parents):
            if parent.exists() and not parent.is_dir():
                raise ValueError('invalid logs directory: '+str(logs))
    # TMPDIR can itself point into the checkout or a native donor. Select an
    # external parent before creating anything, keeping all fixtures ephemeral.
    # Do not call gettempdir(): its directory probes can write into TMPDIR
    # before we have rejected a source/native location.
    parents = [Path(os.environ[name]) for name in ('TMPDIR', 'TEMP', 'TMP')
               if os.environ.get(name)] + [Path('/tmp'), Path('/var/tmp')]
    parent = next((path.resolve() for path in parents
                   if path.is_dir() and not any(path.resolve().is_relative_to(p)
                       for p in [ROOT, *protected])), None)
    if parent is None:
        raise ValueError('no temporary directory outside source/native inputs')
    args.workspace = tempfile.TemporaryDirectory(prefix='memweight-', dir=parent)
    try:
        args.work = Path(args.workspace.name)
        if logs is None:
            args.logs = args.work / 'logs'
            args.logs.mkdir()
        else:
            logs.mkdir(parents=True, exist_ok=True)
            # Reruns must not overwrite earlier command/failure evidence.
            args.logs = Path(tempfile.mkdtemp(prefix='run-', dir=logs))
        (args.logs / 'inputs.txt').write_text(repr(vars(args))+'\n'+version)
        print('memweight evidence:', args.logs,
              '(temporary)' if logs is None else '(preserved)', flush=True)
    except BaseException:
        args.workspace.cleanup()
        raise
    return args


def run_group(args, group):
    src = args.source_root
    args = argparse.Namespace(**vars(args))
    args.logs = args.logs / group
    args.logs.mkdir()
    serial = 0
    failures = []

    def run(cmd, cwd=None, env=None, input_text=None, expect_failure=False):
        nonlocal serial
        serial += 1
        result = subprocess.run(cmd, cwd=cwd, env=env, text=True,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, input=input_text)
        (args.logs / f'{serial:03}.log').write_text(
            shlex.join(map(str, cmd)) + '\n' + result.stdout + f'\nexit={result.returncode}\n')
        if expect_failure:
            assert result.returncode != 0, f'negative control unexpectedly passed: {serial:03}.log'
        else:
            assert result.returncode == 0, f'command failed: {serial:03}.log'
        return result.stdout

    with tempfile.TemporaryDirectory(prefix=group+'-', dir=args.work) as td:
        tmp = Path(td)
        inc = tmp / 'include'
        (inc / 'linux').mkdir(parents=True)
        (inc / 'asm').mkdir()
        (inc / 'linux/export.h').write_text('#define EXPORT_SYMBOL(x)\n')
        (inc / 'asm/types.h').write_text('typedef unsigned long long __u64;\n')
        (inc / 'linux/bitops.h').write_text('''
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
unsigned int __sw_hweight8(unsigned int);
unsigned int __sw_hweight32(unsigned int);
unsigned long __sw_hweight64(unsigned long long);
#define hweight8 __sw_hweight8
#if __SIZEOF_LONG__ == 8
#define hweight_long __sw_hweight64
#else
#define hweight_long __sw_hweight32
#endif
''')
        (inc / 'linux/bug.h').write_text('#define BUG_ON(x) do { if (x) __builtin_trap(); } while (0)\n')
        (inc / 'linux/bitmap.h').write_text('''
#include <stddef.h>
#define INT_MAX __INT_MAX__
#include <linux/bitops.h>
unsigned int __bitmap_weight(const unsigned long *, unsigned int);
#define bitmap_weight __bitmap_weight
''')
        bitmap = (src / 'lib/bitmap.c').read_text()
        begin = bitmap.index('#define BITMAP_WEIGHT(')
        end = bitmap.index('EXPORT_SYMBOL(__bitmap_weight);', begin)
        (tmp / 'bitmap.c').write_text('#include <linux/bitmap.h>\n'
            '#define BITMAP_LAST_WORD_MASK(n) (~0UL >> (-(n) & (BITS_PER_LONG-1)))\n'
            + bitmap[begin:end])
        wrapper = tmp / 'wrapper.rs'
        wrapper.write_text(f'''#[path = "{ROOT / 'lib/memweight.rs'}"] mod implementation;
extern "C" {{ fn __bitmap_weight(p: *const core::ffi::c_ulong, n: u32) -> u32; }}
#[no_mangle] pub unsafe extern "C" fn rust_memweight(p: *const u8, n: usize) -> usize {{
    unsafe {{ implementation::memweight_with(p,n, |p,n| __bitmap_weight(p,n)).unwrap() }}
}}
''')
        if group in ('host', 'boundary'):
            for opt in (('2',) if group == 'boundary' else ('0', '2', 's')):
                objs = []
                for name, source in [('hweight', src / 'lib/hweight.c'), ('bitmap', tmp / 'bitmap.c')]:
                    obj = tmp / f'{name}.o'
                    run([args.cc, '-fPIC', f'-O{opt}', '-I', str(inc), '-c', str(source), '-o', str(obj)])
                    objs.append(obj)
                oracle = tmp / f'oracle-{opt}.so'
                run([args.cc, '-shared', '-fPIC', f'-O{opt}', '-I', str(inc),
                     str(src / 'lib/memweight.c'), *map(str, objs), '-o', str(oracle)])
                rust = tmp / f'rust-{opt}.so'
                run([str(args.rustc), '--edition=2021', '-Dwarnings', '--crate-type=cdylib',
                     '-C', f'opt-level={opt}', str(wrapper), *[f'-Clink-arg={o}' for o in objs], '-o', str(rust)])
                c = ctypes.CDLL(str(oracle)).memweight
                r = ctypes.CDLL(str(rust)).rust_memweight
                for fn in (c, r):
                    fn.argtypes = [ctypes.c_void_p, ctypes.c_size_t]
                    fn.restype = ctypes.c_size_t
                data = (ctypes.c_ubyte * 4104)()
                state = 0x12345678
                for i in range(len(data)):
                    state = (state * 1664525 + 1013904223) & 0xffffffff
                    data[i] = state >> 24
                for offset in range(8):
                    for length in [*range(257), 511, 1023, 4096]:
                        p = ctypes.addressof(data) + offset
                        assert c(p, length) == r(p, length), (opt, offset, length)
                assert c(None, 0) == r(None, 0) == 0
            print('PASS original C '+('O2' if group == 'boundary' else 'O0/O2/Os')+' 64-bit differential')
        if group == 'boundary':
            driver = tmp / 'boundary.py'
            driver.write_text("""
import ctypes
import sys
c = ctypes.CDLL(sys.argv[1]).memweight
r = ctypes.CDLL(sys.argv[2]).rust_memweight
for fn in (c, r):
    fn.argtypes = [ctypes.c_void_p, ctypes.c_size_t]
    fn.restype = ctypes.c_size_t
word = ctypes.sizeof(ctypes.c_ulong)
assert word == 8, '16-case boundary group requires a 64-bit host'
limit = (0x7fffffff // (8 * word)) * word
data = (ctypes.c_ubyte * (limit + 2*word))()
assert ctypes.addressof(data) % word == 0
ctypes.memset(data, 255, len(data))
count = 0
for offset in range(word):
    prefix = (word-offset) % word
    for length in (limit+prefix-1, limit+prefix-word):
        p = ctypes.addressof(data)+offset
        cv = c(p, length)
        print('original C', offset, length, cv, flush=True)
        rv = r(p, length)
        assert cv == rv == length*8, (offset, length, cv, rv)
        print('PASS boundary', offset, length, cv, rv, flush=True)
        count += 1
assert count == 16
print('PASS all 16 real-readable-allocation alignment/threshold cases')
""")
            run([sys.executable, str(driver), str(oracle), str(rust)])
            original = (ROOT / 'lib/memweight.rs').read_text()
            guard = '        if longs >= (i32::MAX as usize) / BITS {\n            return None;\n        }\n'
            assert original.count(guard) == 1
            mutant = tmp / 'early_bug.rs'
            mutant.write_text(original.replace(guard, '').replace(
                '    while bytes != 0 &&',
                '    let longs = bytes / WORD;\n'
                '    if longs >= (i32::MAX as usize) / BITS { return None; }\n'
                '    while bytes != 0 &&', 1))
            check = tmp / 'early_bug_wrapper.rs'
            check.write_text(wrapper.read_text().replace(str(ROOT / 'lib/memweight.rs'), str(mutant)))
            badlib = tmp / 'early_bug.so'
            run([str(args.rustc), '--edition=2021', '-Dwarnings', '--crate-type=cdylib',
                 '-Copt-level=2', str(check), *[f'-Clink-arg={o}' for o in objs], '-o', str(badlib)])
            output = run([sys.executable, str(driver), str(oracle), str(badlib)], expect_failure=True)
            assert 'original C 1 268435454 2147483632' in output
            assert 'Option::unwrap()' in output, 'early-BUG mutant failed for an unexpected reason'
            print('PASS all 16 alignment/threshold cases; early-BUG movement mutant rejected')
        if group == 'contracts':
            contract = tmp / 'contract.rs'
            contract.write_text(f'''#[path = "{ROOT / 'lib/memweight.rs'}"] mod implementation;
    fn main() {{
        let words = [0xffffffffffffffffu64; 4];
        let p = words.as_ptr().cast::<u8>();
        let word = core::mem::size_of::<core::ffi::c_ulong>();
        let limit = (i32::MAX as usize / (word * 8)) * word;
        unsafe {{
            assert_eq!(implementation::memweight_with(p, limit, |_,_| panic!("threshold called helper")), None);
            assert_eq!(implementation::memweight_with(p, limit+word, |_,_| panic!("threshold called helper")), None);
            let mut calls = 0;
            assert_eq!(implementation::memweight_with(p.add(1), 2*word, |q,n| {{
                calls += 1; assert_eq!(q as usize % word, 0); assert_eq!(n, (word*8) as u32);
                u32::MAX
            }}), Some(u32::MAX as usize + word*8));
            assert_eq!(calls, 1);
        assert_eq!(implementation::memweight_with(p, 0, |_,_| panic!()), Some(0));
        assert_eq!(implementation::memweight_with(core::ptr::null(), 0, |_,_| panic!()), Some(0));
        assert_eq!(implementation::memweight_with(core::ptr::null(), limit, |_,_| panic!()), None);
        }}
    }}
    ''')
            run([str(args.rustc), '--edition=2021', '-Dwarnings', str(contract), '-o', str(tmp / 'contract')])
            run([str(tmp / 'contract')])
            print('PASS unsigned callback result, alignment, threshold and zero contracts')
            for label, old, new in (
                ('threshold', 'longs >= (i32::MAX', 'longs > (i32::MAX'),
                ('signed', 'weight(ptr.cast(), (longs * BITS) as u32) as usize',
                 'weight(ptr.cast(), (longs * BITS) as u32) as i32 as usize'),
            ):
                mutated = tmp / (label+'.rs')
                original = (ROOT / 'lib/memweight.rs').read_text()
                assert old in original
                mutated.write_text(original.replace(old, new))
                check = tmp / (label+'_check.rs')
                check.write_text(contract.read_text().replace(str(ROOT / 'lib/memweight.rs'), str(mutated)))
                exe = tmp / (label+'_check')
                run([str(args.rustc), '--edition=2021', '-Dwarnings', str(check), '-o', str(exe)])
                run([str(exe)], expect_failure=True)
            print('PASS negative controls: off-by-one BUG threshold and signed narrowing rejected')
        if group == 'elf32':
            if args.sysroot32 is not None:
                assert (args.sysroot32 / 'lib/rustlib/i686-unknown-linux-gnu/lib/libcore.rlib').is_file(), 'invalid 32-bit sysroot'
                driver = tmp / 'driver.c'
                driver.write_text('''
    #include <stddef.h>
    extern size_t memweight(const void *, size_t);
    extern size_t rust_memweight(const void *, size_t);
    static unsigned char data[4104];
    void _start(void) {
     unsigned int state=0x12345678; int status=0;
     for(unsigned i=0;i<sizeof(data);i++) { state=state*1664525+1013904223; data[i]=state>>24; }
     for(unsigned off=0;off<4;off++) for(unsigned n=0;n<=4096;n++)
      if(memweight(data+off,n)!=rust_memweight(data+off,n)) status=1;
     __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
     __builtin_unreachable();
    }
    ''')
                bare = tmp / 'bare.rs'
                bare.write_text('#![no_std]\n' + wrapper.read_text() + '''
    #[panic_handler] fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }
    ''')
                for opt in ('0', '2', 's'):
                    robj = tmp / f'rust32-{opt}.a'
                    run([str(args.rustc), '--edition=2021', '-Dwarnings', '--target=i686-unknown-linux-gnu',
                         '--sysroot='+str(args.sysroot32), '--crate-type=staticlib', '-Cpanic=abort',
                         '-C', f'opt-level={opt}', str(bare), '-o', str(robj)])
                    exe = tmp / f'diff32-{opt}'
                    run([args.cc, '-m32', '-nostdlib', '-static', '-fno-pie', '-fno-stack-protector', '-Wl,--gc-sections', f'-O{opt}',
                         '-I', str(inc), str(driver), str(src / 'lib/memweight.c'), str(src / 'lib/hweight.c'),
                         str(tmp / 'bitmap.c'), str(robj), '-o', str(exe)])
                    assert exe.read_bytes()[:6] == b'\x7fELF\x01\x01', 'expected real little-endian ELF32'
                    shutil.copyfile(exe, args.logs / exe.name)
                    try:
                        run([str(exe)])
                    except AssertionError:
                        failures.append('ELF32 runtime O'+opt+' failed (including sandbox SIGSYS); rerun saved executable unsandboxed')
                print('ELF32 O0/O2/Os build and execution attempted; see logs')
            else:
                print('SKIP optional ELF32 differential: no --sysroot32 supplied')
        if group == 'native':
            for native in args.native:
                assert native.is_dir(), 'invalid native input'
                donors = list((native / 'lib').glob('.*_rust.o.cmd'))
                assert donors, 'native input has no saved Rust flags'
                donor = next((p for p in donors if p.name == '.bcd_rust.o.cmd'), donors[0])
                raw = donor.read_text().splitlines()[0].split(' := ', 1)[1]
                shutil.copyfile(donor, args.logs / (native.name+'-rust-donor.cmd'))
                tokens = shlex.split(raw)
                env = dict(os.environ, RUSTC_BOOTSTRAP='1')
                while '=' in tokens[0] and not tokens[0].startswith('/'):
                    key, value = tokens.pop(0).split('=', 1)
                    env[key] = value
                env['RUST_MODFILE'] = 'lib/memweight_rust'
                assert Path(tokens[0]).resolve() == Path(args.rustc).resolve(), 'native donor compiler differs from explicit --rustc'
                (args.logs / (native.name+'-environment.txt')).write_text(
                    '\n'.join(key+'='+env[key] for key in ('RUSTC_BOOTSTRAP','RUST_MODFILE','OBJTREE'))+'\n')
                overlay = tmp / native.name
                (overlay / 'lib').mkdir(parents=True)
                for name in ('memweight.rs', 'memweight_rust.rs'):
                    shutil.copyfile(ROOT / 'lib' / name, overlay / 'lib' / name)
                (overlay / 'rust').mkdir()
                shutil.copyfile(src / 'rust/ffi_export.rs', overlay / 'rust/ffi_export.rs')
                (overlay / 'include/linux').mkdir(parents=True)
                shutil.copyfile(src / 'include/linux/export_header.rs', overlay / 'include/linux/export_header.rs')
                for i, token in enumerate(tokens):
                    if token == '--out-dir': tokens[i+1] = str(overlay)
                    elif token.startswith('--emit=dep-info='): tokens[i] = '--emit=dep-info=' + str(overlay / 'memweight.d')
                    elif token.startswith('--emit=obj='): tokens[i] = '--emit=obj=' + str(overlay / 'memweight.o')
                    elif token.endswith('.rs'): tokens[i] = str(overlay / 'lib/memweight_rust.rs')
                assert '-Dwarnings' in tokens
                run(tokens, cwd=native, env=env)
                obj = overlay / 'memweight.o'
                nm = run([args.nm, str(obj)])
                assert re.search(r' T memweight$', nm, re.M)
                assert '__bitmap_weight' in nm and 'rust_helper_BUG' in nm
                assert '__export_symbol_memweight' in nm
                sections = run([args.readelf, '-SW', str(obj)])
                assert '.debug_info' in sections and '.export_symbol' in sections
                shutil.copyfile(obj, args.logs / (native.name + '-memweight.o'))
                # Recompile the actual original C using an always-C donor, retaining
                # all target, sanitizer, include, optimization and debug flags.
                cdonor = native / 'lib/.scatterlist.o.cmd'
                assert cdonor.is_file(), 'missing always-C donor'
                shutil.copyfile(cdonor, args.logs / (native.name+'-c-donor.cmd'))
                ctokens = shlex.split(cdonor.read_text().splitlines()[0].split(' := ', 1)[1])
                # Commands after the compiler are kernel objtool/fixdep operations;
                # the compiler itself is replayed without writing the native tree.
                if ';' in ctokens: ctokens = ctokens[:ctokens.index(';')]
                cobj = overlay / 'original.o'
                for i, token in enumerate(ctokens):
                    if token == '-o': ctokens[i+1] = str(cobj)
                    elif token.startswith('-Wp,-MMD,'): ctokens[i] = '-Wp,-MMD,' + str(overlay / 'original.d')
                    elif token.endswith('/lib/scatterlist.c'): ctokens[i] = str(src / 'lib/memweight.c')
                    elif token.startswith('-DKBUILD_BASENAME='): ctokens[i] = '-DKBUILD_BASENAME="memweight"'
                    elif token.startswith('-DKBUILD_MODNAME='): ctokens[i] = '-DKBUILD_MODNAME="memweight"'
                    elif token.startswith('-D__KBUILD_MODNAME='): ctokens[i] = '-D__KBUILD_MODNAME=kmod_memweight'
                run(ctokens, cwd=native)
                cnm = run([args.nm, str(cobj)])
                def kcfi(path, symbols):
                    match = re.search(r'^([0-9a-f]+) T memweight$', symbols, re.M)
                    assert match, 'memweight symbol missing'
                    address = int(match[1], 16)
                    assert address >= 4, 'KCFI preamble missing'
                    data = path.read_bytes()
                    assert data[:6] == b'\x7fELF\x02\x01'
                    shoff = struct.unpack_from('<Q', data, 40)[0]
                    entsize, count, namesidx = struct.unpack_from('<HHH', data, 58)
                    headers = [struct.unpack_from('<IIQQQQIIQQ', data, shoff+i*entsize) for i in range(count)]
                    names = headers[namesidx]
                    strings = data[names[4]:names[4]+names[5]]
                    for h in headers:
                        name = strings[h[0]:].split(b'\0', 1)[0]
                        if name == b'.text':
                            return data[h[4]+address-4:h[4]+address]
                    raise AssertionError('missing .text')
                assert kcfi(obj, nm) == kcfi(cobj, cnm), 'C/Rust KCFI mismatch'
                adapter = overlay / 'lib/memweight_rust.rs'
                correct = adapter.read_text()
                adapter.write_text(correct.replace('bytes: usize) -> usize', 'bytes: usize) -> u32')
                                   .replace('Some(result) => result,', 'Some(result) => result as u32,'))
                badobj = overlay / 'wrong-signature.o'
                badtokens = [('--emit=obj='+str(badobj)) if t.startswith('--emit=obj=') else t for t in tokens]
                run(badtokens, cwd=native, env=env)
                badnm = run([args.nm, str(badobj)])
                assert kcfi(badobj, badnm) != kcfi(cobj, cnm), 'signature negative control passed incorrectly'
                adapter.write_text(correct)
                shutil.copyfile(cobj, args.logs / (native.name + '-original.o'))
                dwarf = native / 'scripts/gendwarfksyms/gendwarfksyms'
                assert dwarf.is_file(), 'missing genuine DWARF version tool'
                rversion = run([str(dwarf), '--symtypes', str(args.logs / (native.name+'-rust.symtypes')), str(obj)], input_text='memweight\n')
                cversion = run([str(dwarf), '--symtypes', str(args.logs / (native.name+'-c.symtypes')), str(cobj)], input_text='memweight\n')
                rust_crc, c_crc = extract_crc(rversion), extract_crc(cversion)
                print('Genuine DWARF CRCs:', native, 'Rust', rust_crc, 'C', c_crc,
                      '; dependent modules must be rebuilt when switching implementation')
                (args.logs / (native.name+'-crc-policy.txt')).write_text(
                    'Rust '+rust_crc+'; C '+c_crc+'\n'
                    'Genuine CRC differences are permitted; rebuild dependent modules.\n'
                    'Module rebuild/modpost validation remains a root integration gate.\n')
                print('PASS genuine native binding replay:', native)
                if 'x86' in native.name:
                    # Use genuine Makefile.build recipes/fixdep in a private output.
                    # Scope the target to the owner; all component ordering remains
                    # evaluated from the complete production component Makefile.
                    build = overlay / 'build'
                    (build / 'lib').mkdir(parents=True)
                    (build / 'scripts/basic').mkdir(parents=True)
                    shutil.copy2(native / 'scripts/basic/fixdep', build / 'scripts/basic/fixdep')
                    shutil.copyfile(ROOT / 'lib/Makefile', overlay / 'lib/Makefile')
                    shutil.copyfile(src / 'lib/memweight.c', overlay / 'lib/memweight.c')
                    def absolute_flags(values):
                        result = []
                        for value in values:
                            if value.startswith('-I./'): value = '-I' + str(native / value[2:])
                            elif value.startswith('@./'): value = '@' + str(native / value[1:])
                            elif value.startswith('--target=./'): value = '--target=' + str(native / value.split('=', 1)[1])
                            elif value.startswith('./'): value = str(native / value)
                            result.append(value)
                        return result
                    rflags = []
                    skip = False
                    for value in tokens:
                        if skip: skip = False; continue
                        if value == '--out-dir': skip = True; continue
                        if value.startswith('--emit=') or value.endswith('.rs'): continue
                        rflags.append(value)
                    cflags = []
                    skip = False
                    for value in ctokens:
                        if skip: skip = False; continue
                        if value == '-o': skip = True; continue
                        if value.startswith('-Wp,-MMD,') or value.endswith('.c'): continue
                        cflags.append(value)
                    rules = build / 'rules.mk'
                    rules.write_text('include ' + str(src / 'scripts/Makefile.build') + '\n'
                        'rust_common_cmd = ' + shlex.join(absolute_flags(rflags)) + ' --out-dir $(dir $@) --emit=dep-info=$(depfile)\n'
                        'cmd_cc_o_c = ' + shlex.join(absolute_flags(cflags)) + ' -Wp,-MMD,$(depfile) -o $@ $<\n'
                        '$(info MEMWEIGHT_ORDER=$(real-obj-y))\n'
                        '.PHONY: selected\nselected: $(filter lib/memweight.o lib/memweight_rust.o,$(real-obj-y))\n')
                    base = [args.make, '--no-print-directory', '-f', str(rules), 'selected', 'obj=lib',
                            'srctree='+str(src), 'srcroot='+str(overlay), 'objtree='+str(build), 'VPATH='+str(overlay)]
                    for selection in ('n', 'y', 'n'):
                        command = base + ['CONFIG_RUST_MEMWEIGHT='+selection]
                        output = run(command, cwd=build, env=env)
                        owner = 'memweight_rust.o' if selection == 'y' else 'memweight.o'
                        order = re.search(r'MEMWEIGHT_ORDER=(.*)', output).group(1).split()
                        position = order.index('lib/'+owner)
                        assert order[position-1:position+2] == ['lib/lwq.o', 'lib/'+owner, 'lib/kfifo.o']
                        assert sum(x in order for x in ['lib/memweight.o','lib/memweight_rust.o']) == 1
                        built = build / 'lib' / owner
                        before = built.stat().st_mtime_ns
                        run(command, cwd=build, env=env)
                        assert built.stat().st_mtime_ns == before, 'Kbuild no-op rebuilt'
                        if selection == 'y':
                            for dep in ('lib/memweight.rs', 'lib/memweight_rust.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs'):
                                path = overlay / dep
                                path.write_text(path.read_text()+'\n')
                                run(command, cwd=build, env=env)
                                after = built.stat().st_mtime_ns
                                assert after > before, 'dependency failed: '+dep
                                before = after
                                run(command, cwd=build, env=env)
                                assert built.stat().st_mtime_ns == before
                    shutil.copyfile(build / 'lib/.memweight_rust.o.cmd', args.logs / 'kbuild-memweight_rust.cmd')
                    print('PASS real private Kbuild C/Rust/C, original slot, dependencies and no-ops')
            if not args.native:
                print('SKIP optional genuine native replay: no --native supplied')
    assert not failures, '\n'.join(failures)


class MemweightTests(unittest.TestCase):
    """Discoverable semantic, mutation, architecture and Kbuild groups."""

    @classmethod
    def setUpClass(cls):
        cls.args = configuration(CLI_ARGS)
        cls.addClassCleanup(cls.args.workspace.cleanup)

    def test_original_c_differential(self):
        run_group(self.args, 'host')

    def test_alignment_threshold_boundaries(self):
        run_group(self.args, 'boundary')

    def test_callback_contracts_and_mutants(self):
        run_group(self.args, 'contracts')

    def test_crc_extraction_errors_are_fatal(self):
        # These are malformed parser inputs, never substitute production CRCs.
        for output in ('', 'tool failed', '#SYMVER other 0x12345678\n',
                       '#SYMVER memweight garbage\n',
                       '#SYMVER memweight 0x12345678\n' * 2):
            with self.subTest(output=output), self.assertRaises(ValueError):
                extract_crc(output)

    def test_real_elf32(self):
        if self.args.sysroot32 is None:
            self.skipTest('optional ELF32 differential: no sysroot32 supplied')
        run_group(self.args, 'elf32')

    def test_native_abi_versions_and_kbuild(self):
        if not self.args.native:
            self.skipTest('optional genuine native replay: no native supplied')
        run_group(self.args, 'native')


CLI_ARGS = None
if __name__ == '__main__':
    CLI_ARGS = sys.argv[1:]
    unittest.main(argv=[sys.argv[0]], verbosity=2)
