#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit win_minmax native ownership and run independent disposable VM callers.

Only the explicit CLI builds modules and invokes the root boot_kernel.py runner.
Original C under private names is the sole algorithm oracle. No host insmod.
"""
import argparse
import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import sys
import tempfile

from boot_kernel import module_name, verify_module_events
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_int_log_kernel import kunit_runs, verify_kunit_warnings
from kernel_console import normalize_console_transport
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer
from check_prime_numbers_kernel import provider_type_ids, verify_guarded_calls, verify_module_import_versions, verify_rust_entrypoints
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
EXPORTS = ('minmax_running_min', 'minmax_running_max')
WRAPPERS = tuple(('win_minmax_call_' + kind, 'minmax_running_' + kind) for kind in ('min', 'max'))
COUNT = 136840
DESCRIPTION = b'Non-GPL windowed minmax public ABI check'
PRIVATE_C_FLAGS = 'CFLAGS_MODULE+=-D__DISABLE_EXPORTS'


def reference_source():
    return ('#include <linux/module.h>\n#include <linux/win_minmax.h>\n'
            'u32 win_minmax_reference_min(struct minmax *, u32, u32, u32);\n'
            'u32 win_minmax_reference_max(struct minmax *, u32, u32, u32);\n'
            '#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(x)\n'
            '#define minmax_running_min win_minmax_reference_min\n'
            '#define minmax_running_max win_minmax_reference_max\n'
            '#include "' + str(ROOT / 'lib/win_minmax.c') + '"\n')

C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
#include <linux/win_minmax.h>
@WRAPPERS@
static u32 draw(u32 *x) { *x ^= *x << 13; *x ^= *x >> 17; *x ^= *x << 5; return *x; }
static int same(const struct minmax *a, const struct minmax *b)
{ unsigned int i; for (i=0;i<3;i++) if (a->s[i].t!=b->s[i].t || a->s[i].v!=b->s[i].v) return 0; return 1; }
static int step(struct minmax *a, struct minmax *b, u32 kind, u32 win, u32 t, u32 v)
{
    u32 x = kind ? win_minmax_call_max(a,win,t,v) : win_minmax_call_min(a,win,t,v);
    u32 y = kind ? win_minmax_reference_max(b,win,t,v) : win_minmax_reference_min(b,win,t,v);
    return x==y && same(a,b) && minmax_get(a)==x ? 0 : -EINVAL;
}
static int exercise(void)
{
    static const u32 wins[] = {0,1,2,3,4,17,0x80000000U,0xffffffffU};
    u32 k,w,base,shape,d,v,i,j,n=0,first_live=0,rng=0x938acdef;
    for(k=0;k<2;k++) {
        /* Only the fields read before a reset are initialized. */
        for(i=0;i<3;i++) {
            struct minmax a,b;
            u32 value = k ? 100 : 1;
            a.s[0].v=b.s[0].v=i==0 ? value : (k ? 50 : 50);
            if(i==2) { a.s[2].t=b.s[2].t=0; value=k ? 1 : 100; }
            if(step(&a,&b,k,4,10,value)) return -EINVAL;
            n++;
        }
        {
            struct minmax a;
            a.s[0].v=73;
            if(minmax_get(&a)!=73 || minmax_reset(&a,0xffffffffU,31)!=31) return -EINVAL;
            for(i=0;i<3;i++) if(a.s[i].t!=0xffffffffU || a.s[i].v!=31) return -EINVAL;
            n++;
        }
        for(w=0;w<8;w++) for(base=0;base<2;base++) for(shape=0;shape<5;shape++) {
            u32 win=wins[w], t0=base ? 0xfffffff0U : 0;
            u32 deltas[]={0,win/4,win/4+1,win/2,win/2+1,win,win+1,win+2,0xffffffffU};
            for(d=0;d<9;d++) for(v=0;v<4;v++) {
                struct minmax a,b;
                a.s[0].t=t0;
                a.s[1].t=t0+(shape==0 ? 0 : shape==2 ? win/2 : shape==4 ? win : 1);
                a.s[2].t=shape<2 ? a.s[1].t : t0+win;
                for(i=0;i<3;i++) a.s[i].v=k ? ~(10+10*i) : 10+10*i;
                b=a;
                if(step(&a,&b,k,win,t0+deltas[d],k ? ~(10+10*v) : 10+10*v)) return -EINVAL;
                n++;
            }
        }
        for(i=0;i<1024;i++) {
            struct minmax a,b;
            u32 win=draw(&rng), t=draw(&rng);
            for(j=0;j<3;j++) { a.s[j].t=draw(&rng); a.s[j].v=draw(&rng); }
            b=a;
            /* No initial reset or forced tie erases the randomized state. */
            for(j=0;j<64;j++) {
                u32 value=draw(&rng);
                if(j==0 && t-a.s[2].t<=win && (k ? value<a.s[0].v : value>a.s[0].v)) first_live++;
                if(step(&a,&b,k,win,t,value)) return -EINVAL;
                t+=draw(&rng)&255; n++;
            }
        }
    }
    if(first_live<400) return -EINVAL;
    return n;
}
static int __init win_minmax_abi_init(void)
{
    if(exercise()!=@COUNT@) { pr_err("LUPOS_WIN_MINMAX_FAIL\n"); return -EINVAL; }
    pr_info("@MARKER@\n"); return 0;
}
static void __exit win_minmax_abi_exit(void) {}
module_init(win_minmax_abi_init);
module_exit(win_minmax_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Non-GPL windowed minmax public ABI check");
'''

RUST_SOURCE = r'''//! Independent windowed tracker workload using genuine generated binding types.
use kernel::{bindings, ffi};
#[allow(dead_code)]
#[path = "@HEADER@"]
pub mod declarations;
use declarations::{minmax, minmax_sample, minmax_get, minmax_reset};
@WRAPPERS@
fn draw(x: &mut u32) -> u32 { *x ^= *x << 13; *x ^= *x >> 17; *x ^= *x << 5; *x }
fn same(a: &minmax, b: &minmax) -> bool {
    for i in 0..3 { if a.s[i].t!=b.s[i].t || a.s[i].v!=b.s[i].v { return false; } } true
}
unsafe fn step(a: *mut minmax, b: *mut minmax, kind: u32, win: u32, t: u32, v: u32) -> Result<(), ffi::c_int> {
    // SAFETY: Inputs initialize every field read on the selected C path. Both
    // calls completely initialize reset outputs before any full-state read.
    let (x,y) = unsafe { if kind==1 {
        (win_minmax_call_max(a,win,t,v),win_minmax_reference_max(b,win,t,v))
    } else { (win_minmax_call_min(a,win,t,v),win_minmax_reference_min(b,win,t,v)) } };
    // SAFETY: All six output fields are now initialized, including partial cases.
    if unsafe { x!=y || !same(&*a,&*b) || minmax_get(a)!=x } { return Err(-22); } Ok(())
}
fn exercise() -> Result<u32, ffi::c_int> {
    let wins=[0u32,1,2,3,4,17,0x80000000,0xffffffff];
    let mut n=0u32; let mut first_live=0u32; let mut rng=0x938acdefu32;
    for k in 0..2 {
        for i in 0..3 {
            let mut a=core::mem::MaybeUninit::<minmax>::uninit();
            let mut b=core::mem::MaybeUninit::<minmax>::uninit();
            let (ap,bp)=(a.as_mut_ptr(),b.as_mut_ptr());
            let mut value=if k==1 {100} else {1};
            // SAFETY: Raw writes initialize only the fields read on this path.
            unsafe {
                core::ptr::addr_of_mut!((*ap).s[0].v).write(if i==0 {value} else {50});
                core::ptr::addr_of_mut!((*bp).s[0].v).write(if i==0 {value} else {50});
                if i==2 {
                    core::ptr::addr_of_mut!((*ap).s[2].t).write(0);
                    core::ptr::addr_of_mut!((*bp).s[2].t).write(0);
                    value=if k==1 {1} else {100};
                }
                step(ap,bp,k,4,10,value)?;
            }
            n=n.wrapping_add(1);
        }
        let mut a=core::mem::MaybeUninit::<minmax>::uninit();
        let ap=a.as_mut_ptr();
        // SAFETY: get reads only s[0].v; reset writes all fields without reads.
        unsafe {
            core::ptr::addr_of_mut!((*ap).s[0].v).write(73);
            if minmax_get(ap)!=73 || minmax_reset(ap,0xffffffff,31)!=31 { return Err(-22); }
            for i in 0..3 { if (*ap).s[i].t!=0xffffffff || (*ap).s[i].v!=31 { return Err(-22); } }
        }
        n=n.wrapping_add(1);
        for wi in 0..8 { let win=wins[wi]; for base in 0..2 { for shape in 0..5 {
            let t0=if base==1 {0xfffffff0u32} else {0};
            let deltas=[0,win/4,(win/4).wrapping_add(1),win/2,(win/2).wrapping_add(1),win,win.wrapping_add(1),win.wrapping_add(2),0xffffffff];
            for di in 0..9 { let delta=deltas[di]; for v in 0..4u32 {
                let mut a=minmax {s:[minmax_sample {t:0,v:0};3]};
                a.s[0].t=t0;
                a.s[1].t=t0.wrapping_add(if shape==0 {0} else if shape==2 {win/2} else if shape==4 {win} else {1});
                a.s[2].t=if shape<2 {a.s[1].t} else {t0.wrapping_add(win)};
                for i in 0..3 { let value=10+10*i as u32; a.s[i].v=if k==1 {!value} else {value}; }
                let mut b=a;
                // SAFETY: Both trackers have all fields initialized.
                unsafe { step(&mut a,&mut b,k,win,t0.wrapping_add(delta),if k==1 {!(10+10*v)} else {10+10*v})?; }
                n=n.wrapping_add(1);
            } }
        } } }
        for _ in 0..1024 {
            let win=draw(&mut rng); let mut t=draw(&mut rng);
            let mut a=minmax {s:[minmax_sample {t:0,v:0};3]};
            for i in 0..3 { a.s[i].t=draw(&mut rng); a.s[i].v=draw(&mut rng); }
            let mut b=a;
            // No initial reset or forced tie erases randomized state.
            for j in 0..64 {
                let value=draw(&mut rng);
                if j==0 && t.wrapping_sub(a.s[2].t)<=win && (if k==1 {value<a.s[0].v} else {value>a.s[0].v}) {
                    first_live=first_live.wrapping_add(1);
                }
                // SAFETY: Both states stay fully initialized across updates.
                unsafe { step(&mut a,&mut b,k,win,t,value)?; }
                t=t.wrapping_add(draw(&mut rng)&255); n=n.wrapping_add(1);
            }
        }
    }
    if first_live<400 { return Err(-22); }
    Ok(n)
}
/// Execute the entire independent Rust workload.
#[no_mangle]
#[link_section=".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    if exercise()!=Ok(@COUNT@) {
        // SAFETY: Static format without arguments.
        unsafe { bindings::_printk(c"\x013LUPOS_WIN_MINMAX_FAIL\n".as_ptr().cast()); } return -22;
    }
    // SAFETY: Static format without arguments.
    unsafe { bindings::_printk(c"\x016@MARKER@\n".as_ptr().cast()); } 0
}
/// No resources survive the workload.
#[no_mangle]
#[link_section=".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section=".init.data"]
static INIT: extern "C" fn()->ffi::c_int=init_module;
#[used]
#[link_section=".exit.data"]
static EXIT: extern "C" fn()=cleanup_module;
const INFO: &str="license=Proprietary\0description=Non-GPL windowed minmax public ABI check\0";
#[used]
#[link_section=".modinfo"]
static MODINFO: [u8; INFO.len()] = {
    let mut bytes=[0;INFO.len()]; let mut i=0;
    while i<bytes.len() { bytes[i]=INFO.as_bytes()[i]; i+=1; } bytes
};
#[used]
static __IS_RUST_MODULE: ()=();
'''


def sources(caller):
    if caller not in ('c','rust'): raise ValueError('unknown win_minmax caller')
    wrappers=[]
    for kind in ('min','max'):
        if caller=='c':
            wrappers.append(f'u32 win_minmax_reference_{kind}(struct minmax *, u32, u32, u32);\n'
                f'static noinline u32 win_minmax_call_{kind}(struct minmax *m, u32 w, u32 t, u32 v) {{\n'
                f' u32 (*volatile selected)(struct minmax *,u32,u32,u32)=minmax_running_{kind};\n'
                ' return selected(m,w,t,v); }\n')
        else:
            wrappers.append(f'unsafe extern "C" {{ fn win_minmax_reference_{kind}(m: *mut minmax,w:u32,t:u32,v:u32)->u32; }}\n'
                '#[no_mangle]\n#[inline(never)]\n'
                f'unsafe extern "C" fn win_minmax_call_{kind}(m:*mut minmax,w:u32,t:u32,v:u32)->u32 {{\n'
                f' let selected: unsafe extern "C" fn(*mut minmax,u32,u32,u32)->u32=declarations::minmax_running_{kind};\n'
                ' // SAFETY: Caller supplies storage valid for the executed path.\n'
                ' unsafe { core::ptr::read_volatile(&selected)(m,w,t,v) } }\n')
    return (C_SOURCE if caller=='c' else RUST_SOURCE).replace('@WRAPPERS@','\n'.join(wrappers)).replace(
        '@COUNT@',str(COUNT)).replace('@MARKER@',result_marker(caller).decode()).replace(
        '@HEADER@',str(ROOT / 'include/linux/win_minmax_header.rs'))


def provider(config):
    architecture(config)
    if config.get('RUST_WIN_MINMAX','n') not in ('n','y'): raise ValueError('invalid win_minmax selector')
    if config.get('RUST_WIN_MINMAX')=='y' and config.get('RUST')!='y': raise ValueError('Rust provider requires RUST=y')
    return 'Rust' if config.get('RUST_WIN_MINMAX')=='y' else 'C'


def owner_path(build, selection):
    if selection not in ('C','Rust'): raise ValueError('unknown provider')
    return build / ('lib/win_minmax_rust.o' if selection=='Rust' else 'lib/win_minmax.o')

def selected_versions(build, names):
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()
            if len(line.split()) > 1 and line.split()[1].startswith(b'minmax_')]
    if len(rows) != len(names) or {row[1] for row in rows} != {name.encode() for name in names}:
        raise ValueError('win_minmax exported names missing, duplicated or incorrect for backend')
    result = {}
    for row in rows:
        if (len(row) != 4 or row[2:] != [b'vmlinux', b'EXPORT_SYMBOL'] or
                not re.fullmatch(rb'0x[0-9a-fA-F]{8}', row[0])):
            raise ValueError('wrong win_minmax export owner/license/namespace/CRC')
        result[row[1]] = row[0].lower()
    return result


def verify_final_symbols(build):
    for path in (build / 'vmlinux.o', build / 'vmlinux'):
        rows = [line.split() for line in tool('nm', '-g', '--defined-only', path).splitlines()
                if line.split() and line.split()[-1].startswith(b'minmax_')]
        if len(rows) != 2 or {row[-1] for row in rows} != {n.encode() for n in EXPORTS} or any(row[-2] != b'T' for row in rows):
            raise ValueError('final minmax symbol owner differs or exposes header helpers')


def verify_dwarf(build, owner, selection, versions):
    config = configuration(build)
    if config.get('MODVERSIONS') != 'y': return
    if config.get('GENDWARFKSYMS') != 'y': raise ValueError('requires native DWARF version proof')
    with tempfile.TemporaryDirectory(prefix='win-minmax-dwarf-') as temporary:
        path = Path(temporary) / 'types'
        result = subprocess.run([str(build / 'scripts/gendwarfksyms/gendwarfksyms'), '--symtypes', str(path), str(owner)],
            input=b''.join(n.encode() + b'\n' for n in EXPORTS), capture_output=True, check=True, timeout=180)
        rows = [line.split() for line in result.stdout.splitlines()]
        if result.stderr or len(rows) != 2 or any(len(row) != 3 or row[0] != b'#SYMVER' for row in rows):
            raise ValueError('malformed native DWARF CRC proof')
        if {row[1]: row[2] for row in rows} != versions: raise ValueError('defining DWARF CRC differs from symvers')
        types = path.read_text()
        verify_defining_types(types)
        verify_defining_parameters(owner)


def verify_defining_parameters(owner):
    """Prove the defining DWARF DIE is nonvariadic, independently of symtypes.

    gendwarfksyms serializes formal parameters but omits unspecified_parameters;
    a variadic definition can therefore have the same printed signature.
    """
    for name in EXPORTS:
        dump = tool('llvm-dwarfdump', '--debug-info', '--name=' + name, '--show-children', owner).decode()
        entries = list(re.finditer(r'^0x[0-9a-fA-F]+:([ ]+)(DW_TAG_\w+|NULL)\b', dump, re.M))
        if not entries: raise ValueError('missing defining minmax DWARF DIE')
        depth = min(len(entry[1]) for entry in entries)
        roots = [entry for entry in entries if len(entry[1]) == depth and entry[2] != 'NULL']
        if len(roots) != 1 or roots[0][2] != 'DW_TAG_subprogram':
            raise ValueError('ambiguous defining minmax DWARF DIE')
        first = entries.index(roots[0])
        attributes = dump[roots[0].end():entries[first + 1].start() if first + 1 < len(entries) else len(dump)]
        if (not re.search(r'DW_AT_name\s+\("' + re.escape(name) + r'"\)', attributes) or
                'DW_AT_declaration' in attributes or
                not re.search(r'DW_AT_(?:low_pc|ranges)\b', attributes)):
            raise ValueError('minmax DWARF record is not its actual definition')
        children = [entry[2] for entry in entries if len(entry[1]) == depth + 2]
        if children.count('DW_TAG_formal_parameter') != 4 or 'DW_TAG_unspecified_parameters' in children:
            raise ValueError('defining minmax function is variadic or has the wrong parameter count')


def verify_defining_types(types):
    # Resolve real gendwarfksyms typedef records; never supply checker-made ABI.
    # Namespaces/accessibility are real Rust DWARF, not C CRC equivalents.
    # This normalization is ONLY for layout assertions, never CRC calculation.
    types = re.sub(r'(?:\w+::)+(?=minmax(?:_sample)?\b)', '', types)
    types = types.replace(' accessibility(1)', '').replace(' alignment(4)', '')
    aliases = dict(re.findall(r'^(t#\w+) typedef_type \w+ \{ (.*?) \}$', types, re.M))
    records = [line.split(' ', 1) for line in types.splitlines() if ' ' in line]
    rows = dict(records)
    if len(rows) != len(records):
        raise ValueError('duplicate defining type record')
    for _ in range(len(aliases)):
        rows = {key: re.sub(r't#\w+', lambda m: aliases.get(m[0], m[0]), value)
                for key, value in rows.items()}
    scalar = r'base_type \w+(?: \w+)* byte_size\(4\) encoding\(7\)'
    if not re.fullmatch(r'structure_type minmax \{ member array_type\[3\] \{ s#minmax_sample \} s data_member_location\(0\) \} byte_size\(24\)', rows.get('s#minmax','')):
        raise ValueError('wrong defining minmax array/layout')
    if not re.fullmatch(r'structure_type minmax_sample \{ member '+scalar+r' t data_member_location\(0\) , member '+scalar+r' v data_member_location\(4\) \} byte_size\(8\)', rows.get('s#minmax_sample','')):
        raise ValueError('wrong defining sample fields/layout')
    for name in EXPORTS:
        row = rows.get(name,'')
        # Match the entire ordered graph, including the return edge. Counts
        # accept swapped arguments, pointer returns and variadic functions.
        parameter = r'formal_parameter ' + scalar + r'(?: \w+)?'
        signature = (r'subprogram \( formal_parameter pointer_type(?: \*mut minmax)?'
                     r' \{ s#minmax \}(?: \w+)? , ' +
                     r' , '.join([parameter] * 3) + r' \) -> ' + scalar)
        if not re.fullmatch(signature, row):
            raise ValueError('wrong defining minmax function signature')


def verify_strict_lints(flags):
    """Match ordered rustc lint levels, sticky forbid and the first lint cap."""
    levels = {'A': 'allow', 'W': 'warn', 'D': 'deny', 'F': 'forbid'}
    warnings, cap = None, None
    tokens = iter(flags)
    for token in tokens:
        level, value = None, None
        if token in ('--cap-lints', '--force-warn', '--allow', '--warn', '--deny', '--forbid'):
            level, value = token[2:], next(tokens, None)
        elif any(token.startswith('--' + name + '=') for name in
                 ('cap-lints', 'force-warn', 'allow', 'warn', 'deny', 'forbid')):
            level, value = token[2:].split('=', 1)
        elif len(token) >= 2 and token[0] == '-' and token[1] in levels:
            level, value = levels[token[1]], token[2:] or next(tokens, None)
        if level is None: continue
        if not value or value.startswith('='):
            raise ValueError('malformed Rust lint option')
        if level == 'cap-lints':
            if value not in ('allow', 'warn', 'deny', 'forbid'):
                raise ValueError('unsupported Rust lint cap')
            if cap is None: cap = value
        elif level == 'force-warn':
            raise ValueError('Rust force-warn bypasses -Dwarnings')
        elif value == 'warnings':
            if warnings != 'forbid': warnings = level
    if warnings not in ('deny', 'forbid') or cap not in (None, 'deny', 'forbid'):
        raise ValueError('Rust win_minmax requires unsuppressed -Dwarnings')


def response_flags(flags, base, active=()):
    expanded = []
    for flag in flags:
        if '\0' in flag: raise ValueError('NUL in Rust compiler argument')
        if flag.startswith('@'):
            if not flag[1:]: raise ValueError('empty Rust response file path')
            path = (base / flag[1:]).resolve()
            if path in active: raise ValueError('recursive Rust response file')
            # rustc response files contain one argument per line, not shell words.
            expanded.extend(response_flags(path.read_text().splitlines(), base, (*active, path)))
        else: expanded.append(flag)
    return expanded


def verify_native_rust(obj, build=None):
    flags = compilation_flags(obj)
    base = build or obj.parent
    verify_strict_lints(response_flags(flags, base))
    compilers = [flag for flag in flags if Path(flag).name == 'rustc']
    if len(compilers) != 1: raise ValueError('missing actual native Rust compiler')
    compiler = str((base / compilers[0]).resolve()) if '/' in compilers[0] else compilers[0]
    result = subprocess.run([compiler, '--version'], capture_output=True, check=True, timeout=30)
    match = re.search(rb'rustc (\d+)\.(\d+)', result.stdout)
    if not match or tuple(map(int,match.groups())) < (1,85): raise ValueError('requires Rust >=1.85')


def verify_bindings(build):
    generated = build / 'rust/bindings/bindings_generated.rs'
    if not all(token in generated.read_text() for token in ('pub struct minmax {', 'pub struct minmax_sample {')):
        raise ValueError('final kernel lacks genuine generated minmax types')
    verify_build_command(build, generated, ROOT / 'rust/bindings/bindings_helper.h', [ROOT / 'include/linux/win_minmax.h'])
    newer(build / 'rust/libbindings.rmeta', [generated])
    newer(build / 'rust/libkernel.rmeta', [build / 'rust/libbindings.rmeta'])


def verify_independent_imports(obj):
    imports = {line.split()[-1] for line in tool('nm', '-u', obj).splitlines() if line.split()}
    expected = {name.encode() for name in EXPORTS} | {b'win_minmax_reference_min', b'win_minmax_reference_max', b'_printk'}
    # ARM64 may lower fixed-size, typed tracker copies to native memcpy.
    if imports - {b'memcpy'} != expected:
        raise ValueError('caller delegates work or has unexpected imports: ' + repr(imports - expected))


def verify_linked_implementation(build, selection):
    config = configuration(build)
    if selection != provider(config): raise ValueError('win_minmax provider selection differs from configuration')
    arch, owner = architecture(config), owner_path(build, selection)
    archive = build / 'lib/lib.a'
    ordered = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', archive).splitlines()]
    candidates = {owner_path(build, language).resolve() for language in ('C', 'Rust')}
    if [member for member in ordered if member in candidates] != [owner.resolve()]:
        raise ValueError('win_minmax archive owner absent, duplicated or mixed')
    index = ordered.index(owner.resolve())
    if index == 0 or index + 1 == len(ordered) or ordered[index-1].name != 'vsprintf.o' or ordered[index+1].name != 'xarray.o':
        raise ValueError('win_minmax original sorted lib/lib.a slot differs')
    linked = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', build / 'vmlinux.a').splitlines()]
    if [member for member in linked if member in candidates] != [owner.resolve()]:
        raise ValueError('vmlinux archive has absent, duplicate or mixed minmax owner')
    source = ROOT / {'C': 'lib/win_minmax.c', 'Rust': 'lib/win_minmax_rust.rs'}[selection]
    dependencies = [ROOT / name for name in ({'C': ['include/linux/win_minmax.h', 'include/linux/export.h'],
        'Rust': ['lib/win_minmax.rs', 'rust/ffi_export.rs', 'include/linux/export_header.rs', 'include/linux/win_minmax_header.rs']}[selection])]
    verify_build_command(build, owner, source, dependencies)
    if selection == 'Rust':
        verify_native_rust(owner, build)
        verify_build_command(build, owner, source, [build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta'])
        verify_bindings(build)
    elf_target(owner, arch)
    names = EXPORTS
    records = read_exports(owner)
    if len(records) != len(names) or {row['name'] for row in records} != set(names):
        raise ValueError('wrong selected win_minmax export set')
    for row in records:
        expected = dict(license='', namespace='', relocation_target=row['name'], relocation_addend=0,
                        pointer_width=8, relocation_kind=257 if arch == 'aarch64' else 1,
                        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        if any(row[key] != value for key, value in expected.items()):
            raise ValueError('incorrect unrestricted win_minmax export metadata/relocation')
    definitions = [line.split() for line in tool('nm', '-g', '--defined-only', owner).splitlines()
                   if line.split()[-1:] and line.split()[-1].startswith(b'minmax_')]
    if len(definitions) != len(names) or {row[-1] for row in definitions} != {name.encode() for name in names} or any(row[-2] != b'T' for row in definitions):
        raise ValueError('selected win_minmax symbols are not exactly the defining global functions')
    versions = selected_versions(build, names)
    if config.get('MODVERSIONS') == 'y' and version_records(owner.with_name('.' + owner.name + '.cmd')) != versions:
        raise ValueError('win_minmax CRC provenance does not match selected defining unit')
    verify_dwarf(build, owner, selection, versions)
    verify_final_symbols(build)
    if config.get('CFI') == 'y':
        flag = '-Zsanitizer=kcfi' if selection == 'Rust' else '-fsanitize=kcfi'
        if flag not in compilation_flags(owner): raise ValueError('win_minmax owner missing KCFI')
        provider_type_ids(owner, names=names)
    newer(archive, [owner])
    newer(build / 'Module.symvers', [owner])
    newer(build / 'vmlinux.a', [archive])
    newer(build / 'vmlinux.o', [build / 'vmlinux.a'])
    newer(build / 'vmlinux', [build / 'vmlinux.o'])
    if config.get('UML') != 'y':
        newer(build / ('arch/arm64/boot/Image' if arch == 'aarch64' else 'arch/x86/boot/bzImage'), [build / 'vmlinux'])
    return owner


def verify_consumer(build, work, caller):
    name, stem = ('win_minmax_rust_abi', 'win_minmax_rust_main') if caller == 'rust' else ('win_minmax_abi', 'win_minmax_c_main')
    obj, module = work / (stem + '.o'), work / (name + '.ko')
    config = configuration(build)
    arch, selection = architecture(config), provider(config)
    deps = ([build / 'rust/libkernel.rmeta', build / 'rust/libbindings.rmeta', ROOT / 'include/linux/win_minmax_header.rs'] if caller == 'rust' else [ROOT / 'include/linux/win_minmax.h'])
    source = obj.with_suffix('.rs' if caller == 'rust' else '.c')
    if source.read_text() != sources(caller): raise ValueError('win_minmax caller source changed')
    verify_build_command(work, obj, source, deps)
    oracle = work / 'win_minmax_reference.o'
    if oracle.with_suffix('.c').read_text() != reference_source(): raise ValueError('win_minmax original C oracle changed')
    verify_build_command(work, oracle, oracle.with_suffix('.c'), [ROOT / 'lib/win_minmax.c', ROOT / 'include/linux/win_minmax.h'])
    for path in (obj, module):
        elf_target(path, arch)
        verify_references(path, EXPORTS)
    if module_name(module) != name: raise ValueError('wrong win_minmax module identity')
    newer(module, [obj, oracle, selected_metadata(build, module)])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_module_import_versions(build, module)
    require_metadata_field(metadata_fields(module), b'license', b'Proprietary')
    require_metadata_field(metadata_fields(module), b'description', DESCRIPTION)
    if read_exports(module): raise ValueError('private win_minmax caller unexpectedly exports symbols')
    undefined = {line.split()[-1] for line in tool('nm', '-u', module).splitlines() if line.split()}
    rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()]
    if any(len(row) >= 4 and row[1] in undefined and row[3] != b'EXPORT_SYMBOL' for row in rows):
        raise ValueError('Proprietary win_minmax caller imports a GPL-only symbol')
    if caller == 'rust':
        verify_native_rust(obj, build)
        verify_bindings(build)
        verify_independent_imports(obj)
        for path in (obj, module): verify_rust_entrypoints(path, arch)
    for path in (oracle, *([obj] if caller == 'c' else [])):
        flags = compilation_flags(path)
        if '-D__DISABLE_EXPORTS' not in flags or '-U__DISABLE_EXPORTS' in flags or read_exports(path):
            raise ValueError('private win_minmax C fixture retains incidental export metadata')
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    if config.get('CFI') == 'y':
        flags = compilation_flags(obj)
        required = '-Zsanitizer=kcfi' if caller == 'rust' else '-fsanitize=kcfi'
        if required not in flags or any(flag.startswith('-fno-sanitize=') and {'all', 'kcfi'} & set(flag.split('=', 1)[1].split(',')) for flag in flags):
            raise ValueError('win_minmax caller missing native KCFI compilation')
        types = provider_type_ids(owner_path(build, selection), names=EXPORTS)
        for path in (obj, module): verify_guarded_calls(path, arch, types, wrappers=WRAPPERS)



def result_marker(caller):
    if caller not in ('c','rust'): raise ValueError('unknown caller')
    return (b'LUPOS_WIN_MINMAX_' + (b'RUST_' if caller=='rust' else b'') +
            f'ABI_OK checks={COUNT} exports=2 fields=6'.encode())

def verify_console(console, caller, *, reload=False, config=None):
    if re.search(rb'CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version', console, re.I):
        raise ValueError('unexpected win_minmax kernel fault or failed test')
    console = normalize_console_transport(console)
    lines = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip()).strip()
             for line in console.splitlines()]
    allowed = set()
    if config and config.get('INT_LOG_KUNIT_TEST') == 'y':
        ranges = kunit_runs(console, 1)
        allowed = set(verify_kunit_warnings(lines, ranges, 'Rust' if config.get('RUST_INT_LOG') == 'y' else 'C', config.get('BUG') == 'y'))
    if {i for i, line in enumerate(lines) if b'WARNING:' in line} != allowed:
        raise ValueError('warning outside configured original integer-log zero cases')
    marker = result_marker(caller)
    expected = [marker, b'LUPOS_RUST_MODULE_LOAD_OK']
    if reload: expected += [b'LUPOS_RUST_MODULE_UNLOAD_OK 0', marker, b'LUPOS_RUST_MODULE_RELOAD_OK 0']
    expected.append(b'LUPOS_RUST_BUILD_BOOT_OK')
    actual = [re.sub(rb'^win_minmax_(?:rust_)?abi: ', b'', line) for line in lines if b'LUPOS_' in line]
    if actual != expected: raise ValueError('win_minmax workload counts, caller or load/reload events differ')
    verify_module_events(console, module=True, preloads=0, reload=reload)
    return 2 if reload else 1


def executable_command(value, name):
    try:
        command = shlex.split(value)
    except ValueError as error:
        raise ValueError('malformed ' + name + ' command') from error
    if not command or not command[0]: raise ValueError('empty ' + name + ' command')
    if any('\0' in word for word in command): raise ValueError(name + ' command contains NUL')
    executable = shutil.which(command[0])
    if executable is None: raise ValueError('invalid ' + name + ' executable: ' + command[0])
    command[0] = os.path.abspath(executable)
    # Commands run in an isolated cwd, so anchor explicit firmware inputs to
    # the invocation cwd without splitting or interpreting other arguments.
    if name == 'QEMU':
        for index, word in enumerate(command):
            if word in ('-bios', '-L'):
                if index + 1 == len(command) or not command[index + 1]:
                    raise ValueError('missing QEMU firmware path')
                path = Path(command[index + 1]).resolve()
                if not (path.is_file() if word == '-bios' else path.is_dir()):
                    raise ValueError('invalid QEMU firmware path: ' + str(path))
                command[index + 1] = str(path)
    return command


def run_logged(command, log, env):
    """Keep failed launches and outputs in a unique invocation directory."""
    command = list(map(str, command))
    scratch = Path(tempfile.mkdtemp(prefix='command-', dir=log.parent))
    env = dict(env, TMPDIR=str(scratch), TEMP=str(scratch), TMP=str(scratch))
    log.write_text(shlex.join(command) + '\ncwd=' + str(scratch) + '\n')
    try:
        result = subprocess.run(command, cwd=scratch, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    except OSError as error:
        with log.open('ab') as output: output.write(('launch-error=' + repr(error) + '\n').encode())
        raise RuntimeError('command failed to start; preserved log: ' + str(log)) from error
    with log.open('ab') as output:
        output.write(result.stdout + f'\nexit={result.returncode}\n'.encode())
    if result.returncode: raise RuntimeError('command failed; preserved log: ' + str(log))


def main():
    global ROOT
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build')
    parser.add_argument('--source-root', default=str(ROOT))
    parser.add_argument('--caller', choices=('c', 'rust'), default='c')
    parser.add_argument('--allow-c-baseline', action='store_true')
    parser.add_argument('--audit-only', action='store_true')
    parser.add_argument('--reload-modules', action='store_true')
    parser.add_argument('--make-arg', action='append', default=[])
    parser.add_argument('--qemu', default=os.environ.get('QEMU'))
    parser.add_argument('--qemu-data')
    args = parser.parse_args()
    try:
        if not args.build.strip() or not args.source_root.strip(): raise ValueError('empty explicit path')
        ROOT = Path(args.source_root).resolve()
        build = Path(args.build).resolve()
        if not (ROOT / 'lib/win_minmax.c').is_file(): raise ValueError('invalid source root')
        if not build.is_dir(): raise ValueError('invalid build directory')
        if args.qemu is not None: args.qemu = shlex.join(executable_command(args.qemu, 'QEMU'))
        make_command = executable_command(os.environ.get('MAKE', 'make'), 'MAKE')
        if args.qemu_data is not None and (not args.qemu_data.strip() or not Path(args.qemu_data).is_dir()):
            raise ValueError('invalid qemu data directory')
        if args.qemu_data is not None: args.qemu_data = Path(args.qemu_data).resolve()
        if any(not arg.strip() or '\0' in arg for arg in args.make_arg): raise ValueError('invalid make argument')
        config = configuration(build)
        selection = provider(config)
        if selection != 'Rust' and not args.allow_c_baseline: raise ValueError('requires RUST_WIN_MINMAX=y or explicit --allow-c-baseline')
        if not args.audit_only:
            if args.qemu is None:
                emulator = 'qemu-system-aarch64' if architecture(config) == 'aarch64' else 'qemu-system-x86_64'
                args.qemu = shlex.join(executable_command(emulator, 'QEMU'))
            if config.get('UML') == 'y': raise ValueError('generic UML recognized, but the system-QEMU runner cannot boot UML')
            for name in ('MODULES', 'PRINTK', 'MULTIUSER', 'CFI'):
                if config.get(name) != 'y': raise ValueError('requires CONFIG_' + name + '=y')
            if args.caller == 'rust' and config.get('RUST') != 'y': raise ValueError('Rust caller requires CONFIG_RUST=y')
            if args.reload_modules and config.get('MODULE_UNLOAD') != 'y': raise ValueError('reload requires MODULE_UNLOAD=y')
            if config.get('MODULE_SIG_FORCE') == 'y': raise ValueError('disposable unsigned modules cannot satisfy forced signing')
        owner = verify_linked_implementation(build, selection)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    if args.audit_only:
        print(f'{selection} win_minmax: selected owner {owner}; artifact-only audit, no runtime claim.')
        return
    arch = architecture(config)
    parent = build / 'rust-win_minmax-test'
    parent.mkdir(exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix=args.caller + '-', dir=parent))
    print('win_minmax module evidence:', work, flush=True)
    name, stem = ('win_minmax_rust_abi', 'win_minmax_rust_main') if args.caller == 'rust' else ('win_minmax_abi', 'win_minmax_c_main')
    (work / 'win_minmax_reference.c').write_text(reference_source())
    (work / (stem + ('.rs' if args.caller == 'rust' else '.c'))).write_text(sources(args.caller))
    (work / 'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o win_minmax_reference.o\n')
    env = {key: value for key, value in os.environ.items() if not key.startswith(('CONFIG_', 'KCONFIG_', 'KBUILD_')) and key not in
           ('MAKEFLAGS', 'MFLAGS', 'CARGO_MAKEFLAGS', 'MAKELEVEL', 'MAKEOVERRIDES', 'srctree', 'srcroot', 'objtree', 'VPATH', 'sub_make_done')}
    run_logged([*make_command, '-C', str(ROOT), 'O=' + str(build), 'M=' + str(work),
                    *args.make_arg, PRIVATE_C_FLAGS, 'modules'], work / 'build.log', env)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / 'scripts/tests/boot_kernel.py'), '--build', str(build), '--arch', arch,
               '--module', str(work / (name + '.ko')), '--qemu', args.qemu]
    if args.reload_modules: command += ['--reload-modules']
    if args.qemu_data: command += ['--qemu-data', str(args.qemu_data)]
    try:
        run_logged(command, work / 'boot.log', env)
    finally:
        console_log = build / 'rust-boot-test/console.log'
        if console_log.is_file(): shutil.copyfile(console_log, work / 'console.log')
    runs = verify_console((work / 'console.log').read_bytes(), args.caller, reload=args.reload_modules, config=config)
    print(f'{selection} win_minmax: independent {args.caller} caller passed {COUNT} checks/load; {runs} loads on {arch}.')


if __name__ == '__main__': main()
