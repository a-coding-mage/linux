# SPDX-License-Identifier: GPL-2.0-only
"""Independent native callers for the original bsearch differential corpus."""
from pathlib import Path

EXPORTS = ('bsearch',)
CASES = 1 + 258 * 90 + 6 + 1
DESCRIPTION = 'Independent selected bsearch differential and nullable ABI'


def marker(caller):
    if caller not in ('c', 'rust'):
        raise ValueError('unknown bsearch caller')
    return f'LUPOS_BSEARCH_{caller.upper()}_ABI_OK cases={CASES} exports=1'.encode()


def reference_source(root):
    return ('#include <linux/export.h>\n#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(x)\n'
            '#define bsearch original_bsearch\n#include "' + str(root / 'lib/bsearch.c') + '"\n')


def c_source(root):
    source = (root / 'scripts/tests/fixtures/bsearch_driver.c').read_text()
    start = source.index('static const void *trace[128];')
    end = source.index('int main(int argc, char **argv)')
    corpus = source[start:end]
    bad_start = corpus.index('static unsigned int bad_compare')
    bad_end = corpus.index('static int check(', bad_start)
    corpus = corpus[:bad_start] + corpus[bad_end:]
    corpus = corpus.replace('bsearch, rust_consumer', 'bsearch_call_bsearch, __inline_bsearch')
    start = source.index('    if (check(NULL, NULL, 0, (size_t)-1, NULL))')
    cases = source[start:]
    header = '''#include <linux/module.h>
#include <linux/bsearch.h>
extern void *original_bsearch(const void *, const void *, size_t, size_t, cmp_func_t);
static noinline void *bsearch_call_bsearch(const void *key, const void *base,
        size_t num, size_t size, cmp_func_t cmp)
{
    void *(*volatile selected)(const void *, const void *, size_t, size_t, cmp_func_t) = bsearch;
    return selected(key, base, num, size, cmp);
}
'''
    # Match the original host corpus; static storage avoids oversized kernel
    # stack frames. The independent Rust caller also leaves padding uninitialized.
    setup = '''static int exercise(void)
{
    static struct item values[257];
    for (int i = 0; i < 257; ++i) { values[i].tag = 1; values[i].value = i / 3 - 20; values[i].tail = 7; }
    int key = 0;
'''
    return (header + corpus + setup + cases +
            '\nstatic int __init bsearch_abi_init(void) { int error = exercise();\n'
            ' if (error) { pr_err("LUPOS_BSEARCH_FAIL error=%d\\n", error); return -EINVAL; }\n'
            ' pr_info("' + marker('c').decode() + '\\n"); return 0; }\n'
            'static void __exit bsearch_abi_exit(void) {}\n'
            'module_init(bsearch_abi_init); module_exit(bsearch_abi_exit);\n'
            'MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("' + DESCRIPTION + '");\n')


