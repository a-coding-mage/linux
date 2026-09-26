#!/usr/bin/env python3
"""Recompile the complete sort candidate against unchanged original C.

All products and failure logs survive in a fresh private run directory. Missing
explicit input paths are errors, never silent skips. SORT_SHARED_PATCH applies
an unintegrated shared-source candidate only to private binding/Kbuild copies; omit it
after the shared selector and binding changes are integrated.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest

import glob_runtime_fixtures as transport
from check_glob_kernel import verify_flag_policy
from rust_exports_test_support import read_exports

ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = Path(os.environ.get('SORT_SOURCE', ROOT)).resolve()
FIXTURE = Path(__file__).with_name('sort_native')
RUST_DEFAULT = os.environ.get('HOSTRUSTC', shutil.which('rustc'))
BINDGEN_DEFAULT = os.environ.get('BINDGEN', shutil.which('bindgen') or shutil.which('bindgen-0.71'))


def input_path(name, default):
    value = os.environ.get(name, default)
    if not value or not Path(value).is_absolute() or not Path(value).exists():
        raise ValueError(f'{name}: invalid input {value!r}')
    return Path(value)


def ids(path):
    text = path.read_text()
    meta = {k: int(v) & 0xffffffff for k, v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}', text, re.M)}
    found = {k: meta[v] for k, v in re.findall(r'^define [^\n]*?@([^ (]+)\([^\n]*!kcfi_type !(\d+)', text, re.M)}
    for name, dest in re.findall(r'^@(\w+) = [^\n]*alias [^\n]*ptr @(\w+)$', text, re.M):
        if dest in found:
            found[name] = found[dest]
    return found


class SortNativeTests(unittest.TestCase):
    def setUp(self):
        # Validate every explicit path before writes or optional-input decisions.
        self.rust = input_path('SORT_RUSTC', RUST_DEFAULT)
        self.bindgen = input_path('SORT_BINDGEN', BINDGEN_DEFAULT)
        self.i686 = input_path('SORT_I686_SYSROOT', None) if 'SORT_I686_SYSROOT' in os.environ else None
        self.x86 = input_path('SORT_NATIVE_X86', None) if 'SORT_NATIVE_X86' in os.environ else None
        self.arm = input_path('SORT_NATIVE_ARM64', None) if 'SORT_NATIVE_ARM64' in os.environ else None
        self.shared_patch = input_path('SORT_SHARED_PATCH', None) if 'SORT_SHARED_PATCH' in os.environ else None
        if self.i686 is not None:
            for lib in ('libcore', 'libcompiler_builtins'):
                found = {p.resolve() for pattern in (lib+'.rlib', lib+'-*.rlib')
                         for p in (self.i686/'lib/rustlib/i686-unknown-linux-gnu/lib').glob(pattern)}
                if len(found) != 1: raise ValueError('invalid genuine i686 sysroot: '+lib)
        for donor in (self.x86, self.arm):
            if donor is not None:
                for rel in ('lib/.scatterlist.o.cmd', 'rust/.bindings.o.cmd',
                            'rust/bindings/.bindings_generated.rs.cmd', 'include/generated/rustc_cfg'):
                    if not (donor/rel).is_file(): raise ValueError('invalid native donor: '+rel)
        self.out = Path(tempfile.mkdtemp(prefix='sort-run-', dir='/tmp'))
        print('sort evidence: '+str(self.out), flush=True)
        self.log = (self.out / 'commands.log').open('w')
        self.addCleanup(self.log.close)
        self.env = {**os.environ, 'RUSTC_BOOTSTRAP': '1', 'TMPDIR': str(self.out), 'TMP': str(self.out), 'TEMP': str(self.out)}

    def run_cmd(self, args, expected=0, env=None, input=None):
        args = list(map(str, args))
        self.log.write('$ ' + shlex.join(args) + '\n')
        self.log.flush()
        p = subprocess.run(args, cwd=self.out, env=env or self.env, text=True,
                           capture_output=True, timeout=90, input=input,
                           preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        self.log.write(p.stdout + p.stderr + '\nexit=' + str(p.returncode) + '\n')
        self.log.flush()
        if expected is not None:
            self.assertEqual(p.returncode, expected, f'{self.out}/commands.log\n{p.stderr}')
        return p

    def prepare(self):
        inc = self.out / 'include/linux'
        inc.mkdir(parents=True)
        shutil.copyfile(FIXTURE / 'types.h', inc / 'types.h')
        # Verify fixture callbacks against the unchanged original typedefs.
        source = (ORIGINAL / 'include/linux/types.h').read_text()
        for alias in ('cmp_func_t', 'cmp_r_func_t', 'swap_func_t', 'swap_r_func_t'):
            original = re.search(r'^typedef .*?\(\*' + alias + r'\).*?;', source, re.M)[0]
            fixture = re.search(r'^typedef .*?\(\*' + alias + r'\).*?;', (inc / 'types.h').read_text(), re.M)[0]
            self.assertEqual(re.sub(r'\s+', '', original), re.sub(r'\s+', '', fixture))
        (inc / 'export.h').write_text('#define EXPORT_SYMBOL(x)\n')
        (inc / 'sched.h').write_text('void cond_resched(void);\n')
        self.run_cmd([self.bindgen, ORIGINAL / 'include/linux/sort.h', '--use-core',
                      '--rust-target=1.85', '--ctypes-prefix=core::ffi', '--no-layout-tests',
                      '--allowlist-type=.*func_t', '--blocklist-function=.*',
                      '-o', self.out / 'generated.rs', '--', '-I' + str(inc.parent),
                      '-I' + str(ORIGINAL / 'include')])
        self.assertIn('Option<', (self.out / 'generated.rs').read_text())
        (self.out / 'adapter-module.rs').write_text(
            '#[path=' + json.dumps(str(ROOT / 'rust/bindings/sort.rs')) + ']\n'
            'mod sort_bindings;\npub use sort_bindings::*;\n')
        self.env.update(SORT_GENERATED=str(self.out / 'generated.rs'),
                        SORT_ADAPTER_MODULE=str(self.out / 'adapter-module.rs'))
        # Abort-only freestanding execution retains the real core/panic path;
        # discard unreachable unwind records instead of inventing a personality.
        (self.out / 'discard.lds').write_text('SECTIONS { /DISCARD/ : { *(.eh_frame*) '
            '*(.gcc_except_table*) *(.data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n')
        return ['-I' + str(inc.parent), '-I' + str(ORIGINAL / 'include')]

    def differential(self, arch):
        includes = self.prepare()
        summary = []
        for arch in (arch,):
            for opt in ('0', '2', 's'):
                for efficient in (False, True):
                    folder = self.out / f'{arch}-O{opt}-unaligned{int(efficient)}'
                    folder.mkdir()
                    c = ['clang', '-m64' if arch == 'x86_64' else '-m32', '-O' + opt,
                         '-ffreestanding', '-fno-builtin', '-fno-stack-protector', '-fno-pie',
                         '-mstackrealign', '-Wall', '-Wextra', '-Werror', *includes,
                         '-ffunction-sections', '-fdata-sections']
                    r = [self.rust, '--edition=2021', '-Dwarnings', '-Dunsafe-op-in-unsafe-fn',
                         '-Wmissing-docs', '-Wunreachable-pub', '-Wrust-2018-idioms',
                         '-Cpanic=abort', '-Copt-level=' + opt, '-Coverflow-checks=yes',
                         '-Crelocation-model=static']
                    if arch == 'i686':
                        r += ['--target=i686-unknown-linux-gnu', '--sysroot=' + str(self.i686)]
                    else:
                        c += ['-DCONFIG_64BIT']
                        r += ['--cfg=CONFIG_64BIT']
                    if efficient:
                        c += ['-DCONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS']
                        r += ['--cfg=CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS']
                    # Every matrix cell compiles original C and full Rust again.
                    renames = ['-D' + n + '=original_' + n for n in
                               ('sort', 'sort_r', 'sort_nonatomic', 'sort_r_nonatomic')]
                    self.run_cmd([*c, *renames, '-c', ORIGINAL / 'lib/sort.c', '-o', folder / 'original.o'])
                    self.run_cmd([*r, '--crate-type=staticlib', FIXTURE / 'owner.rs', '-o', folder / 'owner.a'])
                    self.run_cmd([*c, '-nostdlib', '-static', '-Wl,--gc-sections',
                                  '-Wl,-T,' + str(self.out / 'discard.lds'),
                                  FIXTURE / 'driver.c', folder / 'original.o', folder / 'owner.a',
                                  '-o', folder / 'differential'])
                    result = self.run_cmd([folder / 'differential'], expected=None)
                    summary.append({'arch': arch, 'opt': opt, 'efficient': efficient, 'exit': result.returncode})
                    (self.out / 'summary.json').write_text(json.dumps(summary, indent=2))
        self.assertTrue(all(row['exit'] == 0 for row in summary),
                        f'{self.out}/summary.json; SIGSYS is not a pass; retained ELF32 binaries require root execution')

    def test_full_original_c_64(self):
        self.differential('x86_64')

    def test_full_original_c_32(self):
        if self.i686 is None: self.skipTest('SORT_I686_SYSROOT absent; ELF32 execution unproven')
        self.differential('i686')

    def test_protected_callbacks(self):
        includes = self.prepare()
        report = []
        for opt in ('0', '2', 's'):
            for protected in (False, True):
                folder = self.out / f'kcfi-O{opt}-{int(protected)}'
                folder.mkdir()
                c = ['clang', '-O' + opt, '-ffreestanding', '-fno-builtin', '-fno-stack-protector',
                     '-fno-pie', '-mstackrealign', '-Wall', '-Wextra', '-Werror', *includes,
                     '-ffunction-sections', '-fdata-sections', '-DCONFIG_64BIT']
                r = [self.rust, '--edition=2021', '-Dwarnings', '-Dunsafe-op-in-unsafe-fn',
                     '-Wmissing-docs', '-Wunreachable-pub', '-Wrust-2018-idioms',
                     '-Cpanic=abort', '-Coverflow-checks=yes', '-Copt-level=' + opt,
                     '-Crelocation-model=static', '--cfg=CONFIG_64BIT']
                if protected:
                    c += ['-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers']
                    r += ['-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                renames = ['-D' + n + '=original_' + n for n in
                           ('sort', 'sort_r', 'sort_nonatomic', 'sort_r_nonatomic')]
                self.run_cmd([*c, *renames, '-c', ORIGINAL / 'lib/sort.c', '-o', folder / 'original.o'])
                self.run_cmd([*c, *renames, '-S', '-emit-llvm', ORIGINAL / 'lib/sort.c', '-o', folder / 'original.ll'])
                self.run_cmd([*c, '-S', '-emit-llvm', FIXTURE / 'driver.c', '-o', folder / 'driver.ll'])
                self.run_cmd([*r, '--crate-type=staticlib', FIXTURE / 'owner.rs', '-o', folder / 'owner.a'])
                self.run_cmd([*r, '--crate-type=rlib', '--emit=llvm-ir=' + str(folder / 'owner.ll'), FIXTURE / 'owner.rs'])
                if protected:
                    ci, ri = ids(folder / 'original.ll'), ids(folder / 'owner.ll')
                    for name in ('sort', 'sort_r', 'sort_nonatomic', 'sort_r_nonatomic'):
                        self.assertEqual(ci['original_' + name], ri[name])
                    callback_ids = ids(folder / 'driver.ll')
                    actual = {int(v) & 0xffffffff for v in re.findall(r'"kcfi"\(i32 (-?\d+)\)', (folder / 'owner.ll').read_text())}
                    self.assertTrue({callback_ids[n] for n in ('cmp_callback', 'cmpr_callback', 'swap_callback', 'swapr_callback')} <= actual)
                for negative in range(8):
                    binary = folder / f'control-{negative}'
                    self.run_cmd([*c, '-DNEGATIVE=' + str(negative), '-nostdlib', '-static', '-Wl,--gc-sections',
                                  '-Wl,-T,' + str(self.out / 'discard.lds'),
                                  FIXTURE / 'driver.c',
                                  folder / 'original.o', folder / 'owner.a', '-o', binary])
                    expected = -signal.SIGILL if protected and negative else 0
                    self.run_cmd([binary], expected=expected)
                    report.append({'opt': opt, 'protected': protected, 'negative': negative, 'exit': expected})
                    (self.out / 'protected-summary.json').write_text(json.dumps(report, indent=2))


    def saved(self, build, relative):
        return shlex.split((build/relative).read_text().splitlines()[0].split(' := ', 1)[1].split(' ; ', 1)[0].replace('$(pound)', '#'))

    def native_flags(self, build, relative, rust):
        words = self.saved(build, relative)
        while '=' in words[0] and not words[0].startswith('-'): words.pop(0)
        compiler = words.pop(0)
        if rust and Path(compiler).resolve() != self.rust.resolve():
            raise ValueError('selected compiler differs from native rustc')
        flags = transport.native_flags(words, build, 'rust' if rust else 'c')
        result = []
        iterator = iter(flags)
        for flag in iterator:
            if flag in ('--crate-name',): next(iterator)
            elif flag.startswith('--crate-name=') or flag == '-c': continue
            elif not flag.startswith('-') and flag.endswith(('.c', '.rs')): continue
            else: result.append(flag)
        verify_flag_policy(result, 'rust' if rust else 'c')
        return [str(self.rust) if rust else compiler, *result]

    def native(self, build):
        if build is None: self.skipTest('native donor absent; actual target proof unrun')
        watch = transport.NativeWriteWatch(build)
        try:
            with watch: self._native(build)
        finally:
            (self.out/'donor-observation.json').write_text(json.dumps(dict(
                root=str(build), directories=len(watch.paths), events=watch.events), indent=2)+'\n')
        self.assertEqual(watch.events, [], 'read-only native donor was modified')

    def _native(self, build):
        fixture = self.shared_source()
        r = self.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
        br = self.native_flags(build, 'rust/.bindings.o.cmd', True)
        c = self.native_flags(build, 'lib/.scatterlist.o.cmd', False)
        command = self.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        split = command.index('--')
        common = command.index('--rust-target')
        parameters = (fixture/'rust/bindgen_parameters').read_text()
        parameters = ' '.join(line for line in parameters.splitlines() if line and not line.startswith('#'))
        generated = self.out/'rust/bindings'; generated.mkdir(parents=True)
        header = self.out/'native.h'
        header.write_text('#include '+json.dumps(str(fixture/'rust/bindings/bindings_helper.h'))+'\n')
        options = command[common:split]
        options[options.index('-o')+1] = str(generated/'bindings_generated.rs')
        # Kbuild inserts bindgen_parameters into a shell command. Use that
        # actual expansion here too: shlex.split alone would hide missing
        # quotes around a parenthesized regular expression.
        self.run_cmd(['/bin/sh', '-c',
            shlex.join([str(self.bindgen), str(header)])+' '+parameters+' '+
            shlex.join([*options, '--', *transport.native_flags(command[split+1:], build, 'c')])])
        path=generated/'bindings_generated.rs'
        path.write_text(re.sub(r'pub const RUST_CONST_HELPER_([a-zA-Z0-9_]*)', r'pub const \1', path.read_text()))
        shutil.copyfile(build/'rust/bindings/bindings_helpers_generated.rs', generated/'bindings_helpers_generated.rs')
        source=self.out/'bindings'; shutil.copytree(ORIGINAL/'rust/bindings', source)
        shutil.copyfile(fixture/'rust/bindings/lib.rs', source/'lib.rs')
        self.run_cmd([*br, '--crate-name=bindings', '--emit=metadata='+str(self.out/'libbindings.rmeta')+',obj='+str(self.out/'bindings.o'), source/'lib.rs'],
                     env={**self.env,'OBJTREE':str(self.out)})
        r += ['--extern', 'bindings='+str(self.out/'libbindings.rmeta')]
        callback=self.out/'callbacks.c'
        callback.write_text('#include <linux/sort.h>\n'
            'int compare(const void *a,const void *b); int compare(const void *a,const void *b){return a==b;}\n'
            'int compare_r(const void *a,const void *b,const void *p); int compare_r(const void *a,const void *b,const void *p){return a==b&&p==a;}\n'
            'void exchange(void *a,void *b,int n); void exchange(void *a,void *b,int n){(void)a;(void)b;(void)n;}\n'
            'void exchange_r(void *a,void *b,int n,const void *p); void exchange_r(void *a,void *b,int n,const void *p){(void)a;(void)b;(void)n;(void)p;}\n')
        self.run_cmd([*c,'-S','-emit-llvm',callback,'-o',self.out/'callbacks.ll'])
        callbacks=set(ids(self.out/'callbacks.ll').values())
        names=('sort','sort_r','sort_nonatomic','sort_r_nonatomic')
        for opt in ('0','2','s'):
            folder=self.out/('native-O'+opt); folder.mkdir()
            self.run_cmd([*r, '-Copt-level='+opt, '--crate-name=sort_rust',
                '--emit=obj='+str(folder/'rust.o')+',llvm-ir='+str(folder/'rust.ll')+',dep-info='+str(folder/'rust.d'), ROOT/'lib/sort_rust.rs'])
            self.run_cmd([*c,'-O'+opt,'-c',ORIGINAL/'lib/sort.c','-o',folder/'c.o'])
            self.run_cmd([*c,'-O'+opt,'-S','-emit-llvm',ORIGINAL/'lib/sort.c','-o',folder/'c.ll'])
            ci,ri=ids(folder/'c.ll'),ids(folder/'rust.ll')
            self.assertEqual({name:ci[name] for name in names},{name:ri[name] for name in names})
            protected={int(value)&0xffffffff for value in re.findall(r'"kcfi"\(i32 (-?\d+)\)',(folder/'rust.ll').read_text())}
            self.assertTrue(callbacks <= protected, (callbacks,protected))
            self.assertEqual(read_exports(folder/'c.o'),read_exports(folder/'rust.o'))
            self.assertEqual(sorted(record['name'] for record in read_exports(folder/'rust.o')),sorted(names))
            for rel in ('sort.rs','sort_sched.rs','sort_header.rs'):
                self.assertIn(rel,(folder/'rust.d').read_text())
            for language in ('c','rust'):
                elf=(folder/(language+'.o')).read_bytes()
                self.assertEqual(elf[:6],b'\x7fELF\x02\x01')
                self.assertEqual(int.from_bytes(elf[18:20],'little'),62 if 'CONFIG_X86_64=y\n' in (build/'.config').read_text() else 183)
                symbols=self.run_cmd(['llvm-nm','-u',folder/(language+'.o')]).stdout
                self.assertNotRegex(symbols,r'alloc|sort_cond_resched')
                # At O0, genuine core's unoptimized UB-precondition helpers
                # retain unreachable panic references; optimized production
                # objects must eliminate them. Host execution uses real core
                # and fails on any reached panic, without replacement stubs.
                if opt != '0': self.assertNotRegex(symbols,r'panic|unwind')
                versions=self.run_cmd([build/'scripts/gendwarfksyms/gendwarfksyms','--dump-versions','-T',folder/(language+'.symtypes'),folder/(language+'.o')],input='\n'.join(names)+'\n')
                self.assertEqual(versions.stdout.count('#SYMVER '),4)
        self.scheduler_configs(build,r,c,command[split+1:])
        self.kbuild(build,c,fixture)

    def shared_source(self):
        fixture=self.out/'kbuild-source'
        for rel in ('lib/sort.rs','lib/sort_rust.rs','lib/sort_sched.rs','include/linux/sort_header.rs',
                    'rust/ffi_export.rs','include/linux/export_header.rs','lib/sort.c','lib/Makefile','lib/Kconfig',
                    'rust/bindings/lib.rs','rust/bindings/bindings_helper.h','rust/bindgen_parameters',
                    'scripts/tests/translated_sources.txt'):
            path=fixture/rel;path.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(ROOT/rel,path)
        if self.shared_patch is not None:
            # The integration candidate can be proven while shared donor sources
            # remain frozen. Apply the exact reviewed patch to private copies.
            self.run_cmd(['git','apply','--directory='+str(fixture),'--unsafe-paths',self.shared_patch])
            (self.out/'shared-patch.sha256').write_text(hashlib.sha256(self.shared_patch.read_bytes()).hexdigest()+'\n')
        return fixture

    def kbuild(self, native, c, fixture):
        out=self.out/'kbuild-output'
        makefile=(fixture/'lib/Makefile').read_text()
        self.assertIn('RUSTFLAGS_sort_rust.o += --extern bindings',makefile,
                      'sort selector not integrated; supply the explicit candidate SORT_SHARED_PATCH')
        (fixture/'lib/Makefile').write_text(makefile+'\n'
            'sort-order := $(obj-y)\n'
            '$(file >$(objtree)/selected-order.txt,$(sort-order))\n'
            'obj-y := $(filter sort.o sort_rust.o,$(obj-y))\nobj-m :=\nlib-y :=\n')
        (out/'lib').mkdir(parents=True);(out/'rust').mkdir()
        shutil.copytree(native/'include',out/'include',symlinks=False)
        for rel in ('scripts/basic/fixdep','scripts/gendwarfksyms/gendwarfksyms'):
            path=out/rel;path.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(native/rel,path)
        words=self.saved(native,'lib/.list_sort_rust.o.cmd')
        while '=' in words[0] and not words[0].startswith('-'):words.pop(0)
        at=next(i for i,flag in enumerate(words) if flag.startswith('-Zallow-features='))
        flags=transport.native_flags(words[1:at],native,'rust')+['-L'+str(native/'rust')]
        verify_flag_policy(flags,'rust')
        response=out/'rust-flags.rsp';response.write_text('\n'.join(flags)+'\n')
        driver=out/'driver.mk';driver.write_text('include '+str(ORIGINAL/'scripts/Makefile.build')+'\n')
        cflags=[flag for flag in c[1:] if 'KBUILD_' not in flag]
        command=['make','-rR','-C',str(out),'-f',str(driver),'srctree='+str(ORIGINAL),'VPATH='+str(fixture),
            'srcroot='+str(fixture),'objtree='+str(out),'obj=lib','V=1','need-builtin=1',
            'CONFIG_RUST=y','RUSTC='+str(self.rust),'RUSTC_OR_CLIPPY='+str(self.rust),'KBUILD_RUSTFLAGS=@'+str(response),
            'RUSTFLAGS_sort_rust.o=--extern bindings='+str(self.out/'libbindings.rmeta'),
            'rust_crate_features=arbitrary_self_types,asm_goto,generic_arg_infer,used_with_arg',
            'c_flags=-Wp,-MMD,$(depfile) '+shlex.join(cflags)+
            ' -DKBUILD_MODFILE=\'"lib/sort"\' -DKBUILD_MODNAME=\'"sort"\' -DKBUILD_BASENAME=\'"sort"\'',
            'CC=clang','LD=ld.lld','AR=llvm-ar','NM=llvm-nm','OBJCOPY=llvm-objcopy','AWK=awk',
            'READELF=llvm-readelf','CONFIG_SHELL=/bin/sh','CONFIG_OBJTOOL=','CONFIG_FTRACE_MCOUNT_USE_RECORDMCOUNT=',
            'KBUILD_BUILTIN=1']
        for host in ('c','rust'):
            for selection in ('','y','','y'):
                selected='sort_rust' if selection else 'sort'
                invocation=[*command,'HOST_TOOLS_LANG='+host,'CONFIG_RUST_SORT='+selection,'lib/built-in.a']
                self.run_cmd(invocation)
                order=(out/'selected-order.txt').read_text().split()
                at=order.index(selected+'.o')
                self.assertEqual(order[at-1], 'bcd_rust.o' if 'CONFIG_RUST_BCD=y\n' in (native/'.config').read_text() else 'bcd.o')
                self.assertEqual(order[at+1], 'parser_rust.o' if 'CONFIG_RUST_PARSER=y\n' in (native/'.config').read_text() else 'parser.o')
                self.assertEqual(order.count(selected+'.o'),1)
                self.assertNotIn(('sort' if selection else 'sort_rust')+'.o',order)
                archive=self.run_cmd(['llvm-ar','t',out/'lib/built-in.a']).stdout.splitlines()
                self.assertEqual([Path(p).name for p in archive],[selected+'.o'])
                obj=out/('lib/'+selected+'.o');stamp=obj.stat().st_mtime_ns
                self.run_cmd(invocation);self.assertEqual(obj.stat().st_mtime_ns,stamp)
                record=(out/('lib/.'+selected+'.o.cmd')).read_text()
                self.assertEqual(record.count('#SYMVER '),4)
                self.assertEqual(read_exports(obj),read_exports(self.out/'native-Os'/('rust.o' if selection else 'c.o')))
                if selection:
                    self.assertIn('RUST_MODFILE=lib/sort ',record)
                    recorded={Path(line.strip().removesuffix('\\').strip()).resolve()
                              for line in record.splitlines() if line.startswith('  /')}
                    for rel in ('lib/sort.rs','lib/sort_sched.rs','include/linux/sort_header.rs'):
                        self.assertIn((fixture/rel).resolve(),recorded)
                        (fixture/rel).touch();self.run_cmd(invocation)
                        self.assertNotEqual(obj.stat().st_mtime_ns,stamp);stamp=obj.stat().st_mtime_ns
        for selection in ('','y'):
            selected='sort_rust' if selection else 'sort'
            for suffix in ('s','ll'):
                invocation=[*command,'CONFIG_RUST_SORT='+selection,'lib/'+selected+'.'+suffix]
                self.run_cmd(invocation)
                artifact=out/('lib/'+selected+'.'+suffix);stamp=artifact.stat().st_mtime_ns
                self.run_cmd(invocation);self.assertEqual(artifact.stat().st_mtime_ns,stamp)
                self.assertTrue(artifact.stat().st_size)

    def scheduler_configs(self, build, rust, c, bindgen_flags):
        # Generate real declarations from the current original scheduler header
        # under each supported preemption/debug branch, using saved target flags.
        keys={'CONFIG_PREEMPTION','CONFIG_PREEMPT_DYNAMIC','CONFIG_HAVE_PREEMPT_DYNAMIC_CALL',
              'CONFIG_HAVE_PREEMPT_DYNAMIC_KEY','CONFIG_DEBUG_ATOMIC_SLEEP'}
        actual=(build/'.config').read_text()
        capability='CONFIG_HAVE_PREEMPT_DYNAMIC_CALL' if 'CONFIG_X86_64=y\n' in actual else 'CONFIG_HAVE_PREEMPT_DYNAMIC_KEY'
        branch='__SCT__cond_resched' if capability.endswith('CALL') else 'dynamic_cond_resched'
        cases=[('nonpreempt',set(),'__cond_resched'),('fixed',{'CONFIG_PREEMPTION'},None),
               ('dynamic',{'CONFIG_PREEMPTION','CONFIG_PREEMPT_DYNAMIC',capability},branch)]
        prefix=['#include <linux/kconfig.h>',*[f'#undef {key}' for key in sorted(keys)]]
        report={}
        for name,selection,target in cases:
            for debug in (False,True):
                enabled=selection|({'CONFIG_DEBUG_ATOMIC_SLEEP'} if debug else set())
                folder=self.out/(name+('-debug' if debug else ''));folder.mkdir()
                header=folder/'scheduler.h'
                header.write_text('\n'.join([*prefix,*[f'#define {key} 1' for key in sorted(enabled)],'#include <linux/sched.h>'])+'\n')
                source=folder/'probe.c'
                source.write_text('#include "scheduler.h"\nvoid sched_probe(void);\nvoid sched_probe(void){cond_resched();}\n')
                self.run_cmd([*c,'-O2','-c',source,'-o',folder/'c.o'])
                self.run_cmd([self.bindgen,header,'--use-core','--rust-target=1.85','--ctypes-prefix=ffi',
                    '--no-layout-tests','--allowlist-function=^(__might_resched|__cond_resched|dynamic_cond_resched|__SCT__cond_resched)$',
                    '--allowlist-var=^__SCK__cond_resched$','--allowlist-type=^static_call_key$',
                    '--opaque-type=static_call_mod','-o',folder/'generated.rs','--',
                    *transport.native_flags(bindgen_flags,build,'c')])
                flags=[];iterator=iter(rust)
                for flag in iterator:
                    if flag=='--cfg':
                        value=next(iterator)
                        if value not in keys: flags += [flag,value]
                    elif flag.startswith('--cfg='):
                        if flag[len('--cfg='):] not in keys: flags.append(flag)
                    elif flag=='--extern':
                        value=next(iterator)
                        if not value.startswith('bindings='): flags += [flag,value]
                    else: flags.append(flag)
                for key in sorted(enabled):flags += ['--cfg',key]
                (folder/'bindings.rs').write_text('//! Actual scheduler declarations.\n'
                    '#![allow(missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals,dead_code)]\ninclude!("generated.rs");\n')
                self.run_cmd([*flags,'--extern','ffi='+str(build/'rust/libffi.rmeta'),
                    '--crate-name=bindings','--emit=metadata='+str(folder/'libbindings.rmeta'),folder/'bindings.rs'])
                (folder/'probe.rs').write_text('//! Original scheduler branch probe.\n'
                    '#[path='+json.dumps(str(ROOT/'lib/sort_sched.rs'))+'] mod scheduling;\n'
                    '/// Call the actual translated scheduler expansion.\n#[no_mangle] pub extern "C" fn sched_probe(){unsafe{scheduling::cond_resched(b"probe\\0".as_ptr(),1)}}\n')
                self.run_cmd([*flags,'--extern','bindings='+str(folder/'libbindings.rmeta'),'-Copt-level=2',
                    '--crate-name=sched_probe','--emit=obj='+str(folder/'rust.o')+',llvm-ir='+str(folder/'rust.ll'),folder/'probe.rs'])
                wanted=({target} if target else set())|({'__might_resched'} if debug else set())
                if target=='__SCT__cond_resched' and 'CONFIG_HAVE_STATIC_CALL_INLINE=y\n' in actual:
                    wanted.add('__SCK__cond_resched')
                symbols={}
                for language in ('c','rust'):
                    output=self.run_cmd(['llvm-nm','-u',folder/(language+'.o')]).stdout
                    symbols[language]={line.split()[-1] for line in output.splitlines() if line.split()}
                    # x86's original headers also retain unrelated memcpy
                    # implementation addresses. Prove those are discard-only
                    # references before excluding them from scheduler dispatch.
                    incidental=set()
                    if language=='c' and 'CONFIG_X86_64=y\n' in actual:
                        incidental={'__clear_pages_unrolled','copy_page','__memset','__memmove'}
                        relocs=self.run_cmd(['llvm-readelf','-rW',folder/'c.o']).stdout
                        section=re.search(r"Relocation section '\.rela.discard.addressable'.*?(?=\nRelocation section|\Z)",relocs,re.S)[0]
                        for symbol in incidental:
                            self.assertRegex(section,r'\b'+symbol+r' \+ 0\n')
                    self.assertEqual(symbols[language]-incidental,wanted,(name,debug,language))
                if '__SCK__cond_resched' in wanted:
                    for language in ('c','rust'):
                        reloc=self.run_cmd(['llvm-readelf','-rW',folder/(language+'.o')]).stdout
                        self.assertRegex(reloc,r"(?s)\.rela.discard.addressable.*__SCK__cond_resched")
                report[folder.name]={k:sorted(v) for k,v in symbols.items()}
        (self.out/'scheduler-configs.json').write_text(json.dumps(report,indent=2)+'\n')

    def test_native_x86(self): self.native(self.x86)

    def test_native_arm64(self): self.native(self.arm)


if __name__ == '__main__':
    unittest.main()