RUST_WORKLOAD = r'''
use core::mem::MaybeUninit;
use core::ptr::{addr_of, addr_of_mut, null};
use kernel::{bindings, ffi};
use bindings::BsearchCmp;
use ffi::c_void;
unsafe extern "C" {
    fn original_bsearch(key: *const c_void, base: *const c_void, num: usize,
        size: usize, cmp: BsearchCmp) -> *mut c_void;
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Item { tag: u8, value: ffi::c_long, tail: u16 }
struct State { values: [MaybeUninit<Item>; 257], trace: [*const c_void; 128],
    count: usize, direction: i32, cases: usize }
static mut STATE: State = State { values: [MaybeUninit::uninit(); 257],
    trace: [null(); 128], count: 0, direction: 0, cases: 0 };
fn state() -> *mut State { addr_of_mut!(STATE) }
unsafe extern "C" fn compare(key: *const c_void, value: *const c_void) -> i32 {
    unsafe {
        let s = state(); let count = (*s).count; assert!(count < 128);
        (*s).trace[count] = value; (*s).count += 1;
        let key = *key.cast::<i32>() as ffi::c_long;
        let value = (*value.cast::<Item>()).value;
        i32::from(key > value) - i32::from(key < value)
    }
}
unsafe extern "C" fn fixed(_key: *const c_void, value: *const c_void) -> i32 {
    unsafe {
        let s = state(); let count = (*s).count; assert!(count < 128);
        (*s).trace[count] = value; (*s).count += 1; (*s).direction
    }
}
/// Protected call through the actual selected public binding.
///
/// # Safety
/// The caller satisfies the original C pointer and comparator contract.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn bsearch_call_bsearch(key: *const c_void, base: *const c_void,
    num: usize, size: usize, cmp: BsearchCmp) -> *mut c_void {
    let selected: unsafe extern "C" fn(_, _, _, _, _) -> _ = bindings::bsearch;
    unsafe { core::ptr::read_volatile(&selected)(key, base, num, size, cmp) }
}
unsafe fn check(key: *const c_void, base: *const c_void, num: usize,
    size: usize, cmp: BsearchCmp) -> bool {
    unsafe {
        let s = state(); (*s).count = 0;
        let want = original_bsearch(key, base, num, size, cmp);
        let total = (*s).count; let expected = (*s).trace;
        (*s).count = 0;
        if bsearch_call_bsearch(key, base, num, size, cmp) != want || (*s).count != total { return false; }
        for (i, pointer) in expected.iter().enumerate().take(total) {
            if (*s).trace[i] != *pointer { return false; }
        }
        (*s).count = 0;
        if header::__inline_bsearch(key, base, num, size, cmp) != want || (*s).count != total { return false; }
        for (i, pointer) in expected.iter().enumerate().take(total) {
            if (*s).trace[i] != *pointer { return false; }
        }
        (*s).cases += 1; true
    }
}
/// Rust owns all cases, state, return-pointer checks and comparator traces.
///
/// # Safety
/// Only the serialized module initialization invokes the private workload.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn bsearch_rust_exercise() -> ffi::c_int {
    unsafe {
        let s = state(); (*s).cases = 0;
        let values = addr_of_mut!((*s).values).cast::<Item>();
        for i in 0..257 {
            addr_of_mut!((*values.add(i)).tag).write(1);
            addr_of_mut!((*values.add(i)).value).write((i / 3) as ffi::c_long - 20);
            addr_of_mut!((*values.add(i)).tail).write(7);
        }
        if !check(null(), null(), 0, usize::MAX, BsearchCmp::from_option(None)) { return 12; }
        for n in 0..=257 {
            for key in -22i32..68 {
                if !check(addr_of!(key).cast(), values.cast(), n,
                    core::mem::size_of::<Item>(), BsearchCmp::from_option(Some(compare))) { return 13; }
            }
        }
        for direction in -1..=1 {
            (*s).direction = direction;
            for n in [257, usize::MAX] {
                if !check(values.cast(), values.cast(), n, 0,
                    BsearchCmp::from_option(Some(fixed))) { return 14; }
            }
        }
        (*s).direction = 0;
        if !check(values.cast(), values.cast(), 1usize << (usize::BITS - 1), 4,
            BsearchCmp::from_option(Some(fixed))) { return 16; }
        if (*s).cases != @CASES@ { return 17; }
        0
    }
}
/// Initialize the independent fixture and announce success only after all checks.
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    let error = unsafe { bsearch_rust_exercise() };
    if error != 0 { return -22; }
    unsafe { bindings::_printk(c"\x016@MARKER@\n".as_ptr().cast()); } 0
}
/// Fixture storage is private; initialization retains no kernel resources.
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn() -> ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=GPL\0description=@DESCRIPTION@\0";
#[used]
#[link_section = ".modinfo"]
static MODINFO: [u8; INFO.len()] = { let mut bytes = [0; INFO.len()]; let mut i = 0;
    while i < bytes.len() { bytes[i] = INFO.as_bytes()[i]; i += 1; } bytes };
#[used]
static __IS_RUST_MODULE: () = ();
'''


def caller_source(root, caller):
    marker(caller)
    if caller == 'c':
        return c_source(root)
    return ('//! Independent Rust bsearch workload using genuine native bindings.\n'
            '/// Canonical translated public header.\n#[path="' +
            str(root / 'include/linux/bsearch_header.rs') + '"] pub mod header;\n' +
            RUST_WORKLOAD.replace('@CASES@', str(CASES)).replace('@MARKER@', marker(caller).decode())
            .replace('@DESCRIPTION@', DESCRIPTION))
